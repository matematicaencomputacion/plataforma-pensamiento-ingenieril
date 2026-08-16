## Why

El hub `/concepts/:id` ya tiene heatmap + drawer por década (D/D.1), pero
la única “lente” es la pestaña de partición: no se pueden cruzar tags
`STEP_PARTITIONS` con keywords del glosario (p. ej. recursión ∩ otra
lente) para acotar una ruta de estudio. Wave B ya busca en el FAB de
`/learn`; el hub sigue siendo una lista larga.

## What Changes

**Filtros multicriterio y búsqueda conceptual en el hub** (Wave D.2):

1. Barra de facetas en `/concepts/:id`: query (keywords del glosario +
   título/id del drill) **AND** chips de partición extra (tags
   `STEP_PARTITIONS`).
2. La lista Wave A y el heatmap se recortan al conjunto filtrado;
   décadas sin match quedan `empty` (sin drawer). El drawer D.1 lista
   solo micros de esa década que pasan el filtro.
3. Elegir un drill filtrado → `/learn/:id`. Sin filtro, el hub D/D.1
   queda igual.
4. Client-side (Leptos WASM). No se retaguea `1..=1000`. Ctrl/Cmd+K en
   `/learn` sigue abriendo el FAB.

## Capabilities

### New Capabilities

- _(ninguna — extiende el hub ya especificado)_

### Modified Capabilities

- `coding-conceptual-partitions`: el hub SHALL exponer búsqueda facetada
  (query glosario + AND de tags de partición) que filtra lista y heatmap
  sin mutar el índice ni añadir analytics Go.

## Alcance incluido

- `#concept-facet-bar` en `ConceptsPage` (query + chips AND + conteo).
- Helpers de filtro sobre `drills_for_partition` + `search_glossary`
  (related_step_id) **sin** editar `STEP_PARTITIONS`.
- Heatmap/lista/drawer D.1 reaccionan al filtro; freeze `1..=1000`.
- Playwright: UI visible → aplicar faceta → lista/heatmap cambian →
  navegar a un learn matching. FAB `/learn` intacto.
- `make web-test` verde. Spec + código en el mismo PR.

## Fuera de alcance

- #3 analytics Go. #4 DAG / Neo4j. #5 visual-regression mega-suite.
- Retaguear `STEP_PARTITIONS` / `1..=1000`.
- Desmontar el editor de `/learn` o rebind de Ctrl/Cmd+K del FAB.
- Endpoint Go de búsqueda. Reemplazar el FAB C3.

## Impact

- `web/src/pages/concepts.rs`, `web/src/concepts/mod.rs` (helpers),
  CSS del hub, `web/e2e/tests/concepts.partitions.spec.ts` (o spec
  hermano `concepts.search.spec.ts`).
- Base = `origin/main` @ `362ec98` (Wave D.1 #247).
- Sin API Go ni ejecución de alumnos (ADR 002).

## Plan de rollback

Revertir este PR. Heatmap D + drawer D.1 y lista Wave A siguen. El
filtro no tiene persistencia.
