# ADR 004: Persistencia SQLite local/CI vs Cloud SQL Postgres en producción

## Estado
Aceptado

## Contexto
El PRD ya nombra PostgreSQL en producción y SQLite en desarrollo. Cloud Run
`ppi` arranca hoy con `DATABASE_URL=sqlite:///tmp/ppi.db`: el disco es efímero,
las réplicas no comparten archivo y un scale-to-zero borra usuarios y progreso.
Un JWT robusto (ADR implícito del fail-closed A) sobre una base que se evaporó
sigue siendo pérdida de datos.

## Decisión

| Entorno | Motor | `DATABASE_URL` |
|---|---|---|
| Local, harness, GHA unit/E2E, smoke Docker | SQLite | `sqlite://…` (incl. `:memory:` y `/tmp`) |
| Cloud Run durable (cuentas reales) | Cloud SQL Postgres | `postgres://` o `postgresql://` |

**Corte demo vs producto**

- **Demo efímera:** SQLite en `/tmp` (o cualquier SQLite bajo `ENV=production`).
  Los datos no sobreviven revisión, crash ni scale-to-zero. No publicar cuentas reales.
- **Producto:** Cloud SQL Postgres. Cloud Run se conecta con **Cloud SQL Auth Proxy
  (Unix socket)** `--add-cloudsql-instances=PROJECT:REGION:INSTANCE` y DSN
  `host=/cloudsql/PROJECT:REGION:INSTANCE` (`sslmode=disable` en el socket).
  IP pública de la instancia es aceptable para el proxy; no se abre Postgres a
  Internet (sin authorized networks). VPC privada queda como endurecimiento posterior.

**Fail-closed:** si `ENV`/`APP_ENV`/`GO_ENV` es `production` y `DATABASE_URL`
falta o es SQLite, el proceso **no arranca**, salvo override explícito
`PPI_ALLOW_EPHEMERAL_SQLITE=1` (solo la demo Cloud Run hasta que exista el secreto
`DATABASE_URL`). No hay “prod” silencioso sobre SQLite.

**Tests:** `go test ./...` permanece en SQLite. Postgres es opt-in
(`PPI_POSTGRES_TEST_URL`).

**Fuera de esta decisión:** Neo4j, Aura, compose Fedora (slice E).

## Consecuencias positivas
- Un solo criterio irrefutable: cuentas reales ⇒ Cloud SQL; CI/local ⇒ SQLite.
- El workflow de deploy puede montar `DATABASE_URL` desde Secret Manager cuando
  exista, sin inventar un nombre de instancia en el repo.

## Restricciones para el Agente IA
- No tratar SQLite `/tmp` en Cloud Run como persistencia de producto.
- No inventar un instance name de Cloud SQL si aún no existe; derivar
  `--add-cloudsql-instances` del DSN (`host=/cloudsql/…`) o dejarlo vacío.
- No commitear JSON de service account ni contraseñas en el repo.
- No añadir Neo4j / Bolt / sidecar Python.

## Referencias
- `docs/PRD.md` §3 (PostgreSQL prod / SQLite dev)
- `docs/ops/gcp-iteration-plan.md` (comandos humanos + riesgo `/tmp`)
- `backend/internal/config/auth.go`, `backend/internal/persistence`
