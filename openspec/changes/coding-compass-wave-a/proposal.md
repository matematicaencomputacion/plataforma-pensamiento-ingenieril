## Why

El compás `[1]…[5]` ya existe (`coding-conceptual-partitions`, PR #232) pero todavía
es un mapa flaco: el índice taggea sobre todo foundations sueltos, el % de dominio
vive solo en `aria-label`, y el E2E no cierra el ciclo “entro a un drill, lo
apruebo, el hub/compás refleja el ✔”. Sin eso, `/concepts/1..3` no son lentes
reales — son un set de prueba.

## What Changes

Cerrar el loop del compás en **una sola rebanada** (Wave A):

1. **Taggeo denso de particiones 1–3** sobre el rail 4..=100 (foundations:
   tipos, strings, listas, tuplas, sets, dicts, control, funciones, clases)
   más tags *applied* puntuales en 101..=160 cuando el costo cognitivo es
   mutabilidad, LEGB o paradigma — no “es un `for`”.
2. **Indicador visible de dominio** en cada botón del header: aro o `%`
   derivado de `completed_levels ∩ drills_for_partition` (ya existe
   `mastery_percent`; falta UI).
3. **Journey E2E conceptual** (ADR 003): login → `/learn` → `[2]` →
   `/concepts/2` → drill → Validar → el badge/compás marca completado.

## Capabilities

### New Capabilities

- _(ninguna — este change densifica la capacidad ya propuesta)_

### Modified Capabilities

- `coding-conceptual-partitions`: índice 1–3 denso, mastery visible en
  PartitionNav, journey de jornada conceptual (no solo smoke de navegación).

## Alcance incluido

- Extender `STEP_PARTITIONS` en `web/src/concepts.rs` (índice externo; sin
  reescribir las 1000 constantes `CodingStep`).
- Unit tests de cobertura mínima (pisos por partición + 4..=100 sin huecos).
- UI del % / aro en `#partition-nav-1..5` + `data-mastery`.
- E2E `journey.concepts.spec.ts` (o extensión del smoke actual) con mock
  Pyodide; `make web-test` + journey en verde.

## Fuera de alcance

- Drawer lateral en `/learn` (Wave B).
- Atajos Ctrl/Cmd+1..5 (Wave B).
- Capas DUA / dual-coding / sandbox / boss fights.
- Mapas interactivos de particiones 4–5 (siguen `map_only`, ADR 002).
- Taggear 161..=1000.
- Endpoints Go de analytics, reintentos o tiempo (el % es 100% cliente).
- Discriminador `Language::Go`.
- Reordenar el rail o crear tracks de progreso por partición.
- Cambiar `current_level` / `completed_levels`.

## Impact

- `web/src/concepts.rs`, `web/src/components/partition_nav.rs`, CSS del
  compás, `web/e2e/tests/`.
- Depende de que #232 (`coding-conceptual-partitions`) esté en la base
  (mergeado o cherry-pick). Sin ese módulo este change no aplica.
- Sin cambios de API Go ni de ejecución de alumnos.

## Plan de rollback

Revertir el PR de Wave A. El compás v1 (#232) sigue usable con el índice
flaco y sin % visible. El rail y `/learn/:step` no dependen de los tags
nuevos.
