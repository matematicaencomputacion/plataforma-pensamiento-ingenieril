#!/usr/bin/env bash
# PPI test harness — orchestrates unit / integration / e2e with per-module reports.
# Invoked via `make harness*` from the monorepo root.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

REPORT_DIR="${PPI_HARNESS_REPORT_DIR:-$ROOT/artifacts/harness}"
STAMP="$(date -u +%Y%m%dT%H%M%SZ)"
RUN_DIR="$REPORT_DIR/$STAMP"
mkdir -p "$RUN_DIR"

API_PID=""
TRUNK_PID=""
MODULES=()
RESULTS=()

log() { printf '%s\n' "$*"; }
hr() { printf '%s\n' "────────────────────────────────────────"; }

cleanup() {
  local code=$?
  if [[ -n "${TRUNK_PID}" ]] && kill -0 "$TRUNK_PID" 2>/dev/null; then
    kill "$TRUNK_PID" 2>/dev/null || true
    wait "$TRUNK_PID" 2>/dev/null || true
  fi
  if [[ -n "${API_PID}" ]] && kill -0 "$API_PID" 2>/dev/null; then
    kill "$API_PID" 2>/dev/null || true
    wait "$API_PID" 2>/dev/null || true
  fi
  # Soft cleanup: do not wipe REPORT_DIR (artifacts are the point).
  exit "$code"
}
trap cleanup EXIT INT TERM

record() {
  local name="$1" status="$2"
  MODULES+=("$name")
  RESULTS+=("$status")
  printf '%s\t%s\n' "$name" "$status" >>"$RUN_DIR/summary.tsv"
  log ">> module ${name}: ${status}"
}

run_backend_unit() {
  hr
  log "MODULE backend-unit"
  if (cd backend && go test ./... -count=1 | tee "$RUN_DIR/backend-unit.log"); then
    record "backend-unit" "PASS"
  else
    record "backend-unit" "FAIL"
    return 1
  fi
}

run_web_unit() {
  hr
  log "MODULE web-unit"
  if (cd web && cargo test -- --nocapture | tee "$RUN_DIR/web-unit.log"); then
    record "web-unit" "PASS"
  else
    record "web-unit" "FAIL"
    return 1
  fi
}

run_backend_integration() {
  hr
  log "MODULE backend-integration (tag: integration)"
  # Skeletons use //go:build integration — skipped unless PPI_HARNESS_INTEGRATION=1
  if [[ "${PPI_HARNESS_INTEGRATION:-0}" != "1" ]]; then
    log "(skip) set PPI_HARNESS_INTEGRATION=1 to enable"
    (cd backend && go test -tags=integration ./internal/integration/... -count=1 -list . \
      >"$RUN_DIR/backend-integration-list.log" 2>&1 || true)
    record "backend-integration" "SKIP"
    return 0
  fi
  export JWT_SECRET="${JWT_SECRET:-harness-jwt-secret}"
  export DATABASE_URL="${DATABASE_URL:-sqlite://./data/ppi-harness.db}"
  export LEARNER_PROFILE_LLM="${LEARNER_PROFILE_LLM:-mock}"
  mkdir -p backend/data
  if (cd backend && go test -tags=integration ./internal/integration/... -count=1 \
      | tee "$RUN_DIR/backend-integration.log"); then
    record "backend-integration" "PASS"
  else
    record "backend-integration" "FAIL"
    return 1
  fi
}

wait_http() {
  local url="$1" tries="${2:-60}"
  local i
  for i in $(seq 1 "$tries"); do
    if curl -fsS "$url" >/dev/null 2>&1; then
      return 0
    fi
    sleep 1
  done
  return 1
}

# Pin Playwright browser cache to a stable host path (Cursor/agent sandboxes
# otherwise redirect installs into ephemeral dirs and `npx playwright test` fails
# with "Executable doesn't exist").
ensure_playwright_browsers() {
  local stable
  case "$(uname -s)" in
    Darwin)
      # Prefer real macOS home — $HOME may be remapped inside agent sandboxes.
      stable="/Users/$(whoami)/Library/Caches/ms-playwright"
      ;;
    *)
      stable="${XDG_CACHE_HOME:-/home/$(whoami)/.cache}/ms-playwright"
      ;;
  esac

  # Override ephemeral / pre-injected sandbox caches unless the operator opts out.
  if [[ "${PPI_KEEP_PLAYWRIGHT_BROWSERS_PATH:-0}" != "1" ]]; then
    case "${PLAYWRIGHT_BROWSERS_PATH:-}" in
      *cursor-sandbox-cache*|*"/var/folders/"*|*"/tmp/"*|*"Temp"*|"")
        export PLAYWRIGHT_BROWSERS_PATH="$stable"
        ;;
    esac
  fi
  if [[ -z "${PLAYWRIGHT_BROWSERS_PATH:-}" ]]; then
    export PLAYWRIGHT_BROWSERS_PATH="$stable"
  fi

  mkdir -p "$PLAYWRIGHT_BROWSERS_PATH"
  log "PLAYWRIGHT_BROWSERS_PATH=$PLAYWRIGHT_BROWSERS_PATH"

  if [[ ! -d web/e2e/node_modules ]]; then
    (cd web/e2e && npm ci)
  fi

  # Idempotent: installs only what is missing for the locked Playwright version.
  if ! (cd web/e2e && npx playwright install chromium); then
    log "Failed to install Playwright Chromium into $PLAYWRIGHT_BROWSERS_PATH"
    return 1
  fi
}

