package jsonstore

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"sync"

	"github.com/tu-usuario/plataforma-edu-backend/internal/domain"
)

// LevelRepository persiste niveles en un archivo JSON local.
type LevelRepository struct {
	mu       sync.RWMutex
	filePath string
}

// NewLevelRepository crea un repositorio JSON de niveles.
func NewLevelRepository(filePath string) *LevelRepository {
	return &LevelRepository{filePath: filePath}
}

// GetByID retorna un nivel por su identificador.
func (r *LevelRepository) GetByID(id int) (domain.Level, error) {
	levels, err := r.load()
	if err != nil {
		return domain.Level{}, err
	}

	for _, level := range levels {
		if level.ID == id {
			return level, nil
		}
	}

	return domain.Level{}, fmt.Errorf("nivel %d no encontrado", id)
}

// GetCurrent retorna el nivel actual (el de menor ID en el seed).
func (r *LevelRepository) GetCurrent() (domain.Level, error) {
	levels, err := r.load()
	if err != nil {
		return domain.Level{}, err
	}
	if len(levels) == 0 {
		return domain.Level{}, fmt.Errorf("no hay niveles disponibles")
	}

	current := levels[0]
	for _, level := range levels[1:] {
		if level.ID < current.ID {
			current = level
		}
	}

	return current, nil
}

// List retorna todos los niveles.
func (r *LevelRepository) List() ([]domain.Level, error) {
	return r.load()
}

func (r *LevelRepository) load() ([]domain.Level, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	data, err := os.ReadFile(r.filePath)
	if err != nil {
		return nil, fmt.Errorf("error al leer niveles desde %s: %w", r.filePath, err)
	}

	var levels []domain.Level
	if err := json.Unmarshal(data, &levels); err != nil {
		return nil, fmt.Errorf("error al unmarshal de niveles: %w", err)
	}

	return levels, nil
}

// DefaultLevelsPath resuelve la ruta por defecto del seed de niveles.
func DefaultLevelsPath(dataDir string) string {
	return filepath.Join(dataDir, "levels.json")
}
