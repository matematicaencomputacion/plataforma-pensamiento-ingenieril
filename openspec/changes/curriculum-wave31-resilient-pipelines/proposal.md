## Why

Ola 30 ya está integrada y verificada hasta el ejercicio 2800, pero la rama histórica de Ola 31 repite exactamente los 60 ejercicios de Ola 26 y contiene integración incompleta. Ola 31 debe reconstruirse sobre el `main` verificado como una progresión original de fiabilidad determinista para pipelines.

## What Changes

- Agregar exactamente 60 ejercicios originales de validación estructurada, errores acumulados, cuarentena, reintentos puros, circuit breaker, límites por ticks, watermarks, eventos tardíos, compensaciones y recuperación para `2801..=2860`.
- Enlazar 2800 con 2801 y dejar 2860 como único terminal del rail.
- Extender catálogo, particiones conceptuales, pruebas Rust y tres conteos E2E al techo 2860.
- Incorporar generación, aplicación y validación deterministas de Ola 31, incluida ejecución real de las 60 soluciones y no duplicación con Olas 26–30.
- Registrar como cerrados los gates post-merge de Ola 30 antes de integrar Ola 31.

### Alcance incluido

- Ola 31 y únicamente el rango `2801..=2860`.
- Modelado de fallos y recuperación como transformaciones puras sobre datos en memoria.
- Tiempo lógico representado por enteros explícitos en entradas y estado.
- Ejecución Python enteramente en el navegador, sin cambios de API.

### Fuera de alcance

- Ola 32 o identificadores mayores a 2860.
- Threads, procesos, async, sleeps, red, archivos externos, reloj de pared o aleatoriedad.
- Repetir pipelines funcionales, agregación online o reconciliación de Olas 26–30.
- Backend, persistencia, infraestructura o búsqueda global.
- Fusionar o reutilizar la rama histórica acumulativa.

### Plan de rollback

- Revertir el squash de Ola 31 para restaurar el catálogo estable `1..=2800`.
- Restaurar `PY2800_OLA30_SUITE.next` a `None`, retirar `2801..=2860` y devolver particiones y conteos a 2800.

## Capabilities

### New Capabilities

- `curriculum-wave31-resilient-pipelines`: extensión contigua y verificable del catálogo con validación, aislamiento de fallos y recuperación determinista entre 2801 y 2860.

### Modified Capabilities

Ninguna.

## Impact

- Código: `web/src/curriculum.rs`, `web/src/concepts/mod.rs` y scripts de Ola 31.
- Pruebas: unit tests Rust, validadores Python y tres journeys E2E canónicos.
- Catálogo visible: pasa de 2800 a 2860 sin nuevas dependencias ni APIs.
- Integración: un único PR sobre `main`; Ola 32 permanece bloqueada hasta verificar CI, Docker, E2E y Deploy post-merge.
