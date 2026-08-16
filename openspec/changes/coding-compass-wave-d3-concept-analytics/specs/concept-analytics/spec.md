## ADDED Requirements

### Requirement: Authenticated conceptual friction events
The Go API SHALL accept `POST /api/concept-events` only with a valid
Bearer session. The body MUST be JSON with a closed `type` enum:
`concept_dwell`, `heatmap_decade_open`, `dua_fab_open`,
`learn_step_enter`, `learn_validate_fail`, `learn_validate_pass`.
The handler MUST persist `user_id` from the session token and MUST NOT
store email or other PII. The handler MUST reject payloads that include
a `code` field (ADR 002: learner Python stays in the browser). Unknown
`type` values MUST return HTTP 400. Missing Bearer MUST return HTTP 401.
A valid event MUST return HTTP 204. `concept_dwell` MUST include
`partition_id` in 1..=5. `heatmap_decade_open` MUST include
`partition_id` and `decade_lo` where `decade_lo` is the start of a
rail decade (1, 11, …, 991). Learn validate/enter events MUST include
a non-empty `step_id`. Persistence MUST follow Clean Architecture
(domain + usecase + handler + repository) on SQLite.

#### Scenario: Anonymous ingest is rejected
- **GIVEN** no Authorization header
- **WHEN** a client POSTs `/api/concept-events`
- **THEN** the response status is 401
- **AND** no row is stored

#### Scenario: Student code is rejected
- **GIVEN** an authenticated learner
- **WHEN** they POST an event that includes `"code"`
- **THEN** the response status is 400
- **AND** the error mentions ADR 002

#### Scenario: Heatmap decade open is stored
- **GIVEN** an authenticated learner
- **WHEN** they POST `{ "type": "heatmap_decade_open", "partition_id": 1, "decade_lo": 1 }`
- **THEN** the response status is 204
- **AND** a later GET summary for that user counts the decade

#### Scenario: Unknown type is rejected
- **GIVEN** an authenticated learner
- **WHEN** they POST `{ "type": "page_view" }`
- **THEN** the response status is 400

### Requirement: Per-user bottleneck summary
The Go API SHALL expose `GET /api/concept-analytics` for the
authenticated user. The response MUST include counts grouped by
`partition_id` and by `decade_lo`, plus a `bottleneck` object (or null)
pointing at the decade (preferred) or partition with the highest
friction score. Friction MUST be
`dwell + decade_open + dua_fab_open + 3 * validate_fail`.
`learn_step_enter` and `learn_validate_pass` MUST be counted but MUST
NOT increase friction. The summary MUST only include events of the
Bearer user. Missing Bearer MUST return HTTP 401.

#### Scenario: Summary shows the high-friction decade
- **GIVEN** an authenticated learner who recorded `heatmap_decade_open`
  on decade 1 of partition 1 and `learn_validate_fail` on a step in
  that decade
- **WHEN** they GET `/api/concept-analytics`
- **THEN** `bottleneck.kind` is `"decade"`
- **AND** `bottleneck.decade_lo` is 1
- **AND** `bottleneck.friction` is greater than 0

#### Scenario: Empty summary has no bottleneck
- **GIVEN** an authenticated learner with no events
- **WHEN** they GET `/api/concept-analytics`
- **THEN** `bottleneck` is null
- **AND** `partitions` and `decades` are empty arrays

### Requirement: Hub widget shows a bottleneck hint
Authenticated `/concepts/:id` SHALL render `#concept-analytics` with
`#concept-analytics-hint`. After the learner dwells on the hub or
opens a heatmap decade, the widget MUST expose `data-hint` of
`partition` or `decade` (not `none`) once the summary returns, and
the hint text MUST mention fricción or the decade range. Opening a
decade MUST emit `heatmap_decade_open` without breaking the D.1
drawer, D.2 facets, or the FAB on `/learn`. Learner Python MUST still
run only in the browser (ADR 002). This change MUST NOT retag
`STEP_PARTITIONS` (`1..=1000`).

#### Scenario: Decade open surfaces the analytics hint
- **GIVEN** an authenticated learner on `/concepts/1`
- **WHEN** they open a non-empty heatmap decade
- **THEN** `#concept-decade-drawer` is visible
- **AND** `#concept-analytics-hint` has `data-hint` equal to `decade`
  or `partition`
- **AND** the hint is visible

#### Scenario: Index and existing hub remain intact
- **GIVEN** `STEP_PARTITIONS` at the Wave D.2 baseline (`57c7d8a`)
- **WHEN** unit tests collect rows with `micro_step` in `1..=1000`
- **THEN** the set of `(micro_step, tags)` pairs is unchanged
- **AND** `#concept-heatmap`, `#concept-facet-bar`, and `#concept-fab`
  on `/learn` still exist
- **AND** learner Python still runs only in the browser (ADR 002)
