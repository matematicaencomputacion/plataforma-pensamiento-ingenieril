## Slide 1 — Hook
Cloud Run servía `web/index.html` fuente (`data-trunk`) → body vacío, sin Wasm.

## Slide 2 — Insight
Hace falta el `dist/` de `trunk build --release` (con `import init`). Un STATIC_DIR mal apuntado o un copy incompleto deja el HTML sin bootstrap.

## Slide 3 — Move
- Dockerfile: assert anti-`data-trunk` + copy `dist/` → `backend/static` pre-`go build`
- `go:embed` del dist Trunk + rechazo de index fuente
- Fallback disco solo si el index es dist real

## Slide 4 — Proof
- `make harness`
- Asserts en imagen: `import init` + `*.wasm`

## Slide 5 — Ask
Merge a `main` y forzá rebuild Cloud Run (sin cache de capas web si hace falta).
