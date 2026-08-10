## Slide 1 — Hook
El Paso 1 cierra el loop: el perfil sintetizado se guarda en la cuenta Go y vuelve al recargar.

## Slide 2 — Insight
Reviewing sin PUT dejaba el coaching efímero. El contrato real es `GET|PUT /api/user/profile` (Bearer), no `/api/learner/profile`.

## Slide 3 — Move
- Cliente Wasm fetch/put profile + tipos `lifePurpose`/`vision5Years`/`techStack`.
- Hidratación al montar → `saved`; Guardar activo; CTA «Continuar al Paso 2».
- E2E save → reload → hydrate; OpenSpec tareas 3.x.

## Slide 4 — Proof
- `make harness` → RESULT: PASS.

## Slide 5 — Ask
Merge para cerrar coaching persistente y desbloquear editor Paso 2 / Pyodide.
