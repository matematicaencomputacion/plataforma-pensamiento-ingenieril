## Why

Wave B (#236, `a81a026`) dejó el índice applied hasta el micro-paso 300 y
**congeló** `STEP_PARTITIONS` ≥ 301. El rail 301..=450 ya tiene ~47 filas
legado (árboles, backtracking, UF, ADTs) y ~100 huecos; bits, ventanas y
two-pointers siguen sin política explícita en este tramo. Sin C1, el
compás no cubre el siguiente bloque DSA y C2 no puede mover el freeze.

## What Changes

**Una sola rebanada de índice** (Wave C1). Continuidad de Wave B, no
densificar como `4..=100`:

1. Auditar y extender tags applied en `301..=450` (piso ≥ 90 de 150
   catálogo; no 100% P3).
2. Mover el freeze: dejar de congelar ≥ 301; **congelar filas ≥ 451**
   (incluye 451..=600 y 601..=1000) para que C2/C3 no pisen el índice.
3. Tests de cobertura + no-bulk-P3 + familias untagged (bits / window /
   two-pointers del tramo). Conservar floors Wave A y piso Wave B 101..=300.

## Capabilities

### New Capabilities

- _(ninguna — densifica el índice ya especificado)_

### Modified Capabilities

- `coding-conceptual-partitions`: cobertura applied `301..=450`; el
  freeze Wave B de filas ≥ 301 se retira **solo** para ese rango. Filas
  ≥ 451 quedan congeladas vs `a81a026`.

## Alcance incluido

- Editar `STEP_PARTITIONS` en `web/src/concepts/mod.rs` **solo** para
  `301..=450` (índice externo; sin reescribir `CodingStep`).
- Reemplazar el test `wave_b_freeze_rows_301_and_up` por freeze ≥ 451
  anclado a `a81a026`.
- Unit tests: piso ≥ 90 en `301..=450`; no 100% tag `3`; representantes
  untagged; `4..=100` denso; piso 101..=300 intacto; índice ordenado.
- `make web-test` verde. PR propio de implementación **después** de
  mergear este change de planificación (no en este PR).

## Fuera de alcance

- Tags `451..=600` (C2) y `601..=1000`.
- Diagramas DUA / dual-coding en el drawer (C3).
- Densificar `301..=450` como foundations (`4..=100`).
- Enciclopedia de glosario, analytics Go, sandbox, boss fights, mapas 4–5.
- Cambiar `current_level` / `completed_levels` o el FAB de Wave B.
- Discriminador `Language::Go`.
- Implementar código de producto en **este** PR de planificación.

## Impact

- `web/src/concepts/mod.rs` (+ tests del freeze). Base = `origin/main` @
  `a81a026` (Wave B #236).
- C2 y C3 **quedan bloqueados** hasta merge de la implementación C1:
  ambos asumen el freeze ≥ 451 y un `STEP_PARTITIONS` ya auditado en
  301..=450.
- Sin API Go ni ejecución de alumnos (ADR 002).

## Plan de rollback

Revertir el PR de implementación C1. Restaurar el freeze ≥ 301 de Wave B.
El drawer, el glosario y el compás `4..=300` siguen usables.
