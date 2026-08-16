## ADDED Requirements

### Requirement: Decade heatmap opens a filtered micro-step list
Authenticated `/concepts/:id` SHALL, when the learner activates a heatmap
cell whose state is `pending`, `partial`, or `done`, open a dialog list
(`#concept-decade-drawer`) of the catalogued micro-steps in that decade
that are tagged with the active partition. A decade remains ten consecutive
micro-steps (`1..=10`, `11..=20`, … `991..=1000`). The list MUST NOT include
micros tagged only with other partitions, nor untagged steps. Activating a
list item MUST navigate to `/learn/:id` for that exercise. Activating an
`empty` cell MUST NOT open the list and MUST NOT change the location.
The dialog MUST expose `role="dialog"`, a visible title naming the decade
range, and close on Escape without navigating. The Wave A drill list
(`#concepts-drill-list`) MUST remain visible. This change MUST NOT retag
`STEP_PARTITIONS` (including `1..=1000`), MUST NOT unmount `#learn-editor`,
and MUST NOT add Go routes or analytics events.

#### Scenario: Non-empty decade opens the filtered list
- **GIVEN** an authenticated learner on `/concepts/1`
- **AND** a heatmap cell whose state is not `empty`
- **WHEN** they activate that cell
- **THEN** `#concept-decade-drawer` is visible
- **AND** the location remains `/concepts/1`
- **AND** every listed micro-step falls in that cell's decade range
- **AND** every listed micro-step is tagged with partition 1
- **AND** `#concepts-drill-list` is still visible

#### Scenario: Choosing a listed micro-step opens learn
- **GIVEN** the decade dialog is open on `/concepts/1`
- **WHEN** the learner activates a listed micro-step
- **THEN** they land on `/learn/:id` for that catalogued exercise

#### Scenario: Empty decades do not open the list
- **GIVEN** a cell with `data-state="empty"`
- **WHEN** the learner activates it
- **THEN** `#concept-decade-drawer` is not present
- **AND** the location remains `/concepts/:id`

#### Scenario: Escape closes the decade list
- **GIVEN** `#concept-decade-drawer` is visible
- **WHEN** the learner presses Escape
- **THEN** the dialog is gone
- **AND** the location remains `/concepts/:id`

#### Scenario: Index 1..=1000 is untouched
- **GIVEN** `STEP_PARTITIONS` at the Wave D baseline (`f7734a8`)
- **WHEN** unit tests collect rows with `micro_step` in `1..=1000`
- **THEN** the set of `(micro_step, tags)` pairs is unchanged
- **AND** there are no new Go routes and no analytics events
- **AND** learner Python still runs only in the browser (ADR 002)

## MODIFIED Requirements

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
remain visible. Activating a non-`empty` cell MUST open the decade list
(not navigate directly to the first pending drill). This change MUST NOT
retag `STEP_PARTITIONS` (including `1..=1000`) and MUST NOT add Go routes
or analytics events.

#### Scenario: Hub shows a 100-cell heatmap
- **GIVEN** an authenticated learner on `/concepts/1`
- **WHEN** the hub panel renders
- **THEN** `#concept-heatmap` is visible
- **AND** it contains 100 cells
- **AND** `#concepts-drill-list` is still visible

#### Scenario: Clicking a covered decade opens the decade list
- **GIVEN** an authenticated learner on `/concepts/1` and a cell whose
  state is `pending`, `partial`, or `done`
- **WHEN** they activate that cell
- **THEN** `#concept-decade-drawer` lists the partition-tagged micros in
  that decade
- **AND** the location stays `/concepts/1` until they choose an item
