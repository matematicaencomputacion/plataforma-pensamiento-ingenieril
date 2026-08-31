## Why

Ola 27 está integrada y verificada en producción, pero la rama histórica de Ola 28 no representa una implementación confiable: acumula Olas 26–30, reutiliza literalmente el generador de Ola 24 y contiene un aplicador de placeholders que no prueba soluciones reales. Ola 28 debe reconstruirse sobre el `main` verificado con contenido original y un alcance único comprobable.

## What Changes

- Agregar exactamente 60 ejercicios originales de pipelines paralelizables y reducción avanzada para `2621..=2680`.
- Enseñar particionado, batching, alineación, fan-out/fan-in, reducciones parciales y combinación asociativa mediante simulaciones deterministas en Python cliente.
- Enlazar el micro-step 2620 con 2621 y dejar 2680 como único terminal del rail.
- Extender catálogo, particiones conceptuales, pruebas Rust y tres conteos E2E al techo 2680.
- Incorporar generación, aplicación y validación deterministas de Ola 28, incluida no duplicación contra Olas 26 y 27.
- Cerrar documentalmente los gates post-merge de Ola 27 antes de integrar Ola 28.

### Alcance incluido

- Ola 28 y únicamente el rango `2621..=2680`.
- Pipelines paralelizables modelados como transformaciones puras, chunks, resultados parciales y combinación ordenada.
- Reducciones asociativas, agrupación, ventanas y un cierre map-reduce local acotado.
- Ejecución Python enteramente en el navegador, sin cambios de API.

### Fuera de alcance

- Ola 29 o identificadores mayores a 2680.
- Threads, procesos, red o paralelismo real dentro del runtime del alumno.
- Repetir los ejercicios de pipelines de Ola 26 o recursión/comprensiones de Ola 27.
- Backend, persistencia, infraestructura o búsqueda global.
- Fusionar o reutilizar la rama histórica acumulativa de Olas 28–30.

### Plan de rollback

- Revertir el squash de Ola 28 para restaurar el catálogo estable `1..=2620`.
- Restaurar `PY2620_OLA27_SUITE.next` a `None`, retirar `2621..=2680` y devolver particiones y conteos a 2620.

## Capabilities

### New Capabilities

- `curriculum-wave28-parallel-reduction`: Define la extensión original, contigua y comprobable del catálogo con particionado, fan-out/fan-in y reducción avanzada entre 2621 y 2680.

### Modified Capabilities

Ninguna.

## Impact

- Código: `web/src/curriculum.rs`, `web/src/concepts/mod.rs` y scripts de Ola 28.
- Pruebas: unit tests Rust, validadores Python y tres journeys E2E canónicos.
- Catálogo visible: pasa de 2620 a 2680 sin nuevas dependencias ni APIs.
- Integración: un único PR sobre `main`; Ola 29 permanece bloqueada hasta verificar CI, Docker, E2E y Deploy post-merge.
