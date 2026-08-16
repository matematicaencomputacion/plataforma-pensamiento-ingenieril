package config

import (
	"errors"
	"net/url"
	"os"
	"strings"
)

const defaultDevJWTSecret = "dev-only-change-me-ppi-jwt-secret"
const defaultDevDatabaseURL = "sqlite://./data/ppi.db"

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

// ErrEphemeralSQLiteInProduction is returned when production would otherwise
// persist users on SQLite (missing DATABASE_URL or sqlite://, including /tmp).
var ErrEphemeralSQLiteInProduction = errors.New("DATABASE_URL must be postgres:// or postgresql:// in production; missing and sqlite values (including /tmp) are refused unless PPI_ALLOW_EPHEMERAL_SQLITE=1")

// DriverKind is the persistence engine selected from DATABASE_URL.
type DriverKind string

const (
	DriverSQLite   DriverKind = "sqlite"
	DriverPostgres DriverKind = "postgres"
)

// AuthConfig parámetros de autenticación y persistencia de usuarios.
type AuthConfig struct {
	JWTSecret              string
	DatabaseURL            string
	EphemeralSQLiteAllowed bool // production sqlite only when PPI_ALLOW_EPHEMERAL_SQLITE=1
}

// LoadAuthConfig lee JWT_SECRET y DATABASE_URL con defaults seguros solo para dev.
// En production el proceso no arranca con secreto vacío o de distro/CI, ni con
// SQLite / DATABASE_URL ausente, salvo PPI_ALLOW_EPHEMERAL_SQLITE=1.
func LoadAuthConfig() (AuthConfig, error) {
	secret := strings.TrimSpace(os.Getenv("JWT_SECRET"))
	if secret == "" {
		secret = defaultDevJWTSecret
	}
	if isProductionEnv() && isInsecureJWTSecret(secret) {
		return AuthConfig{}, ErrInsecureJWTInProduction
	}

	allowEphemeral := strings.TrimSpace(os.Getenv("PPI_ALLOW_EPHEMERAL_SQLITE")) == "1"
	dbURL := strings.TrimSpace(os.Getenv("DATABASE_URL"))
	if dbURL == "" {
		if isProductionEnv() && !allowEphemeral {
			return AuthConfig{}, ErrEphemeralSQLiteInProduction
		}
		dbURL = defaultDevDatabaseURL
	}

	cfg := AuthConfig{
		JWTSecret:   secret,
		DatabaseURL: dbURL,
	}

	if isProductionEnv() && !IsPostgresURL(dbURL) {
		if !allowEphemeral {
			return AuthConfig{}, ErrEphemeralSQLiteInProduction
		}
		cfg.EphemeralSQLiteAllowed = true
	}
	return cfg, nil
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

// IsPostgresURL reports postgres:// or postgresql:// DSNs.
func IsPostgresURL(raw string) bool {
	u := strings.ToLower(strings.TrimSpace(raw))
	return strings.HasPrefix(u, "postgres://") || strings.HasPrefix(u, "postgresql://")
}

// IsSQLiteURL reports sqlite://, :memory: and file: DSNs.
func IsSQLiteURL(raw string) bool {
	u := strings.TrimSpace(raw)
	switch {
	case strings.HasPrefix(strings.ToLower(u), "sqlite://"):
		return true
	case u == ":memory:" || strings.HasPrefix(u, "file:"):
		return true
	default:
		return false
	}
}

// Driver selects sqlite vs postgres from DATABASE_URL.
func (c AuthConfig) Driver() DriverKind {
	if IsPostgresURL(c.DatabaseURL) {
		return DriverPostgres
	}
	return DriverSQLite
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

// CloudSQLInstance extracts PROJECT:REGION:INSTANCE from host=/cloudsql/… if present.
// Empty when the DSN uses a TCP host. Never invents a name.
func CloudSQLInstance(raw string) string {
	u, err := url.Parse(raw)
	if err != nil {
		return cloudSQLInstanceFromQuery(raw)
	}
	host := u.Query().Get("host")
	if host == "" {
		host = u.Host
	}
	return trimCloudSQLHost(host)
}

func cloudSQLInstanceFromQuery(raw string) string {
	_, query, ok := strings.Cut(raw, "?")
	if !ok {
		return ""
	}
	values, err := url.ParseQuery(query)
	if err != nil {
		return ""
	}
	return trimCloudSQLHost(values.Get("host"))
}

func trimCloudSQLHost(host string) string {
	const prefix = "/cloudsql/"
	if !strings.HasPrefix(host, prefix) {
		return ""
	}
	inst := strings.TrimPrefix(host, prefix)
	inst = strings.TrimSuffix(inst, "/")
	if strings.Count(inst, ":") != 2 {
		return ""
	}
	return inst
}

// RedactedDatabaseURL masks the password for logs.
func RedactedDatabaseURL(raw string) string {
	u, err := url.Parse(raw)
	if err != nil || u.Scheme == "" {
		if IsSQLiteURL(raw) {
			return raw
		}
		return "(unparseable DATABASE_URL)"
	}
	if u.User != nil {
		name := u.User.Username()
		if _, has := u.User.Password(); has {
			u.User = url.UserPassword(name, "***")
		}
	}
	return u.String()
}
