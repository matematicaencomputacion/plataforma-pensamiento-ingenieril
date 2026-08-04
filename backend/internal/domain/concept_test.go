package domain

import (
	"encoding/json"
	"strings"
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

func TestConceptOptionalMediaAndActiveSegment(t *testing.T) {
	t.Parallel()

	plain := Concept{ID: "concept:plain", Title: "Sin media"}
	if plain.HasMedia() {
		t.Fatal("concepto sin resource_url/transcript no tiene media")
	}

	raw := `{
		"id":"concept:string-literals",
		"title":"Literales",
		"summary":"demo",
		"resource_url":"https://www.youtube.com/watch?v=kqtD5dpn9C8",
		"transcript":[
			{"start_sec":0,"end_sec":10,"text":"intro"},
			{"start_sec":10,"end_sec":25,"text":"cuerpo"}
		]
	}`
	var concept Concept
	if err := json.Unmarshal([]byte(raw), &concept); err != nil {
		t.Fatalf("unmarshal media: %v", err)
	}
	if !concept.HasMedia() {
		t.Fatal("se esperaba HasMedia tras unmarshal")
	}
	if concept.ResourceURL == "" || len(concept.Transcript) != 2 {
		t.Fatalf("media incompleta: %+v", concept)
	}

	seg, ok := concept.ActiveTranscriptSegment(12)
	if !ok || seg.Text != "cuerpo" {
		t.Fatalf("segmento activo inesperado: ok=%v seg=%+v", ok, seg)
	}
	seg, ok = concept.ActiveTranscriptSegment(3)
	if !ok || seg.Text != "intro" {
		t.Fatalf("segmento intro inesperado: ok=%v seg=%+v", ok, seg)
	}
}

func TestConceptMultilingualResourcesUnmarshalAndResolve(t *testing.T) {
	t.Parallel()

	raw := `{
		"id":"concept:string-literals",
		"title":"Literales",
		"summary":"demo",
		"resources":{
			"es":{
				"resource_url":"https://www.youtube.com/watch?v=es-video",
				"transcript":[{"start_sec":0,"end_sec":10,"text":"hola"}]
			},
			"en":{
				"resource_url":"https://www.youtube.com/watch?v=en-video",
				"transcript":[{"start_sec":0,"end_sec":10,"text":"hello"}]
			}
		}
	}`
	var concept Concept
	if err := json.Unmarshal([]byte(raw), &concept); err != nil {
		t.Fatalf("unmarshal resources: %v", err)
	}
	if !concept.HasMedia() {
		t.Fatal("HasMedia debería detectar resources")
	}
	locales := concept.AvailableMediaLocales()
	if len(locales) != 2 {
		t.Fatalf("locales: %+v", locales)
	}

	es, ok := concept.MediaFor("es")
	if !ok || !strings.Contains(es.ResourceURL, "es-video") {
		t.Fatalf("MediaFor(es): %+v ok=%v", es, ok)
	}
	en, ok := concept.MediaFor("en")
	if !ok || !strings.Contains(en.ResourceURL, "en-video") {
		t.Fatalf("MediaFor(en): %+v ok=%v", en, ok)
	}

	seg, ok := concept.ActiveTranscriptSegmentFor("en", 2)
	if !ok || seg.Text != "hello" {
		t.Fatalf("segmento EN: ok=%v %+v", ok, seg)
	}
	seg, ok = concept.ActiveTranscriptSegmentFor("es", 2)
	if !ok || seg.Text != "hola" {
		t.Fatalf("segmento ES: ok=%v %+v", ok, seg)
	}
}

func TestConceptLegacyMediaFallback(t *testing.T) {
	t.Parallel()

	concept := Concept{
		ID:          "concept:legacy",
		ResourceURL: "https://www.youtube.com/watch?v=legacy",
		Transcript: []TranscriptSegment{
			{StartSec: 0, EndSec: 5, Text: "legado"},
		},
	}
	media, ok := concept.MediaFor("es")
	if !ok || media.ResourceURL != concept.ResourceURL || media.Transcript[0].Text != "legado" {
		t.Fatalf("fallback legado ES falló: ok=%v %+v", ok, media)
	}
	// Sin resources, EN cae al legado es.
	media, ok = concept.MediaFor("en")
	if !ok || media.Transcript[0].Text != "legado" {
		t.Fatalf("fallback EN→legado: ok=%v %+v", ok, media)
	}
}
