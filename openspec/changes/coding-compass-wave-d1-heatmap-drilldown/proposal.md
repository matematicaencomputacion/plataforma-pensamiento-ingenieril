## Why

Wave D (#246, `f7734a8`) puso el heatmap de 100 décadas en `/concepts/:id`
y el clic salta **directo** al primer drill pendiente. El alumno no ve
los otros micros de esa década ni puede elegir cuál practicar. D.1 cierra
ese hueco: la celda abre una lista filtrada, y recién entonces se navega.

## What Changes

**Drill-down por década en el heatmap** (Wave D.1, TED pick #1):

1. Clic en una celda no-`empty` abre un drawer/lista de los micro-pasos
   de esa década (`lo..=hi`, 10 consecutivos) **filtrados a la partición
   activa**.
2. Clic en un ítem de esa lista → `/learn/:id` del ejercicio elegido.
3. Celdas `empty` siguen sin lista y sin navegación.
4. Teclado/a11y: el panel tiene roles (`dialog` + lista); Esc cierra.
5. La lista Wave A (`#concepts-drill-list`) permanece intacta.

## Capabilities

### New Capabilities

- _(ninguna — extiende el heatmap ya especificado)_

### Modified Capabilities

- `coding-conceptual-partitions`: clic en una década cubierta SHALL abrir
  la lista filtrada de micros de esa década; la navegación a `/learn/:id`
  ocurre al elegir un ítem, no al activar la celda.

## Alcance incluido

- Drawer/lista `#concept-decade-drawer` en `ConceptsPage`.
- Helper de micros por década (sin editar `STEP_PARTITIONS`).
- Unit tests del filtro de década + freeze `1..=1000` intacto.
- Playwright enfocado en `concepts.partitions.spec.ts` (no mega-suite
  de visual regression).
- `make web-test` verde. Spec + código en el mismo PR.

## Fuera de alcance

- #2 búsqueda facetada. #3 analytics Go. #4 DAG / Neo4j.
- Suite completa de visual regression (#5).
- Retaguear `STEP_PARTITIONS` / `1..=1000`.
- Desmontar el editor de `/learn` (este change es el hub).
- Reemplazar la lista Wave A. Recolorear FAB C3.

## Impact

- `web/src/pages/concepts.rs`, `web/src/concepts/mod.rs` (helpers), CSS
  del hub, `web/e2e/tests/concepts.partitions.spec.ts`.
- Base = `origin/main` @ `f7734a8` (Wave D #246).
- Sin API Go ni ejecución de alumnos (ADR 002).

## Plan de rollback

Revertir este PR. El heatmap Wave D (clic → primer pendiente) y la lista
Wave A siguen. El drawer D.1 no tiene persistencia.
