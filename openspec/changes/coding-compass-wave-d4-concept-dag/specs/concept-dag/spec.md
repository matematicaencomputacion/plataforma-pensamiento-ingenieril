## ADDED Requirements

### Requirement: Static glossary DAG in WASM
The coding shell SHALL expose a static in-memory DAG of glossary
concept ids in `web/src/concepts/dag.rs`. Each edge MUST be
`from: ConceptId → to: ConceptId` with kind `Requires` or `Reinforces`.
Both endpoints MUST exist in `GLOSSARY_ENTRIES`. `Requires` edges MUST
form a directed acyclic graph (no self-loops, no cycles). `Reinforces`
edges MUST NOT be used for cycle checks. Nodes MUST be glossary ids,
not micro-step numbers. The seed MUST include
`python-lists Requires model-mutability`. Every `Requires` target MUST
have at least one `related_step_id` that resolves in the coding catalog.
This change MUST NOT retag `STEP_PARTITIONS` (`1..=1000`) and MUST NOT
add Go endpoints or a graph database.

#### Scenario: Canonical lists-requires-mutability edge exists
- **GIVEN** the static `CONCEPT_EDGES` table
- **WHEN** a client looks up edges from `python-lists`
- **THEN** there is an edge `python-lists → model-mutability` with kind
  `Requires`
- **AND** both ids exist in `GLOSSARY_ENTRIES`

#### Scenario: Requires edges are acyclic
- **GIVEN** the static `CONCEPT_EDGES` table
- **WHEN** unit tests walk only `Requires` edges
- **THEN** there is no cycle
- **AND** no edge has `from == to`

#### Scenario: Index freeze remains intact
- **GIVEN** `STEP_PARTITIONS` at the Wave D.3 baseline (`f478bd3`)
- **WHEN** unit tests collect rows with `micro_step` in `1..=1000`
- **THEN** the set of `(micro_step, tags)` pairs is unchanged

### Requirement: Missing-base detection from existing progress
A glossary concept SHALL be considered started when any of its
`related_step_id` micro-steps is present in `completed_levels` OR the
learner's `current_level` is greater than or equal to that micro-step.
A required base of concept `A` is missing when `A Requires B` and `B`
is not started. Concepts with no resolvable related drills MUST NOT be
reported as missing bases.

#### Scenario: Fresh learner has not started mutability
- **GIVEN** `completed_levels` is empty and `current_level` is 1
- **WHEN** missing bases are computed for partition 1
- **THEN** `model-mutability` is included
- **AND** the lists-requires-mutability edge is among the partition edges

#### Scenario: Completing the mutability drill clears that base
- **GIVEN** `completed_levels` contains the micro-step of
  `py-26-list-copy`
- **WHEN** missing bases are computed for partition 1
- **THEN** `model-mutability` is not included
