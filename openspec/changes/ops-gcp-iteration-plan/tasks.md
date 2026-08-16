## 0. Este PR (planificación)

- [x] 0.1 Redactar `docs/ops/gcp-iteration-plan.md` anclado a `e9548d8`
- [x] 0.2 OpenSpec mínimo (proposal + tasks); sin código de producto
- [x] 0.3 PR TED de docs; merge con Backend + Frontend verdes (Playwright puede skip)

## A. Fail-closed JWT en producción

- [x] A.1 Si `ENV`/`APP_ENV`/`GO_ENV=production` y `JWT_SECRET` vacío o de distro/CI → el proceso no arranca
- [x] A.2 Tests unitarios del fail-closed; local/CI siguen usando el default de desarrollo
- [x] A.3 PR propio (`fix/ops-jwt-fail-closed` o similar); no mezcla GCP

## B. ADR de persistencia

- [x] B.1 ADR: SQLite = local / harness / GHA; Postgres (Cloud SQL) = Cloud Run durable
- [x] B.2 Criterio explícito demo efímera (`/tmp`) vs producto con cuentas reales
- [x] B.3 ADR `docs/adr/0004-persistencia-sqlite-postgres.md` (mismo PR que el driver)

## C. Driver Postgres + fail-closed (tests en SQLite)

- [x] C.1 Repositorio Postgres detrás de `DATABASE_URL` postgres/postgresql
- [x] C.2 Tests opt-in (`PPI_POSTGRES_TEST_URL`); SQLite permanece el default de `go test ./...`
- [x] C.3 `deploy.yml` monta secreto `DATABASE_URL` si existe; no inventa instance name; sin JSON de SA

## D. Deploy GHA (solo con bloqueos humanos resueltos)

- [x] D.1 Humano entrega: project id, billing, WIF (no JSON de SA), Cloud SQL vs demo, dominio
- [x] D.2 Workflow: WIF → push Artifact Registry (`southamerica-east1`) → Cloud Run
- [x] D.3 Env/secrets en runtime (`JWT_SECRET`, `DATABASE_URL`, SMTP); imagen distroless existente
- [x] D.4 GHA de producto (Backend/Frontend/shards/`docker-build`) sigue siendo el gate; no Cloud Build

## E. Compose Fedora después de que la nube sea fuente de verdad

- [ ] E.1 Compose que clona el contrato de prod (mismos env, misma imagen o Dockerfile)
- [ ] E.2 Sin Neo4j, sin Dockerfile Python, sin invertir el flujo “local define prod”
