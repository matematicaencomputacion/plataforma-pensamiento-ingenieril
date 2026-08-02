package domain

// User representa a un estudiante en la plataforma.
// ID se modela como string para admitir UUIDs sin acoplar el dominio a un proveedor concreto.
type User struct {
	ID           string
	Email        string
	PasswordHash string
	CurrentLevel int
}
