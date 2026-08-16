## ADDED Requirements

### Requirement: Faceted conceptual search on the partition hub
Authenticated `/concepts/:id` SHALL expose a client-side facet bar
(`#concept-facet-bar`) that filters the active partition's drills using
existing `STEP_PARTITIONS` tags and glossary keywords. The bar MUST
include a search field (`#concept-facet-query`) and AND-chips for the
other four partitions (`#concept-facet-p{n}`). A drill remains in the
filtered set only when it is tagged with the active partition AND every
selected extra-partition chip AND every whitespace token of the query
(empty query and no extra chips = unfiltered Wave D/D.1 set). A token
matches a drill when the catalog title or id contains it (case-insensitive)
OR when `search_glossary` returns an entry whose `related_step_id`
resolves to that drill's micro-step. The Wave A list (`#concepts-drill-list`)
and the decade heatmap (`#concept-heatmap`) MUST both render from the
filtered set; heatmap cells with at least one matching drill MUST expose
`data-facet="hit"`. Activating a listed drill MUST navigate to `/learn/:id`.
The decade drawer MUST list only filtered drills in that decade. This
change MUST NOT retag `STEP_PARTITIONS` (including `1..=1000`), MUST NOT
rebind `Ctrl`/`Cmd`+`K` on `/learn`, MUST NOT unmount `#learn-editor`,
and MUST NOT add Go routes or analytics events.

#### Scenario: Facet bar is visible on the hub
- **GIVEN** an authenticated learner on `/concepts/1`
- **WHEN** the hub panel renders
- **THEN** `#concept-facet-bar` is visible
- **AND** `#concept-facet-query` is visible
- **AND** `#concept-heatmap` and `#concepts-drill-list` remain visible

#### Scenario: Glossary keyword filters list and heatmap
- **GIVEN** an authenticated learner on `/concepts/1`
- **AND** `#concepts-drill-20` is visible while unfiltered
- **WHEN** they apply the query `append`
- **THEN** `#concepts-drill-20` remains visible
- **AND** `#concepts-drill-1` is not present
- **AND** at least one heatmap cell has `data-facet="hit"`
- **AND** the hit count is smaller than the unfiltered cell count

#### Scenario: Filtered drill opens learn
- **GIVEN** the query `append` is applied on `/concepts/1`
- **WHEN** the learner activates `#concepts-drill-20`
- **THEN** they land on `/learn/py-20-list-change`

#### Scenario: Extra partition chip is AND with the active tab
- **GIVEN** an authenticated learner on `/concepts/1`
- **WHEN** they activate `#concept-facet-p3`
- **THEN** every remaining listed drill is tagged with partitions 1 and 3
- **AND** `#concepts-drill-20` is not present

#### Scenario: Index 1..=1000 is untouched
- **GIVEN** `STEP_PARTITIONS` at the Wave D.1 baseline (`362ec98`)
- **WHEN** unit tests collect rows with `micro_step` in `1..=1000`
- **THEN** the set of `(micro_step, tags)` pairs is unchanged
- **AND** there are no new Go routes and no analytics events
- **AND** learner Python still runs only in the browser (ADR 002)
- **AND** `Ctrl`/`Cmd`+`K` on `/learn` still opens the concept FAB

## MODIFIED Requirements

### Requirement: Decade coverage heatmap on the partition hub
Authenticated `/concepts/:id` SHALL show a static 100-cell coverage heatmap
for the active partition (`#concept-heatmap`). Each cell MUST represent one
inclusive decade of the coding rail (`1..=10`, `11..=20`, … `991..=1000`).
A cell's visible state MUST be derived from the **filtered** drill set
intersected with `completed_levels` in that decade (unfiltered = all
drills tagged with the active partition):

- `empty` — no matching drills for this partition (and current facets) in the decade
- `pending` — at least one matching drill, none completed
- `partial` — some but not all matching drills completed
- `done` — every matching drill in the decade is completed

Cells with at least one matching drill MUST expose `data-facet="hit"`.
Color alone MUST NOT convey state: each cell MUST expose `data-state` and an
accessible name that includes the decade range and a `done/total` count
(zero when `empty`). The existing drill list (`#concepts-drill-list`) MUST
remain visible and MUST list only matching drills. Activating a non-`empty`
cell MUST open the decade list of **filtered** micros (not navigate
directly). This change MUST NOT retag `STEP_PARTITIONS` (including
`1..=1000`) and MUST NOT add Go routes or analytics events.

#### Scenario: Hub shows a 100-cell heatmap
- **GIVEN** an authenticated learner on `/concepts/1`
- **WHEN** the hub panel renders
- **THEN** `#concept-heatmap` is visible
- **AND** it contains 100 cells
- **AND** `#concepts-drill-list` is still visible

#### Scenario: Applying a facet updates heatmap hits
- **GIVEN** an authenticated learner on `/concepts/1` with no facets
- **WHEN** they apply the query `append`
- **THEN** heatmap cells with matching drills expose `data-facet="hit"`
- **AND** the number of hit cells is less than the unfiltered hit count
