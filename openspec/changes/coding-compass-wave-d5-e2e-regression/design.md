## Context

`origin/main` @ `a6811ac` (Wave D.4 #250) dejó heatmap, drawer de
década, facetas AND y `#concept-prereq-alert` en `/concepts/:id`.
CI E2E ya corre 6 shards con `trunk build` + `STATIC_DIR` (Go sirve
el SPA; `trunk serve` paniquea notify-rs en GHA). `make harness-journeys`
lista specs a mano: `journey.auth-hub`, `journey.concepts` (Wave A,
Pyodide mock), drawer, validation, session. No hay journey nombrado
del hub D.

## Goals / Non-Goals

**Goals**

- Journey `journey.concepts-hub.spec.ts` sobre el catálogo real
  (cero `page.route` / mock de `STEP_PARTITIONS`).
- Camino: register → login → `#partition-nav-1` → `/concepts/1`.
- Contratos D.1/D.2/D.4 ya shippeados: 100 celdas, drawer al clic
  no-`empty`, chip AND actualiza lista+heatmap, alerta de base
  faltante para usuario nuevo.
- Enganchar el spec a `run_web_journeys` y dejar que los 6 shards
  lo recojan (`npx playwright test --shard=N/6`).
- Verificación Playwright = GHA. Local solo `make web-test`.

**Non-Goals**

- Docker / Cloud Run / compose / load tests.
- Cambiar el matrix de shards o volver a `trunk serve`.
- Retags, Go nuevo, UI nueva, mock Pyodide en este spec.

## Decisions

1. **Journey de página, no más smokes duplicados como fuente**
   - `concepts.partitions.spec.ts` permanece (cobertura granular).
   - El journey es el contrato ADR 003 del hub D: un describe, un
     test oilado, un register.
   - Register vía API + login UI (mismo patrón que Wave A/D smokes);
     el alta UI ya la cubre `journey.auth-hub`.

2. **Sin mock de tags**
   - El heatmap/drawer/AND/alerta se derivan del índice WASM real.
   - No interceptar `/api/*` de catálogo (no existe: el índice es
     cliente). No stubbear `STEP_PARTITIONS`.

3. **CI intacta**
   - No tocar `strategy.matrix.shard: [1..6]`.
   - Conservar `trunk build` + `STATIC_DIR` + agregador
     `Playwright Chromium smoke`.
   - Hook local: añadir el path al array de `run_web_journeys`.

4. **Docs**
   - Página P4 `/concepts/:id` en `docs/testing/journeys.md`.
   - Matriz + Mermaid del Journey D (hub conceptual).
   - ADR 003: tercer journey productivo (hub conceptual).

## Risks / Trade-offs

| Riesgo | Mitigación |
|---|---|
| Journey largo / un flake tumba el shard | Un test; asserts ya verdes en smokes D.1–D.4 |
| AND chip deja el drawer abierto | Escape antes de facetas |
| Re-shard tentación al sumar 1 spec | 1 test extra no justifica 7 shards |

## Migration Plan

1. OpenSpec + journey + hook en
   `feat/coding-compass-wave-d5-e2e-regression` desde `a6811ac`.
2. `make web-test`. Push. GHA 6 shards.
3. PR TED. Squash-merge con Backend + Frontend + agregador verdes.

## Open Questions

- ¿Sustituir `concepts.partitions.spec.ts`? → **No**; el journey es
  el contrato nombrado; los smokes siguen como red de regresión.
- ¿Correr Playwright en la laptop? → **No** en este slice.
