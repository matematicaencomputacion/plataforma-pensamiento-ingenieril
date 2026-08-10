## Slide 1 — Hook
El Paso 1 deja de ser solo un borrador: el alumno obtiene propósito/urgencia/visión/stack vía la API Go real (mock en harness).

## Slide 2 — Insight
El shell de #36 habilitaba la ruta, pero sin synthesize el cutover seguía ciego al contrato `raw_notes` → profile. Había que cablear cliente Wasm + estados analyzing/reviewing antes de persistir.

## Slide 3 — Move
- Tipos + `POST /api/learner/profile/synthesize` en `web/` (sin tocar Go ni Qwik).
- UI: analyzing / error / reviewing con 4 campos editables; Guardar deshabilitado (siguiente rebanada).
- Unit JSON + E2E mock LLM (`estudiante`/`urgencia`).

## Slide 4 — Proof
- `make harness` → RESULT: PASS.

## Slide 5 — Ask
Merge para desbloquear PUT `/api/user/profile` + continue al Paso 2.
