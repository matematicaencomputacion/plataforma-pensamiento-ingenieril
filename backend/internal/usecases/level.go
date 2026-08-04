package usecases

import (
	"fmt"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories"
)

// LevelService expone consultas de niveles/retos.
type LevelService struct {
	levels repositories.LevelRepository
}

// NewLevelService crea el servicio de niveles.
func NewLevelService(levels repositories.LevelRepository) *LevelService {
	return &LevelService{levels: levels}
}

// GetByID retorna un nivel por identificador.
func (s *LevelService) GetByID(id int) (domain.Level, error) {
	if id <= 0 {
		return domain.Level{}, fmt.Errorf("level_id inválido: %d", id)
	}

	level, err := s.levels.GetByID(id)
	if err != nil {
		return domain.Level{}, err
	}

	return level, nil
}

// GetCurrent retorna el nivel actual del recorrido.
func (s *LevelService) GetCurrent() (domain.Level, error) {
	level, err := s.levels.GetCurrent()
	if err != nil {
		return domain.Level{}, err
	}

	return level, nil
}
