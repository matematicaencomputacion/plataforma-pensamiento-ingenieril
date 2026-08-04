package domain

import (
	"encoding/json"
	"fmt"
)

// LessonNode representa un nodo del grafo curricular (DAG).
// Implementa ConceptSet para el contrato de conceptos compartidos (PPI 1.2).
type LessonNode struct {
	ID            string           `json:"id"`
	Title         string           `json:"title"`
	TrackType     TrackType        `json:"track_type"`
	Description   string           `json:"description"`
	Prerequisites PrerequisiteList `json:"prerequisites"`
	Concepts      []ConceptID      `json:"concepts"`
	Competencies  []Competency     `json:"competencies,omitempty"`
	// SkillTarget mantiene el puente con CognitiveProfile (Vectorial).
	SkillTarget string `json:"skill_target"`
}

// ConceptIDs cumple el contrato ConceptSet.
func (n LessonNode) ConceptIDs() []ConceptID {
	out := make([]ConceptID, len(n.Concepts))
	copy(out, n.Concepts)
	return out
}

// HasConcept indica si el nodo declara el concepto indicado.
func (n LessonNode) HasConcept(id ConceptID) bool {
	for _, conceptID := range n.Concepts {
		if conceptID == id {
			return true
		}
	}
	return false
}

// CurriculumGraph modela la malla de lecciones, el catálogo de conceptos y las aristas curables.
type CurriculumGraph struct {
	Version  int                   `json:"version,omitempty"`
	Lessons  map[string]LessonNode `json:"lessons,omitempty"`
	Concepts map[ConceptID]Concept `json:"concepts,omitempty"`
	Edges    []CurriculumEdge      `json:"edges,omitempty"`
}

type curriculumGraphJSON struct {
	Version  int                      `json:"version"`
	Lessons  map[string]LessonNode    `json:"lessons"`
	Concepts json.RawMessage          `json:"concepts"`
	Edges    []CurriculumEdge         `json:"edges"`
}

// UnmarshalJSON acepta concepts como objeto (legado) o como arreglo (esquema unificado).
// Si no hay lessons explícitas, proyecta nodos desde concepts + aristas requires.
func (g *CurriculumGraph) UnmarshalJSON(data []byte) error {
	var raw curriculumGraphJSON
	if err := json.Unmarshal(data, &raw); err != nil {
		return fmt.Errorf("curriculum graph: %w", err)
	}

	g.Version = raw.Version
	g.Edges = raw.Edges
	g.Lessons = raw.Lessons

	concepts, err := unmarshalConcepts(raw.Concepts)
	if err != nil {
		return err
	}
	g.Concepts = concepts

	if len(g.Lessons) == 0 {
		g.Lessons = projectLessonsFromConcepts(g.Concepts, g.Edges)
	}
	return nil
}

func unmarshalConcepts(raw json.RawMessage) (map[ConceptID]Concept, error) {
	if len(raw) == 0 || string(raw) == "null" {
		return map[ConceptID]Concept{}, nil
	}

	var asMap map[ConceptID]Concept
	if err := json.Unmarshal(raw, &asMap); err == nil {
		if asMap == nil {
			asMap = map[ConceptID]Concept{}
		}
		return asMap, nil
	}

	var asList []Concept
	if err := json.Unmarshal(raw, &asList); err != nil {
		return nil, fmt.Errorf("concepts: se esperaba objeto o arreglo: %w", err)
	}
	out := make(map[ConceptID]Concept, len(asList))
	for _, concept := range asList {
		if concept.ID == "" {
			continue
		}
		out[concept.ID] = concept
	}
	return out, nil
}

// projectLessonsFromConcepts materializa LessonNodes 1:1 con concepts.
// Para kind=requires, la arista from→to significa "from requiere to" (to es prerrequisito).
func projectLessonsFromConcepts(concepts map[ConceptID]Concept, edges []CurriculumEdge) map[string]LessonNode {
	lessons := make(map[string]LessonNode, len(concepts))
	prereqs := make(map[string]PrerequisiteList, len(concepts))

	for _, edge := range edges {
		if edge.Kind != EdgeKindRequires || edge.From == "" || edge.To == "" {
			continue
		}
		prereqs[edge.From] = append(prereqs[edge.From], LessonPrerequisite(edge.To))
	}

	for id, concept := range concepts {
		lessonID := string(id)
		lessons[lessonID] = LessonNode{
			ID:            lessonID,
			Title:         concept.Title,
			TrackType:     trackTypeForConcept(concept.Track),
			Description:   concept.Summary,
			Prerequisites: prereqs[lessonID],
			Concepts:      []ConceptID{id},
			SkillTarget:   lessonID,
		}
	}
	return lessons
}

func trackTypeForConcept(track string) TrackType {
	switch track {
	case "plataforma":
		return TrackRetoIngenieril
	default:
		return TrackMicroPaso
	}
}

// ResolveConcepts proyecta IDs a definiciones del catálogo compartido.
func (g *CurriculumGraph) ResolveConcepts(ids []ConceptID) []Concept {
	if g == nil || len(ids) == 0 {
		return nil
	}

	out := make([]Concept, 0, len(ids))
	for _, id := range ids {
		if concept, ok := g.Concepts[id]; ok {
			out = append(out, concept)
			continue
		}
		out = append(out, Concept{ID: id})
	}
	return out
}

// ConceptsForLesson retorna los conceptos resueltos de un nodo.
func (g *CurriculumGraph) ConceptsForLesson(lessonID string) ([]Concept, error) {
	if g == nil {
		return nil, nil
	}
	lesson, ok := g.Lessons[lessonID]
	if !ok {
		return nil, nil
	}
	return g.ResolveConcepts(lesson.ConceptIDs()), nil
}

// Ensure LessonNode satisfies ConceptSet at compile time.
var _ ConceptSet = LessonNode{}
