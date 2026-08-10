## ADDED Requirements

### Requirement: Authenticated onboarding entry
The Leptos CSR shell SHALL expose an `/onboarding` route that is only usable
with a hydrated authenticated session. Unauthenticated access MUST redirect to
`/login` with `replace` navigation (same guard semantics as `/workspace`).

#### Scenario: Anonymous deep-link
- **GIVEN** no valid session token
- **WHEN** the user opens `/onboarding`
- **THEN** they are redirected to `/login` and do not see the coaching editor

#### Scenario: Authenticated deep-link
- **GIVEN** a hydrated session with user email
- **WHEN** the user opens `/onboarding`
- **THEN** they see the onboarding coaching surface with a clear page title

### Requirement: Coaching drafting surface
The onboarding page SHALL present at least one coaching prompt, a notes
textarea, and primary actions consistent with drafting state. Notes MUST be
editable while `drafting`.

#### Scenario: Draft notes
- **WHEN** an authenticated learner types in the notes field
- **THEN** the typed text remains visible (controlled Leptos signal) without a
  full page reload

### Requirement: Workspace entry point
The Workspace hub SHALL expose a clear navigation affordance to `/onboarding`
so authenticated learners can reach coaching without using the legacy Qwik app.

#### Scenario: From workspace hub
- **GIVEN** the learner is on `/workspace` with a live session
- **WHEN** they activate the onboarding CTA/link
- **THEN** they navigate to `/onboarding`

### Requirement: Qwik legacy untouched
This change MUST NOT modify files under `frontend/` (Qwik). Backend Go JSON
contracts for synthesize/profile MUST remain unchanged in the shell slice.

#### Scenario: Repo boundary
- **WHEN** reviewers inspect the shell PR diff
- **THEN** there are no edits under `frontend/` and no breaking Go handler
  signature changes

### Requirement: Profile synthesize from drafting notes
The onboarding surface SHALL call `POST /api/learner/profile/synthesize` with
`raw_notes` and `source_step_id` when the learner submits for analysis. On
success it MUST move to a reviewing state with editable purpose, urgency,
vision, and stack fields populated from the API JSON.

#### Scenario: Happy-path analyze (mock LLM)
- **GIVEN** an authenticated learner on `/onboarding` with notes of at least 12 unicode runes
- **WHEN** they activate «Enviar para análisis»
- **THEN** the UI shows analyzing feedback, then four editable profile fields
  reflecting the synthesize response

#### Scenario: Too-short notes
- **GIVEN** notes shorter than the backend minimum
- **WHEN** the learner attempts analysis (client guard or API 400)
- **THEN** they remain in drafting with a clear error and do not see a completed profile builder

### Requirement: Persist and hydrate learner profile
The onboarding surface SHALL load any existing profile via
`GET /api/user/profile` (Bearer) on entry. From reviewing, «Guardar perfil»
SHALL `PUT /api/user/profile` with `lifePurpose`, `urgency`, `vision5Years`,
and `techStack`. On success the UI MUST enter a saved state with a clear
continue affordance toward Paso 2 / workspace.

#### Scenario: Save then reload
- **GIVEN** an authenticated learner who synthesized and edited profile fields
- **WHEN** they save successfully and later reopen `/onboarding`
- **THEN** the UI shows the saved state with the previously persisted field values
