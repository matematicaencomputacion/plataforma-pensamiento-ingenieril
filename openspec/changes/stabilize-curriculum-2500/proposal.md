## Why

Las olas de contenido posteriores al micro-paso 2500 produjeron inconsistencias entre el catálogo, las particiones conceptuales y los contratos E2E. Se acepta 2500 como límite canónico temporal para recuperar una base determinista y un CI confiable antes de reintroducir contenido.

## What Changes

- **BREAKING**: fijar temporalmente el catálogo navegable y validable en los micro-pasos `1..=2500`.
- Alinear la cadena curricular, las particiones conceptuales y las expectativas E2E con ese límite.
- Retirar del estado activo los generadores y validadores de las olas 26–33, conservando solo herramientas coherentes con el catálogo vigente.
- Eliminar artefactos temporales comprobados y ejecutar los gates unitarios, backend y journeys definidos por ADR 003.

### Alcance incluido

- Catálogo Rust, tests conceptuales y E2E que dependen de la cantidad de micro-pasos.
- Scripts de generación, aplicación y validación asociados a las olas del catálogo.
- Documentación OpenSpec y evidencia reproducible de validación.

### Fuera de alcance

- Generar los micro-pasos 2501 en adelante.
- Cambiar la pedagogía, el runtime Pyodide, autenticación, persistencia o APIs productivas.
- Introducir ejecución de código del alumno en el backend.

### Plan de rollback

Revertir el commit de estabilización restaura el catálogo y las herramientas previas hasta 2980. La reintroducción parcial de olas requerirá un change OpenSpec independiente y gates verdes por ola.

## Capabilities

### New Capabilities

- `curriculum-catalog-stability`: define el límite canónico del catálogo, su cadena continua y la alineación obligatoria de tests y herramientas.

### Modified Capabilities

Ninguna.

## Impact

- `web/src/curriculum.rs` y `web/src/concepts/mod.rs`.
- Tests Playwright que verifican la cantidad de pasos.
- Scripts `scripts/{gen,apply,validate}_wave*.py`.
- Pipeline local `make web-test`, `make test` y journeys ADR 003.
- No cambia ningún contrato HTTP ni dependencia productiva.
