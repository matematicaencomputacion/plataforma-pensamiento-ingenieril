package config

import (
	"errors"
	"testing"
)

func clearAuthEnv(t *testing.T) {
	t.Helper()
	t.Setenv("ENV", "")
	t.Setenv("APP_ENV", "")
	t.Setenv("GO_ENV", "")
	t.Setenv("JWT_SECRET", "")
	t.Setenv("DATABASE_URL", "")
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

			_, err := LoadAuthConfig()
			if !errors.Is(err, ErrInsecureJWTInProduction) {
				t.Fatalf("got err %v, want ErrInsecureJWTInProduction", err)
			}
		})
	}
}

func TestLoadAuthConfig_ProductionAcceptsRealSecret(t *testing.T) {
	clearAuthEnv(t)
	t.Setenv("ENV", "production")
	t.Setenv("JWT_SECRET", "unique-prod-secret-not-in-distro")
	t.Setenv("DATABASE_URL", "sqlite:///tmp/ppi.db")

	cfg, err := LoadAuthConfig()
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if cfg.JWTSecret != "unique-prod-secret-not-in-distro" {
		t.Fatalf("jwt secret: %q", cfg.JWTSecret)
	}
	if cfg.DatabaseURL != "sqlite:///tmp/ppi.db" {
		t.Fatalf("database url: %q", cfg.DatabaseURL)
	}
}
