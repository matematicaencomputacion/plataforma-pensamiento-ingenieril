# Plan de iteración GCP (PPI)

**Estado:** vigente. Slice D (WIF → Artifact Registry → Cloud Run) vive en `.github/workflows/deploy.yml`.
**Ancla:** `origin/main` @ `e9548d8` (`feat/ops-docker-multistage`, PR #252).
**Producto:** Go + Leptos CSR (Trunk Wasm), no FastAPI + Neo4j.
**Objetivo:** loop GHA de producto (CI + docker-build + Playwright) y deploy a Cloud Run **después** de gates verdes en `main`.

Este archivo es la fuente operativa. El change OpenSpec `openspec/changes/ops-gcp-iteration-plan/` guarda el *por qué* y los checkboxes. Slices A–D tienen código en `main`; E (compose Fedora) sigue aparte.

Un borrador Neo4j (4 gigantes / Aura Free / ESCO / recomendador) quedó aparcado en [`docs/ops/knowledge-graph-vision.md`](knowledge-graph-vision.md): **fuera de la cola A–E**. No es el siguiente slice; no añade Aura, Cypher ni sidecar Python.

---

## 1. Loop funcional hoy (la nube de iteración ya existe)

**GitHub Actions es el loop de iteración y el único CI.** Cada PR de producto corre tests, shards E2E y construye la imagen. Cloud Build no es el CI. Artifact Registry + Cloud Run son el **deploy** post-merge (`deploy.yml`), no un reemplazo de esos gates.

Cursor Cloud Agents **GitHub App no está instalada**. Los ciclos se empujan con `gh` desde un worktree (humano o agente local). No esperar que un Cloud Agent abra PRs por sí solo.

### Checks que ya corren contra `main`

| Workflow | Job | Qué prueba | Cuándo |
|---|---|---|---|
| `.github/workflows/ci.yml` | **Backend** | `go test ./...` en `backend/` | todo PR / push a `main` |
| `.github/workflows/ci.yml` | **Frontend** | lint + Vitest + build de `frontend/` (Qwik legado) | todo PR / push a `main` |
| `.github/workflows/e2e.yml` | **Playwright Chromium shard 1–6** | misma suite que `make harness-e2e` (ADR 003); 6 shards × 120 min | PRs que tocan `web/**`, `backend/**`, `Dockerfile`, `.dockerignore` o los workflows E2E/Docker |
| `.github/workflows/docker.yml` | **docker-build** | `docker build` (Buildx, `push: false`) + smoke `GET /api/health` + index SPA | todo PR / push a `main` (sin path filter) |
| `.github/workflows/deploy.yml` | **Artifact Registry + Cloud Run** | WIF → push imagen → `gcloud run deploy ppi` | `workflow_run` de **Docker** en `main` (push) **después** de CI verde; no corre en PRs |

PRs **solo-docs**: Backend + Frontend sí corren; Playwright **se salta** por `paths:`. `docker-build` no tiene path filter. El deploy **no** es un check de PR.

### Cómo correr un ciclo de mejora

1. Worktree nuevo desde `origin/main` (`./scripts/worktree/wt new feat/…`). Checkout primario sucio = off-limits.
2. Slice pequeño → tests locales (`make test` / `make web-test`; harness si tocás auth/nav).
3. PR hacia `main`. CI **es** el entorno cloud de verificación.
4. Merge cuando los gates aplicables estén verdes. No force-push a `main`.
5. El siguiente ciclo parte de `origin/main` actualizado. Repetir.

La verificación de producto sigue siendo GHA. La URL pública del servicio Cloud Run `ppi` aparece tras el primer deploy exitoso en `main` (gateway **sin** IAM; JWT en la app).

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
| Neo4j Aura / Bolt | Relacional: SQLite local/CI; Postgres (Cloud SQL) en Cloud Run durable (ADR 004) | `DATABASE_URL` |
| Python Dockerfile | Imagen multi-stage: Trunk Wasm → Go estático → **distroless** | `Dockerfile` @ `e9548d8` |
| Cloud Build `cloudbuild.yaml` | GitHub Actions (`ci.yml` + `e2e.yml` + `docker.yml` + `deploy.yml`) | `.github/workflows/` |
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

Orden **A → B → C → D → E**. A y D están en `main`. B + C (ADR 004 + driver Postgres + wiring de `DATABASE_URL`) van juntos. Checkboxes: `openspec/changes/ops-gcp-iteration-plan/tasks.md`.

### A — Fail-closed `JWT_SECRET` si `ENV=production`

Hoy `LoadAuthConfig()` (`backend/internal/config/auth.go`) sustituye un secreto vacío por `dev-only-change-me-ppi-jwt-secret`. Eso es correcto en local/CI; es un incidente si un contenedor de prod arranca sin env.

- Si `ENV` (o `APP_ENV` / `GO_ENV`) es `production` y `JWT_SECRET` está vacío o es un secreto conocido de distro/CI → **no arrancar**.
- Tests unitarios del fail-closed. No toca GCP.

### B — ADR de persistencia: Cloud SQL Postgres vs SQLite local/CI

Cerrado en [`docs/adr/0004-persistencia-sqlite-postgres.md`](../adr/0004-persistencia-sqlite-postgres.md):

- SQLite = local, harness, GHA E2E, smoke Docker (`sqlite:///tmp/ppi.db`).
- Postgres = única persistencia de usuarios/progreso en un servicio Cloud Run durable.
- Corte: demo efímera (`/tmp`, `PPI_ALLOW_EPHEMERAL_SQLITE=1`) vs producto con cuentas reales (Cloud SQL).

### C — Driver Postgres + fail-closed + wiring Cloud Run

- `DATABASE_URL=postgres://…` / `postgresql://` abre el repositorio Postgres (`backend/internal/repositories/postgres`). Tests unitarios siguen en SQLite; Postgres es opt-in (`PPI_POSTGRES_TEST_URL`).
- `ENV=production` rechaza `DATABASE_URL` ausente o sqlite (incl. `/tmp`) salvo override explícito `PPI_ALLOW_EPHEMERAL_SQLITE=1`.
- `deploy.yml` monta el secreto `DATABASE_URL` **si existe**. El instance name **no está en el repo**: `--set-cloudsql-instances` se deriva de `host=/cloudsql/PROJECT:REGION:INSTANCE`. Hasta que el humano cree instancia + secreto, Cloud Run sigue en SQLite `/tmp` con el override explícito.

### D — Workflow GHA de deploy (WIF → Artifact Registry → Cloud Run)

Implementado en `.github/workflows/deploy.yml` (PR propio; no Cloud Build; no JSON de SA):

1. WIF (GitHub OIDC → `github-deploy-sa`). Trigger: Docker verde en `main` + CI verde del mismo SHA. Playwright **no** bloquea este primer deploy.
2. Push a Artifact Registry `southamerica-east1-docker.pkg.dev/project-2dc3a0ed-9735-4291-b0b/ppi/ppi:$SHA` (y `:latest` en `main`).
3. `gcloud run deploy ppi` con `--port=8080`, `ENV=production`, `--set-secrets=JWT_SECRET=JWT_SECRET:latest`, `--allow-unauthenticated`.
4. Persistencia: si el secreto `DATABASE_URL` existe, se monta (Postgres). Si no, demo `sqlite:///tmp/ppi.db` + `PPI_ALLOW_EPHEMERAL_SQLITE=1`. SMTP no se inyecta en este slice.

### E — Compose Fedora **después** de que la nube sea fuente de verdad

El compose local debe clonar el contrato de prod (misma imagen o mismo `Dockerfile`, mismos nombres de env), no al revés. Fedora no introduce Neo4j ni un segundo Dockerfile Python.

---

## 4. Recursos GCP (slice D) y lo que sigue en el humano

Valores entregados (no son secretos; el JSON de una SA **nunca** va al repo):

| Recurso | Valor |
|---|---|
| **GCP project id** | `project-2dc3a0ed-9735-4291-b0b` |
| **Número de proyecto** (WIF) | `289591172332` |
| **Región** | `southamerica-east1` |
| **Artifact Registry repo** | `ppi` |
| **Imagen** | `southamerica-east1-docker.pkg.dev/project-2dc3a0ed-9735-4291-b0b/ppi/ppi:$SHA` |
| **Servicio Cloud Run** | `ppi` |
| **WIF provider** | `projects/289591172332/locations/global/workloadIdentityPools/github-pool/providers/github-provider` |
| **SA de deploy** | `github-deploy-sa@project-2dc3a0ed-9735-4291-b0b.iam.gserviceaccount.com` |
| **Secret Manager** | `JWT_SECRET` (existe). `DATABASE_URL` (Postgres) lo crea el humano; el workflow lo monta cuando está |
| **Persistencia viva hoy** | Demo efímera: `DATABASE_URL=sqlite:///tmp/ppi.db` + `PPI_ALLOW_EPHEMERAL_SQLITE=1` hasta Cloud SQL |
| **Ingress** | `--allow-unauthenticated` (JWT en la app, no IAM en el gateway) |

Sigue en el humano (no bloquea el merge de D):

| Dato | Por qué | Cómo |
|---|---|---|
| **Binding WIF ↔ repo GitHub** | Si el job `google-github-actions/auth` falla, el provider no admite este repo | En el pool `github-pool`, attribute condition tipo `assertion.repository == 'matematicaencomputacion/plataforma-pensamiento-ingenieril'` |
| **Dominio / SMTP** | `APP_PUBLIC_URL` y reset por correo | Confirmar host público; no va en este workflow |
| **Cloud SQL Postgres** | SQLite `/tmp` no es producto con cuentas reales (ADR 004) | Comandos en §6; el repo no inventa el nombre de instancia |

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

**Regla (ADR 004):** no publicar cuentas reales mientras `DATABASE_URL` apunte a SQLite. Producto durable = Cloud SQL + secreto `DATABASE_URL` (sin `PPI_ALLOW_EPHEMERAL_SQLITE`). El binario con `ENV=production` se niega a arrancar en sqlite salvo ese override explícito.

El fail-closed del slice A no arregla este riesgo: un JWT robusto sobre una base que se evaporó sigue siendo pérdida de datos.

---

## 6. Comandos humanos: Cloud SQL + secreto `DATABASE_URL`

No hay instancia Cloud SQL en el repo. Elegí **vos** el nombre (`INSTANCE`) y corré esto con billing activo. El proxy Unix de Cloud Run **no** necesita VPC connector: IP pública en la instancia, **sin** authorized networks (el acceso va por IAM + `--set-cloudsql-instances`).

Sustituí `INSTANCE` y `DB_PASSWORD`. No commitear la contraseña ni JSON de SA.

```bash
PROJECT_ID=project-2dc3a0ed-9735-4291-b0b
PROJECT_NUMBER=289591172332
REGION=southamerica-east1
INSTANCE=   # elegí un nombre; el repo no lo fija
DB_USER=ppi
DB_NAME=ppi
DB_PASSWORD=   # generá uno fuerte; no lo subas al git
DEPLOY_SA=github-deploy-sa@${PROJECT_ID}.iam.gserviceaccount.com
RUNTIME_SA=${PROJECT_NUMBER}-compute@developer.gserviceaccount.com

gcloud services enable sqladmin.googleapis.com secretmanager.googleapis.com \
  --project="${PROJECT_ID}"

gcloud sql instances create "${INSTANCE}" \
  --project="${PROJECT_ID}" \
  --database-version=POSTGRES_16 \
  --tier=db-f1-micro \
  --region="${REGION}" \
  --storage-size=10GiB \
  --availability-type=ZONAL

gcloud sql databases create "${DB_NAME}" \
  --instance="${INSTANCE}" \
  --project="${PROJECT_ID}"

gcloud sql users create "${DB_USER}" \
  --instance="${INSTANCE}" \
  --project="${PROJECT_ID}" \
  --password="${DB_PASSWORD}"

# DSN para el Auth Proxy de Cloud Run (sslmode=disable en el socket).
DATABASE_URL="postgres://${DB_USER}:${DB_PASSWORD}@/${DB_NAME}?host=/cloudsql/${PROJECT_ID}:${REGION}:${INSTANCE}&sslmode=disable"

printf '%s' "${DATABASE_URL}" | gcloud secrets create DATABASE_URL \
  --project="${PROJECT_ID}" \
  --replication-policy=automatic \
  --data-file=-

gcloud secrets add-iam-policy-binding DATABASE_URL \
  --project="${PROJECT_ID}" \
  --member="serviceAccount:${DEPLOY_SA}" \
  --role="roles/secretmanager.secretAccessor"

# secrets.get: el deploy SA lo necesita para --set-secrets (mismo rol que JWT_SECRET).
gcloud secrets add-iam-policy-binding DATABASE_URL \
  --project="${PROJECT_ID}" \
  --member="serviceAccount:${DEPLOY_SA}" \
  --role="roles/secretmanager.viewer"

gcloud secrets add-iam-policy-binding DATABASE_URL \
  --project="${PROJECT_ID}" \
  --member="serviceAccount:${RUNTIME_SA}" \
  --role="roles/secretmanager.secretAccessor"

gcloud projects add-iam-policy-binding "${PROJECT_ID}" \
  --member="serviceAccount:${RUNTIME_SA}" \
  --role="roles/cloudsql.client"

gcloud projects add-iam-policy-binding "${PROJECT_ID}" \
  --member="serviceAccount:${DEPLOY_SA}" \
  --role="roles/cloudsql.client"
```

Si el servicio Cloud Run `ppi` ya usa otra runtime SA, reemplazá `RUNTIME_SA` con:

```bash
gcloud run services describe ppi --project="${PROJECT_ID}" --region="${REGION}" \
  --format='value(spec.template.spec.serviceAccountName)'
```

Tras crear el secreto, el **siguiente** deploy en `main` (Docker verde) monta `DATABASE_URL` y `--set-cloudsql-instances` derivado del DSN. El workflow detecta el secreto con `versions access` (`secretAccessor`); `secrets describe` exige Viewer y **no** debe tratar un PERMISSION_DENIED como “secreto ausente” (eso reinyectaba SQLite `/tmp`).

Si el secreto ya existe y Cloud Run sigue en SQLite, falta el binding Viewer del deploy SA (como `JWT_SECRET`):

```bash
gcloud secrets add-iam-policy-binding DATABASE_URL \
  --project=project-2dc3a0ed-9735-4291-b0b \
  --member="serviceAccount:github-deploy-sa@project-2dc3a0ed-9735-4291-b0b.iam.gserviceaccount.com" \
  --role="roles/secretmanager.viewer"
```

---

## 7. Qué no hacer

- No añadir `cloudbuild.yaml` ni reemplazar GHA por Cloud Build.
- No commitear JSON de service account.
- No copiar compose Neo4j / Aura / sidecar Python.
- No implementar el borrador de grafo (`docs/ops/knowledge-graph-vision.md` está **fuera de la cola A–E**).
- No force-push a `main`.
- No mezclar A–E en un solo PR. E (compose Fedora) espera a que la nube sea fuente de verdad.
