## Why

C2 cubre applied `451..=600` y congela `≥ 601`. El tramo `601..=750`
(prefix/diff, UF, tries, intervalos, DP, grafos, árboles, heaps,
Fenwick/segtree, binary lift) sigue a medias (~38 filas legado). C3
cerró DUA y no tocó el índice. Sin C4 el freeze ≥ 601 bloquea el
siguiente bloque DSA.

## What Changes

**Una sola rebanada de índice** (Wave C4), misma política applied que
C1/C2:

1. Auditar y extender tags applied en `601..=750` (piso ≥ 90 de 150;
   no 100% P3).
2. Mover el freeze: dejar de congelar ≥ 601; **congelar filas ≥ 751**.
3. Tests de piso + no-bulk-P3 + recetas untagged. No tocar `451..=600`
   (contrato C2) ni el glosario/drawer C3.

## Capabilities

### New Capabilities

- _(ninguna)_

### Modified Capabilities

- `coding-conceptual-partitions`: cobertura applied `601..=750`; el
  freeze C2 de filas ≥ 601 se retira **solo** para ese rango. Filas
  ≥ 751 quedan congeladas. **No retaguear `751..=1000`.**

## Alcance incluido

- Editar `STEP_PARTITIONS` **solo** en `601..=750`.
- Reemplazar freeze ≥ 601 por freeze ≥ 751 vs el SHA de C3 mergeado.
- Unit tests: piso ≥ 90; no 100% P3; C2 `451..=600` intacto; floors
  A/B/C1 intactos; índice ordenado.
- `make web-test` verde. PR propio.

## Fuera de alcance

- Tags `451..=600` (C2) y `751..=1000`.
- Diagramas DUA (C3). Densificar como `4..=100`.
- Glosario, FAB, analytics Go, mapas 4–5.

## Impact

- `web/src/concepts/mod.rs`. Base = `main` **post-C3**.
- Sin API Go ni ejecución de alumnos (ADR 002).

## Plan de rollback

Revertir el PR de implementación C4. El freeze ≥ 601 de C2 y el drawer
C3 permanecen.
