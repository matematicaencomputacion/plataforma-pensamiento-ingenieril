package embedded

import (
	"encoding/json"
	"fmt"
	"sync"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/assets"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

// CurriculumRepository expone el curriculum.json embebido en memoria.
type CurriculumRepository struct {
	mu    sync.RWMutex
	raw   []byte
	once  sync.Once
	err   error
	graph domain.CurriculumGraph
}

// NewCurriculumRepository crea un repositorio respaldado por go:embed.
func NewCurriculumRepository() *CurriculumRepository {
	return &CurriculumRepository{
		raw: append([]byte(nil), assets.CurriculumJSON...),
	}
}

// RawJSON retorna una copia del JSON embebido.
func (r *CurriculumRepository) RawJSON() []byte {
	r.mu.RLock()
	defer r.mu.RUnlock()

	out := make([]byte, len(r.raw))
	copy(out, r.raw)
	return out
}

// GetGraph deserializa y cachea el grafo curricular desde memoria.
func (r *CurriculumRepository) GetGraph() (domain.CurriculumGraph, error) {
	r.once.Do(func() {
		var graph domain.CurriculumGraph
		if err := json.Unmarshal(r.raw, &graph); err != nil {
			r.err = fmt.Errorf("error al unmarshal del curriculum embebido: %w", err)
			return
		}
		if graph.Lessons == nil {
			graph.Lessons = map[string]domain.LessonNode{}
		}
		if graph.Concepts == nil {
			graph.Concepts = map[domain.ConceptID]domain.Concept{}
		}
		r.graph = graph
	})

	r.mu.RLock()
	defer r.mu.RUnlock()

	if r.err != nil {
		return domain.CurriculumGraph{}, r.err
	}

	return cloneGraph(r.graph), nil
}

// GetLesson retorna una lección del curriculum embebido.
func (r *CurriculumRepository) GetLesson(id string) (domain.LessonNode, error) {
	graph, err := r.GetGraph()
	if err != nil {
		return domain.LessonNode{}, err
	}

	lesson, ok := graph.Lessons[id]
	if !ok {
		return domain.LessonNode{}, fmt.Errorf("lección %q no encontrada", id)
	}

	return lesson, nil
}

func cloneGraph(graph domain.CurriculumGraph) domain.CurriculumGraph {
	cloned := domain.CurriculumGraph{
		Lessons:  make(map[string]domain.LessonNode, len(graph.Lessons)),
		Concepts: make(map[domain.ConceptID]domain.Concept, len(graph.Concepts)),
	}
	for id, concept := range graph.Concepts {
		cloned.Concepts[id] = concept
	}
	for id, lesson := range graph.Lessons {
		prereqs := make(domain.PrerequisiteList, len(lesson.Prerequisites))
		copy(prereqs, lesson.Prerequisites)
		lesson.Prerequisites = prereqs

		concepts := make([]domain.ConceptID, len(lesson.Concepts))
		copy(concepts, lesson.Concepts)
		lesson.Concepts = concepts

		competencies := make([]domain.Competency, len(lesson.Competencies))
		copy(competencies, lesson.Competencies)
		lesson.Competencies = competencies

		cloned.Lessons[id] = lesson
	}
	return cloned
}
