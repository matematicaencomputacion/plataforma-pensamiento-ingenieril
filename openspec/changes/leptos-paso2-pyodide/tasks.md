## 1. Foundation

- [x] 1.1 OpenSpec change `leptos-paso2-pyodide` (proposal/design/spec/tasks)
- [x] 1.2 Glue JS `web/js/ppi-pyodide.js` (paridad Qwik engine 0.27.7)
- [x] 1.3 Trunk copy-file + script en `index.html`
- [x] 1.4 Interop Rust (`pyodide.rs`) ensure/run/check vía JSON

## 2. Learn surface

- [x] 2.1 Embebido `py-02-variables` (`CodingStep`) + página `/learn`
- [x] 2.2 Guard de sesión + UI enunciado/editor/consola/status motor
- [x] 2.3 Run / Validar / Continuar (unlock tras pass)
- [x] 2.4 CTA onboarding + workspace → `/learn`
- [x] 2.5 Estilos `.learn*` coherentes con el design system

## 3. Proof

- [x] 3.1 E2E mock: auth → `/learn` → Validar pass → Continuar
- [x] 3.2 Asset smoke: `ppi-pyodide.js` servido por Trunk
- [x] 3.3 `make harness` PASS + PR TED vía `gh`

## 5. Exercise evaluation (rebanada 3)

- [x] 5.1 Harness client-side con `cases[]` (name/passed/message) en Pyodide JS
- [x] 5.2 UI «Validar solución» + panel test cases + banner de éxito
- [x] 5.3 `POST /api/progress/complete` (Bearer, sin `code`) + wire Wasm
- [x] 5.4 E2E fail/pass + assert progreso ADR 002
