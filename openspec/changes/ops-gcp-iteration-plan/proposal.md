## Why

El doc Gemini de “4 capas en GCP” (FastAPI + Neo4j + Cloud Build + compose Fedora) no es el stack de PPI (Go + Leptos, GHA, imagen distroless en `e9548d8`). Sin un plan anclado a `main`, el siguiente impulso operativo copia Cloud Run / Artifact Registry / JSON de SA y rompe el loop que **ya funciona**: GitHub Actions.

Necesitamos un artefacto de planificación **accionable** para iterar varios ciclos en CI/nube *ahora*, y una cola de PRs *después* de que el humano complete project id, billing y WIF.

## What Changes

- `docs/ops/gcp-iteration-plan.md`: loop GHA de hoy, mapeo Gemini→PPI (conservar/rechazar), riesgo SQLite en Cloud Run, bloqueos humanos.
- Este change: proposal + tasks (slices A–E). **Sin código de producto.**

## Capabilities

### New Capabilities

- _(ninguna — planning only; los slices futuros abrirán capabilities al aplicarse)_

### Modified Capabilities

- _(ninguna)_

## Impact

- **Alcance incluido:** documento + OpenSpec de cola. Checkboxes A–E para PRs posteriores.
- **Fuera de alcance:** Cloud Run deploy, Artifact Registry, `cloudbuild.yaml`, docker-compose Fedora, driver Postgres, fail-closed JWT, copiar FastAPI/Neo4j/MIP.
- **CI hoy:** GitHub Actions (Backend, Frontend, 6 shards Playwright, `docker-build`). Cursor Cloud Agents GitHub App **no** está instalada.
- **Rollback:** revert del PR de docs. Cero infra GCP que deshacer.
