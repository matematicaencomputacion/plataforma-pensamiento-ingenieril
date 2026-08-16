## ADDED Requirements

### Requirement: Applied tagging through micro-step 1000
The conceptual index SHALL extend applied (not dense) partition tags through
catalogued micro-steps `901..=1000`. A step in that range MAY remain untagged
when the cognitive load is an algorithm recipe (bit tricks, two pointers,
sliding window, interval geometry, randomized recipe, or LeetCode-easy
array recipe) rather than mutability, LEGB, paradigm, stdlib map, or
application domain. Tagged steps in `901..=1000` MUST number at least 60.
This change MUST NOT add, remove, or edit `STEP_PARTITIONS` rows whose
`micro_step` is `> 1000`. This change MUST NOT edit rows in `751..=900`
(Wave C5 contract).

#### Scenario: Applied floor in 901..=1000
- **GIVEN** the concept index and the coding catalog after C5
- **WHEN** unit tests enumerate catalogued micro-steps `901..=1000` that have
  at least one tag
- **THEN** that count is ≥ 60
- **AND** every tag is in `1..=5`
- **AND** each tagged micro-step exists in the catalog

#### Scenario: No bulk DSA-as-imperative on 901..=1000
- **GIVEN** tagged micro-steps in `901..=1000`
- **WHEN** unit tests inspect partition-3 membership
- **THEN** not every tagged step in that range is labeled `3`

#### Scenario: Recipe families stay untagged
- **GIVEN** representative recipe steps (at least micro-steps 913, 955, and
  1000)
- **WHEN** `partitions_for_micro_step` is resolved
- **THEN** each returns an empty tag slice

#### Scenario: No rows beyond 1000
- **GIVEN** `STEP_PARTITIONS` at C5 merge SHA
- **WHEN** unit tests collect rows with `micro_step > 1000`
- **THEN** the set of `(micro_step, tags)` pairs is empty

#### Scenario: C5 range is not retagged
- **GIVEN** `STEP_PARTITIONS` at C5 merge SHA
- **WHEN** unit tests collect rows with `micro_step` in `751..=900`
- **THEN** the set of `(micro_step, tags)` pairs equals the C5 baseline

## MODIFIED Requirements

### Requirement: Multi-tag index without catalog duplication
The external index (`STEP_PARTITIONS` in `web/src/concepts/mod.rs`) remains
the source of drill tags. Wave C6 extends applied coverage through `1000`;
the C5 prohibition on tagging `901..=1000` is withdrawn. Rows `> 1000`
must not be introduced. Numeric partition tags remain `u8` `1..=5`.

#### Scenario: Invalid tag rejected in tests
- **GIVEN** the concept index
- **WHEN** unit tests validate mappings
- **THEN** no tagged micro-step points outside the catalog
- **AND** `STEP_PARTITIONS` stays sorted by `micro_step` ascending

#### Scenario: Prior floors remain
- **GIVEN** catalogued micro-steps `4..=100`, `101..=300`, `301..=450`,
  `451..=600`, `601..=750`, and `751..=900`
- **WHEN** unit tests enumerate them
- **THEN** `4..=100` stays dense, `101..=300` tagged count is ≥ 120,
  `301..=450` tagged count is ≥ 90, `451..=600` tagged count is ≥ 90,
  `601..=750` tagged count is ≥ 90, and `751..=900` tagged count is ≥ 90
