## Context

Wave D.1 (#247, `362ec98`) dejó heatmap de 100 décadas + drawer por
década en `/concepts/:id`. Wave B ya tiene `search_glossary` +
`PartitionId` en el FAB de `/learn` (`Ctrl/Cmd+K`). El índice
`STEP_PARTITIONS` cubre `1..=1000` (multi-label). Stack: Leptos CSR,
progreso solo desde `completed_levels` (ADR 002).

## Goals / Non-Goals

**Goals**

- Cruzar la partición activa con (a) query de glosario/título y
  (b) chips AND de otras particiones, y recortar lista + heatmap a esa
  intersección.
- Tokens de query en AND (`recursion dfs`): un drill matchea un token
  si el título/id lo contiene **o** un hit de `search_glossary` apunta
  a su `related_step_id`.
- Chips extra: el drill debe llevar **todos** esos tags además del de
  la pestaña activa.
- Playwright: barra visible, aplicar faceta, lista/heatmap cambian,
  clic → `/learn/:id`. FAB en `/learn` no se toca.
- Freeze `1..=1000` intacto.

**Non-Goals**

- Analytics Go, DAG, visual-regression suite, retag `1..=1000`.
- Rebind Ctrl/Cmd+K en el hub (el atajo sigue siendo del FAB en
  `/learn`).
- Persistencia de filtros (query string / localStorage).

## Decisions

1. **Filtro en el hub, no en el FAB**
   - IDs nuevos: `#concept-facet-bar`, `#concept-facet-query`,
     `#concept-facet-p{n}`, `#concept-facet-count`.
   - No reutilizar `#concept-glossary-search` ni `#concept-lens-n`.
   - Sin listener Ctrl/Cmd+K en `/concepts`.

2. **AND, no OR**
   - Query tokens AND extra-partition tags AND partición activa.
   - Vacío = comportamiento D/D.1 (lista completa de la pestaña).

3. **Heatmap derivado del set filtrado**
   - Reusar el modelo de celdas (`empty|pending|partial|done`) sobre
     `filtered_drills`, no un quinto estado. Décadas sin match =
     `empty` (drawer no abre).
   - Celdas con match exponen `data-facet="hit"` para el smoke.
   - Drawer D.1 lista `filtered ∩ década`, no el set crudo de la
     partición.

4. **Índice intocado**
   - `STEP_PARTITIONS` read-only. `search_glossary` / `GLOSSARY_ENTRIES`
     read-only (no seed nuevo). El freeze Wave D sigue siendo el gate.

5. **Gates**
   - Unit: `append` en P1 incluye micro 20 (related `py-20-list-change`)
     y excluye micro 1; `recursion`+`dfs` en P3 incluye 109 y no 133;
     P1+chip P3 ⊆ tags `[1,3]`; freeze `1..=1000`.
   - Playwright en spec de concepts (no mega-suite visual).
   - `make web-test`. E2E CI = `trunk build` + `STATIC_DIR` (no
     `trunk serve`).

## Risks / Trade-offs

| Riesgo | Mitigación |
|---|---|
| Query huérfana (0 drills) | conteo `0 drills`; heatmap todo `empty`; clear |
| Confundir con FAB Wave B | IDs y página distintos; sin Ctrl+K en el hub |
| AND demasiado estricto | documentado; un token o un chip basta para el smoke |
| related_step fuera de la pestaña | el filtro parte de `drills_for_partition`; no se cuela |

## Migration Plan

1. Change OpenSpec + implementación en
   `feat/coding-compass-wave-d2-concept-search`.
2. Helpers + barra + heatmap/lista/drawer cableados al filtro.
3. `make web-test` + smoke Playwright.
4. PR TED. Rollback: revert; hub D.1 vuelve.

## Open Questions

- ¿Sincronizar la query con `?q=`? → **No** en D.2 (sin persistencia).
