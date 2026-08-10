## Why

El Paso 1 (coaching) ya cierra en Leptos (`/onboarding` → synthesize → persist).
El Paso 2 — micro-ejercicios Python con validación client-side — sigue atrapado
en Qwik (`frontend/` + Pyodide). Sin el motor en el shell canónico, el CTA
«Continuar al Paso 2» desemboca en el workspace sin editor ejecutable, y el
cutover Quik→Leptos queda a medias respecto de ADR 002.

## What Changes

- Nueva capacidad Leptos CSR: ruta autenticada `/learn` con layout coding
  (enunciado + editor + Run/Validar/Continuar) y runtime Pyodide en el browser.
- Glue JS (`web/js/ppi-pyodide.js`) con paridad del harness Qwik (run + check
  `test_*` / `capsys` sin micropip pytest en el first load).
- Primer micro-paso embebido: `py-02-variables` de la semilla foundations
  (starter + pytest). Multi-step seed loader queda como rebanada siguiente.
- CTA desde onboarding (`saved`) y workspace → `/learn`.
- `frontend/` intacto; Go sin endpoints de ejecución de código (ADR 002).

### Alcance incluido (esta rebanada)

- OpenSpec + `/learn` + Pyodide glue + Run/Validar + E2E (mock determinista).
- Estilos y guards de sesión alineados al shell.

### Fuera de alcance

- Monaco / JupyterLite.
- Loader completo de `docs/seeds/*.json` (todos los steps).
- Persistencia de progreso/código en Go.
- Speech / MCQ Casting panel completo.
- Borrar Qwik `/exercise`.

## Capabilities

### New Capabilities

- `leptos-coding-pyodide`: Superficie Paso 2 en Leptos con motor Python
  client-side (Pyodide) y checks del micro-reto.

### Modified Capabilities

- (ninguna main-spec previa de coding en Leptos)

## Impact

- `web/`: JS glue, página `/learn`, interop Wasm, E2E, styles, Trunk assets.
- OpenSpec change `leptos-paso2-pyodide`.
- Sin tocar contratos Go de ejecución; sin tocar `frontend/`.
