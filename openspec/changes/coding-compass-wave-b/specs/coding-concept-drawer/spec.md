## ADDED Requirements

### Requirement: Progressive conceptual FAB on learn
Authenticated `/learn/:step` SHALL expose a progressive conceptual widget
with exactly four states that grow in place (not a full-page Spotlight):
State 0 `Collapsed` (≈40px bubble `#concept-fab`, labeled for lentes
`[1]…[5]`), State 1 `Search` (quick search `#concept-glossary-search` **and**
five chromatic chips `#concept-lens-1` through `#concept-lens-5` plus hits
grouped by search intent), State 2 `MicroCard` (peek of one glossary entry),
and State 3 `Docked` (lateral drawer `#concept-drawer` ≈40% / editor ≈60%).
The widget MUST NOT appear on `/learn` without a step id, `/concepts/:id`,
or `/workspace`. Changing state MUST NOT navigate away from `/learn/:step`.
A header search box is out of scope.

#### Scenario: Default is collapsed bubble
- **GIVEN** an authenticated learner on `/learn/:step`
- **WHEN** the learn page hydrates
- **THEN** `#concept-fab` is visible at collapsed size
- **AND** `#concept-glossary-search` is not shown
- **AND** `#concept-drawer` is not docked

#### Scenario: Successive click grows to search
- **GIVEN** the FAB is in State 0
- **WHEN** the learner activates `#concept-fab`
- **THEN** the widget is State 1
- **AND** `#concept-glossary-search` is visible
- **AND** chips `#concept-lens-1` through `#concept-lens-5` are visible

#### Scenario: Hit opens micro-card peek
- **GIVEN** State 1 with search hits
- **WHEN** the learner activates one hit
- **THEN** State 2 shows a single peek for that entry
- **AND** the learn route is unchanged

#### Scenario: Escape from peek returns to the exercise
- **GIVEN** the widget is in State 2
- **WHEN** the learner presses Escape
- **THEN** the widget returns to State 0
- **AND** `#learn-editor` remains mounted with its current text

### Requirement: Keyboard search shortcut
On `/learn/:step` the product SHALL treat `Ctrl+K` (Windows/Linux) and
`Cmd+K` (macOS) as opening **State 1**. The shortcut MUST `preventDefault`
on that view. If the widget is already in State 1–3, the shortcut MUST focus
`#concept-glossary-search` rather than close it. The shortcut MUST NOT be
registered on other routes.

#### Scenario: Cmd/Ctrl+K opens State 1
- **GIVEN** an authenticated learner on `/learn/:step` in State 0
- **WHEN** they press `Meta+K` or `Control+K`
- **THEN** `#concept-glossary-search` is visible and focused
- **AND** the five lens chips are visible

#### Scenario: Shortcut focuses existing search
- **GIVEN** the widget is in State 1, 2, or 3
- **WHEN** the learner presses `Meta+K` or `Control+K`
- **THEN** `#concept-glossary-search` receives focus
- **AND** the widget does not collapse to State 0

### Requirement: Chromatic lens selector
State 1 and docked surfaces SHALL offer one chromatic lens chip per
partition with canonical colors: P1 Azul Memoria `#3B82F6`, P2 Violeta
Ámbitos `#8B5CF6`, P3 Ámbar Paradigmas `#F59E0B`, P4 Verde Ecosistema
`#10B981`, P5 Magenta Dominios `#EC4899`. Each chip MUST include a text
label (color alone is not sufficient), MUST set `data-lens` to `1..=5`,
and MUST use `PartitionId::color_badge()` / `label()`. An active lens
filter MUST set `aria-pressed`. Filtering MUST NOT write `current_level`
or `completed_levels`.

#### Scenario: Five labeled chips in State 1
- **GIVEN** the widget is in State 1
- **WHEN** the lens row renders
- **THEN** chips 1–5 exist with the Spanish lens names above
- **AND** each has `data-lens` equal to its partition id

#### Scenario: Lens filters glossary hits
- **GIVEN** a query that matches entries with more than one lens
- **WHEN** the learner activates `#concept-lens-2`
- **THEN** visible hits include only entries that have a P2 lens
- **AND** `#concept-lens-2` has `aria-pressed="true"`

### Requirement: Four search intents
State 1 SHALL group hits into four intents derived from `GlossaryEntry.id`
prefixes (`model-`, `syntax-`, `pattern-`, `trap-`; unprefixed canonical
entries such as `python-lists` group as mental models). The four groups are:
mental models / partitions, syntax & stdlib methods, patterns & algorithms,
and traps & common errors. Grouping MUST be client-side over
`GLOSSARY_ENTRIES`.

