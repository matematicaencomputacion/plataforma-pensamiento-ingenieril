## ADDED Requirements

### Requirement: Authenticated learn entry
The Leptos CSR shell SHALL expose `/learn` usable only with a hydrated
authenticated session. Anonymous deep-links MUST redirect to `/login` with
`replace` navigation.

#### Scenario: Anonymous deep-link
- **GIVEN** no live session
- **WHEN** the user opens `/learn`
- **THEN** they are redirected to `/login` and do not see the coding editor

#### Scenario: Authenticated entry
- **GIVEN** a hydrated session
- **WHEN** the user opens `/learn`
- **THEN** they see the coding surface with enunciado and editor

### Requirement: Client-side Python runtime
Python execution for learners MUST run only in the browser via Pyodide (ADR 002).
The page SHALL lazy-bootstrap the engine and surface clear status
(`loading` / `ready` / `error`). The Go API MUST NOT receive learner source for
execution.

#### Scenario: Engine ready
- **GIVEN** `/learn` for an authenticated learner
- **WHEN** the Pyodide glue finishes `ensure`
- **THEN** Run and Validar become enabled and the status indicates the engine is ready

### Requirement: Run and validate
The coding surface SHALL provide Run (stdout/stderr capture) and Validar
(execute seed `test_*` harness against learner code). Successful Validar MUST
unlock Continuar toward the workspace hub (v1).

#### Scenario: Happy-path validate
- **GIVEN** an authenticated learner on `/learn` with the first coding step
- **WHEN** they submit correct code and activate Validar
- **THEN** the console shows a pass summary and Continuar is enabled

### Requirement: Onboarding continue targets learn
After onboarding profile is saved, «Continuar al Paso 2» SHALL navigate to
`/learn` (not only `/workspace`).

#### Scenario: From saved onboarding
- **GIVEN** the learner is on `/onboarding` in saved state
- **WHEN** they activate Continuar al Paso 2
- **THEN** they land on `/learn`

### Requirement: Qwik and Go execution boundaries
This change MUST NOT modify `frontend/` and MUST NOT add Go endpoints that
execute learner Python.

#### Scenario: Diff boundary
- **WHEN** reviewers inspect the PR
- **THEN** there are no `frontend/` edits and no new Go code-execution routes
