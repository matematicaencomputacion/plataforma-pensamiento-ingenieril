## Why

C1 cubre applied `301..=450` y mueve el freeze a ≥ 451. El tramo
`451..=600` (150 micro-pasos: arrays LeetCode-easy, grafos/DP replay,
listas, matrices, greedy, ventanas, backtracking, ADTs) sigue a medias
(~47 filas legado). Hasta que C1 merge, **no se implementa C2**: el
test de freeze y el índice se pisan.

**Bloqueo:** implementación C2 MUST esperar merge de
`coding-compass-wave-c1` a `main`.

## What Changes

**Una sola rebanada de índice** (Wave C2), misma política applied que
C1/Wave B:

1. Auditar y extender tags applied en `451..=600` (piso ≥ 90 de 150;
   no 100% P3).
2. Mover el freeze: dejar de congelar ≥ 451; **congelar filas ≥ 601**.
3. Tests de piso + no-bulk-P3 + familias untagged del tramo. No tocar
   `301..=450` (contrato C1) ni el glosario/drawer.

## Capabilities

### New Capabilities

- _(ninguna)_

### Modified Capabilities

- `coding-conceptual-partitions`: cobertura applied `451..=600`; el
  freeze C1 de filas ≥ 451 se retira **solo** para ese rango. Filas
  ≥ 601 quedan congeladas. **No retaguear `601..=1000`.**

## Alcance incluido

- Editar `STEP_PARTITIONS` **solo** en `451..=600`.
- Reemplazar freeze ≥ 451 por freeze ≥ 601 vs el SHA de C1 mergeado.
- Unit tests: piso ≥ 90; no 100% P3; C1 `301..=450` intacto; Wave A/B
  floors intactos; índice ordenado.
- `make web-test` verde. Implementación en PR propio **después** de C1.

## Fuera de alcance

- Tags `301..=450` (C1) y `601..=1000`.
- Diagramas DUA (C3). Densificar como `4..=100`.
- Glosario, FAB, analytics Go, mapas 4–5, `Language::Go`.
- Cambiar `current_level` / `completed_levels`.
- Código de producto en este PR de planificación.

## Impact

- `web/src/concepts/mod.rs`. Base = `main` **post-C1**, no `a81a026`.
- C3 sigue bloqueado en C1 (secuencia documentada); C3 no edita el
  índice, pero no se implementa en paralelo a C1/C2.
- Sin API Go ni ejecución de alumnos (ADR 002).

## Plan de rollback

Revertir el PR de implementación C2. El freeze ≥ 451 de C1 y el drawer
Wave B permanecen. No hay migración de datos.
