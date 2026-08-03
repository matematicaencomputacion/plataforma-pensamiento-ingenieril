package repositories

import "github.com/tu-usuario/plataforma-edu-backend/internal/domain"

// CurriculumRepository define el puerto de acceso al grafo curricular.
type CurriculumRepository interface {
	GetGraph() (domain.CurriculumGraph, error)
	GetLesson(id string) (domain.LessonNode, error)
}
