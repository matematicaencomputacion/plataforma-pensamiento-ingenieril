## Slide 1 — Hook
El alumno sale del coaching y escribe Python de verdad en el shell Leptos — Run/Validar en el browser, sin servidor ejecutor.

## Slide 2 — Insight
Sin Paso 2, «Continuar» era un callejón al workspace vacío. ADR 002 exige Pyodide en cliente; Qwik ya lo tenía, Leptos no. Había que portar el glue y abrir `/learn`.

## Slide 3 — Move
- OpenSpec `leptos-paso2-pyodide` + glue `ppi-pyodide.js` (paridad Qwik 0.27.7).
- `/learn` autenticado con `py-02-variables`, hint/solución, Run/Validar/Continuar.
- CTA onboarding/workspace; E2E mock + smoke del asset JS.

## Slide 4 — Proof
- `make harness` → RESULT: PASS.

## Slide 5 — Ask
Merge para fijar el puente coding; próxima rebanada: loader multi-step de la semilla.
