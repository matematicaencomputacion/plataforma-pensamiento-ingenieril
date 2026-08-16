package config

import (
	"errors"
	"os"
	"strings"
)

const defaultDevJWTSecret = "dev-only-change-me-ppi-jwt-secret"

// Known distro/CI secrets that must never sign production tokens.
var insecureJWTSecrets = map[string]struct{}{
	defaultDevJWTSecret:          {},
	"harness-jwt-secret":         {},
	"test-secret":                {},
	"ci-e2e-only-ppi-jwt-secret": {},
	"ci-docker-smoke-only":       {},
}

// ErrInsecureJWTInProduction is returned when ENV/APP_ENV/GO_ENV is production
// and JWT_SECRET is empty or a known distro/CI placeholder.
var ErrInsecureJWTInProduction = errors.New("JWT_SECRET must be a unique production secret; empty and known distro/CI values are refused when ENV/APP_ENV/GO_ENV is production")

// AuthConfig parámetros de autenticación y persistencia de usuarios.
type AuthConfig struct {
	JWTSecret   string
	DatabaseURL string
}

// LoadAuthConfig lee JWT_SECRET y DATABASE_URL con defaults seguros solo para dev.
// En production el proceso no arranca con secreto vacío o de distro/CI.
func LoadAuthConfig() (AuthConfig, error) {
	secret := strings.TrimSpace(os.Getenv("JWT_SECRET"))
	if secret == "" {
		secret = defaultDevJWTSecret
	}
	if isProductionEnv() && isInsecureJWTSecret(secret) {
		return AuthConfig{}, ErrInsecureJWTInProduction
	}
	dbURL := strings.TrimSpace(os.Getenv("DATABASE_URL"))
	if dbURL == "" {
		dbURL = "sqlite://./data/ppi.db"
	}
	return AuthConfig{
		JWTSecret:   secret,
		DatabaseURL: dbURL,
	}, nil
}

func isProductionEnv() bool {
	for _, key := range []string{"ENV", "APP_ENV", "GO_ENV"} {
		v := strings.ToLower(strings.TrimSpace(os.Getenv(key)))
		switch v {
		case "production", "prod":
			return true
		}
	}
	return false
}

func isInsecureJWTSecret(secret string) bool {
	_, ok := insecureJWTSecrets[strings.TrimSpace(secret)]
	return ok
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
