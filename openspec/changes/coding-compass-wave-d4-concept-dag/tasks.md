## 1. Planificación

- [x] 1.1 Change `coding-compass-wave-d4-concept-dag` (proposal /
      design / spec / tasks) anclado a `origin/main` @ `f478bd3`
- [x] 1.2 Fuera de alcance explícito: Neo4j, analytics Go, visual-
      regression suite, retags `1..=1000`, nodos = micro-pasos

## 2. DAG WASM

- [x] 2.1 `web/src/concepts/dag.rs`: `EdgeKind`, `ConceptEdge`,
      `CONCEPT_EDGES` (~12 aristas, incluye lists → mutability)
- [x] 2.2 Helpers: edges por partición, concepto empezado,
      bases faltantes; tests de DAG + missing-base
- [x] 2.3 Exportar desde `concepts/mod.rs` sin tocar `STEP_PARTITIONS`

## 3. Hub UI

- [x] 3.1 `#concept-prereq-list` + `#concept-prereq-alert` en
      `ConceptsPage`; heatmap / facetas / analytics intactos
- [x] 3.2 CSS del bloque de prerrequisitos

## 4. Tests y proof

- [x] 4.1 Playwright: login → `/concepts/1` → alerta o lista de la
      arista `python-lists` requires `model-mutability`
- [ ] 4.2 `make web-test`; PR TED; squash-merge cuando Backend +
      Frontend + Playwright aggregator estén verdes
