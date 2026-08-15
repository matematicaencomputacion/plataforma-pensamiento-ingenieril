## Why

Wave B dejó anatomía de micro-card (TL;DR + snippet + CTA) y **aplazó**
dual-coding / DUA producto. El peek y el drawer son texto plano: el
alumno no tiene un diagrama del modelo mental (LEGB, alias, call stack)
junto al editor. C3 cierra esa capa visual **sin** retaguear 601..=1000.

**Bloqueo:** implementación C3 MUST esperar merge de
`coding-compass-wave-c1` a `main` (secuencia Wave C: índice primero,
DUA después; un solo PR de código a la vez).

## What Changes

**Diagramas visuales DUA en peek/drawer** (Wave C3):

1. Lectura por capas en Estado 2 (peek) y Estado 3 (dock): diagrama →
   TL;DR → snippet → pitfall. El diagrama no sustituye el texto
   (dual-coding; el color no es el único canal).
2. SVG inline estático en el glosario cliente (semilla de modelos
   mentales, no enciclopedia). Sin red, sin Go, sin Mermaid runtime.
3. Smoke E2E: el diagrama es visible en peek/dock y el editor conserva
   el marcador.

## Capabilities

### New Capabilities

- _(ninguna — extiende el drawer ya especificado)_

### Modified Capabilities

- `coding-concept-drawer`: peek y dock SHALL mostrar un diagrama
  visual opcional por lente/entrada de modelo mental, en lectura
  estratificada, sin desmontar `#learn-editor`.

## Alcance incluido

- Campo opcional de diagrama en el modelo de glosario (additive;
  `PartitionId` / `search_glossary` intactos).
- Seed SVG para entradas `model-*` canónicas (`python-lists`,
  `model-legb`, `model-mutability`, `model-recursion` como mínimo).
- Render en `ConceptMicroCard` (peek + dock) con `#concept-diagram`
  y texto accesible.
- Unit tests del seed + Playwright smoke extendiendo
  `concepts.drawer.spec.ts`.
- `make web-test` verde. Implementación en PR propio **después** de C1.

## Fuera de alcance

- Retaguear `601..=1000` o cualquier `STEP_PARTITIONS`.
- Tags C1/C2 (`301..=600`).
- Audio, TTS, “Libres para Aprender” completo, sandbox, boss fights.
- Mapas ricos 4–5, analytics Go, búsqueda en el header.
- Enciclopedia de 1000 diagramas; imágenes remotas; Mermaid.js.
- Cambiar `current_level` / `completed_levels`.
- Código de producto en este PR de planificación.

## Impact

- `web/src/concepts/glossary.rs`, `concept_fab.rs`, CSS del peek,
  `web/e2e/tests/concepts.drawer.spec.ts`.
- Base = `main` **post-C1**. No conflictúa con el índice si C2 aún no
  arrancó; no se implementa en paralelo a C1.
- Sin API Go ni ejecución de alumnos (ADR 002).

## Plan de rollback

Revertir el PR de implementación C3. El FAB Wave B (texto) y el índice
C1/C2 siguen. Los SVG no tienen persistencia.
