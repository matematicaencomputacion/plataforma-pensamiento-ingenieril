# ADR 003: Sistema de pruebas integral (Harness + Journeys E2E)

## Estado
Aceptado

## Contexto
El producto combina API Go + shell Leptos CSR (Trunk). Los bloqueos de acceso
reales (409 vs 401, SQLite dual `ppi.db`/`ppi-harness.db`, hidratación Wasm,
tokens de reset sin SMTP) no se atrapan solo con unit tests. Necesitamos un
**sistema de pruebas** que:

1. Sea ejecutable de un comando (`make harness` / `make harness-journeys`).
2. Cubra journeys de punta a punta por **páginas nombradas** (no solo endpoints).
3. Documente caminos felices y fallos conocidos (Mermaid en `docs/testing/`).
4. Quede fijado como decisión de arquitectura para agentes y humanos.

## Decisión

### Capas obligatorias

| Capa | Contrato | Gate |
|---|---|---|
| Backend unit | `cd backend && go test ./...` | `make test` / harness |
| Web unit | `cd web && cargo test` | `make web-test` / harness |
| Backend integration (opt-in) | `go test -tags=integration ./internal/integration/...` | `PPI_HARNESS_INTEGRATION=1` |
| Web E2E Chromium | Playwright bajo `web/e2e/tests/` | `make harness-e2e` / CI `e2e.yml` (4 shards `--shard=N/4` + agregador `Playwright Chromium smoke`) |

La **fuente de verdad local** es `scripts/harness/run.sh` (`make harness*`).
CI debe permanecer alineada a esas mismas suites (no inventar smoke paralelo
sin documentarlo aquí).

### Journeys E2E canónicos (páginas)

Se definen dos journeys productivos (ambos deben permanecer verdes):

1. **Auth journey** — anónimo → identidad → workspace  
   Paginas: `/` → `/register` o `/login` → (`/forgot-password` → `/reset-password`) → `/workspace`
2. **Hub journey** — sesión viva sin expulsión  
   Paginas: `/workspace` ↔ `/` (portada autenticada) con session bar (`Portada` / `Workspace` / `Salir`)

El spec transversal vive en `web/e2e/tests/journey.auth-hub.spec.ts` y se
ejecuta en `make harness-journeys` y dentro de `make harness-e2e`.

### Reglas anti-regresión de autenticación

- Toda contraseña de usuario de desarrollo se resetea con
  `make dev-set-password EMAIL=… PASSWORD=…` (CLI `ppi-authctl`), nunca editando
  hashes a mano.
- E2E **no** depende de cuentas humanas (`@gmail.com`); siembra emails efímeros
  vía API o `PPI_E2E_EMAIL` uniquely tagged.
- En local/CI con secreto de distro / `PPI_EXPOSE_RESET_TOKEN=1` /
  `ENV=development`, `forgot-password` **debe** devolver `resetToken` para DX.
- Login/register ante 401/409 deben purgar `ppi.auth.token` + signals de sesión.
- Documentar siempre qué `DATABASE_URL` usa el proceso bajo prueba (default
  `sqlite://./data/ppi.db`; harness usa `ppi-harness.db`).

## Consecuencias positivas
- Un fallo de “no puedo entrar” tiene un diagnostico reproducible en minutos.
- Los agentes IA tienen un checklist irrefutable (este ADR + Mermaid).
- CI y local comparten el mismo mapa de journeys.

## Consecuencias / costos
- E2E es más lento y flaky si no se respetan esperas de hidratación Wasm.
- Hay dos SQLite legítimas (app vs harness); la confusión es un riesgo operativo
  mitigado por docs + `ppi-authctl -db=…`.

## Restricciones para el Agente IA
- No declarar un hito auth/navegación “terminado” sin:
  1. `go test ./...` verde,
  2. `make web-test` verde,
  3. journey Playwright auth+hub verde (`make harness-journeys` o harness-e2e).
- No introducir un tercer frontend de pruebas (mantener Playwright + Leptos CSR).
- No ejecutar código de alumnos en el servidor (ADR 002).
- No sugerir OAuth ni SMTP real en este ADR; el mailer es change futuro.
- Actualizar `docs/testing/journeys.md` si se agrega una página auth/hub nueva.

## Referencias
- `TESTING.md` — operación del harness
- `docs/testing/journeys.md` — diagramas Mermaid y matriz página↔test
- `Makefile` targets: `harness`, `harness-e2e`, `harness-journeys`, `dev-set-password`
- ADR 001 (stack), ADR 002 (ejecución client-side)
