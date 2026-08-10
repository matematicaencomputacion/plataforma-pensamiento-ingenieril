## Why

El cimiento Leptos (Landing → Auth → Workspace) ya está endurecido en `main`
(PRs #33–#35). El Paso 1 de aprendizaje — coaching/onboarding Rogeriano con
síntesis de perfil — sigue atrapado en Qwik (`frontend/`). Sin una superficie
Leptos, el cutover del producto se queda partido y el alumno autenticado no
puede completar el relato en el shell canónico.

## What Changes

- Nueva capacidad en Leptos CSR: ruta `/onboarding` (coach + profile builder) que
  reutiliza los contratos Go existentes **sin modificar** backends JSON.
- Paridad funcional gradual con Qwik: drafting → analyze (`POST /api/learner/profile/synthesize`)
  → review → save (`PUT /api/user/profile`) → continuar al Paso 2 (placeholder hasta Pyodide).
- Entrada desde Workspace hub; guard de sesión alineado al resto del shell.
- `frontend/` Qwik permanece **intacto** (solo se deja de ser el entry canónico).

### Alcance incluido

- Ruta `/onboarding` autenticada + UI coaching mínima (rebanadas atómicas).
- Cliente Wasm para synthesize + get/put profile.
- E2E smoke Playwright del happy-path shell (con `LEARNER_PROFILE_LLM=mock`).
- OpenSpec de paridad y rollback.

### Fuera de alcance

- Editor Pyodide / Paso 2 coding (change aparte).
- SpeechRecognition (opcional en rebanada posterior).
- Cambios al contrato Go de synthesize/profile.
- Neo4j / migas de pan persistidas fuera de SQLite profile.
- Borrar o migrar rutas Qwik todavía usadas en demos.

### Plan de rollback

- Remover ruta Leptos `/onboarding` y el link del hub; Qwik `/exercise` sigue vivo.
- Feature flag env `PPI_ONBOARDING_LEPTOS=0` (opcional) si hace falta toggle.

## Capabilities

### New Capabilities

- `coaching-onboarding-leptos`: Superficie de onboarding/coaching en Leptos CSR
  con paridad de estados drafting/reviewing/saved y wiring a la API Go.

### Modified Capabilities

- (ninguna spec main previa de coaching en Leptos; el change `onboarding-gemini-profile`
  describe el backend ya entregado)

## Impact

- `web/`: pages, API client, styles, E2E.
- `openspec/changes/leptos-coaching-onboarding/`.
- Sin tocar `backend/` salvo docs; sin tocar `frontend/`.
