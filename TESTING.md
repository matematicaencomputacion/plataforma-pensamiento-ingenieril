# PPI — Integral Test Harness

Harness unificado para validar **backend Go** + **frontend Leptos (unit + E2E Playwright)** desde la raíz del monorepo, con reportes por módulo y teardown del stack.

## Vista rápida

```bash
# Suite completa (unit + integration opcional + e2e con stack efímero)
make harness

# Solo capas rápidas (sin levantar UI)
make harness-unit

# Integración Go (opt-in)
PPI_HARNESS_INTEGRATION=1 make harness-integration

# Solo E2E (levanta API+Trunk, corre Playwright, apaga procesos)
make harness-e2e
```

Reportes: `artifacts/harness/<timestamp>/` (`summary.tsv`, logs por módulo, report Playwright si falló).

## Arquitectura

```
make harness
    │
    ▼
scripts/harness/run.sh
    ├── backend-unit      →  go test ./...
    ├── web-unit          →  cargo test (web/)
    ├── backend-integration → go test -tags=integration ./internal/integration/...
    │                         (SKIP salvo PPI_HARNESS_INTEGRATION=1)
    └── web-e2e
          ├── start Go :8080 (SQLite harness DB)
          ├── start Trunk :3001
          ├── playwright smoke (+ suites futuras)
          └── cleanup PIDs (trap EXIT)
```

| Módulo | Qué valida | Dónde vive |
|---|---|---|
| `backend-unit` | Use cases, handlers httptest, repos | `backend/**/*_test.go` |
| `web-unit` | Contratos API / URLs Wasm | `web/` (`cargo test`) |
| `backend-integration` | Endpoints críticos contra mux real + SQLite archivo | `backend/internal/integration/` (`//go:build integration`) |
| `web-e2e` | Flujos de punta a punta en Chromium | `web/e2e/tests/` |

## Ampliación de la batería (roadmap de suites)

### Playwright (`web/e2e/tests/`)

| Spec | Estado | Intención |
|---|---|---|
| `auth.login.spec.ts` | **Activo** | Landing CTAs + register/login → `/workspace` |
| `auth.validation.spec.ts` | Esqueleto | Errores de formulario (password corta, email inválido, 401 genérico) |
| `workspace.navigation.spec.ts` | Esqueleto | Sesión en header, logout → `/`, guard sin token |
| `levels.browse.spec.ts` | Esqueleto (futuro cutover) | Listar/navegar niveles cuando el harness pedagógico esté en Leptos |

Convención: specs nuevos van bajo `web/e2e/tests/<dominio>.*.spec.ts`. Marcar pendientes con `test.describe.skip` hasta que el UI exista; no borrar Qwik en E2E de cutover.

### Go integration (`backend/internal/integration/`)

| Test | Estado | Intención |
|---|---|---|
| `auth_api_test.go` | Esqueleto | Register/login/me/logout sobre `ServeMux` + SQLite temp file |
| `levels_api_test.go` | Esqueleto | `GET /api/levels/current` + `GET /api/levels/{id}` |
| `health_api_test.go` | Esqueleto | `GET /api/health` + headers CORS mínimos |

Activar con:

```bash
PPI_HARNESS_INTEGRATION=1 make harness-integration
```

## Orquestación y códigos de salida

- Cada módulo escribe `PASS` | `FAIL` | `SKIP` en `summary.tsv`.
- El harness termina **≠ 0** si algún módulo es `FAIL`.
- `SKIP` (integración off / e2e off con `PPI_HARNESS_SKIP_E2E=1`) no falla el run.
- Logs de API/Trunk viven junto al summary para diagnóstico rápido.

## Variables útiles

| Variable | Default | Uso |
|---|---|---|
| `PPI_HARNESS_INTEGRATION` | `0` | `1` ejecuta tests `-tags=integration` |
| `PPI_HARNESS_SKIP_E2E` | `0` | `1` omite Playwright en `make harness` |
| `PPI_HARNESS_REPORT_DIR` | `artifacts/harness` | Destino de reportes |
| `PPI_E2E_*` | efímero en harness | Igual que `web/e2e/README.md` |
| `JWT_SECRET` / `DATABASE_URL` | valores harness | Stack efímero E2E |

## CI

- Unit backend: `.github/workflows/ci.yml`
- E2E Playwright: `.github/workflows/e2e.yml` (usuario efímero)
- El target `make harness` es la **fuente de verdad local**; CI puede ir sumando jobs por módulo alineados a este mapa.

## Reglas

- No secretos en git (`.env.local` / GitHub Secrets / usuario efímero).
- No borrar `frontend/` Qwik desde pruebas.
- Código de alumnos no se ejecuta en el servidor (ADR 002).
