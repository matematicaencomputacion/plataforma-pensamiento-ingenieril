## ADDED Requirements

### Requirement: Module Path Importable

El backend SHALL declarar un module path público y estable bajo la organización GitHub del proyecto, sin placeholders.

#### Scenario: Import interno resuelve

- GIVEN el módulo `github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend`
- WHEN un paquete interno importa `.../backend/internal/domain`
- THEN `go test ./...` y `go build` completan sin errores de resolución de módulos

### Requirement: Toolchain Go 1.25

El backend SHALL fijar Go 1.25.0 (o superior compatible en la serie 1.25) mediante `go.mod` y `toolchain`.

#### Scenario: Build con toolchain automática

- GIVEN un entorno con `GOTOOLCHAIN=auto`
- WHEN se ejecuta `go test ./...` desde `backend/`
- THEN la toolchain Go 1.25 se usa o se descarga automáticamente

### Requirement: Curriculum Python Module 1 Layout

El repositorio SHALL exponer un directorio versionable para el Module 1 — Declarative Foundations (variables, tipos, foundations declarativas).

#### Scenario: Scaffold presente

- GIVEN el path `curriculum/python/module-01-declarative-foundations/`
- WHEN un autor de contenido inspecciona el repo
- THEN existen subdirectorios `concepts/`, `levels/` y `assessments/` listos para poblar niveles