#### Scenario: Seed covers all four intents
- **GIVEN** `GLOSSARY_ENTRIES`
- **WHEN** unit tests classify entries by id prefix (and unprefixed as model)
- **THEN** at least one entry exists in each of the four intents

### Requirement: Multifaceted micro-card anatomy
A glossary term MAY expose up to five `ConceptLens` values, one per
`PartitionId`. The UI MUST render up to five color pills from `lenses`.
Activating a pill SHALL show that partition's `headline`, `tldr`, and
`code_example` — not a generic dump. The State 2 peek MUST include: a
one-line TL;DR, a key-difference snippet of at most three lines, the
optional `common_pitfall`, and a CTA “Ver modelo mental en Partición N”
that navigates to `/concepts/{n}` for that lens. Escape MUST return to
the exercise (State 0) without clearing the editor.

#### Scenario: python-lists has five lenses
- **GIVEN** the seed entry `id = "python-lists"`
- **WHEN** its `lenses` are inspected
- **THEN** there is exactly one lens per `PartitionId` P1..=P5
- **AND** keywords include `append`, `extend`, and `slice`

#### Scenario: Pill selects the partition explanation
- **GIVEN** State 2 for a multi-lens entry
- **WHEN** the learner activates the P1 pill
- **THEN** the peek shows the P1 `tldr` and `code_example`
- **AND** the CTA targets `/concepts/1`

### Requirement: Docked split-view preserves editor
The learner SHALL be able to dock the glossary as a lateral drawer without
losing editor contents. Docking MUST keep the same `code` signal and MUST
NOT unmount `#learn-editor`. On viewports ≥ 1100px the drawer SHALL occupy
≈40% and the editor pane ≈60%. On narrower viewports the drawer MAY overlay
the theory pane but MUST NOT `display: none` the editor. Undocking MUST
restore State 0 without clearing `code`.

#### Scenario: Marker text survives dock
- **GIVEN** the learner has typed a distinctive marker into `#learn-editor`
- **WHEN** they activate Anclar and the drawer docks
- **THEN** `#concept-drawer` is visible
- **AND** `#learn-editor` still contains the marker
- **AND** the URL remains `/learn/:step`

#### Scenario: Undock does not clear code
- **GIVEN** a docked drawer and non-empty editor text
- **WHEN** the learner undocks or presses Escape from State 3
- **THEN** the editor text is unchanged
- **AND** `#concept-fab` is available again in State 0

### Requirement: Client-side glossary module
The web crate SHALL expose `web/src/concepts/glossary.rs` with the locked
types `PartitionId` (enum `P1MemoryData` … `P5Domains` plus `label`,
`color_badge`, `as_u8`, `from_u8`), `ConceptLens`, `GlossaryEntry`, static
`GLOSSARY_ENTRIES`, and `search_glossary(query, lens)`. The catalog MUST be
`'static` in-memory data (no Go endpoint, no database, no network). Search
MUST be case-insensitive over `title` and `keywords`, complete well under
5ms for this seed size, and return `python-lists` for queries `append`,
`extend`, or `slice`. `PartitionId::from_u8` MUST accept only `1..=5` and
MUST map onto existing `ConceptPartition.id` values. `related_step_id`,
when `Some`, MUST be a catalogued `CodingStep.id`. Wave B MUST ship a seed
of at most 32 entries (target 12–20), not a full encyclopedia. This change
MUST NOT introduce a second partition-id type beside `PartitionId` and
`ConceptPartition.id: u8`.

#### Scenario: Typed partition id maps to Wave A numeric ids
- **GIVEN** `PartitionId::from_u8` and `as_u8`
- **WHEN** unit tests convert `0`, `1`, `3`, and `6`
- **THEN** `0` and `6` are none/errors, `1` is `P1MemoryData`, and `3`
  round-trips to `3`

#### Scenario: Search hits python-lists via keywords
- **GIVEN** `GLOSSARY_ENTRIES`
- **WHEN** `search_glossary("extend", None)` runs
- **THEN** the results include the entry whose `id` is `python-lists`

#### Scenario: No server glossary API
- **WHEN** reviewers inspect the change
- **THEN** there are no new Go routes for glossary or conceptual search
- **AND** learner Python still runs only in the browser (ADR 002)
