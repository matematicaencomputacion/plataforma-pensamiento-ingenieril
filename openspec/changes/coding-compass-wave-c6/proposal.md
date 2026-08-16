## Why

C5 cubre applied `751..=900` y congela `≥ 901`. El tramo final
`901..=1000` (monotonic queue, Fenwick/segtree, sparse table, binary
lift, strings/hash, suffix, flow, matching, game/random, offline,
meet-in-middle, matrix exp, geometry, review) no tiene filas applied.
Sin C6 el freeze ≥ 901 deja el último bloque DSA fuera del compás.

## What Changes

**Una sola rebanada de índice** (Wave C6), misma política applied que
C1–C5; banda de 100 pasos (piso escalado ≥ 60):

1. Auditar y extender tags applied en `901..=1000` (piso ≥ 60 de 100;
   no 100% P3).
2. Retirar el freeze ≥ 901; **no añadir filas `> 1000`**.
3. Tests de piso + no-bulk-P3 + recetas untagged. No tocar `751..=900`
   (contrato C5) ni el glosario/drawer C3.

## Capabilities

### New Capabilities

- _(ninguna)_

### Modified Capabilities

- `coding-conceptual-partitions`: cobertura applied `901..=1000`; el
  freeze C5 de filas ≥ 901 se retira **solo** para ese rango. No hay
  filas `> 1000`.

## Alcance incluido

- Editar `STEP_PARTITIONS` **solo** en `901..=1000`.
- Reemplazar freeze ≥ 901 por freeze `> 1000` vs el SHA de C5 mergeado.
- Unit tests: piso ≥ 60; no 100% P3; C5 `751..=900` intacto; floors
  A/B/C1/C2/C4 intactos; índice ordenado.
- `make web-test` verde. PR propio.

## Fuera de alcance

- Tags `751..=900` (C5). Filas `> 1000`.
- Diagramas DUA (C3). Densificar como `4..=100`.
- Glosario, FAB, analytics Go, mapas 4–5.

## Impact

- `web/src/concepts/mod.rs`. Base = `main` **post-C5** (`e200ec1`).
- Sin API Go ni ejecución de alumnos (ADR 002).

## Plan de rollback

Revertir el PR de implementación C6. El freeze ≥ 901 de C5 permanece.
