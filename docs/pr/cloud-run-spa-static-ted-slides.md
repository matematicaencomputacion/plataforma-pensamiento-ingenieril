## Slide 1 — Hook
Cloud Run devolvía 404 en `/`: la API Go no servía el dist de Trunk/Leptos.

## Slide 2 — Insight
La imagen solo tenía el binario Go. Sin `STATIC_DIR` + fallback SPA, Leptos Router nunca carga.

## Slide 3 — Move
- Dockerfile: etapa Rust/Trunk → `/app/static`
- SPA handler: assets reales + `index.html` fuera de `/api/`
- Tests de raíz, asset, fallback y aislamiento API

## Slide 4 — Proof
- `cd backend && go test .` (SPA)
- `make harness`
- CI del PR

## Slide 5 — Ask
Review/merge a `main` para redeploy Cloud Run (Developer Connect).
