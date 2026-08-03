package repositories

import "github.com/tu-usuario/plataforma-edu-backend/internal/domain"

// CognitiveProfileRepository define el puerto de acceso al perfil cognitivo.
type CognitiveProfileRepository interface {
	GetByUserID(userID string) (domain.CognitiveProfile, error)
	Save(profile domain.CognitiveProfile) error
}
