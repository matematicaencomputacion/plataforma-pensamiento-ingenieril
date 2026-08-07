package repositories

import (
	"errors"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

var (
	// ErrUserNotFound indica que no existe un usuario con ese criterio.
	ErrUserNotFound = errors.New("usuario no encontrado")
	// ErrEmailTaken indica conflicto de unicidad de email.
	ErrEmailTaken = errors.New("email ya registrado")
)

// UserRepository puerto de persistencia de usuarios autenticables.
type UserRepository interface {
	Create(user domain.User) error
	GetByEmail(email string) (domain.User, error)
	GetByID(id string) (domain.User, error)
}
