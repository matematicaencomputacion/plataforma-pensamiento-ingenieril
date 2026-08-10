## Slide 1 — Hook
Prod Wasm crasheaba al init: `Table.grow()` / `__wbindgen_init_externref_table` → pantalla en blanco.

## Slide 2 — Insight
`data-wasm-opt="z"` + Binaryen viejo (apt bookworm) rompe la tabla externref de wasm-bindgen 0.2.127.

## Slide 3 — Move
- `data-wasm-opt="0"`, pin `[tools] wasm_bindgen = "0.2.127"`
- `.cargo/config.toml` con `+bulk-memory,+reference-types`
- Dockerfile sin binaryen apt; release `opt-level = "s"`

## Slide 4 — Proof
- `cd web && trunk build --release`
- `make harness` / CI del PR

## Slide 5 — Ask
Merge a `main` y redeploy Cloud Run (rebuild web stage sin cache).
