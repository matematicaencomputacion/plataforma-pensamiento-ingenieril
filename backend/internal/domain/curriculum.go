package domain

// LessonNode representa un nodo del grafo curricular (DAG).
// Implementa ConceptSet para el contrato de conceptos compartidos (PPI 1.2).
type LessonNode struct {
	ID            string          `json:"id"`
	Title         string          `json:"title"`
	TrackType     TrackType       `json:"track_type"`
	Description   string          `json:"description"`
	Prerequisites PrerequisiteList `json:"prerequisites"`
	Concepts      []ConceptID     `json:"concepts"`
	Competencies  []Competency    `json:"competencies,omitempty"`
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

// CurriculumGraph modela la malla de lecciones y el catálogo de conceptos compartidos.
type CurriculumGraph struct {
	Lessons  map[string]LessonNode `json:"lessons"`
	Concepts map[ConceptID]Concept `json:"concepts,omitempty"`
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
