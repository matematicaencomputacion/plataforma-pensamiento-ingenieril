## Why

C6 cerró el índice applied `1..=1000` y C3 ya puso diagramas DUA en peek/dock.
El hub `/concepts/:id` sigue siendo una lista larga + un `%`: el alumno no ve
dónde está el dominio a lo largo del rail ni puede saltar por cobertura.
Las particiones 4–5 siguen `map_only` con la misma lista (el “mapa” nunca
fue mapa). Wave D abre esa capa visual **sin** retaguear ni analytics Go.

## What Changes

**Heatmap de navegación conceptual en el hub** (Wave D):

1. Grilla de cobertura por décadas del rail (`1–10` … `991–1000`) en
   `/concepts/:id`, derivada de `STEP_PARTITIONS ∩ completed_levels`.
2. Clic en una década con drills de esa partición → primer drill pendiente
   (o el primero si todos están hechos) en `/learn/:step`.
3. Color no es el único canal: `data-state` + texto accesible (`2/3`).
4. La lista de drills Wave A permanece (títulos); el heatmap no la sustituye.
5. Smoke Playwright: heatmap visible → clic → land en learn; editor/rail
   intactos.

## Capabilities

### New Capabilities

- _(ninguna — extiende el hub ya especificado)_

### Modified Capabilities

- `coding-conceptual-partitions`: el hub SHALL mostrar un heatmap de
  cobertura por décadas y permitir saltar al drill desde una celda,
  sin mutar el índice ni el progreso.

## Alcance incluido

- Widget `#concept-heatmap` en `ConceptsPage` (las 5 particiones, incluidas
  4–5 `map_only`).
- Modelo cliente de celdas (100 décadas; estados `empty|pending|partial|done`).
- Unit tests del modelo + freeze de `STEP_PARTITIONS` (ningún retag
  `1..=1000`).
- Extender `concepts.partitions.spec.ts` (no journey auth/hub nuevo).
- `make web-test` verde. Implementación en PR propio **después** de mergear
  este change de planificación.

## Fuera de alcance

- Retaguear `STEP_PARTITIONS` / `1..=1000` / freeze `>1000`.
- Endpoints Go, eventos de analytics, tiempo o reintentos.
- Enciclopedia DUA (más SVG), audio, TTS, sandbox, boss fights.
- Reemplazar la lista de drills; search en header; atajos `Ctrl/Cmd+1..5`.
- DAG / Neo4j / tracks de progreso por partición.
- Cambiar `current_level` / `completed_levels`.
- Código de producto en este PR de planificación.

## Impact

- `web/src/pages/concepts.rs`, `web/src/concepts/mod.rs` (helpers de
  celdas, no el índice), CSS del hub, `web/e2e/tests/concepts.partitions.spec.ts`.
- Base = `origin/main` @ `87a5334` (C6 #244). C3 peek/dock no se toca.
- Sin API Go ni ejecución de alumnos (ADR 002).

## Plan de rollback

Revertir el PR de implementación D. El hub Wave A (lista + %) y el drawer
C3 siguen. El heatmap no tiene persistencia.
