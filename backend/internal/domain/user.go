package domain

import "strings"

// User representa a un estudiante en la plataforma.
// ID se modela como string para admitir UUIDs sin acoplar el dominio a un proveedor concreto.
type User struct {
	ID           string
	Email        string
	PasswordHash string
	CurrentLevel int
	Profile      LearnerProfile
}

// LearnerProfile almacena el coaching de onboarding (propósito, urgencia, visión, stack).
type LearnerProfile struct {
	LifePurpose  string `json:"lifePurpose"`
	Urgency      string `json:"urgency"`
	Vision5Years string `json:"vision5Years"`
	TechStack    string `json:"techStack"`
}

// IsEmpty indica que no hay ningún campo persistido de coaching.
func (p LearnerProfile) IsEmpty() bool {
	return strings.TrimSpace(p.LifePurpose) == "" &&
		strings.TrimSpace(p.Urgency) == "" &&
		strings.TrimSpace(p.Vision5Years) == "" &&
		strings.TrimSpace(p.TechStack) == ""
}
