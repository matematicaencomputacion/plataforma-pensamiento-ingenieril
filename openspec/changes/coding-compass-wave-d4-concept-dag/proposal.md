## Why

El hub ya cubre lentes, heatmap, facetas y fricción, pero no dice **qué
concepto hay que haber tocado antes**. El alumno puede abrir Listas
sin haber empezado Mutabilidad. Un DAG estático entre ids del glosario
(no entre 1000 micro-pasos) es la señal mínima de prerrequisito.

## What Changes

**Grafo conceptual de prerrequisitos — primer slice** (Wave D.4):

1. Tabla WASM de aristas `ConceptId → ConceptId` con kind
   `Requires | Reinforces`, junto al glosario (`web/src/concepts/dag.rs`).
2. En `/concepts/:id`, lista de prerrequisitos de los conceptos de esa
   partición y alerta de base faltante si los drills del concepto
   requerido no están empezados (`completed_levels` / `current_level`).
3. Un smoke Playwright: alerta o lista visible para la arista
   `python-lists Requires model-mutability`.

## Capabilities

### New Capabilities

- `concept-dag`: DAG estático cliente entre ids del glosario; consulta
  de prerrequisitos y bases no empezadas a partir del progreso local.

### Modified Capabilities

- `coding-conceptual-partitions`: el hub `/concepts/:id` SHALL mostrar
  `#concept-prereq-list` y, si hay bases sin empezar, `#concept-prereq-alert`
  (sin desmontar heatmap, facetas, analytics ni FAB).

## Alcance incluido

- `CONCEPT_EDGES` sobre ids existentes del glosario (p. ej. lists
  requiere mutability). `Requires` es acíclico.
- Alerta + lista en el hub; progreso = `completed_levels` y
  `current_level` (un concepto está empezado si algún `related_step_id`
  está completado o el cursor ya lo alcanzó).
- Unit tests del DAG + freeze `STEP_PARTITIONS` `1..=1000` intacto.
- Un Playwright en el spec de concepts. Spec + código en el mismo PR.
- `make web-test`. Sin Go.

## Fuera de alcance

- Retaguear `1..=1000` / freeze `>1000`.
- Expandir analytics Go, Neo4j, grafo persistido, ML adaptativo.
- Visual-regression mega-suite (#5).
- Nodos = micro-pasos. Tracks de progreso por partición.
- Romper FAB / heatmap / facetas / `#concept-analytics`.

## Impact

- `web/src/concepts/dag.rs`, `web/src/pages/concepts.rs`, CSS, e2e.
- Base = `origin/main` @ `f478bd3` (Wave D.3 #249).
- Python del alumno sigue solo en Pyodide (ADR 002).

## Plan de rollback

Revertir este PR. El hub D.3 y el glosario Wave B siguen. El DAG no
tiene persistencia.
