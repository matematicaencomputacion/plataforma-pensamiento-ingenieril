## 1. Image

- [x] 1.1 Multi-stage Dockerfile: Trunk dist → static Go (`CGO_ENABLED=0`) → distroless nonroot
- [x] 1.2 Embed Trunk `web/dist` before `go build`; keep `STATIC_DIR=/app/static` fallback
- [x] 1.3 No secrets in image; `PORT` / `DATABASE_URL` from env (sqlite `/tmp` default)

## 2. Context and env

- [x] 2.1 `.dockerignore`: do not exclude `backend/data` seeds
- [x] 2.2 `.env.example`: `PORT` + `DATABASE_URL` placeholders (no `ANALYTICS_SECRET`; D.3 no lo usa)

## 3. CI

- [x] 3.1 GHA job `docker-build`: buildx load `ppi:ci` / `ppi:<sha>` + smoke `/api/health`
- [x] 3.2 Makefile `docker-build` / `docker-smoke` (opt-in; not part of `make ci`)
- [x] 3.3 Trigger Playwright aggregator when Dockerfile/CI docker change (path filter)
