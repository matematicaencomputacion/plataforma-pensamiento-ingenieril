## ADDED Requirements

### Requirement: Decade coverage heatmap on the partition hub
Authenticated `/concepts/:id` SHALL show a static 100-cell coverage heatmap
for the active partition (`#concept-heatmap`). Each cell MUST represent one
inclusive decade of the coding rail (`1..=10`, `11..=20`, … `991..=1000`).
A cell's visible state MUST be derived only from
`drills_for_partition(id) ∩ completed_levels` in that decade:

- `empty` — no tagged drills for this partition in the decade
- `pending` — at least one tagged drill, none completed
- `partial` — some but not all tagged drills completed
- `done` — every tagged drill in the decade is completed

Color alone MUST NOT convey state: each cell MUST expose `data-state` and an
accessible name that includes the decade range and a `done/total` count
(zero when `empty`). The existing drill list (`#concepts-drill-list`) MUST
remain visible. This change MUST NOT retag `STEP_PARTITIONS` (including
`1..=1000`) and MUST NOT add Go routes or analytics events.

#### Scenario: Hub shows a 100-cell heatmap
- **GIVEN** an authenticated learner on `/concepts/1`
- **WHEN** the hub panel renders
- **THEN** `#concept-heatmap` is visible
- **AND** it contains 100 cells
- **AND** `#concepts-drill-list` is still visible

#### Scenario: A tagged decade is not empty
- **GIVEN** partition 2 and the decade that contains micro-step 52
- **WHEN** the heatmap is computed for empty `completed_levels`
- **THEN** that cell has `data-state="pending"`
- **AND** its accessible name includes a total greater than 0

#### Scenario: Completing a drill updates the cell
- **GIVEN** a decade with at least one partition-1 drill
- **WHEN** `completed_levels` includes that drill and not every other drill
  in the same decade
- **THEN** the cell has `data-state="partial"` or `data-state="done"`
- **AND** browsing the hub MUST NOT write progress

#### Scenario: Empty decades do not navigate
- **GIVEN** a cell with `data-state="empty"`
- **WHEN** the learner activates it
- **THEN** the location remains `/concepts/:id`

#### Scenario: Clicking a covered decade opens a drill
- **GIVEN** an authenticated learner on `/concepts/1` and a cell whose
  state is `pending`, `partial`, or `done`
- **WHEN** they activate that cell
- **THEN** they land on `/learn/:step` for a catalogued drill in that decade
  tagged with partition 1
- **AND** if any drill in the decade is not completed, the target is the
  first pending drill in rail order

#### Scenario: Index 1..=1000 is untouched
- **GIVEN** `STEP_PARTITIONS` at the C6 baseline (`87a5334`)
- **WHEN** unit tests collect rows with `micro_step` in `1..=1000`
- **THEN** the set of `(micro_step, tags)` pairs is unchanged
- **AND** there are no new Go routes and no analytics events
- **AND** learner Python still runs only in the browser (ADR 002)
