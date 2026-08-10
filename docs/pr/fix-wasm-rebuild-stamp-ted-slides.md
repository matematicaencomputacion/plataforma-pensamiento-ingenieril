## Slide 1 — Hook
Tras #48 Cloud Run seguía sirviendo Wasm con `Table.grow` — artefacto viejo o release demasiado agresivo.

## Slide 2 — Insight
BuildKit puede cachear la etapa web; sin stamp verificable no hay prueba de redeploy. LTO + reference-types forzados empeoran externref.

## Slide 3 — Move
- `PPI_BUILD_ID` cache-bust + `ppi-build.txt` embebido + `GET /api/spa-build`
- Release: `opt-level=2`, sin LTO; solo `+bulk-memory`
- `Cache-Control: no-store` en `index.html`

## Slide 4 — Proof
- `make harness-unit`
- Tras deploy: `curl .../api/spa-build` debe mostrar `stamp-v3-20260810-table-grow`

## Slide 5 — Ask
Merge + forzá rebuild Cloud Run. Verificá stamp antes de mirar DevTools otra vez.
