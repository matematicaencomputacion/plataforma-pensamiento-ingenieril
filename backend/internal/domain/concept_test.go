package domain

import (
	"encoding/json"
	"testing"
)

func TestConceptAndCompetencyInstantiation(t *testing.T) {
	t.Parallel()

	concept := Concept{
		ID:      ConceptID("variables"),
		Title:   "Variables",
		Summary: "Binding y rebinding de valores.",
	}
	comp := Competency{
		ConceptID: concept.ID,
		Level:     CompetencyDeveloping,
	}

	if concept.ID != "variables" {
		t.Fatalf("Concept.ID inesperado: %q", concept.ID)
	}
	if comp.Level != CompetencyDeveloping {
		t.Fatalf("Competency.Level inesperado: %q", comp.Level)
	}
}

func TestPrerequisiteHelpersAndLegacyJSON(t *testing.T) {
	t.Parallel()

	lessonPrereq := LessonPrerequisite("print-basics")
	conceptPrereq := ConceptPrerequisite(ConceptID("variables"))

	if lessonPrereq.Kind != PrerequisiteKindLesson || lessonPrereq.RefID != "print-basics" {
		t.Fatalf("LessonPrerequisite inválido: %+v", lessonPrereq)
	}
	if conceptPrereq.Kind != PrerequisiteKindConcept || conceptPrereq.RefID != "variables" {
		t.Fatalf("ConceptPrerequisite inválido: %+v", conceptPrereq)
	}

	var legacy PrerequisiteList
	if err := json.Unmarshal([]byte(`["print-basics","variables-and-types"]`), &legacy); err != nil {
		t.Fatalf("unmarshal legado: %v", err)
	}
	ids := legacy.LessonIDs()
	if len(ids) != 2 || ids[0] != "print-basics" || ids[1] != "variables-and-types" {
		t.Fatalf("LessonIDs legado inesperado: %+v", ids)
	}

	var structured PrerequisiteList
	if err := json.Unmarshal([]byte(`[
		{"kind":"lesson","ref_id":"print-basics"},
		{"kind":"concept","ref_id":"variables"}
	]`), &structured); err != nil {
		t.Fatalf("unmarshal estructurado: %v", err)
	}
	if len(structured.LessonIDs()) != 1 || structured.LessonIDs()[0] != "print-basics" {
		t.Fatalf("LessonIDs estructurado inesperado: %+v", structured.LessonIDs())
	}
}

func TestLessonNodeConceptSetMapping(t *testing.T) {
	t.Parallel()

	node := LessonNode{
		ID:    "variables-and-types",
		Title: "Variables y tipos",
		Concepts: []ConceptID{
			"variables",
			"data_types",
			"print_io",
		},
		Prerequisites: PrerequisiteList{
			LessonPrerequisite("print-basics"),
		},
		Competencies: []Competency{
			{ConceptID: "variables", Level: CompetencyDeveloping},
		},
		SkillTarget: "variables",
	}

	var carrier ConceptSet = node
	ids := carrier.ConceptIDs()
	if len(ids) != 3 {
		t.Fatalf("ConceptIDs: got %d want 3", len(ids))
	}
	if !node.HasConcept("data_types") {
		t.Fatal("HasConcept debería encontrar data_types")
	}
	if node.HasConcept("abstraction") {
		t.Fatal("HasConcept no debería encontrar abstraction")
	}
	if len(node.Prerequisites.LessonIDs()) != 1 {
		t.Fatalf("prerrequisitos de lección inesperados: %+v", node.Prerequisites.LessonIDs())
	}
}

func TestCurriculumGraphResolveConcepts(t *testing.T) {
	t.Parallel()

	graph := CurriculumGraph{
		Concepts: map[ConceptID]Concept{
			"variables": {ID: "variables", Title: "Variables", Summary: "Binding"},
			"print_io":  {ID: "print_io", Title: "Print", Summary: "Salida"},
		},
		Lessons: map[string]LessonNode{
			"variables-and-types": {
				ID:       "variables-and-types",
				Concepts: []ConceptID{"variables", "print_io", "naming"},
			},
		},
	}

	resolved, err := graph.ConceptsForLesson("variables-and-types")
	if err != nil {
		t.Fatalf("ConceptsForLesson: %v", err)
	}
	if len(resolved) != 3 {
		t.Fatalf("se esperaban 3 conceptos, got %d", len(resolved))
	}
	if resolved[0].Title != "Variables" {
		t.Fatalf("primer concepto no resuelto desde catálogo: %+v", resolved[0])
	}
	if resolved[2].ID != "naming" || resolved[2].Title != "" {
		t.Fatalf("concepto ausente del catálogo debería degradar a ID: %+v", resolved[2])
	}
}
