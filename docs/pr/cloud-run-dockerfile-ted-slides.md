# PR TED slides — Cloud Run Dockerfile

## Slide 1 — Hook
Cloud Run no puede buildear el monorepo a ciegas: hace falta un Dockerfile en la raíz apuntando a `backend/`.

## Slide 2 — Insight
`go.mod` exige Go **1.25** (no 1.22). Cloud Run inyecta `$PORT`; el binario debe respetarlo y llevar seeds `data/`.

## Slide 3 — Move
- `Dockerfile` multi-stage + `.dockerignore`
- `listenAddr()` respeta `PORT`
- Defaults Cloud Run: `DATA_DIR`, SQLite en `/tmp`

## Slide 4 — Proof
- `go test` (listenAddr) + build Docker cuando haya daemon

## Slide 5 — Ask
Merge a `main` para disparar el deploy automático vía Developer Connect.
