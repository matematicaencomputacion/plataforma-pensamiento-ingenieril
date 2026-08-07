package usecases_test

import (
	"context"
	"errors"
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/crypto"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/jwtauth"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories/sqlite"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/usecases"
)

func newTestAuth(t *testing.T) *usecases.AuthService {
	t.Helper()
	db, err := sqlite.OpenDB(":memory:")
	if err != nil {
		t.Fatalf("open db: %v", err)
	}
	t.Cleanup(func() { _ = db.Close() })
	repo, err := sqlite.NewUserRepository(db)
	if err != nil {
		t.Fatalf("repo: %v", err)
	}
	return usecases.NewAuthService(repo, crypto.NewBcryptHasher(), jwtauth.NewHS256Issuer("test-secret"))
}

func TestRegisterAndLogin(t *testing.T) {
	svc := newTestAuth(t)
	ctx := context.Background()

	reg, err := svc.Register(ctx, "alum@example.com", "secreto12")
	if err != nil {
		t.Fatalf("register: %v", err)
	}
	if reg.Token == "" || reg.User.Email != "alum@example.com" {
		t.Fatalf("unexpected register result: %+v", reg)
	}

	_, err = svc.Register(ctx, "alum@example.com", "secreto12")
	if !errors.Is(err, repositories.ErrEmailTaken) {
		t.Fatalf("expected ErrEmailTaken, got %v", err)
	}

	login, err := svc.Login(ctx, "alum@example.com", "secreto12")
	if err != nil {
		t.Fatalf("login: %v", err)
	}
	if login.User.ID != reg.User.ID {
		t.Fatalf("user id mismatch")
	}

	_, err = svc.Login(ctx, "alum@example.com", "wrong-password")
	if !errors.Is(err, domain.ErrInvalidCredentials) {
		t.Fatalf("expected invalid credentials, got %v", err)
	}

	me, err := svc.Me(ctx, login.Token)
	if err != nil {
		t.Fatalf("me: %v", err)
	}
	if me.Email != "alum@example.com" {
		t.Fatalf("me email: %s", me.Email)
	}
}

func TestRegisterValidation(t *testing.T) {
	svc := newTestAuth(t)
	ctx := context.Background()

	if _, err := svc.Register(ctx, "not-an-email", "secreto12"); !errors.Is(err, domain.ErrInvalidEmail) {
		t.Fatalf("expected invalid email, got %v", err)
	}
	if _, err := svc.Register(ctx, "ok@example.com", "short"); !errors.Is(err, domain.ErrInvalidPassword) {
		t.Fatalf("expected invalid password, got %v", err)
	}
}
