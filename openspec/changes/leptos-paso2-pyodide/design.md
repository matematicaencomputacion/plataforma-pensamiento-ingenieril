## Context

ADR 002 exige Python del alumno **solo en el browser**. Qwik ya lo resolvió en
`frontend/src/lib/pyodide/engine.ts` (CDN Pyodide 0.27.7 + harness `test_*`).
Leptos CSR vive en `web/` (Trunk :3001) y no puede importar módulos TS de Qwik
sin acoplar el monorepo.

El API Go `GET /api/levels/current` expone `statement`/`title` pero **no**
`starter_code` ni `pytest`. La fuente pedagógica canónica del Paso 2 es la
semilla `docs/seeds/python-foundations-microsteps-v0.2.json` (`py-02-variables`
es el primer coding step post-onboarding).

## Goals / Non-Goals

**Goals**

- `/learn` autenticado con Run/Validar reales vía Pyodide (lazy-load).
- Paridad de harness con Qwik (solution.py + test_step.py + capsys).
- Primer step embebido `py-02-variables` (contenido idéntico a la semilla).
- E2E determinista con mock del glue (CI estable) + smoke de asset presente.
- Continuar desde onboarding apunta a `/learn`.

**Non-Goals**

- Port total de MCQ / type chips / multi-step navigation en el primer PR.
- Endpoint Go para ejecutar código.
- Monaco.

## Decisions

1. **Glue JS en `web/js/ppi-pyodide.js`**, expuesto como `window.ppiPyodide`,
   cargado por Trunk (`copy-file` + `<script>`). Rust llama vía `js_sys` +
   `JsFuture` y deserializa JSON plano (sin compartir crate TS).
2. **Misma versión Pyodide** que Qwik (`0.27.7`, jsDelivr) para no divergir.
3. **Curriculum v1 embebido en Rust** (`CodingStep` estático) para no depender
   aún de fetch/parse del JSON enorme; sync con seed documentado en tasks.
4. **E2E default mock**: `addInitScript` reemplaza `ppiPyodide` antes del
   hydrate Wasm para evitar CDN/flake. Flag `PPI_E2E_REAL_PYODIDE=1` reservado
   para smoke opcional local.
5. **Ruta `/learn`** (no `/exercise`) para no colisionar mentalmente con Qwik
   legacy mientras cohabitan.

## Risks / Trade-offs

| Riesgo | Mitigación |
|--------|------------|
| Latencia primera carga Pyodide | Lazy bootstrap al montar `/learn` + UI status |
| Flaky CI por CDN | Mock E2E por defecto |
| Drift seed ↔ embebido | Comment + task 2.x de sync loader JSON |
| Harness JS desalineado vs Qwik | Port literal del engine + comentario de origen |

## Migration Plan

1. Ship `/learn` + Pyodide + un step.
2. Wire CTA onboarding/workspace.
3. Luego: seed loader multi-step + continue chain.
4. Cutover Qwik `/exercise` cuando paridad lo permita.

## Open Questions

- ¿Avance de nivel en Go al pasar Validar? → **No en esta rebanada** (solo UX
  local `canContinue`).
