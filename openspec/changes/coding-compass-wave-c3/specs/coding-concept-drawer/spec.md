## ADDED Requirements

### Requirement: Layered DUA diagrams in peek and dock
Authenticated `/learn/:step` SHALL show an optional static visual diagram
for glossary mental-model entries in widget State 2 (peek) and State 3
(dock). The diagram MUST appear in a layered reading order **before** the
TL;DR and code snippet (diagram → TL;DR → snippet → pitfall). The diagram
MUST NOT be the only channel: adjacent visible text (TL;DR or accessible
name) MUST describe the same model. Color alone MUST NOT convey meaning.
Entries without a diagram MUST keep the Wave B text anatomy. Changing
diagram visibility MUST NOT unmount `#learn-editor` or clear editor text.
This change MUST NOT retag `STEP_PARTITIONS` (including `601..=1000`).
Implementation MUST NOT start until `coding-compass-wave-c1` is merged to
`main`.

#### Scenario: Mental-model peek shows a diagram
- **GIVEN** an authenticated learner on `/learn/:step` in State 2 for a
  seeded mental-model entry that has a diagram (at least `python-lists`
  or `model-legb`)
- **WHEN** the peek renders
- **THEN** `#concept-diagram` is visible inside `#concept-peek`
- **AND** the TL;DR text is still visible
- **AND** `#learn-editor` remains mounted

#### Scenario: Dock keeps the diagram and the editor
- **GIVEN** State 2 with a visible diagram and a distinctive marker in
  `#learn-editor`
- **WHEN** the learner docks to State 3
- **THEN** `#concept-diagram` remains visible in `#concept-drawer`
- **AND** `#learn-editor` still contains the marker

#### Scenario: Text-only entries stay valid
- **GIVEN** a glossary entry with no diagram
- **WHEN** its peek renders
- **THEN** `#concept-diagram` is absent
- **AND** TL;DR and snippet still render

#### Scenario: No remote diagram fetch
- **WHEN** reviewers inspect the change
- **THEN** diagrams are compile-time static assets in the web crate
- **AND** there are no new Go routes and no network fetch for diagrams
- **AND** learner Python still runs only in the browser (ADR 002)

#### Scenario: Index 601..=1000 is untouched
- **GIVEN** `STEP_PARTITIONS` at the C1 (or later C2) baseline
- **WHEN** unit tests collect rows with `micro_step ≥ 601`
- **THEN** the set of `(micro_step, tags)` pairs is unchanged
