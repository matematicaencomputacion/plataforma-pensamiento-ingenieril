package usecases

import (
	"fmt"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories"
)

// EmbeddedCurriculumService expone el curriculum embebido en el binario.
type EmbeddedCurriculumService struct {
	repo repositories.CurriculumRepository
}

// NewEmbeddedCurriculumService crea el servicio sobre un CurriculumRepository.
func NewEmbeddedCurriculumService(repo repositories.CurriculumRepository) *EmbeddedCurriculumService {
	return &EmbeddedCurriculumService{repo: repo}
}

// RawJSON retorna el JSON embebido tal cual está en memoria.
func (s *EmbeddedCurriculumService) RawJSON() ([]byte, error) {
	if s.repo == nil {
		return nil, fmt.Errorf("repositorio de curriculum no configurado")
	}
	raw := s.repo.RawJSON()
	if len(raw) == 0 {
		return nil, fmt.Errorf("curriculum embebido vacío")
	}
	return raw, nil
}

// GetGraph retorna el grafo curricular deserializado desde el embed.
func (s *EmbeddedCurriculumService) GetGraph() (domain.CurriculumGraph, error) {
	if s.repo == nil {
		return domain.CurriculumGraph{}, fmt.Errorf("repositorio de curriculum no configurado")
	}
	return s.repo.GetGraph()
}

// GetLesson retorna una lección por ID desde el curriculum embebido.
func (s *EmbeddedCurriculumService) GetLesson(id string) (domain.LessonNode, error) {
	if s.repo == nil {
		return domain.LessonNode{}, fmt.Errorf("repositorio de curriculum no configurado")
	}
	if id == "" {
		return domain.LessonNode{}, fmt.Errorf("lesson id es obligatorio")
	}
	return s.repo.GetLesson(id)
}
