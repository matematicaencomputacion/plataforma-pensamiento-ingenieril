package repositories

import "github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"

// CurriculumRepository define el puerto de acceso al grafo curricular.
type CurriculumRepository interface {
	// RawJSON retorna el payload JSON crudo del curriculum.
	RawJSON() []byte
	// GetGraph retorna el grafo curricular deserializado.
	GetGraph() (domain.CurriculumGraph, error)
	// GetLesson retorna una lección por ID.
	GetLesson(id string) (domain.LessonNode, error)
}
