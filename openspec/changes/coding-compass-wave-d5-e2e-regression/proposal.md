## Why

Wave D.1–D.4 ya viven en `/concepts/:id` (heatmap, drawer, facetas AND,
DAG de prerrequisitos). El blindaje E2E sigue fragmentado en smokes de
partición y el journey Wave A mockea Pyodide / no cubre el hub D. Un
regresión silenciosa del mapa conceptual no rompe `journey.auth-hub`.
Este slice nombra el journey de página y lo engancha al harness y a los
6 shards de CI **sin** re-shard ni `trunk serve`.

## What Changes

**Journey Playwright del hub conceptual** (Wave D.5):

1. Spec `web/e2e/tests/journey.concepts-hub.spec.ts`: register →
   `/concepts/1` con el índice real (sin mock de tags).
2. Asserts D.1–D.4: 100 celdas `#concept-heatmap`, drawer de década,
   filtros AND que actualizan lista/heatmap, `#concept-prereq-alert`
   para usuario nuevo.
3. Enganche a `make harness-journeys` y a la suite shardeada existente.
   CI sigue `trunk build` + `STATIC_DIR` (no `trunk serve`).

## Capabilities

### New Capabilities

- _(ninguna — el hub ya está especificado; este change añade el journey)_

### Modified Capabilities

- `coding-conceptual-partitions`: el producto SHALL ship un journey
  Playwright nombrado por página para el hub `/concepts/1` que cubre
  heatmap, drawer, facetas AND y alerta de base faltante, sin mockear
  `STEP_PARTITIONS`.

## Alcance incluido

- `journey.concepts-hub.spec.ts` + hook en `scripts/harness/run.sh`
  (`run_web_journeys`) y docs ADR 003 (`docs/testing/journeys.md`).
- Conservar 6 shards y el agregador `Playwright Chromium smoke`.
- `make web-test` local. Playwright = GitHub Actions (no harness
  completo en la laptop).

## Fuera de alcance

- Docker, GCP Cloud Run, compose, load tests.
- Re-shard (`--shard=N/6` ya existe). `trunk serve` en CI.
- Retaguear `STEP_PARTITIONS` / `1..=1000`.
- Mock de tags o de Pyodide en este journey.
- Producto UI/Go nuevo (solo blindaje E2E).

## Impact

- `web/e2e/tests/journey.concepts-hub.spec.ts`, harness journeys,
  `docs/testing/journeys.md` / `TESTING.md` / ADR 003.
- Base = `origin/main` @ `a6811ac` (Wave D.4 #250).
- Python del alumno sigue solo en Pyodide (ADR 002).

## Plan de rollback

Revertir este PR. Los smokes `concepts.partitions.spec.ts` y el journey
Wave A siguen. El hub D.4 no cambia.
