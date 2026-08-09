# PPI — Makefile (optimizado para macOS Apple Silicon / ARM64)
# Uso típico desde la raíz del monorepo:
#   make toolchain   # asegura Go 1.25
#   make test
#   make fmt
#   make build

SHELL := /bin/zsh
.SHELLFLAGS := -euo pipefail -c

BACKEND_DIR := backend
WEB_DIR     := web
BIN_DIR     := $(BACKEND_DIR)/bin
BIN_NAME    := ppi-api
MODULE      := github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend

# Detecta host; en Apple Silicon fuerza arm64 nativo.
UNAME_S := $(shell uname -s)
UNAME_M := $(shell uname -m)

ifeq ($(UNAME_S),Darwin)
  export GOOS ?= darwin
  ifeq ($(UNAME_M),arm64)
    export GOARCH ?= arm64
  else
    export GOARCH ?= amd64
  endif
else
  export GOOS ?= $(shell go env GOOS 2>/dev/null || echo linux)
  export GOARCH ?= $(shell go env GOARCH 2>/dev/null || echo amd64)
endif

export GOTOOLCHAIN ?= auto
export CGO_ENABLED ?= 0

GO ?= go
GOFLAGS ?=

.PHONY: help toolchain fmt vet test build run clean openspec-validate web-test web-build web-e2e \
	harness harness-unit harness-integration harness-e2e ci dev-set-password

help: ## Muestra targets disponibles
	@awk 'BEGIN {FS = ":.*##"; printf "\nTargets PPI:\n"} /^[a-zA-Z0-9_-]+:.*?##/ { printf "  %-18s %s\n", $$1, $$2 }' $(MAKEFILE_LIST)

toolchain: ## Instala/activa Go 1.25 (macOS ARM64 o via GOTOOLCHAIN)
	@if [ "$(UNAME_S)" = "Darwin" ]; then \
	  ./scripts/setup-go-macos.sh; \
	else \
	  echo ">> Entorno no-Darwin: usando GOTOOLCHAIN=auto con go.mod (1.25.0)"; \
	  cd $(BACKEND_DIR) && $(GO) version; \
	fi

fmt: ## gofmt + go mod tidy
	@cd $(BACKEND_DIR) && $(GO) fmt ./...
	@cd $(BACKEND_DIR) && $(GO) mod tidy

vet: ## go vet
	@cd $(BACKEND_DIR) && $(GO) vet ./...

test: ## Ejecuta tests unitarios del backend
	@cd $(BACKEND_DIR) && $(GO) test $(GOFLAGS) ./...

build: ## Compila el binario del API (backend/bin/ppi-api)
	@mkdir -p $(BIN_DIR)
	@cd $(BACKEND_DIR) && \
	  $(GO) build $(GOFLAGS) -trimpath -ldflags="-s -w" \
	    -o bin/$(BIN_NAME) .
	@echo ">> built $(BIN_DIR)/$(BIN_NAME) ($(GOOS)/$(GOARCH))"

run: build ## Compila y levanta el API en :8080
	@cd $(BACKEND_DIR) && ./bin/$(BIN_NAME)

dev-set-password: ## Reset local password: make dev-set-password EMAIL=a@b.com PASSWORD=secreto12 [DB=sqlite://./data/ppi.db]
	@test -n "$(EMAIL)" || (echo "EMAIL requerido" >&2; exit 2)
	@test -n "$(PASSWORD)" || (echo "PASSWORD requerido" >&2; exit 2)
	@# Always pass -db so ambient DATABASE_URL (p.ej. harness) no redirige por accidente.
	@cd $(BACKEND_DIR) && $(GO) run ./cmd/ppi-authctl set-password \
	  -email="$(EMAIL)" -password="$(PASSWORD)" \
	  -db="$(if $(DB),$(DB),sqlite://./data/ppi.db)"

clean: ## Limpia binarios Go y artefactos web (dist/target)
	@rm -rf $(BIN_DIR)
	@rm -rf $(WEB_DIR)/dist $(WEB_DIR)/target

web-test: ## Tests unitarios del shell Leptos (aislado del Go)
	@cd $(WEB_DIR) && cargo test

web-build: ## Build Wasm release del shell Leptos via Trunk
	@# Trunk 0.21 trata NO_COLOR=1 como flag clap inválido; limpiar en el target.
	@cd $(WEB_DIR) && env -u NO_COLOR trunk build --release
	@echo ">> built $(WEB_DIR)/dist (wasm)"

web-e2e: ## Smoke Playwright auth (requiere PPI_E2E_EMAIL/PASSWORD + API :8080 + Trunk :3001)
	@cd $(WEB_DIR)/e2e && npm test

harness: ## Harness integral (unit + integration opt-in + e2e con stack) — ver TESTING.md
	@chmod +x scripts/harness/run.sh
	@./scripts/harness/run.sh all

harness-unit: ## Solo backend-unit + web-unit con reporte por módulo
	@chmod +x scripts/harness/run.sh
	@./scripts/harness/run.sh unit

harness-integration: ## Go integration tags (PPI_HARNESS_INTEGRATION=1)
	@chmod +x scripts/harness/run.sh
	@PPI_HARNESS_INTEGRATION=1 ./scripts/harness/run.sh integration

harness-e2e: ## Levanta stack efímero + Playwright + teardown
	@chmod +x scripts/harness/run.sh
	@./scripts/harness/run.sh e2e

openspec-validate: ## Valida el change PPI 1.1
	@openspec validate ppi-1-1-foundations --no-interactive

ci: fmt vet test build openspec-validate ## Pipeline local rápido (Go; web/harness son opt-in)
	@echo ">> CI local OK — module $(MODULE)"
