# Plan de iteración GCP (PPI)

**Estado:** vigente (planificación). No hay deploy a Cloud Run en este documento.
**Ancla:** `origin/main` @ `e9548d8` (`feat/ops-docker-multistage`, PR #252).
**Producto:** Go + Leptos CSR (Trunk Wasm), no FastAPI + Neo4j.
**Objetivo:** dejar un loop **funcional hoy** para varios ciclos de mejora en CI / nube, y una cola de PRs futuros **cuando el humano desbloquee GCP**.

Este archivo es la fuente operativa. El change OpenSpec `openspec/changes/ops-gcp-iteration-plan/` guarda el *por qué* y los checkboxes de slices futuros. Ningún slice A–E se implementa aquí.

---

## 1. Loop funcional hoy (la nube de iteración ya existe)

**GitHub Actions es el loop de iteración en la nube.** Cada PR de producto ya “despliega a CI”: corre tests, shards E2E y construye la imagen. No hace falta Cloud Build, Artifact Registry ni Cloud Run para mejorar el producto.

Cursor Cloud Agents **GitHub App no está instalada**. Los ciclos se empujan con `gh` desde un worktree (humano o agente local). No esperar que un Cloud Agent abra PRs por sí solo.

### Checks que ya corren contra `main`

| Workflow | Job | Qué prueba | Cuándo |
|---|---|---|---|
| `.github/workflows/ci.yml` | **Backend** | `go test ./...` en `backend/` | todo PR / push a `main` |
| `.github/workflows/ci.yml` | **Frontend** | lint + Vitest + build de `frontend/` (Qwik legado) | todo PR / push a `main` |
| `.github/workflows/e2e.yml` | **Playwright Chromium shard 1–6** | misma suite que `make harness-e2e` (ADR 003); 6 shards × 120 min | PRs que tocan `web/**`, `backend/**`, `Dockerfile`, `.dockerignore` o los workflows E2E/Docker |
| `.github/workflows/docker.yml` | **docker-build** | `docker build` (Buildx, `push: false`) + smoke `GET /api/health` + index SPA | todo PR / push a `main` (sin path filter) |

PRs **solo-docs** (como este): Backend + Frontend sí corren; Playwright **se salta** por `paths:`. `docker-build` hoy no tiene path filter: puede correr igual. No es un deploy GCP; es un gate de imagen.

### Cómo correr un ciclo de mejora

1. Worktree nuevo desde `origin/main` (`./scripts/worktree/wt new feat/…`). Checkout primario sucio = off-limits.
2. Slice pequeño → tests locales (`make test` / `make web-test`; harness si tocás auth/nav).
3. PR hacia `main`. CI **es** el entorno cloud de verificación.
4. Merge cuando los gates aplicables estén verdes. No force-push a `main`.
5. El siguiente ciclo parte de `origin/main` actualizado. Repetir.

No hay URL pública nueva en este loop. La “nube” es GitHub-hosted runners + la imagen distroless que ya fabrica `docker-build`.

---

## 2. Mapeo: doc Gemini de 4 capas → PPI

Fuente leída (local, no versionada): `despliegue_gcp_ci_cd_4_capas.md` (FastAPI + Neo4j + Cloud Build + compose Fedora). **No se copia.** Se traduce al stack que `main` ya tiene.

### Equivalencias (qué significa cada capa en PPI)

| Capa Gemini (rechazada como stack) | Equivalente PPI actual | Dónde vive |
|---|---|---|
| Capa 1 Mercado MIP / Leontief | No hay motor insumo-producto. El “mercado de skills” del producto es curriculum versionado + niveles | `curriculum/`, `backend/data/` |
| Capa 2 Conocimiento / particiones | Particiones conceptuales, tags, DAG de prerrequisitos, heatmap | `web/` hub conceptual; API Go |
| Capa 3 Estudiante | Auth JWT + progreso + perfil de onboarding | `backend/` + SQLite; Leptos CSR |
| Capa 4 Telemetría / fricción | Analytics conceptual de fricción (Wave D.3) | backend analytics + hub |
| FastAPI + `uvicorn` | Un proceso Go (`ppi-api`) que sirve `/api/*` + SPA embebida | `backend/`, `Dockerfile` |
| Neo4j Aura / Bolt | Relacional: SQLite hoy; Postgres (Cloud SQL) es decisión futura (slice B) | `DATABASE_URL` |
| Python Dockerfile | Imagen multi-stage: Trunk Wasm → Go estático → **distroless** | `Dockerfile` @ `e9548d8` |
| Cloud Build `cloudbuild.yaml` | GitHub Actions (`ci.yml` + `e2e.yml` + `docker.yml`) | `.github/workflows/` |
| Compose Fedora + Neo4j | Fuera de alcance hasta que la nube sea fuente de verdad (slice E) | — |

### Conservar (más adelante, no en este PR)

- Región **`southamerica-east1`** cuando exista proyecto GCP (latencia AR/BR, alineada al doc Gemini).
- Imagen **distroless** `gcr.io/distroless/static-debian12:nonroot` (ya en `main`).
- **Inyección de env en runtime**: `PORT`, `DATABASE_URL`, `JWT_SECRET`, SMTP, API keys. Cero secretos en layers de la imagen (comentario explícito en el `Dockerfile`).
- Un proceso, un contenedor: API + SPA (no nginx aparte). Smoke `/api/health` como contrato.

### Rechazar (no portar)

- Cloud Build como CI primario (GHA ya es el loop).
- Neo4j / AuraDB / driver Bolt / Secret Manager de `NEO4J_*`.
- Mercado MIP, matriz de Leontief, numpy como runtime de producto.
- Dockerfile Python 3.11 + `uvicorn`.
- FastAPI routers `mercado` / `conocimiento` / `estudiante` / `recomendador`.
- `cloudbuild.yaml`, trigger “push a main → Cloud Run” copiado del doc.
- `docker-compose.yml` Fedora + Neo4j **ahora** (slice E, y solo después de D).
- JSON de service account en el repo (nunca).
- Ejecutar código de alumnos en el servidor (ADR 002: Pyodide en el cliente).

---

## 3. PRs futuros (secuencia fija)

Orden **A → B → C → D → E**. No saltar D antes de tener project id + billing + WIF. Checkboxes vivos: `openspec/changes/ops-gcp-iteration-plan/tasks.md`.

### A — Fail-closed `JWT_SECRET` si `ENV=production`

Hoy `LoadAuthConfig()` (`backend/internal/config/auth.go`) sustituye un secreto vacío por `dev-only-change-me-ppi-jwt-secret`. Eso es correcto en local/CI; es un incidente si un contenedor de prod arranca sin env.

- Si `ENV` (o `APP_ENV` / `GO_ENV`) es `production` y `JWT_SECRET` está vacío o es un secreto conocido de distro/CI → **no arrancar**.
- Tests unitarios del fail-closed. No toca GCP.

### B — ADR de persistencia: Cloud SQL Postgres vs SQLite local/CI

El PRD ya dice PostgreSQL en producción / SQLite en desarrollo. Falta un ADR que cierre:

- SQLite = local, harness, GHA E2E, smoke Docker (`sqlite:///tmp/ppi.db`).
- Postgres = única persistencia de usuarios/progreso en un servicio Cloud Run durable.
- Criterio de corte: demo efímera (SQLite `/tmp`, datos que se evaporan) vs producto con cuentas reales (Cloud SQL).

Sin este ADR no se escribe driver ni se crea instancia SQL.

### C — Driver Postgres opcional, todavía sin credenciales GCP

Puerto de repositorio ya existe (`UserRepository`). Implementar Postgres detrás de `DATABASE_URL=postgres://…` (o `postgresql://`), tests con contenedor o skip si no hay DSN.

**Todavía no:** project id, WIF, Cloud SQL Auth Proxy, secrets de GCP. El driver debe poder probarse en CI con un Postgres efímero (service container) o quedarse opt-in.

### D — Workflow GHA de deploy (WIF → Artifact Registry → Cloud Run)

**Solo después** de que el humano complete la §4 (project id + billing + WIF).

Boceto (no implementar en este PR):

1. Workload Identity Federation (GitHub OIDC → SA de deploy). **No** JSON de service account en secrets ni en git.
2. `docker push` a Artifact Registry en `southamerica-east1`.
3. `gcloud run deploy` del mismo tag SHA, `--set-env-vars` / Secret Manager para `JWT_SECRET`, SMTP, `DATABASE_URL`.
4. CI de producto (Backend, Frontend, shards, docker-build) sigue siendo el gate; el deploy es un workflow **aparte**, no un reemplazo de GHA por Cloud Build.

### E — Compose Fedora **después** de que la nube sea fuente de verdad

El compose local debe clonar el contrato de prod (misma imagen o mismo `Dockerfile`, mismos nombres de env), no al revés. Fedora no introduce Neo4j ni un segundo Dockerfile Python.

---

## 4. Bloqueos que solo el humano puede llenar

Nada de D (ni Cloud SQL real) avanza sin estas respuestas. Dejarlas por escrito en el PR del slice D, no en código.

| Dato | Por qué bloquea | Cómo se entrega |
|---|---|---|
| **GCP project id** | Registry, Cloud Run, IAM, SQL viven en un proyecto | Pegarlo en el workflow / vars de GitHub, no hardcodear si va a rotar |
| **Billing habilitado** | Cloud Run / Artifact Registry / Cloud SQL no arrancan en proyecto sin billing | Consola GCP; confirmar en el PR D |
| **WIF vs JSON de SA** | Decisión: **WIF**. El JSON de una service account **nunca** se commitea ni se pega en el chat | Pool OIDC GitHub → SA con roles mínimos (`run.admin`, `artifactregistry.writer`, `secretmanager.secretAccessor` si aplica) |
| **Cloud SQL vs demo efímera** | SQLite en `/tmp` no sobrevive scale-to-zero ni multi-instancia (véase §5) | Elegir: (1) demo descartable, o (2) Cloud SQL Postgres + `DATABASE_URL` |
| **Dominio** | `APP_PUBLIC_URL`, cookies/reset SMTP, TLS | Ej. `ingenieria.wechgat.com.ar` ya aparece en `.env.example`; confirmar si sigue siendo el público |

Hasta que esa tabla esté completa, el loop de mejora **sigue siendo GHA** (esta sección 1).

---

## 5. Riesgo explícito: SQLite en Cloud Run

El `Dockerfile` de `e9548d8` fija:

```text
DATABASE_URL=sqlite:///tmp/ppi.db
USER nonroot
```

Eso es correcto para **smoke CI** y para un contenedor local. Es **incorrecto como persistencia de producción**.

| Hecho de Cloud Run | Efecto con SQLite en `/tmp` |
|---|---|
| El disco del contenedor es efímero | Cada revisión nueva, crash o scale-to-zero **borra usuarios y progreso** |
| `/tmp` no se comparte entre instancias | Dos réplicas = dos bases; login en A no existe en B |
| Scale-out / request concurrentes | SQLite no es el motor de un servicio multi-instancia |
| Distroless `nonroot` no escribe `/app` | Por eso el default es `/tmp` — no lo “arregles” montando el `.db` en `/app` |

**Regla:** no publicar un servicio Cloud Run con cuentas reales mientras `DATABASE_URL` apunte a SQLite. Demo efímera (datos de juguete, se espera perderlos) = OK. Producto = esperar slices B + C + instancia Cloud SQL (o declarar explícitamente “demo descartable” en el PR D).

El fail-closed del slice A no arregla este riesgo: un JWT robusto sobre una base que se evaporó sigue siendo pérdida de datos.

---

## 6. Qué no hacer en los próximos ciclos (hasta D desbloqueado)

- No añadir `cloudbuild.yaml`.
- No crear repo en Artifact Registry “por las dudas”.
- No copiar el compose Neo4j del doc Gemini.
- No instalar Cursor Cloud Agents GitHub App como requisito del loop (el loop ya es GHA).
- No force-push a `main`.
- No implementar A–E en el mismo PR que este plan.

Cuando A–E existan, cada uno es un PR propio con slides TED y CI verde.
