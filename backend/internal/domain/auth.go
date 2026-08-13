package domain

import "errors"

var (
	// ErrInvalidCredentials credenciales incorrectas (mensaje genérico hacia el cliente).
	ErrInvalidCredentials = errors.New("credenciales inválidas")
	// ErrInvalidPassword política de contraseña no cumplida.
	ErrInvalidPassword = errors.New("password inválida")
	// ErrInvalidEmail formato de email inválido.
	ErrInvalidEmail = errors.New("email inválido")
	// ErrUnauthorized token ausente o inválido.
	ErrUnauthorized = errors.New("no autorizado")
	// ErrEmptyProfile indica un body de perfil sin campos útiles.
	ErrEmptyProfile = errors.New("el perfil no tiene campos para guardar")
	// ErrInvalidResetToken token ausente, usado o expirado.
	ErrInvalidResetToken = errors.New("token de recuperación inválido o expirado")
	// ErrInvalidLevelID level_id ausente o no positivo.
	ErrInvalidLevelID = errors.New("level_id inválido")
	// ErrInvalidStepID step_id vacío.
	ErrInvalidStepID = errors.New("step_id inválido")
)

// PublicUser es la proyección segura del usuario (sin hash).
type PublicUser struct {
	ID              string `json:"id"`
	Email           string `json:"email"`
	CurrentLevel    int    `json:"current_level"`
	CompletedLevels []int  `json:"completed_levels"`
}

// ToPublic convierte User a PublicUser.
func (u User) ToPublic() PublicUser {
	level := u.CurrentLevel
	if level <= 0 {
		level = 1
	}
	completed := u.CompletedLevels
	if completed == nil {
		completed = []int{}
	}
	return PublicUser{
		ID:              u.ID,
		Email:           u.Email,
		CurrentLevel:    level,
		CompletedLevels: completed,
	}
}

// HasCompletedLevel reports whether levelID is in the earned set.
func HasCompletedLevel(completed []int, levelID int) bool {
	for _, id := range completed {
		if id == levelID {
			return true
		}
	}
	return false
}

// WithCompletedLevel returns a copy of completed that includes levelID (idempotent).
func WithCompletedLevel(completed []int, levelID int) []int {
	if HasCompletedLevel(completed, levelID) {
		out := make([]int, len(completed))
		copy(out, completed)
		return out
	}
	out := make([]int, len(completed)+1)
	copy(out, completed)
	out[len(completed)] = levelID
	return out
}

// AdvanceCursorThroughCompleted walks the progress cursor past contiguous earned levels.
// Starting at current, while current is completed, current++. Returns the new cursor
// and whether it moved.
func AdvanceCursorThroughCompleted(current int, completed []int) (int, bool) {
	if current <= 0 {
		current = 1
	}
	next := current
	for HasCompletedLevel(completed, next) {
		next++
	}
	return next, next != current
}

// PasswordHasher abstrae el hashing de contraseñas (bcrypt en infraestructura).
type PasswordHasher interface {
	Hash(plain string) (string, error)
	Compare(hash, plain string) error
}

// TokenIssuer abstrae emisión/validación de JWT.
type TokenIssuer interface {
	Issue(userID, email string) (string, error)
	Parse(token string) (userID string, email string, err error)
}
