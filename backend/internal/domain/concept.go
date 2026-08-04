package domain

import (
	"encoding/json"
	"fmt"
)

// ConceptID es la clave estable de un concepto de aprendizaje compartido.
type ConceptID string

// Concept describe una unidad cognitiva reutilizable en el curriculum.
type Concept struct {
	ID      ConceptID `json:"id"`
	Title   string    `json:"title"`
	Summary string    `json:"summary"`
	Track   string    `json:"track,omitempty"`
	Tags    []string  `json:"tags,omitempty"`
	Source  string    `json:"source,omitempty"`
}

// CurriculumEdge es una arista del grafo unificado de conceptos (con rationale curable).
type CurriculumEdge struct {
	From        string  `json:"from"`
	To          string  `json:"to"`
	Kind        string  `json:"kind"`
	Strength    float64 `json:"strength"`
	RationaleES string  `json:"rationale_es"`
	Source      string  `json:"source"`
}

// Edge kinds del esquema unificado vectorial / PPI.
const (
	EdgeKindRequires    = "requires"
	EdgeKindDeepens     = "deepens"
	EdgeKindContinues   = "continues"
	EdgeKindAlternative = "alternative"
)

// PrerequisiteKind distingue el destino de un prerrequisito.
type PrerequisiteKind string

const (
	PrerequisiteKindLesson  PrerequisiteKind = "lesson"
	PrerequisiteKindConcept PrerequisiteKind = "concept"
)

// Prerequisite expresa una dependencia hacia una lección o un concepto.
type Prerequisite struct {
	Kind  PrerequisiteKind `json:"kind"`
	RefID string           `json:"ref_id"`
}

// LessonPrerequisite construye un prerrequisito de lección.
func LessonPrerequisite(lessonID string) Prerequisite {
	return Prerequisite{Kind: PrerequisiteKindLesson, RefID: lessonID}
}

// ConceptPrerequisite construye un prerrequisito de concepto.
func ConceptPrerequisite(conceptID ConceptID) Prerequisite {
	return Prerequisite{Kind: PrerequisiteKindConcept, RefID: string(conceptID)}
}

// CompetencyLevel indica el grado de dominio esperado sobre un concepto.
type CompetencyLevel string

const (
	CompetencyIntroductory CompetencyLevel = "introductory"
	CompetencyDeveloping   CompetencyLevel = "developing"
	CompetencyProficient   CompetencyLevel = "proficient"
)

// Competency vincula un ConceptID con un nivel de competencia esperado.
type Competency struct {
	ConceptID ConceptID       `json:"concept_id"`
	Level     CompetencyLevel `json:"level"`
}

// ConceptSet es el contrato compartido para entidades que portan conceptos.
type ConceptSet interface {
	ConceptIDs() []ConceptID
}

// PrerequisiteList soporta JSON estructurado y el formato legado []string de lecciones.
type PrerequisiteList []Prerequisite

// UnmarshalJSON acepta [{"kind":"lesson","ref_id":"..."}] o ["lesson-id"].
func (p *PrerequisiteList) UnmarshalJSON(data []byte) error {
	var structured []Prerequisite
	if err := json.Unmarshal(data, &structured); err == nil {
		*p = structured
		return nil
	}

	var legacy []string
	if err := json.Unmarshal(data, &legacy); err != nil {
		return fmt.Errorf("prerequisites: formato inválido: %w", err)
	}

	out := make(PrerequisiteList, len(legacy))
	for i, id := range legacy {
		out[i] = LessonPrerequisite(id)
	}
	*p = out
	return nil
}

// LessonIDs retorna solo los prerrequisitos de tipo lección.
func (p PrerequisiteList) LessonIDs() []string {
	ids := make([]string, 0, len(p))
	for _, item := range p {
		kind := item.Kind
		if kind == "" {
			kind = PrerequisiteKindLesson
		}
		if kind == PrerequisiteKindLesson && item.RefID != "" {
			ids = append(ids, item.RefID)
		}
	}
	return ids
}

// ConceptIDs retorna solo los prerrequisitos de tipo concepto.
func (p PrerequisiteList) ConceptIDs() []ConceptID {
	ids := make([]ConceptID, 0, len(p))
	for _, item := range p {
		if item.Kind == PrerequisiteKindConcept && item.RefID != "" {
			ids = append(ids, ConceptID(item.RefID))
		}
	}
	return ids
}
