## Context

Wave D.3 (#249, `f478bd3`) dejó heatmap + facetas + widget de fricción
en `/concepts/:id`. Wave B dejó `GLOSSARY_ENTRIES` con ids canónicos
(`python-lists`, `model-mutability`, …) y `related_step_id` por lente.
No hay aristas entre conceptos. Stack: Leptos CSR, progreso en
`AuthUser.completed_levels` + `current_level` (ADR 002).

## Goals / Non-Goals

**Goals**

- DAG estático en WASM: aristas entre **ids del glosario**, no entre
  micro-pasos.
- Kinds `Requires` (prerrequisito) y `Reinforces` (refuerzo).
- Hub: lista de prerrequisitos de la partición abierta; alerta si un
  `Requires` apunta a un concepto cuyos drills no están empezados.
- Playwright: en `/concepts/1`, visible la arista lists → mutability
  (alerta o lista).
- Freeze `1..=1000` intacto. Cero Go.

**Non-Goals**

- Neo4j, graph DB, ML, retags, visual-regression suite.
- Página `/concepts/glossary/:id`. Desbloquear el rail con el DAG.
- SVG rico; una lista basta (SVG opcional, no gate).

## Decisions

1. **Nodos = glossary ids**
   - `ConceptEdge { from, to, kind }` con `from`/`to` ∈ `GLOSSARY_ENTRIES`.
   - Seed pequeño (~12 aristas). Canónica de e2e:
     `python-lists Requires model-mutability`.
   - `Requires` MUST ser un DAG (sin ciclos, sin self-loops).
   - `Reinforces` puede ir en sentido inverso y NO entra al chequeo
     topológico (es señal pedagógica, no bloqueo).

2. **Empezado = progreso existente**
   - Drills de un concepto = `related_step_id` de sus lentes, resueltos
     a `micro_step` vía `coding_step_by_id`.
   - Empezado si algún micro está en `completed_levels` **o**
     `current_level >= micro_step`.
   - Sin drills relacionados → no se marca como base faltante (no hay
     señal medible). Todo `to` de `Requires` MUST tener ≥1 drill.

3. **Hub, no grafo interactivo**
   - IDs: `#concept-prereq-alert` (`role="alert"`), `#concept-prereq-list`.
   - Arista relevante a la pestaña `P` si `from` tiene lente en `P`.
   - Lista: esas aristas (`data-from`, `data-to`, `data-kind`).
   - Alerta: `to` de `Requires` relevantes que no están empezados.
   - No sustituye heatmap / facetas / analytics / FAB.

4. **Índice intocado**
   - `STEP_PARTITIONS` y `GLOSSARY_ENTRIES` read-only (salvo el módulo
     DAG nuevo). El freeze Wave D sigue siendo el gate.

5. **Gates**
   - Unit: arista lists→mutability; `Requires` acíclico; P1 vacío de
     progreso incluye mutability en missing bases; freeze `1..=1000`.
   - Un Playwright en `concepts.partitions.spec.ts`.
   - `make web-test`. E2E CI = `trunk build` + `STATIC_DIR` (no
     `trunk serve`). Sin `go test` (no se toca Go).

## Risks / Trade-offs

| Riesgo | Mitigación |
|---|---|
| Confundir con 1000 nodos de micro-pasos | spec + tests: solo glossary ids |
| Alerta en P4/P5 porque `python-lists` tiene 5 lentes | aceptable en v1; `from` con lente en P |
| `current_level` adelantado sin completar el drill | “empezado” = cursor o completed; no es mastery |
| SVG que nadie usa | lista es el contrato; SVG opcional |

## Migration Plan

1. OpenSpec + implementación en
   `feat/coding-compass-wave-d4-concept-dag` desde `origin/main` @ `f478bd3`.
2. `dag.rs` + hub + CSS + smoke.
3. `make web-test` + Playwright del spec de concepts.
4. PR TED. Rollback: revert; hub D.3 intacto.

## Open Questions

- ¿Bloquear drills hasta completar la base? → **No** en D.4 (alerta, no gate).
- ¿Persistir el DAG en Go? → **No**; estático en WASM.
