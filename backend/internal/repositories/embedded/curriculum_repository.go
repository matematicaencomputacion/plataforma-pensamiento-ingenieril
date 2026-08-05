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
		Version:  graph.Version,
		Lessons:  make(map[string]domain.LessonNode, len(graph.Lessons)),
		Concepts: make(map[domain.ConceptID]domain.Concept, len(graph.Concepts)),
		Edges:    make([]domain.CurriculumEdge, len(graph.Edges)),
	}
	copy(cloned.Edges, graph.Edges)
	for id, concept := range graph.Concepts {
		if len(concept.Tags) > 0 {
			tags := make([]string, len(concept.Tags))
			copy(tags, concept.Tags)
			concept.Tags = tags
		}
		if len(concept.Transcript) > 0 {
			transcript := make([]domain.TranscriptSegment, len(concept.Transcript))
			copy(transcript, concept.Transcript)
			concept.Transcript = transcript
		}
		if len(concept.Resources) > 0 {
			resources := make(map[string]domain.MediaResource, len(concept.Resources))
			for lang, media := range concept.Resources {
				if len(media.Transcript) > 0 {
					segs := make([]domain.TranscriptSegment, len(media.Transcript))
					copy(segs, media.Transcript)
					media.Transcript = segs
				}
				if len(media.Chapters) > 0 {
					chapters := make([]domain.MediaChapter, len(media.Chapters))
					for i, ch := range media.Chapters {
						if len(ch.Transcript) > 0 {
							segs := make([]domain.TranscriptSegment, len(ch.Transcript))
							copy(segs, ch.Transcript)
							ch.Transcript = segs
						}
						chapters[i] = ch
					}
					media.Chapters = chapters
				}
				resources[lang] = media
			}
			concept.Resources = resources
		}
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
