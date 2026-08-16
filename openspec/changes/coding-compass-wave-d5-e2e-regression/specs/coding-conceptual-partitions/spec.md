## ADDED Requirements

### Requirement: Conceptual hub regression journey (ADR 003)
The product SHALL ship a Playwright journey named
`web/e2e/tests/journey.concepts-hub.spec.ts` that verifies the Wave D
partition hub as a page-named path. The journey MUST register a learner,
open `/concepts/1`, and assert the live catalog (no mock of
`STEP_PARTITIONS` tags, no Pyodide mock). It MUST be listed in
`make harness-journeys` (`scripts/harness/run.sh` `run_web_journeys`) and
MUST run inside the existing 6-shard CI suite (`npx playwright test
--shard=N/6` with `trunk build` + `STATIC_DIR`, not `trunk serve`).
Learner Python MUST still run only in the browser (ADR 002). This change
MUST NOT retag `STEP_PARTITIONS` (`1..=1000`) and MUST NOT add Go routes.

#### Scenario: Fresh register lands on hub 1 with a 100-cell heatmap
- **GIVEN** a newly registered authenticated learner
- **WHEN** they activate `#partition-nav-1`
- **THEN** they land on `/concepts/1`
- **AND** `#concept-heatmap` is visible
- **AND** `#concept-heatmap` contains 100 cells

#### Scenario: Non-empty decade opens the drawer (D.1)
- **GIVEN** that learner on `/concepts/1`
- **WHEN** they activate a heatmap cell whose `data-state` is not `empty`
- **THEN** `#concept-decade-drawer` is visible
- **AND** the location remains `/concepts/1`

#### Scenario: AND facet filters list and heatmap (D.2)
- **GIVEN** that learner on `/concepts/1` with the decade drawer closed
- **WHEN** they activate `#concept-facet-p3`
- **THEN** every remaining listed drill is tagged with partitions 1 and 3
- **AND** the heatmap `data-facet="hit"` count is smaller than before
  the chip

#### Scenario: New user sees the missing-base alert (D.4)
- **GIVEN** a newly registered learner with empty `completed_levels`
- **WHEN** they are on `/concepts/1`
- **THEN** `#concept-prereq-alert` is visible
- **AND** it has `role="alert"`
