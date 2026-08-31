## Why

Ola 26 está integrada y verificada en `main`, pero la implementación histórica de Ola 27 contradice su propio título: declara recursión y comprensiones mientras duplica literalmente los ejercicios de pipelines de Ola 26. Ola 27 debe reconstruirse con contenido pedagógico original y un alcance único verificable.

## What Changes

- Agregar exactamente 60 ejercicios originales de recursión y comprensiones para `2561..=2620`.
- Encadenar el micro-step 2560 con 2561 y dejar 2620 como único terminal del rail.
- Extender catálogo, particiones conceptuales, pruebas Rust y tres conteos E2E al techo 2620.
- Incorporar generación, aplicación y validación deterministas de Ola 27.
- Cerrar documentalmente el gate post-merge de Ola 26 antes de integrar Ola 27.

### Alcance incluido

- Ola 27 y únicamente el rango `2561..=2620`.
- Recursión segura, casos base, descomposición, comprensiones de listas/dicts/sets y combinaciones acotadas.
- Ejecución Python enteramente en el navegador, sin cambios de API.

### Fuera de alcance

- Ola 28 o identificadores mayores a 2620.
- Copiar los ejercicios de pipelines y reducción de Ola 26.
- Backend, persistencia, infraestructura o búsqueda global.
- Fusionar la rama histórica acumulativa de Olas 27–30.

### Plan de rollback

- Revertir el squash de Ola 27 para restaurar el catálogo estable `1..=2560`.
- Restaurar `PY2560_SCORE_CHECK.next` a `None`, retirar `2561..=2620` y devolver particiones y conteos a 2560.

## Capabilities

### New Capabilities

- `curriculum-wave27-recursion-comprehensions`: Define la extensión original, contigua y comprobable del catálogo con recursión y comprensiones entre 2561 y 2620.

### Modified Capabilities

Ninguna.

## Impact

- Código: `web/src/curriculum.rs`, `web/src/concepts/mod.rs` y scripts de Ola 27.
- Pruebas: unit tests Rust, validadores Python y tres journeys E2E canónicos.
- Catálogo visible: pasa de 2560 a 2620 sin nuevas dependencias ni APIs.
- Integración: un único PR sobre `main`; Ola 28 permanece bloqueada hasta verificar los workflows post-merge.
