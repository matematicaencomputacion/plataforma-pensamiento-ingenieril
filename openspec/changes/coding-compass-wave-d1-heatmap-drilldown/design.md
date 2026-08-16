## Context

Wave D ya entregó la grilla de 100 celdas (`#concept-heatmap`) derivada
de `drills_for_partition(id) ∩ completed_levels` por década. El clic
usa `heatmap_click_target` → primer pendiente (o el primero si todos
done). Base = `origin/main` @ `f7734a8`. Stack: Leptos CSR, CSS del
crate, progreso solo desde `completed_levels` (ADR 002).

## Goals / Non-Goals

**Goals**

- Clic no-`empty` abre drawer con los micros de esa década filtrados a
  la partición activa (máximo 10; suele ser menos si hay untagged u
  otras lentes).
- Elegir un ítem navega a `/learn/:id`.
- `empty` no abre panel ni cambia la URL.
- Roles + Esc. Lista Wave A intacta.
- Playwright: heatmap → clic década → lista de esos micros → clic →
  `/learn/:id`; celda vacía sin lista/nav.

**Non-Goals**

- Faceted search, analytics Go, DAG, visual-regression suite.
- Retag `1..=1000`. Tocar `/learn` o el FAB C3.
- Grilla de 1000 celdas. Reemplazar `#concepts-drill-list`.

## Decisions

1. **Lista filtrada, no salto directo**
   - La celda deja de ser `<A href="/learn/…">`. Pasa a botón que
     abre `#concept-decade-drawer`.
   - Ítems = `heatmap_decade_drills(partition_id, band)` (tagged de
     esta partición cuyo `micro_step` cae en `lo..=hi`).
   - `heatmap_click_target` permanece como helper de “primer pendiente”
     (tests Wave D); el hub ya no navega con él.

2. **Drawer en el hub, no el dock C3**
   - IDs nuevos (`#concept-decade-drawer`, `#concept-decade-list`,
     `#concept-decade-drill-{n}`). No reutilizar `#concept-drawer`.
   - `role="dialog"` + `aria-modal="true"` + título de década.
   - Esc y botón Cerrar. Overlay cierra. Cambiar de pestaña de
     partición cierra.

3. **Índice intocado**
   - `STEP_PARTITIONS` read-only. El freeze Wave D `1..=1000` sigue
     siendo el gate.

4. **Gates**
   - Unit: década `1..=10` en P1 no incluye micros de otras lentes;
     década `empty` → vec vacío.
   - Playwright en `concepts.partitions.spec.ts` (extender, no suite
     nueva de visual regression).
   - `make web-test`. Sin `make harness` completo.

## Risks / Trade-offs

| Riesgo | Mitigación |
|---|---|
| Década con 2 drills vs “los 10” | el rango es 10 slots; la lista es el filtro de partición |
| Confundir con drawer C3 | IDs y página distintos (`/concepts`, no `/learn`) |
| Regresión Wave A | `#concepts-drill-list` no se toca; smoke sigue clicando Ej 20 |

## Migration Plan

1. Change OpenSpec + implementación en `feat/coding-compass-wave-d1-heatmap-drilldown`.
2. Helpers + drawer + tests.
3. `make web-test` + smoke Playwright del drawer.
4. PR TED. Rollback: revert; Wave D clic-directo vuelve.

## Open Questions

- ¿Listar también micros untagged de la década? → **No.** Solo la
  partición activa (el heatmap ya es por lente).
