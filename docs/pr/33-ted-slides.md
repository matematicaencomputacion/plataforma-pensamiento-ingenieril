## Slide 1 — Hook
`make harness` no puede mentir en verde: si Playwright no encuentra Chromium, el cimiento pre-coaching se declara roto aunque el producto esté sano.

## Slide 2 — Insight
El harness dejaba el cache de browsers al azar del entorno (sandboxes/agent). `npm ci` + path efímero ⇒ “Executable doesn't exist” y `web-e2e: FAIL` falso-negativo.

## Slide 3 — Move
- `ensure_playwright_browsers` en `scripts/harness/run.sh` fija `PLAYWRIGHT_BROWSERS_PATH` al cache del host e instala Chromium idempotente.
- Documentado en `TESTING.md`.

## Slide 4 — Proof
- `make harness` → `RESULT: PASS` (backend-unit, web-unit, stack, web-e2e).

## Slide 5 — Ask
Merge para desbloquear la auditoría de calidad (siguientes rebanadas: sesión, UI mobile, trim email).
