package persistence

import (
	"testing"

	"github.com/google/uuid"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/config"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

func TestOpen_SQLiteMemoryRoundTrip(t *testing.T) {
	store, err := Open(config.AuthConfig{DatabaseURL: ":memory:"}, "")
	if err != nil {
		t.Fatalf("open: %v", err)
	}
	t.Cleanup(func() { _ = store.Close() })
	if store.Driver != config.DriverSQLite {
		t.Fatalf("driver: %s", store.Driver)
	}

	user := domain.User{
		ID:           uuid.NewString(),
		Email:        "mem@example.com",
		PasswordHash: "hash",
		CurrentLevel: 1,
	}
	if err := store.Users.Create(user); err != nil {
		t.Fatalf("create: %v", err)
	}
	got, err := store.Users.GetByEmail(user.Email)
	if err != nil {
		t.Fatalf("get: %v", err)
	}
	if got.ID != user.ID {
		t.Fatalf("id: %q", got.ID)
	}
}

func TestOpen_PostgresURLSelectsDriverWithoutDialing(t *testing.T) {
	cfg := config.AuthConfig{DatabaseURL: "postgres://ppi:x@127.0.0.1:1/ppi"}
	if cfg.Driver() != config.DriverPostgres {
		t.Fatalf("driver: %s", cfg.Driver())
	}
}