start_stack() {
  hr
  log "Starting stack (API :8080, Trunk :3001)"
  mkdir -p backend/data artifacts/harness
  export JWT_SECRET="${JWT_SECRET:-harness-jwt-secret}"
  export DATABASE_URL="${DATABASE_URL:-sqlite://./data/ppi-harness.db}"
  export LEARNER_PROFILE_LLM="${LEARNER_PROFILE_LLM:-mock}"
  export PPI_EXPOSE_RESET_TOKEN="${PPI_EXPOSE_RESET_TOKEN:-1}"

  (cd backend && go build -o bin/ppi-api .)
  (cd backend && ./bin/ppi-api >"$RUN_DIR/api.log" 2>&1) &
  API_PID=$!
  if ! wait_http "http://127.0.0.1:8080/api/health" 45; then
    log "API failed to become healthy"
    cat "$RUN_DIR/api.log" || true
    record "stack-api" "FAIL"
    return 1
  fi
  record "stack-api" "PASS"

  unset NO_COLOR || true
  (cd web && env -u NO_COLOR trunk serve --port 3001 --address 127.0.0.1 \
      >"$RUN_DIR/trunk.log" 2>&1) &
  TRUNK_PID=$!
  if ! wait_http "http://127.0.0.1:3001/" 120; then
    log "Trunk failed to become healthy"
    cat "$RUN_DIR/trunk.log" || true
    record "stack-trunk" "FAIL"
    return 1
  fi
  record "stack-trunk" "PASS"
  # Warm the Wasm shell so the first Playwright test is not racing hydration.
  curl -fsS "http://127.0.0.1:3001/" >/dev/null || true
  sleep 2
}

run_web_e2e() {
  hr
  log "MODULE web-e2e"
  export PPI_E2E_BASE_URL="${PPI_E2E_BASE_URL:-http://127.0.0.1:3001}"
  # Login smoke seeds via API; MODE is retained for older specs / docs.
  export PPI_E2E_MODE="${PPI_E2E_MODE:-login}"
  export PPI_E2E_PASSWORD="${PPI_E2E_PASSWORD:-secreto12}"
  export PPI_EXPOSE_RESET_TOKEN="${PPI_EXPOSE_RESET_TOKEN:-1}"
  # Optional sticky email — auth.login.spec uniquifies with a +tag on each run.
  : "${PPI_E2E_EMAIL:=}"

  ensure_playwright_browsers || {
    record "web-e2e" "FAIL"
    return 1
  }

  if (cd web/e2e && npx playwright test --reporter=list \
      | tee "$RUN_DIR/web-e2e.log"); then
    [[ -d web/e2e/playwright-report ]] && cp -R web/e2e/playwright-report "$RUN_DIR/" || true
    record "web-e2e" "PASS"
  else
    [[ -d web/e2e/playwright-report ]] && cp -R web/e2e/playwright-report "$RUN_DIR/" || true
    [[ -d web/e2e/test-results ]] && cp -R web/e2e/test-results "$RUN_DIR/" || true
    record "web-e2e" "FAIL"
    return 1
  fi
}

# ADR 003: page journeys Auth+Hub (subset of playwright pack).
run_web_journeys() {
  hr
  log "MODULE web-journeys (ADR 003)"
  export PPI_E2E_BASE_URL="${PPI_E2E_BASE_URL:-http://127.0.0.1:3001}"
  export PPI_E2E_PASSWORD="${PPI_E2E_PASSWORD:-secreto12}"
  export PPI_EXPOSE_RESET_TOKEN="${PPI_EXPOSE_RESET_TOKEN:-1}"
  : "${PPI_E2E_EMAIL:=}"

  ensure_playwright_browsers || {
    record "web-journeys" "FAIL"
    return 1
  }

  if (cd web/e2e && npx playwright test \
      tests/journey.auth-hub.spec.ts \
      tests/auth.validation.spec.ts \
      tests/session.navigation.spec.ts \
      --reporter=list | tee "$RUN_DIR/web-journeys.log"); then
    [[ -d web/e2e/playwright-report ]] && cp -R web/e2e/playwright-report "$RUN_DIR/" || true
    record "web-journeys" "PASS"
  else
    [[ -d web/e2e/playwright-report ]] && cp -R web/e2e/playwright-report "$RUN_DIR/" || true
    [[ -d web/e2e/test-results ]] && cp -R web/e2e/test-results "$RUN_DIR/" || true
    record "web-journeys" "FAIL"
    return 1
  fi
}

print_summary() {
  hr
  log "HARNESS SUMMARY  ($RUN_DIR)"
  local i fail=0
  for i in "${!MODULES[@]}"; do
    printf '  %-24s %s\n' "${MODULES[$i]}" "${RESULTS[$i]}"
    if [[ "${RESULTS[$i]}" == "FAIL" ]]; then
      fail=1
    fi
  done
  if [[ "$fail" -eq 0 ]]; then
    log "RESULT: PASS"
    return 0
  fi
  log "RESULT: FAIL"
  return 1
}

MODE="${1:-all}"

case "$MODE" in
  unit)
    run_backend_unit
    run_web_unit || true
    print_summary
    ;;
  integration)
    run_backend_integration
    print_summary
    ;;
  e2e)
    start_stack
    run_web_e2e || true
    print_summary
    ;;
  journeys)
    start_stack
    run_web_journeys || true
    print_summary
    ;;
  all)
    fail=0
    run_backend_unit || fail=1
    run_web_unit || fail=1
    run_backend_integration || fail=1
    if [[ "${PPI_HARNESS_SKIP_E2E:-0}" == "1" ]]; then
      record "web-e2e" "SKIP"
    else
      start_stack || fail=1
      if [[ "$fail" -eq 0 ]]; then
        run_web_e2e || fail=1
      else
        record "web-e2e" "SKIP"
      fi
    fi
    print_summary || exit 1
    exit "$fail"
    ;;
  *)
    log "Usage: $0 {unit|integration|e2e|journeys|all}"
    exit 2
    ;;
esac
