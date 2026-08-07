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
)

// PublicUser es la proyección segura del usuario (sin hash).
type PublicUser struct {
	ID    string `json:"id"`
	Email string `json:"email"`
}

// ToPublic convierte User a PublicUser.
func (u User) ToPublic() PublicUser {
	return PublicUser{ID: u.ID, Email: u.Email}
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
