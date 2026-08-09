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

# Terminal B — UI Leptos en :3001 (proxy /api → :8080; evita CORS)
cd web && env -u NO_COLOR trunk serve
```

El cliente usa rutas relativas (`/api/...`). `Trunk.toml` define `[[proxy]]` con
`rewrite = "/api/"` → `http://127.0.0.1:8080/api/` (trailing slash obligatorio;
sin él Trunk cae al SPA y responde **405** a POST).

**Importante:** no levantar otro `trunk serve` en `:8080`. Ese puerto es exclusivo del API Go (`make run`). Si Trunk ocupa `:8080`, el proxy reenvía al SPA y el login/registro fallan con 405.

Rutas del spike auth:

| Ruta | Descripción |
|---|---|
| `/` | Portada + CTAs registro/login |
| `/login` | Login real → `POST /api/auth/login` |
| `/register` | Registro → `POST /api/auth/register` |
| `/workspace` | Placeholder post-auth (harness Qwik intacto) |

Token Bearer en `localStorage` (`ppi.auth.token`), mismos contratos JSON que el cliente Qwik.

Targets Makefile (desde la raíz, aislados de `make test` Go):

- `make web-test` — `cargo test` en `web/`
- `make web-build` — `trunk build --release` en `web/`

## Notas

- Legacy Qwik sigue en `frontend/` hasta el gate humano de cutover (OpenSpec `frontend-leptos-csr`).
- No ejecutar código de alumnos en el servidor (ADR 002).
