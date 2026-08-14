## ADDED Requirements

### Requirement: Canonical five partitions
The product SHALL expose exactly five conceptual partitions with stable numeric
ids `1..=5` and stable slugs (`data-model`, `scope-legb`, `paradigms`,
`ecosystem`, `application-domains`). Each partition MUST provide a short mental
model and a list of thematic axes for the hub view.

#### Scenario: Authenticated compass
- **GIVEN** a hydrated session on `/workspace` or `/learn`
- **WHEN** the shell renders
- **THEN** the learner sees five numbered partition controls `1`–`5`

### Requirement: Multi-tag index without catalog duplication
Micro-steps MAY belong to zero or more partitions via an external index. The
system MUST NOT require rewriting every `CodingStep` constant to ship v1. Every
tagged `micro_step` MUST exist in the coding catalog.

#### Scenario: Invalid tag rejected in tests
- **GIVEN** the concept index
- **WHEN** unit tests validate mappings
- **THEN** no tagged micro-step points outside `1..=MICRO_STEP_COUNT` / catalog

### Requirement: Partition hub anatomy
Opening partition `N` SHALL show: (1) mental model, (2) thematic axes,
(3) drill links to tagged micro-steps with completion from `completed_levels`.

#### Scenario: Drill opens canonical learn route
- **GIVEN** the learner is on `/concepts/1`
- **WHEN** they activate a drill for micro-step 20
- **THEN** they navigate to the canonical `/learn/:step` for that micro-step

### Requirement: Progress isolation
Filtering or browsing by partition MUST NOT alter `current_level` or
`completed_levels`. Completion marks MUST reflect earned levels only.

#### Scenario: Filter does not invent completion
- **GIVEN** completed_levels = `[20]`
- **WHEN** partition 1 hub renders
- **THEN** only drills whose micro_step is in completed_levels show as done

### Requirement: Active lens on learn
While on `/learn/:step`, partition controls that tag the current micro-step
MUST be visually emphasized. A badge on the learn surface SHALL link back to
the primary partition hub when tags exist.

#### Scenario: Mutability exercise highlights partition 1
- **GIVEN** the learner opens a step tagged with partition 1
- **WHEN** the learn page loads
- **THEN** control `1` is emphasized and a conceptual badge is visible

### Requirement: ADR 002 boundary
Partitions 4 and 5 MUST operate as conceptual maps / light Pyodide-safe drills.
The Go API MUST NOT execute learner Python. No new server-side code-execution
endpoints are allowed.

#### Scenario: Diff boundary
- **WHEN** reviewers inspect the change
- **THEN** there are no Go routes that execute learner source
