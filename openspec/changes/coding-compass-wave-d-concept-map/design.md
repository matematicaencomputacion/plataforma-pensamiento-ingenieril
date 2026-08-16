## Context

Waves A–C6 ya entregaron: índice applied hasta 1000, mastery `%` en el
compás, journey conceptual, drawer peek/dock y diagramas DUA estáticos.
El hub `/concepts/:id` no cambió de forma: modelo + ejes + lista. Wave A
aplazó “mapas interactivos 4–5”; C3 aplazó “mapas ricos” y analytics Go.

Ahora el índice existe: el hueco es **ver y saltar**, no taggear. Base =
`origin/main` @ `87a5334`. Stack: Leptos CSR, CSS del crate, progreso
solo desde `completed_levels` (ADR 002). Sin Tailwind JIT.

## Goals / Non-Goals

**Goals**

- Heatmap de 100 celdas (décadas `1..=10` … `991..=1000`) por partición
  activa.
- Navegación: clic en década con drills → `/learn/:step` del primer
  pendiente (si todos done, el primero de la década).
- Dual-coding de cobertura: grilla + lista de drills intacta.
- Color no es el único canal (`data-state` + `aria-label` / texto `done/total`).
- Cero red / Go / retags.

**Non-Goals**

- Retag `1..=1000`. Más SVG DUA. Analytics, eventos, tiempo.
- Grilla de 1000 celdas (un micro-paso = un pixel).
- Reemplazar `#concepts-drill-list`. DAG / Neo4j.
- Recolorear `PartitionNav` o el FAB C3.

## Decisions

1. **Décadas, no un pixel por micro-paso**
   - 100 celdas fijas. Escala a P1 denso y a P4/P5 `map_only` (muchas
     `empty`).
   - Una década cuenta solo drills `drills_for_partition(id)` cuyo
     `micro_step` cae en el rango.
   - **Descartado:** 1000 celdas (ilegible); heatmap solo de tagged
     (tamaño variable, P1 explota).

2. **Cuatro estados observables**
   - `empty`: 0 drills de esta partición en la década.
   - `pending`: ≥1 drill, 0 completed.
   - `partial`: algunos completed, no todos.
   - `done`: todos los drills de la década completed.
   - `empty` no es clicable. Las otras tres sí.

3. **Helpers en `concepts/mod.rs`, índice intocado**
   - `heatmap_bands()` / `heatmap_cell(partition_id, band, completed)`
     (nombres tentativos). `STEP_PARTITIONS` es read-only.
   - Freeze test: el set `(micro_step, tags)` de `1..=1000` coincide con
     `87a5334`.

4. **DOM**
   - `#concept-heatmap` dentro del panel de partición.
   - Celdas `#concept-heat-{lo}` (`lo` = inicio de década: 1, 11, …).
   - `data-state="{empty|pending|partial|done}"`.
   - Orden: heatmap **antes** de la lista (visión → detalle).

5. **Gates**
   - Unit: 100 celdas; P2 década de 52 no `empty`; completed sintético
     mueve `pending` → `partial`/`done`; freeze del índice.
   - Playwright: `/concepts/1` muestra `#concept-heatmap`; clic en una
     celda no-`empty` aterriza en `/learn/:step` del catálogo.
   - `make web-test`. Sin `make harness` completo.

## Risks / Trade-offs

| Riesgo | Mitigación |
|---|---|
| Década con 8 drills: clic “pierde” el título | lista Wave A sigue debajo |
| P4/P5 casi todo `empty` | esperado; el mapa *es* la escasez |
| Scope a analytics Go | este change no emite eventos |
| Conflicto con C3 (`concept_fab`) | D no toca peek/dock ni `glossary.rs` |

## Migration Plan

1. Mergear este PR de planificación a `main`.
2. Rama `feat/coding-compass-wave-d-concept-map` desde `origin/main`.
3. Helpers + render hub + tests.
4. `make web-test` + smoke `concepts.partitions.spec.ts`.
5. PR TED de implementación. Rollback: revert; hub lista permanece.

## Open Questions

- ¿Heatmap también en `/workspace`? → **No**; solo `/concepts/:id`.
- ¿Clic en `done` va al primer drill o se ignora? → **primer drill**
  (repaso). `empty` no navega.
