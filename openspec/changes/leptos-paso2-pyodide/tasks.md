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

## 4. Execution engine (rebanada 2)

- [x] 4.1 Interop en `web/src/interop/pyodide.rs` + bodies stdout/stderr
- [x] 4.2 Consola estructurada + «Ejecutar código» + busy copy
- [x] 4.3 Hidratar consignas desde `GET /api/levels/current` (sin ejecutar en Go)
- [x] 4.4 E2E `exercise.spec.ts`: Ready → print → stdout
