## ADDED Requirements

### Requirement: Applied tagging through micro-step 750
The conceptual index SHALL extend applied (not dense) partition tags through
catalogued micro-steps `601..=750`. A step in that range MAY remain untagged
when the cognitive load is an algorithm recipe (bit tricks, two pointers,
sliding window, interval geometry, binary-search recipe, or LeetCode-easy
array recipe) rather than mutability, LEGB, paradigm, stdlib map, or
application domain. Tagged steps in `601..=750` MUST number at least 90.
This change MUST NOT add, remove, or edit `STEP_PARTITIONS` rows whose
`micro_step` is `≥ 751`. This change MUST NOT edit rows in `451..=600`
(Wave C2 contract).

#### Scenario: Applied floor in 601..=750
- **GIVEN** the concept index and the coding catalog after C3
- **WHEN** unit tests enumerate catalogued micro-steps `601..=750` that have
  at least one tag
- **THEN** that count is ≥ 90
- **AND** every tag is in `1..=5`
- **AND** each tagged micro-step exists in the catalog

#### Scenario: No bulk DSA-as-imperative on 601..=750
- **GIVEN** tagged micro-steps in `601..=750`
- **WHEN** unit tests inspect partition-3 membership
- **THEN** not every tagged step in that range is labeled `3`

#### Scenario: Recipe families stay untagged
- **GIVEN** representative recipe steps (at least micro-steps 613, 630, and
  739)
- **WHEN** `partitions_for_micro_step` is resolved
- **THEN** each returns an empty tag slice

#### Scenario: Freeze rows 751..=1000
- **GIVEN** `STEP_PARTITIONS` at C3 merge SHA
- **WHEN** unit tests collect rows with `micro_step ≥ 751`
- **THEN** the set of `(micro_step, tags)` pairs equals that frozen baseline
- **AND** no new keys in `751..=1000` are introduced

#### Scenario: C2 range is not retagged
- **GIVEN** `STEP_PARTITIONS` at C3 merge SHA
- **WHEN** unit tests collect rows with `micro_step` in `451..=600`
- **THEN** the set of `(micro_step, tags)` pairs equals the C2 baseline

## MODIFIED Requirements

### Requirement: Multi-tag index without catalog duplication
The external index (`STEP_PARTITIONS` in `web/src/concepts/mod.rs`) remains
the source of drill tags. Wave C4 extends applied coverage through `750`;
the C2 prohibition on tagging `601..=1000` is withdrawn **only** for
`601..=750`. Rows `≥ 751` stay frozen. **This change MUST NOT retag
`751..=1000`.** Numeric partition tags remain `u8` `1..=5`.

#### Scenario: Invalid tag rejected in tests
- **GIVEN** the concept index
- **WHEN** unit tests validate mappings
- **THEN** no tagged micro-step points outside the catalog
- **AND** `STEP_PARTITIONS` stays sorted by `micro_step` ascending

#### Scenario: Prior floors remain
- **GIVEN** catalogued micro-steps `4..=100`, `101..=300`, `301..=450`, and
  `451..=600`
- **WHEN** unit tests enumerate them
- **THEN** `4..=100` stays dense, `101..=300` tagged count is ≥ 120,
  `301..=450` tagged count is ≥ 90, and `451..=600` tagged count is ≥ 90
