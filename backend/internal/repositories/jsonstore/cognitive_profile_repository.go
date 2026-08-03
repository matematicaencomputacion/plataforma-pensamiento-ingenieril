package jsonstore

import (
	"encoding/json"
	"fmt"
	"os"
	"path/filepath"
	"sync"

	"github.com/tu-usuario/plataforma-edu-backend/internal/domain"
)

// CognitiveProfileRepository persiste perfiles cognitivos en JSON con acceso concurrente seguro.
type CognitiveProfileRepository struct {
	mu       sync.RWMutex
	filePath string
}

// NewCognitiveProfileRepository crea un repositorio JSON de perfiles cognitivos.
func NewCognitiveProfileRepository(filePath string) *CognitiveProfileRepository {
	return &CognitiveProfileRepository{filePath: filePath}
}

// GetByUserID retorna el perfil cognitivo de un usuario.
func (r *CognitiveProfileRepository) GetByUserID(userID string) (domain.CognitiveProfile, error) {
	r.mu.RLock()
	defer r.mu.RUnlock()

	profiles, err := r.readUnlocked()
	if err != nil {
		return domain.CognitiveProfile{}, err
	}

	for _, profile := range profiles {
		if profile.UserID == userID {
			return profile, nil
		}
	}

	return domain.CognitiveProfile{}, fmt.Errorf("perfil cognitivo no encontrado para usuario %q", userID)
}

// Save crea o actualiza el perfil cognitivo de un usuario.
func (r *CognitiveProfileRepository) Save(profile domain.CognitiveProfile) error {
	if profile.UserID == "" {
		return fmt.Errorf("user_id es obligatorio para guardar el perfil cognitivo")
	}

	r.mu.Lock()
	defer r.mu.Unlock()

	profiles, err := r.readUnlocked()
	if err != nil {
		return err
	}

	updated := false
	for i := range profiles {
		if profiles[i].UserID == profile.UserID {
			profiles[i] = profile
			updated = true
			break
		}
	}
	if !updated {
		profiles = append(profiles, profile)
	}

	payload, err := json.MarshalIndent(profiles, "", "  ")
	if err != nil {
		return fmt.Errorf("error al marshal del perfil cognitivo: %w", err)
	}
	payload = append(payload, '\n')

	if err := os.WriteFile(r.filePath, payload, 0o644); err != nil {
		return fmt.Errorf("error al escribir perfiles en %s: %w", r.filePath, err)
	}

	return nil
}

func (r *CognitiveProfileRepository) readUnlocked() ([]domain.CognitiveProfile, error) {
	data, err := os.ReadFile(r.filePath)
	if err != nil {
		return nil, fmt.Errorf("error al leer perfiles desde %s: %w", r.filePath, err)
	}

	var profiles []domain.CognitiveProfile
	if err := json.Unmarshal(data, &profiles); err != nil {
		return nil, fmt.Errorf("error al unmarshal de perfiles cognitivos: %w", err)
	}

	return profiles, nil
}

// DefaultCognitiveProfilesPath resuelve la ruta por defecto del seed de perfiles.
func DefaultCognitiveProfilesPath(dataDir string) string {
	return filepath.Join(dataDir, "cognitive_profiles.json")
}
