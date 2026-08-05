## ADDED Requirements

### Requirement: Single-viewport exercise layout
The system SHALL present a dedicated Exercise Workspace that occupies the primary viewport with three regions: theory/prompt, code editor, and check results, without rendering the home dashboard chrome (tool-stack mastery bar, InteractiveStage video) inside that viewport.

#### Scenario: Student opens exercise route
- **WHEN** the student navigates to the Exercise Workspace route
- **THEN** the UI shows the current step title, theory/prompt panel, editor with starter code, and controls to run/check
- **AND** the home InteractiveStage video player is not mounted in that viewport

#### Scenario: Exit back to hub
- **WHEN** the student activates “Salir al workspace” (or equivalent)
- **THEN** the system navigates back to the main student hub route

### Requirement: Step navigation after successful check
The system SHALL allow advancing to the step referenced by `next` only after the current step’s required checks have passed for the active `checks.mode`.

#### Scenario: Continue enabled after pass
- **WHEN** the student runs Check and all required validations for the step pass
- **THEN** the Continuar control becomes available
- **AND** activating it loads the next step’s prompt and starter code

#### Scenario: Continue blocked on fail
- **WHEN** Check fails
- **THEN** Continuar remains unavailable
- **AND** the check results panel shows a failing signal the student can act on

### Requirement: Hint and solution on demand
The system SHALL expose optional Hint and Show Solution actions that reveal `hint` and `solution_example` for the current step without submitting progress to a backend.

#### Scenario: Reveal hint
- **WHEN** the student activates Hint on a step that defines `hint`
- **THEN** the hint text becomes visible in the workspace

#### Scenario: Reveal solution
- **WHEN** the student activates Show Solution on a step that defines `solution_example`
- **THEN** the example solution becomes visible
- **AND** the student MAY copy it into the editor manually (auto-fill is optional)

### Requirement: Client-side run and check
The system SHALL execute student Python code and evaluate step checks in the browser. The system MUST NOT send student solution source to a Go endpoint for execution.

#### Scenario: Run prints to console panel
- **WHEN** the student activates Run with valid Python that prints output
- **THEN** the console/results panel shows that stdout from the client runtime

#### Scenario: Check uses browser tests
- **WHEN** the student activates Check on a step with pytest-mode checks
- **THEN** the system evaluates the student’s code against the step tests in the client runtime
- **AND** reports pass or fail in the results panel

### Requirement: Optional MCQ in the same viewport
When a step defines MCQ data (`checks.mcq` or `content.mcq_bank`), the Exercise Workspace SHALL render the question and options in the same viewport and incorporate MCQ correctness into the step’s completion rules according to `checks.mode`.

#### Scenario: Casting-style MCQ
- **WHEN** the current step includes an MCQ such as the result of `float(35)`
- **THEN** the student can select an option in the workspace
- **AND** an incorrect selection does not satisfy an MCQ-required completion mode
