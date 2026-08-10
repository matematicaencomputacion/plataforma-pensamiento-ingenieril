## Slide 1 — Hook
El cliente Leptos deja de quedar congelado o críptico cuando la API cae: mensajes claros, foco accesible y cero panics en el camino crítico.

## Slide 2 — Insight
Los `.unwrap` de producción ya estaban limpios, pero los errores de red filtraban strings crudos del browser y los alerts no tenían `aria-live`. Un form podía parecer “busy” eterno si no se liberaba el estado tras el fallo.

## Slide 3 — Move
- Copy estable `MSG_NETWORK_UNAVAILABLE` / `MSG_INVALID_RESPONSE` en todos los fetches auth/level.
- `busy` se libera siempre al completar el future (ok/err).
- Alerts con `role="alert"` + `aria-live="polite"`; `:focus-visible` en CTAs/inputs.
- Clippy limpio; E2E keyboard + offline abort en login.

## Slide 4 — Proof
- `cargo clippy` sin warnings del crate.
- `make harness` → RESULT: PASS.

## Slide 5 — Ask
Merge para cerrar la auditoría pre-coaching (cimiento auth/sesión/UX listo para migrar Coaching/Pyodide).
