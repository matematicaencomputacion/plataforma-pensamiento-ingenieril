## ADDED Requirements

### Requirement: Applied tagging through micro-step 450
The conceptual index SHALL extend applied (not dense) partition tags through
catalogued micro-steps `301..=450`. A step in that range MAY remain untagged
when the cognitive load is an algorithm recipe (bit tricks, two pointers,
sliding window, interval geometry, binary-search recipe) rather than
mutability, LEGB, paradigm, stdlib map, or application domain. Tagged steps
in `301..=450` MUST number at least 90. This change MUST NOT add, remove, or
edit `STEP_PARTITIONS` rows whose `micro_step` is `≥ 451`.

#### Scenario: Applied floor in 301..=450
- **GIVEN** the concept index and the coding catalog
- **WHEN** unit tests enumerate catalogued micro-steps `301..=450` that have
  at least one tag
- **THEN** that count is ≥ 90
- **AND** every tag is in `1..=5`
- **AND** each tagged micro-step exists in the catalog

#### Scenario: No bulk DSA-as-imperative on 301..=450
- **GIVEN** tagged micro-steps in `301..=450`
- **WHEN** unit tests inspect partition-3 membership
- **THEN** not every tagged step in that range is labeled `3`

#### Scenario: Bit and two-pointer families stay untagged
- **GIVEN** representative recipe steps (at least micro-steps 301, 329, and
  371)
- **WHEN** `partitions_for_micro_step` is resolved
- **THEN** each returns an empty tag slice

#### Scenario: Freeze rows 451..=1000
- **GIVEN** `STEP_PARTITIONS` at Wave B (`a81a026`)
- **WHEN** unit tests collect rows with `micro_step ≥ 451`
- **THEN** the set of `(micro_step, tags)` pairs equals the frozen baseline
- **AND** no new keys in `451..=1000` are introduced

## MODIFIED Requirements

### Requirement: Multi-tag index without catalog duplication
The external index (`STEP_PARTITIONS` in `web/src/concepts/mod.rs`) remains
the source of drill tags (no `partition` field on each `CodingStep`). The
glossary in `glossary.rs` is a separate search corpus and MUST NOT replace
this index. Wave A densifies `4..=100` and applied `101..=160`. Wave B
extends applied coverage through `300`. Wave C1 extends applied coverage
through `450` as specified above; the Wave B prohibition on tagging
`301..=1000` is withdrawn **only** for `301..=450`. Rows `≥ 451` stay
frozen. Numeric partition tags remain `u8` `1..=5`
(`ConceptPartition.id`); they map 1:1 to glossary `PartitionId` without
forking a second numbering scheme. The v1 “foundations sample only”
allowance remains withdrawn for `4..=100`.

#### Scenario: Invalid tag rejected in tests
- **GIVEN** the concept index
- **WHEN** unit tests validate mappings
- **THEN** no tagged micro-step points outside the catalog
- **AND** `STEP_PARTITIONS` stays sorted by `micro_step` ascending

#### Scenario: Wave A foundations remain dense
- **GIVEN** catalogued micro-steps `4..=100`
- **WHEN** unit tests enumerate them
- **THEN** each still has `partitions_for_micro_step(n).len() >= 1`

#### Scenario: Wave B applied floor remains
- **GIVEN** catalogued micro-steps `101..=300`
- **WHEN** unit tests count tagged steps
- **THEN** that count is still ≥ 120
