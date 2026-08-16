package config

import (
	"errors"
	"strings"
	"testing"
)

func clearAuthEnv(t *testing.T) {
	t.Helper()
	t.Setenv("ENV", "")
	t.Setenv("APP_ENV", "")
	t.Setenv("GO_ENV", "")
	t.Setenv("JWT_SECRET", "")
	t.Setenv("DATABASE_URL", "")
	t.Setenv("PPI_ALLOW_EPHEMERAL_SQLITE", "")
}

func TestLoadAuthConfig_DevDefaultWhenNotProduction(t *testing.T) {
	clearAuthEnv(t)

	cfg, err := LoadAuthConfig()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if cfg.JWTSecret != defaultDevJWTSecret {
		t.Fatalf("jwt secret: got %q want default dev secret", cfg.JWTSecret)
	}
	if cfg.DatabaseURL != "sqlite://./data/ppi.db" {
		t.Fatalf("database url: %q", cfg.DatabaseURL)
	}
	if cfg.Driver() != DriverSQLite {
		t.Fatalf("driver: %s", cfg.Driver())
	}
}

func TestLoadAuthConfig_DevKeepsKnownCISecret(t *testing.T) {
	clearAuthEnv(t)
	t.Setenv("ENV", "development")
	t.Setenv("JWT_SECRET", "ci-e2e-only-ppi-jwt-secret")

	cfg, err := LoadAuthConfig()
	if err != nil {
		t.Fatalf("local/CI must still accept distro secrets: %v", err)
	}
	if cfg.JWTSecret != "ci-e2e-only-ppi-jwt-secret" {
		t.Fatalf("jwt secret: %q", cfg.JWTSecret)
	}
}

func TestLoadAuthConfig_ProductionRefusesInsecureSecrets(t *testing.T) {
	cases := []struct {
		name   string
		envKey string
		envVal string
		secret string
	}{
		{name: "ENV empty secret", envKey: "ENV", envVal: "production", secret: ""},
		{name: "ENV whitespace secret", envKey: "ENV", envVal: "production", secret: "  "},
		{name: "ENV default distro", envKey: "ENV", envVal: "production", secret: defaultDevJWTSecret},
		{name: "APP_ENV harness", envKey: "APP_ENV", envVal: "production", secret: "harness-jwt-secret"},
		{name: "GO_ENV test-secret", envKey: "GO_ENV", envVal: "production", secret: "test-secret"},
		{name: "ENV e2e secret", envKey: "ENV", envVal: "production", secret: "ci-e2e-only-ppi-jwt-secret"},
		{name: "ENV docker smoke", envKey: "ENV", envVal: "production", secret: "ci-docker-smoke-only"},
		{name: "prod alias", envKey: "ENV", envVal: "prod", secret: ""},
		{name: "PRODUCTION case", envKey: "ENV", envVal: "PRODUCTION", secret: defaultDevJWTSecret},
	}

	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			clearAuthEnv(t)
			t.Setenv(tc.envKey, tc.envVal)
			t.Setenv("JWT_SECRET", tc.secret)
			t.Setenv("DATABASE_URL", "postgres://ppi:x@/ppi?host=/cloudsql/p:southamerica-east1:i")

			_, err := LoadAuthConfig()
			if !errors.Is(err, ErrInsecureJWTInProduction) {
				t.Fatalf("got err %v, want ErrInsecureJWTInProduction", err)
			}
		})
	}
}

func TestLoadAuthConfig_ProductionAcceptsPostgres(t *testing.T) {
	clearAuthEnv(t)
	t.Setenv("ENV", "production")
	t.Setenv("JWT_SECRET", "unique-prod-secret-not-in-distro")
	t.Setenv("DATABASE_URL", "postgres://ppi:secret@/ppi?host=/cloudsql/project-2dc3a0ed-9735-4291-b0b:southamerica-east1:ppi")

	cfg, err := LoadAuthConfig()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if cfg.JWTSecret != "unique-prod-secret-not-in-distro" {
		t.Fatalf("jwt secret: %q", cfg.JWTSecret)
	}
	if cfg.Driver() != DriverPostgres {
		t.Fatalf("driver: %s", cfg.Driver())
	}
	if cfg.EphemeralSQLiteAllowed {
		t.Fatal("postgres must not set ephemeral override")
	}
}

func TestLoadAuthConfig_ProductionRefusesSQLiteAndMissing(t *testing.T) {
	cases := []struct {
		name string
		url  string
	}{
		{name: "missing", url: ""},
		{name: "tmp sqlite", url: "sqlite:///tmp/ppi.db"},
		{name: "relative sqlite", url: "sqlite://./data/ppi.db"},
		{name: "memory", url: ":memory:"},
	}
	for _, tc := range cases {
		t.Run(tc.name, func(t *testing.T) {
			clearAuthEnv(t)
			t.Setenv("ENV", "production")
			t.Setenv("JWT_SECRET", "unique-prod-secret-not-in-distro")
			t.Setenv("DATABASE_URL", tc.url)

			_, err := LoadAuthConfig()
			if !errors.Is(err, ErrEphemeralSQLiteInProduction) {
				t.Fatalf("got err %v, want ErrEphemeralSQLiteInProduction", err)
			}
		})
	}
}

func TestLoadAuthConfig_ProductionAllowsExplicitEphemeralSQLite(t *testing.T) {
	clearAuthEnv(t)
	t.Setenv("ENV", "production")
	t.Setenv("JWT_SECRET", "unique-prod-secret-not-in-distro")
	t.Setenv("DATABASE_URL", "sqlite:///tmp/ppi.db")
	t.Setenv("PPI_ALLOW_EPHEMERAL_SQLITE", "1")

	cfg, err := LoadAuthConfig()
	if err != nil {
		t.Fatalf("explicit demo override must boot: %v", err)
	}
	if !cfg.EphemeralSQLiteAllowed {
		t.Fatal("want EphemeralSQLiteAllowed")
	}
	if cfg.Driver() != DriverSQLite {
		t.Fatalf("driver: %s", cfg.Driver())
	}
}

func TestCloudSQLInstance(t *testing.T) {
	want := "project-2dc3a0ed-9735-4291-b0b:southamerica-east1:ppi"
	got := CloudSQLInstance("postgres://ppi:s@/ppi?host=/cloudsql/" + want + "&sslmode=disable")
	if got != want {
		t.Fatalf("got %q want %q", got, want)
	}
	if CloudSQLInstance("postgres://ppi:s@10.0.0.1:5432/ppi") != "" {
		t.Fatal("tcp DSN must not invent an instance")
	}
	if CloudSQLInstance("sqlite:///tmp/ppi.db") != "" {
		t.Fatal("sqlite must not invent an instance")
	}
}

func TestRedactedDatabaseURL(t *testing.T) {
	got := RedactedDatabaseURL("postgres://ppi:hunter2@/ppi?host=/cloudsql/p:r:i")
	if got == "" || strings.Contains(got, "hunter2") {
		t.Fatalf("password leaked: %q", got)
	}
	if !strings.Contains(got, "***") && !strings.Contains(got, "%2A%2A%2A") {
		t.Fatalf("want masked password: %q", got)
	}
}
