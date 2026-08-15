## MODIFIED Requirements

### Requirement: Multi-tag index with curriculum-wide family coverage

Micro-steps MAY belong to zero or more conceptual partitions through a static
external index. The index MUST support compact range rules and explicit
exceptions, MUST resolve deterministically without parsing user-facing titles,
and MUST NOT require modifying every `CodingStep`.

#### Scenario: Representative families are discoverable

- **GIVEN** the 1000-step Python curriculum
- **WHEN** a learner opens partitions 1, 2 or 3
- **THEN** the drills include representative Foundations, collection, function,
  class and DSA families whose learning objective materially uses that lens

#### Scenario: Untagged is an explicit valid outcome

- **GIVEN** a micro-step whose objective does not materially exercise partitions
  1–3
- **WHEN** the conceptual index resolves it
- **THEN** it MAY return no tags rather than assigning a generic default

### Requirement: Concept index integrity

Every tagged micro-step MUST exist in the coding catalog, every partition id MUST
be in `1..=5`, and overlapping rules MUST have deterministic precedence.

#### Scenario: Range boundary remains stable

- **GIVEN** a range rule and an explicit exception at one of its steps
- **WHEN** tests resolve the first, exception and last micro-step
- **THEN** the expected static tag slice is returned for each boundary

### Requirement: Progress isolation

Expanding conceptual coverage MUST NOT mutate `current_level` or
`completed_levels`. Mastery MUST continue to equal earned tagged drills divided
by all tagged drills in that partition.

#### Scenario: Expanded denominator uses earned completions only

- **GIVEN** a learner with a sparse `completed_levels` set
- **WHEN** a partition contains drills from multiple curriculum families
- **THEN** only exact completed micro-step ids count as mastered
