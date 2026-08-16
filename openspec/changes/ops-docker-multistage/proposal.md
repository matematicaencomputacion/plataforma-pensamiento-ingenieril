## Why

PPI already serves API + Trunk SPA from one Go process (`STATIC_DIR` / `go:embed`). The existing image is Alpine with a baked runtime user and no CI gate, so a broken Dockerfile can reach `main` unnoticed. The laptop has no Fedora/docker team yet: the image MUST build in GitHub Actions.

## What Changes

- Multi-stage Dockerfile: Trunk Wasm → static Go binary → distroless runtime (one image, API + SPA).
- Runtime config via env (`PORT`, `DATABASE_URL`). No secrets in the image.
- GHA job `docker-build`: `docker build` + smoke `curl /api/health` (no registry push).
- `.env.example` documents `PORT` / `DATABASE_URL` placeholders.

## Capabilities

### New Capabilities

- `container-image`: una imagen de runtime distroless que sirve API Go + SPA Trunk, construida en CI.

### Modified Capabilities

- _(ninguna)_

## Impact

- **Backend / web:** sin cambio de comportamiento; el binario sigue embebiendo `web/dist` como en E2E (`STATIC_DIR`).
- **CI:** nuevo check `docker-build` en PRs. No Artifact Registry, no Cloud Run.
- **Fuera de alcance:** Cloud Run, Artifact Registry, Secret Manager, Cloud SQL, docker-compose (Fedora), cambios Playwright, retags.
- **Rollback:** revert del PR. No hay deploy GCP que deshacer.
