## MODIFIED Requirements

### Requirement: Hub shows conceptual prerequisites
Authenticated `/concepts/:id` SHALL render `#concept-prereq-list` with
the `Requires` and `Reinforces` edges whose `from` concept has a lens
in that partition. Each item MUST expose `data-from`, `data-to`, and
`data-kind`. When any `Requires` target in that list is not started
(see concept-dag missing-base detection), the hub MUST also render
`#concept-prereq-alert` with `role="alert"` naming at least one missing
base title (e.g. mutability). The alert and list MUST NOT replace
`#concept-heatmap`, `#concept-facet-bar`, `#concept-analytics`, or the
FAB on `/learn`. Learner Python MUST still run only in the browser
(ADR 002). This change MUST NOT retag `STEP_PARTITIONS` (`1..=1000`).

#### Scenario: Fresh learner sees lists-requires-mutability on hub 1
- **GIVEN** an authenticated learner with empty `completed_levels` and
  `current_level` 1
- **WHEN** they open `/concepts/1`
- **THEN** `#concept-prereq-list` is visible
- **AND** an item with `data-from="python-lists"` and
  `data-to="model-mutability"` is visible
- **AND** `#concept-prereq-alert` is visible and mentions mutabilidad

#### Scenario: Existing hub widgets remain intact
- **GIVEN** an authenticated learner on `/concepts/1`
- **WHEN** the prerequisite list is shown
- **THEN** `#concept-heatmap`, `#concept-facet-bar`, and
  `#concept-analytics` still exist
- **AND** `#concept-fab` on `/learn` still exists
