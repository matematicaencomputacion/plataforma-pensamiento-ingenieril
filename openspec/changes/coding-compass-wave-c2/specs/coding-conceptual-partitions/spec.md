## ADDED Requirements

### Requirement: Applied tagging through micro-step 600
The conceptual index SHALL extend applied (not dense) partition tags through
catalogued micro-steps `451..=600`. A step in that range MAY remain untagged
when the cognitive load is an algorithm recipe (bit tricks, two pointers,
sliding window, interval geometry, binary-search recipe, or LeetCode-easy
array recipe) rather than mutability, LEGB, paradigm, stdlib map, or
application domain. Tagged steps in `451..=600` MUST number at least 90.
This change MUST NOT add, remove, or edit `STEP_PARTITIONS` rows whose
`micro_step` is `≥ 601`. This change MUST NOT edit rows in `301..=450`
(Wave C1 contract). Implementation MUST NOT start until
`coding-compass-wave-c1` is merged to `main`.

#### Scenario: Applied floor in 451..=600
- **GIVEN** the concept index and the coding catalog after C1
- **WHEN** unit tests enumerate catalogued micro-steps `451..=600` that have
  at least one tag
- **THEN** that count is ≥ 90
- **AND** every tag is in `1..=5`
- **AND** each tagged micro-step exists in the catalog

#### Scenario: No bulk DSA-as-imperative on 451..=600
- **GIVEN** tagged micro-steps in `451..=600`
- **WHEN** unit tests inspect partition-3 membership
- **THEN** not every tagged step in that range is labeled `3`

#### Scenario: Recipe families stay untagged
- **GIVEN** representative recipe steps (at least micro-steps 518, 543, and
  547)
- **WHEN** `partitions_for_micro_step` is resolved
- **THEN** each returns an empty tag slice

#### Scenario: Freeze rows 601..=1000
- **GIVEN** `STEP_PARTITIONS` at C1 merge SHA
- **WHEN** unit tests collect rows with `micro_step ≥ 601`
- **THEN** the set of `(micro_step, tags)` pairs equals that frozen baseline
- **AND** no new keys in `601..=1000` are introduced

#### Scenario: C1 range is not retagged
- **GIVEN** `STEP_PARTITIONS` at C1 merge SHA
- **WHEN** unit tests collect rows with `micro_step` in `301..=450`
- **THEN** the set of `(micro_step, tags)` pairs equals the C1 baseline

## MODIFIED Requirements

### Requirement: Multi-tag index without catalog duplication
The external index (`STEP_PARTITIONS` in `web/src/concepts/mod.rs`) remains
the source of drill tags (no `partition` field on each `CodingStep`). Wave
C1 extends applied coverage through `450`. Wave C2 extends applied coverage
through `600` as specified above; the C1 prohibition on tagging `451..=1000`
is withdrawn **only** for `451..=600`. Rows `≥ 601` stay frozen. **This
change MUST NOT retag `601..=1000`.** Numeric partition tags remain `u8`
`1..=5`. The v1 “foundations sample only” allowance remains withdrawn for
`4..=100`.

#### Scenario: Invalid tag rejected in tests
- **GIVEN** the concept index
- **WHEN** unit tests validate mappings
- **THEN** no tagged micro-step points outside the catalog
- **AND** `STEP_PARTITIONS` stays sorted by `micro_step` ascending

#### Scenario: Prior floors remain
- **GIVEN** catalogued micro-steps `4..=100`, `101..=300`, and `301..=450`
- **WHEN** unit tests enumerate them
- **THEN** `4..=100` stays dense, `101..=300` tagged count is ≥ 120, and
  `301..=450` tagged count is ≥ 90
