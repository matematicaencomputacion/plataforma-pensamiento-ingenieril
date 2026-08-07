package config

import (
	"os"
	"strings"
)

// AuthConfig parámetros de autenticación y persistencia de usuarios.
type AuthConfig struct {
	JWTSecret   string
	DatabaseURL string
}

// LoadAuthConfig lee JWT_SECRET y DATABASE_URL con defaults seguros solo para dev.
func LoadAuthConfig() AuthConfig {
	secret := strings.TrimSpace(os.Getenv("JWT_SECRET"))
	if secret == "" {
		secret = "dev-only-change-me-ppi-jwt-secret"
	}
	dbURL := strings.TrimSpace(os.Getenv("DATABASE_URL"))
	if dbURL == "" {
		dbURL = "sqlite://./data/ppi.db"
	}
	return AuthConfig{
		JWTSecret:   secret,
		DatabaseURL: dbURL,
	}
}

// SQLitePath extrae la ruta de archivo desde DATABASE_URL tipo sqlite://path o :memory:.
func (c AuthConfig) SQLitePath() string {
	u := c.DatabaseURL
	switch {
	case strings.HasPrefix(u, "sqlite://"):
		return strings.TrimPrefix(u, "sqlite://")
	case u == ":memory:" || strings.HasPrefix(u, "file:"):
		return u
	default:
		return u
	}
}
