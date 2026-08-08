# IngenierIA — web shell (Leptos CSR)

Frontend Rust → WebAssembly. Backend Go permanece en `:8080` (`make run` desde la raíz).

## Requisitos

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk
```

## Desarrollo

```bash
# Terminal A — API Go
make run

# Terminal B — UI Leptos
cd web && trunk serve
```

Targets Makefile (desde la raíz, aislados de `make test` Go):

- `make web-test` — `cargo test` en `web/`
- `make web-build` — `trunk build --release` en `web/`

## Notas

- Legacy Qwik sigue en `frontend/` hasta el gate humano de cutover (OpenSpec `frontend-leptos-csr`).
- No ejecutar código de alumnos en el servidor (ADR 002).
