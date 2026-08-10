# PR TED slides — Pyodide exercise evaluation + progress (rebanada 3)

## Slide 1 — Hook
Sin validación estructurada ni persistencia, el alumno ejecuta código pero no “completa” el ejercicio en el producto.

## Slide 2 — Insight
La evaluación debe vivir 100% en Pyodide (ADR 002); Go solo recibe progreso (`level_id` / `step_id` / `passed`), nunca el código Python.

## Slide 3 — Move
- Harness client-side `cases[]` + UI «Validar solución» / panel verde-rojo / banner de éxito
- `POST /api/progress/complete` (Bearer; rechaza `code`) + wire Wasm
- E2E fail/pass + assert de body sin `code`

## Slide 4 — Proof
`make harness` → `RESULT: PASS` (backend-unit, web-unit, stack, web-e2e).

## Slide 5 — Ask
Review/merge hacia `main` cuando CI esté verde.
