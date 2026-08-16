## ADDED Requirements

### Requirement: Single runtime image serves API and SPA

The repository SHALL provide a multi-stage Dockerfile at the monorepo root whose final stage is a single `linux/amd64` image running the static Go binary. That binary SHALL serve `/api/*` and the Trunk-built SPA (embedded and/or `STATIC_DIR`).

#### Scenario: Health and SPA index from one container

- **GIVEN** an image built from the root Dockerfile with `PPI_BUILD_ID` set
- **WHEN** the container starts with `PORT` and `DATABASE_URL` supplied via environment
- **THEN** `GET /api/health` returns JSON `"status":"ok"` and `GET /` is a Trunk dist (`import init`, no `data-trunk`)

### Requirement: Static binary on a distroless runtime

The final image SHALL use a distroless (or scratch) base with a `CGO_ENABLED=0` linux/amd64 binary and a non-root user. The image SHALL NOT contain JWT secrets, API keys, or SMTP passwords.

#### Scenario: Runtime config is environment-only

- **GIVEN** the built image
- **WHEN** an operator inspects image config
- **THEN** `PORT` and `DATABASE_URL` are overridable env vars and no secret-bearing env (JWT_SECRET, SMTP_PASSWORD, CEREBRAS_API_KEY, GROK_API_KEY) is baked in

### Requirement: GitHub Actions builds the image

PRs targeting `main` SHALL run a `docker-build` job that builds the image in GitHub Actions and smokes `GET /api/health`. The job SHALL NOT push to Artifact Registry.

#### Scenario: PR check without a local Docker daemon

- **GIVEN** a pull request that includes the Dockerfile
- **WHEN** GitHub Actions runs
- **THEN** the `docker-build` check builds `ppi:ci` (and `ppi:<sha>`) on the runner and the health smoke passes
