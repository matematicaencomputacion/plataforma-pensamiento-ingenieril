## 1. Toolchain y módulo

- [x] 1.1 Actualizar `backend/go.mod` a Go 1.25.0 + toolchain
- [x] 1.2 Reescribir imports al module path real
- [ ] 1.3 Actualizar `.github/workflows/ci.yml` a Go 1.25 (si CI falla)

## 2. OpenSpec y curriculum

- [x] 2.1 Crear change `ppi-1-1-foundations` con proposal/design/specs/tasks
- [x] 2.2 Scaffold `curriculum/python/module-01-declarative-foundations/`
- [x] 2.3 Actualizar `openspec/config.yaml` con contexto PPI 1.1

## 3. DX local

- [x] 3.1 Añadir `Makefile` (test/fmt/build/run)
- [x] 3.2 Añadir `scripts/setup-go-macos.sh` para Apple Silicon
- [ ] 3.3 Verificar `make test` y `make build` en macOS ARM64 nativo

## 4. Cierre

- [ ] 4.1 `openspec validate ppi-1-1-foundations`
- [ ] 4.2 Commit en `feat/ppi-1.1-foundations` (sin merge a main)
