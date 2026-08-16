package postgres

import (
	"os"
	"testing"

	"github.com/google/uuid"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories"
)

func TestUserRepository_CreateGet_SkipWithoutDSN(t *testing.T) {
	dsn := os.Getenv("PPI_POSTGRES_TEST_URL")
	if dsn == "" {
		t.Skip("PPI_POSTGRES_TEST_URL unset; SQLite remains the default test driver")
	}

	db, err := OpenDB(dsn)
	if err != nil {
		t.Fatalf("open: %v", err)
	}
	t.Cleanup(func() { _ = db.Close() })

	repo, err := NewUserRepository(db)
	if err != nil {
		t.Fatalf("migrate: %v", err)
	}

	user := domain.User{
		ID:           uuid.NewString(),
		Email:        "pg-" + uuid.NewString() + "@example.com",
		PasswordHash: "hash",
		CurrentLevel: 1,
	}
	if err := repo.Create(user); err != nil {
		t.Fatalf("create: %v", err)
	}
	got, err := repo.GetByEmail(user.Email)
	if err != nil {
		t.Fatalf("get: %v", err)
	}
	if got.ID != user.ID {
		t.Fatalf("id: %q want %q", got.ID, user.ID)
	}
	if err := repo.Create(user); err != repositories.ErrEmailTaken {
		t.Fatalf("duplicate: %v", err)
	}
}
