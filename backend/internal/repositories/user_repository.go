package repositories

import (
	"errors"
	"time"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

var (
	// ErrUserNotFound indica que no existe un usuario con ese criterio.
	ErrUserNotFound = errors.New("usuario no encontrado")
	// ErrEmailTaken indica conflicto de unicidad de email.
	ErrEmailTaken = errors.New("email ya registrado")
	// ErrResetTokenNotFound token de recuperación desconocido.
	ErrResetTokenNotFound = errors.New("token de recuperación no encontrado")
)

// PasswordResetToken fila de un challenge de recuperación (solo hash).
type PasswordResetToken struct {
	ID        string
	UserID    string
	TokenHash string
	ExpiresAt time.Time
	UsedAt    *time.Time
}

// UserRepository puerto de persistencia de usuarios autenticables.
type UserRepository interface {
	Create(user domain.User) error
	GetByEmail(email string) (domain.User, error)
	GetByID(id string) (domain.User, error)
	UpdateProfile(userID string, profile domain.LearnerProfile) error
	UpdateCurrentLevel(userID string, currentLevel int) error
	UpdatePasswordHash(userID, passwordHash string) error
	CreatePasswordResetToken(token PasswordResetToken) error
	GetPasswordResetTokenByHash(tokenHash string) (PasswordResetToken, error)
	MarkPasswordResetTokenUsed(id string, usedAt time.Time) error
}
