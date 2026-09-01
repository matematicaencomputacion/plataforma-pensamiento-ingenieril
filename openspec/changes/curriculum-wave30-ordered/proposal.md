## Why

Ola 29 ya está integrada y verificada hasta el ejercicio 2740, pero la rama histórica de Ola 30 acumula olas anteriores y repite exactamente el contenido de Olas 26 y 29. Ola 30 debe reconstruirse sobre el `main` verificado como una progresión original de reconciliación determinista y agregación online.

## What Changes

- Agregar exactamente 60 ejercicios originales de merge ordenado, heaps, top-k, estadísticas online, ventanas incrementales, checkpoints puros, idempotencia y reconciliación determinista para `2741..=2800`.
- Enlazar 2740 con 2741 y dejar 2800 como único terminal del rail.
- Extender catálogo, particiones conceptuales, pruebas Rust y tres conteos E2E al techo 2800.
- Incorporar generación, aplicación y validación deterministas de Ola 30, incluida ejecución real de las 60 soluciones y no duplicación con Olas 26–29.
- Registrar como cerrados los gates post-merge de Ola 29 antes de integrar Ola 30.

### Alcance incluido

- Ola 30 y únicamente el rango `2741..=2800`.
- Algoritmos deterministas y stdlib-only para streams ya disponibles en memoria.
- Estado explícito e inmutable o copiado para checkpoints y reanudación reproducible.
- Ejecución Python enteramente en el navegador, sin cambios de API.

### Fuera de alcance

- Ola 31 o identificadores mayores a 2800.
- Threads, procesos, async, red, archivos externos, reloj de pared o aleatoriedad.
- Repetir chunks, fan-out/fan-in, map-reduce o pipelines lazy de Olas 26–29.
- Backend, persistencia, infraestructura o búsqueda global.
- Fusionar o reutilizar la rama histórica acumulativa.

### Plan de rollback

- Revertir el squash de Ola 30 para restaurar el catálogo estable `1..=2740`.
- Restaurar `PY2740_OLA29_SUITE.next` a `None`, retirar `2741..=2800` y devolver particiones y conteos a 2740.

## Capabilities

### New Capabilities

- `curriculum-wave30-stream-reconciliation`: extensión contigua y verificable del catálogo con merge ordenado, agregación online, checkpoints puros e idempotencia entre 2741 y 2800.

### Modified Capabilities

Ninguna.

## Impact

- Código: `web/src/curriculum.rs`, `web/src/concepts/mod.rs` y scripts de Ola 30.
- Pruebas: unit tests Rust, validadores Python y tres journeys E2E canónicos.
- Catálogo visible: pasa de 2740 a 2800 sin nuevas dependencias ni APIs.
- Integración: un único PR sobre `main`; Ola 31 permanece bloqueada hasta verificar CI, Docker, E2E y Deploy post-merge.
