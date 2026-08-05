## ADDED Requirements

### Requirement: Load versioned microstep seed
The system SHALL load a versioned microstep seed document that includes `metadata` (at least `id`, `version`, `total_steps`) and an ordered `steps` array for the Python Foundations lot (HOME through Casting).

#### Scenario: Seed available to Exercise Workspace
- **WHEN** the Exercise Workspace initializes
- **THEN** the runtime loads the seed asset bundled for the frontend
- **AND** exposes at least 10 steps with stable `id` values

#### Scenario: Unknown step id
- **WHEN** navigation requests a step id not present in the seed
- **THEN** the system falls back to the first step or shows a recoverable error
- **AND** MUST NOT crash the application shell

### Requirement: Typed step contract
Each step MUST provide `id`, `step_number`, `title`, `content.prompt_md`, `content.starter_code`, and `checks.mode`. Steps MAY include `hint`, `solution_example`, `next`, MCQ fields, and a pytest/test payload.

#### Scenario: Normalize starter into editor
- **WHEN** a step is selected
- **THEN** the editor is initialized with that step’s `starter_code`
- **AND** the theory panel renders `prompt_md`

### Requirement: Check adapter for client runner
The seed runtime SHALL adapt authoring-time check placeholders (including pseudo `exec(open('solution.py'))` snippets) into a client-runner contract that supplies student code and test source to the in-browser Python engine.

#### Scenario: Adapter produces runnable suite
- **WHEN** the loader prepares a coding step for Check
- **THEN** it produces a normalized payload with student code and test source suitable for Pyodide
- **AND** does not require a filesystem path on the host machine

#### Scenario: No server execution
- **WHEN** checks are evaluated
- **THEN** evaluation completes without calling a Go API to execute student code

### Requirement: Sequential next pointer
Steps that define `next` SHALL reference another step `id` in the same seed (except the frontier step, which MAY point to a documented future id such as Strings).

#### Scenario: Advance along next
- **WHEN** the student completes a step that defines `next` to an existing id
- **THEN** Continuar targets that step id

#### Scenario: Frontier after Casting
- **WHEN** the student completes the Casting step whose `next` is outside the loaded seed
- **THEN** the UI indicates the lot is complete (or shows a non-blocking “próximamente Strings” state)
- **AND** MUST NOT navigate to a broken empty editor without explanation
