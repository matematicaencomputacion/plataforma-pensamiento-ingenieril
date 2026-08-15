## ADDED Requirements

### Requirement: Dense foundations tagging (partitions 1–3)
Every catalogued coding micro-step in the inclusive range `4..=100` SHALL
have at least one valid partition tag. Partition 1 (data-model), 2 (LEGB)
and 3 (paradigms) MUST meet minimum drill floors: P1 ≥ 40, P2 ≥ 15, P3 ≥ 35.
Tags on `101..=160` MAY be applied only when the exercise's cognitive load
is mutability, scope, or paradigm — not merely because the solution iterates.
This change MUST NOT bulk-tag `161..=1000`.

#### Scenario: No holes in 4..=100
- **GIVEN** the concept index and the coding catalog
- **WHEN** unit tests enumerate micro-steps `4..=100` that exist in the catalog
- **THEN** each one has `partitions_for_micro_step(n).len() >= 1`
- **AND** every tag is in `1..=5`

#### Scenario: Drill floors
- **GIVEN** the concept index
- **WHEN** `drills_for_partition` is computed
- **THEN** partition 1 has at least 40 drills, partition 2 at least 15, and
  partition 3 at least 35

#### Scenario: No bulk DSA-as-imperative
- **GIVEN** tagged micro-steps in `101..=160`
- **WHEN** unit tests inspect partition-3 membership
- **THEN** not every tagged step in that range is labeled `3`

### Requirement: Visible mastery on the compass
Each authenticated compass control `[1]…[5]` SHALL expose domain mastery as
a visible percentage (or equivalent ring) derived only from
`completed_levels ∩ drills_for_partition(id)`. The control MUST set
`data-mastery` to the integer percent `0..=100`. Browsing a hub MUST NOT
write progress.

#### Scenario: Zero mastery is visible
- **GIVEN** a newly registered learner with empty `completed_levels`
- **WHEN** the shell renders `PartitionNav`
- **THEN** `#partition-nav-1` has `data-mastery="0"` and shows `0%` (or a
  0-valued ring)

#### Scenario: Earned drill raises percent
- **GIVEN** a learner whose `completed_levels` includes a partition-2 drill
- **WHEN** the compass renders
- **THEN** `#partition-nav-2` has `data-mastery` greater than `0`

### Requirement: Conceptual journey (ADR 003)
The product SHALL ship a Playwright journey named by page path that verifies
the full conceptual loop without uploading learner source to Go.

#### Scenario: Compass to hub 2 to drill to completion
- **GIVEN** a freshly registered authenticated learner
- **WHEN** they open `/learn` for a partition-2 step (unlocking prior rail
  cells as needed)
- **AND** they activate `#partition-nav-2`
- **THEN** they land on `/concepts/2`
- **AND** when they open a listed drill, land on its `/learn/:step`, submit
  the catalog `solution_example`, and Validar succeeds (E2E mock)
- **THEN** `#learn-progress-check` is visible
- **AND** returning to `/concepts/2` marks that drill done
- **AND** `#partition-nav-2` `data-mastery` is greater than `0`

## MODIFIED Requirements

### Requirement: Multi-tag index without catalog duplication
The external index remains the source of tags (no `partition` field on each
`CodingStep`). Wave A densifies 1–3 as specified above; the v1 “foundations
sample only” allowance is withdrawn for `4..=100`.

#### Scenario: Invalid tag rejected in tests
- **GIVEN** the concept index
- **WHEN** unit tests validate mappings
- **THEN** no tagged micro-step points outside the catalog
- **AND** `STEP_PARTITIONS` stays sorted by `micro_step` ascending
