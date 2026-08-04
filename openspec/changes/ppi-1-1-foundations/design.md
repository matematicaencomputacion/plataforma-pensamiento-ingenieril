## Context

Monorepo PPI con backend Go en `backend/` (Clean Architecture) y frontend Qwik. Roadmap hacia Spark beta: PPI 1.1 → Vectorial B0 → PPI 1.2/1.3 → Curso Python → PPI 1.5.

## Goals / Non-Goals

**Goals:**

- Module path real e importable.
- Go 1.25.0 fijado para builds reproducibles (macOS ARM64 y CI).
- Layout OpenSpec + curriculum Python Module 1.
- DX local: `make test|fmt|build` rápidos.

**Non-Goals:**

- Autoría completa de los ~10 niveles.
- Migración del jsonstore a Postgres.
- Cambios de API REST en esta fase.

## Decisions

1. **Module path** = `github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend` (módulo acotado al backend del monorepo).
2. **Toolchain** = `go 1.25.0` + `toolchain go1.25.0` con `GOTOOLCHAIN=auto`.
3. **Curriculum** vive en `curriculum/` (contenido pedagógico versionado) separado de `backend/data/` (runtime jsonstore).
4. **Makefile** detecta `darwin/arm64` y usa `GOOS`/`GOARCH` nativos; binario en `backend/bin/ppi-api`.
5. **main limpia**: todo el trabajo en `feat/ppi-1.1-foundations`.

## Risks / Trade-offs

- CI actual puede fallar hasta actualizar setup-go a 1.25.
- `GOTOOLCHAIN=auto` descarga ~50MB la primera vez en máquinas sin 1.25.
