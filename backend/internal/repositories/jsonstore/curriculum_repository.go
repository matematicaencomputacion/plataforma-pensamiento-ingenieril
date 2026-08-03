package jsonstore

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"sync"

	"github.com/tu-usuario/plataforma-edu-backend/internal/domain"
)

// CurriculumRepository carga el grafo curricular desde JSON con acceso concurrente seguro.
type CurriculumRepository struct {
	mu       sync.RWMutex
	filePath string
}

// NewCurriculumRepository crea un repositorio JSON del curriculum.
func NewCurriculumRepository(filePath string) *CurriculumRepository {
	return &CurriculumRepository{filePath: filePath}
}

// GetGraph retorna una copia del grafo curricular.
func (r *CurriculumRepository) GetGraph() (domain.CurriculumGraph, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	graph, err := r.readUnlocked()
	if err != nil {
		return domain.CurriculumGraph{}, err
	}

	return cloneGraph(graph), nil
}

// GetLesson retorna una lección por ID.
func (r *CurriculumRepository) GetLesson(id string) (domain.LessonNode, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	graph, err := r.readUnlocked()
	if err != nil {
		return domain.LessonNode{}, err
	}

	lesson, ok := graph.Lessons[id]
	if !ok {
		return domain.LessonNode{}, fmt.Errorf("lección %q no encontrada", id)
	}

	return lesson, nil
}

func (r *CurriculumRepository) readUnlocked() (domain.CurriculumGraph, error) {
	data, err := os.ReadFile(r.filePath)
	if err != nil {
		return domain.CurriculumGraph{}, fmt.Errorf("error al leer curriculum desde %s: %w", r.filePath, err)
	}

	var graph domain.CurriculumGraph
	if err := json.Unmarshal(data, &graph); err != nil {
		return domain.CurriculumGraph{}, fmt.Errorf("error al unmarshal del curriculum: %w", err)
	}
	if graph.Lessons == nil {
		graph.Lessons = map[string]domain.LessonNode{}
	}

	return graph, nil
}

func cloneGraph(graph domain.CurriculumGraph) domain.CurriculumGraph {
	cloned := domain.CurriculumGraph{
		Lessons: make(map[string]domain.LessonNode, len(graph.Lessons)),
	}
	for id, lesson := range graph.Lessons {
		prereqs := make([]string, len(lesson.Prerequisites))
		copy(prereqs, lesson.Prerequisites)
		lesson.Prerequisites = prereqs
		cloned.Lessons[id] = lesson
	}
	return cloned
}

// DefaultCurriculumPath resuelve la ruta por defecto del seed curricular.
func DefaultCurriculumPath(dataDir string) string {
	return filepath.Join(dataDir, "curriculum.json")
}
