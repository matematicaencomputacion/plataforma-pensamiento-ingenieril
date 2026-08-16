## 0. Este PR (planificación)

- [x] 0.1 Redactar `docs/ops/gcp-iteration-plan.md` anclado a `e9548d8`
- [x] 0.2 OpenSpec mínimo (proposal + tasks); sin código de producto
- [x] 0.3 PR TED de docs; merge con Backend + Frontend verdes (Playwright puede skip)

## A. Fail-closed JWT en producción

- [x] A.1 Si `ENV`/`APP_ENV`/`GO_ENV=production` y `JWT_SECRET` vacío o de distro/CI → el proceso no arranca
- [x] A.2 Tests unitarios del fail-closed; local/CI siguen usando el default de desarrollo
- [x] A.3 PR propio (`fix/ops-jwt-fail-closed` o similar); no mezcla GCP

## B. ADR de persistencia

- [ ] B.1 ADR: SQLite = local / harness / GHA; Postgres (Cloud SQL) = Cloud Run durable
- [ ] B.2 Criterio explícito demo efímera (`/tmp`) vs producto con cuentas reales
- [ ] B.3 PR `docs/adr-persistence-postgres` (o equivalente); sin driver todavía

## C. Driver Postgres opcional (sin credenciales GCP)

- [ ] C.1 Implementar repositorio Postgres detrás de `DATABASE_URL` postgres/postgresql
- [ ] C.2 Tests opt-in (service container o skip sin DSN); SQLite permanece el default
- [ ] C.3 Cero project id, WIF, Cloud SQL proxy o JSON de SA en este slice

## D. Deploy GHA (solo con bloqueos humanos resueltos)

- [ ] D.1 Humano entrega: project id, billing, WIF (no JSON de SA), Cloud SQL vs demo, dominio
- [ ] D.2 Workflow: WIF → push Artifact Registry (`southamerica-east1`) → Cloud Run
- [ ] D.3 Env/secrets en runtime (`JWT_SECRET`, `DATABASE_URL`, SMTP); imagen distroless existente
- [ ] D.4 GHA de producto (Backend/Frontend/shards/`docker-build`) sigue siendo el gate; no Cloud Build

## E. Compose Fedora después de que la nube sea fuente de verdad

- [ ] E.1 Compose que clona el contrato de prod (mismos env, misma imagen o Dockerfile)
- [ ] E.2 Sin Neo4j, sin Dockerfile Python, sin invertir el flujo “local define prod”
