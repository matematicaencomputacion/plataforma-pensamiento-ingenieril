## Why

C4 cubre applied `601..=750` y congela `≥ 751`. El tramo `751..=900`
(stacks/queues/listas, BST/árboles, DFS/BFS, toposort, UF, shortest
path, MST, trie, heaps, intervalos, greedy, DP, backtracking, bits,
math, prefix/diff, monotonic stack) no tiene filas applied. Sin C5 el
freeze ≥ 751 bloquea el siguiente bloque DSA. C6 tomará `901..=1000`.

## What Changes

**Una sola rebanada de índice** (Wave C5), misma política applied que
C1/C2/C4:

1. Auditar y extender tags applied en `751..=900` (piso ≥ 90 de 150;
   no 100% P3).
2. Mover el freeze: dejar de congelar ≥ 751; **congelar filas ≥ 901**.
3. Tests de piso + no-bulk-P3 + recetas untagged. No tocar `601..=750`
   (contrato C4) ni el glosario/drawer C3.

## Capabilities

### New Capabilities

- _(ninguna)_

### Modified Capabilities

- `coding-conceptual-partitions`: cobertura applied `751..=900`; el
  freeze C4 de filas ≥ 751 se retira **solo** para ese rango. Filas
  ≥ 901 quedan congeladas. **No retaguear `901..=1000`.**

## Alcance incluido

- Editar `STEP_PARTITIONS` **solo** en `751..=900`.
- Reemplazar freeze ≥ 751 por freeze ≥ 901 vs el SHA de C4 mergeado.
- Unit tests: piso ≥ 90; no 100% P3; C4 `601..=750` intacto; floors
  A/B/C1/C2 intactos; índice ordenado.
- `make web-test` verde. PR propio.

## Fuera de alcance

- Tags `601..=750` (C4) y `901..=1000` (C6).
- Diagramas DUA (C3). Densificar como `4..=100`.
- Glosario, FAB, analytics Go, mapas 4–5.

## Impact

- `web/src/concepts/mod.rs`. Base = `main` **post-C4** (`6a8ac3b`).
- Sin API Go ni ejecución de alumnos (ADR 002).

## Plan de rollback

Revertir el PR de implementación C5. El freeze ≥ 751 de C4 y el drawer
C3 permanecen.
