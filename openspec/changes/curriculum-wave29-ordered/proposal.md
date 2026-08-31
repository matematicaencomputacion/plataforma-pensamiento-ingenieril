## Why

Ola 28 ya está integrada y verificada en producción, pero la rama histórica de Ola 29 acumula Olas 26–30 y combina un generador útil con un aplicador de placeholders y un validador que no ejecuta las soluciones. Ola 29 debe reconstruirse sobre el `main` verificado como una extensión única, lazy y comprobable del catálogo.

## What Changes

- Agregar exactamente 60 ejercicios originales de funciones de orden superior, pipelines lazy, folding, logs, cortocircuito y agregación streaming para `2681..=2740`.
- Enlazar 2680 con 2681 y dejar 2740 como único terminal del rail.
- Extender catálogo, particiones conceptuales, pruebas Rust y tres conteos E2E al techo 2740.
- Incorporar generación, aplicación y validación deterministas de Ola 29, incluida ejecución real de las 60 soluciones y no duplicación con Olas 26–28.
- Registrar como cerrados los gates post-merge de Ola 28 antes de integrar Ola 29.

### Alcance incluido

- Ola 29 y únicamente el rango `2681..=2740`.
- Funciones como valores, callbacks, generadores, materialización controlada, folding, pipelines de logs, terminación temprana, streaming y ranking determinista.
- Ejecución Python enteramente en el navegador, sin cambios de API.

### Fuera de alcance

- Ola 30 o identificadores mayores a 2740.
- Threads, procesos, red, archivos externos o paralelismo real.
- Repetir los ejercicios de chunks, fan-out/fan-in y map-reduce de Ola 28.
- Backend, persistencia, infraestructura o búsqueda global.
- Fusionar o reutilizar la rama histórica acumulativa.

### Plan de rollback

- Revertir el squash de Ola 29 para restaurar el catálogo estable `1..=2680`.
- Restaurar `PY2680_OLA28_SUITE.next` a `None`, retirar `2681..=2740` y devolver particiones y conteos a 2680.

## Capabilities

### New Capabilities

- `curriculum-wave29-lazy-streaming`: extensión contigua y verificable del catálogo con funciones de orden superior, consumo lazy, folding y agregación streaming entre 2681 y 2740.

### Modified Capabilities

Ninguna.

## Impact

- Código: `web/src/curriculum.rs`, `web/src/concepts/mod.rs` y scripts de Ola 29.
- Pruebas: unit tests Rust, validadores Python y tres journeys E2E canónicos.
- Catálogo visible: pasa de 2680 a 2740 sin nuevas dependencias ni APIs.
- Integración: un único PR sobre `main`; Ola 30 permanece bloqueada hasta verificar CI, Docker, E2E y Deploy post-merge.
