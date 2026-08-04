# Proposal: PPI 1.1 — Fundaciones del workspace

## Why

El backend aún usa un module path placeholder (`tu-usuario`) y Go 1.22, lo que impide imports reproducibles, toolchains consistentes (macOS ARM64) y un curriculum Python versionable bajo OpenSpec. Antes de Vectorial Bloque 0 y el digrafo (PPI 1.3) necesitamos cimientos estables.

## What Changes

- **BREAKING** (imports): module path real `github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend`.
- Actualización a **Go 1.25.0** (`go` + `toolchain` en `go.mod`).
- Estructura `curriculum/python/module-01-declarative-foundations/` para el Curso Python fundamentos.
- Change OpenSpec `ppi-1-1-foundations` + endurecimiento de `openspec/config.yaml`.
- `Makefile` orientado a macOS Apple Silicon (test/fmt/build).

## Capabilities

### New Capabilities

- `workspace-foundations`: requisitos de module path, toolchain Go 1.25 y layout de curriculum/OpenSpec.

### Modified Capabilities

- _(ninguna — specs baseline aún no archivados en `openspec/specs/`)_

## Alcance incluido

- go.mod/toolchain, rewrite de imports internos.
- Scaffold del Module 1 (Declarative Foundations).
- Makefile + script de bootstrap Go para darwin/arm64.
- Artefactos OpenSpec de este change.

## Fuera de alcance

- Implementación completa de ~10 niveles Python (siguiente fase del roadmap).
- Digrafo validador (PPI 1.3).
- IA tutora endurecida (PPI 1.5).
- Merge a `main` (solo rama feature).

## Impact

- Todos los imports Go del backend.
- CI GitHub Actions deberá usar Go ≥ 1.25.
- Desarrolladores macOS ARM64: bootstrap vía `scripts/setup-go-macos.sh` o `make toolchain`.

## Plan de rollback

- Revertir `go.mod` e imports al path anterior y Go 1.22.2.
