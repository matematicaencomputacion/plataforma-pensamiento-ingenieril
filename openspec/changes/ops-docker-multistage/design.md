## Context

El proceso Go ya hace de origin único: `/api/*` + fallback SPA (`backend/static_spa.go`). E2E D.5 usa `STATIC_DIR` apuntando al `dist/` de Trunk. La imagen de producción debe ser ese mismo contrato, no un nginx aparte.

## Goals / Non-Goals

**Goals:**

- Una imagen `linux/amd64`: binario estático + dist Trunk embebido + fallback on-disk.
- Runtime distroless (`gcr.io/distroless/static-debian12:nonroot`) o equivalente scratch.
- Build en GitHub Actions; el laptop no es un requisito de build.
- `PORT` y `DATABASE_URL` inyectados por env. Cero secretos en layers.

**Non-Goals:**

- Push a Artifact Registry / Cloud Run (paso 3).
- docker-compose en Fedora (paso 4).
- Cambiar Playwright o el aggregator E2E.

## Decisions

1. **Una imagen, un proceso.** Go sirve API + SPA (`go:embed` + `STATIC_DIR=/app/static`). Coincide con el harness E2E; evita un stage nginx.
2. **Distroless static, no Alpine.** `CGO_ENABLED=0` + `modernc.org/sqlite` (pure Go). Distroless trae CA certs (LLM/SMTP) sin shell.
3. **SQLite en `/tmp`.** El user `nonroot` no escribe en `/app`. Default de contenedor: `DATABASE_URL=sqlite:///tmp/ppi.db` (no es secreto).
4. **CI build, no registry.** Job `docker-build` hace `load: true` + `docker run` + `GET /api/health`. Tags: `ppi:ci` y `ppi:<sha>`. Push queda para paso 3.
5. **`.dockerignore`:** `/data` solo en la raíz; `backend/data` (seeds `levels.json`) SÍ entra al contexto.

## Risks / Trade-offs

- Distroless no tiene shell: el smoke vive en GHA (`docker run` + curl), no en un `HEALTHCHECK` interno.
- `--platform=linux/amd64` fija GHA; un build nativo ARM en Fedora vendrá en paso 4.
- El job de imagen es más lento que `go test` (Trunk + Wasm). Cache GHA (`type=gha`) mitiga reruns.
