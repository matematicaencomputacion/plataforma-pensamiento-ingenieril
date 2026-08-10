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
	return usecases.NewAuthService(
		repo,
		crypto.NewBcryptHasher(),
		jwtauth.NewHS256Issuer("test-secret"),
		usecases.AuthOptions{ExposeResetToken: true},
	)
}

func TestForgotAndResetPassword(t *testing.T) {
	svc := newTestAuth(t)
	ctx := context.Background()

	_, err := svc.Register(ctx, "reset@example.com", "secreto12")
	if err != nil {
		t.Fatalf("register: %v", err)
	}

	unknown, err := svc.ForgotPassword(ctx, "nobody@example.com")
	if err != nil {
		t.Fatalf("forgot unknown: %v", err)
	}
	if unknown.ResetToken != "" {
		t.Fatal("unknown email must not expose token")
	}
	if unknown.Message == "" {
		t.Fatal("expected generic message")
	}

	forgot, err := svc.ForgotPassword(ctx, "reset@example.com")
	if err != nil {
		t.Fatalf("forgot: %v", err)
	}
	if forgot.ResetToken == "" {
		t.Fatal("expected exposed reset token")
	}

	_, err = svc.ResetPassword(ctx, forgot.ResetToken, "corta")
	if !errors.Is(err, domain.ErrInvalidPassword) {
		t.Fatalf("expected weak password error, got %v", err)
	}

	reset, err := svc.ResetPassword(ctx, forgot.ResetToken, "nuevaClave9")
	if err != nil {
		t.Fatalf("reset: %v", err)
	}
	if reset.Token == "" {
		t.Fatal("expected session token after reset")
	}

	_, err = svc.Login(ctx, "reset@example.com", "secreto12")
	if !errors.Is(err, domain.ErrInvalidCredentials) {
		t.Fatalf("old password should fail, got %v", err)
	}
	if _, err := svc.Login(ctx, "reset@example.com", "nuevaClave9"); err != nil {
		t.Fatalf("new password login: %v", err)
	}

	_, err = svc.ResetPassword(ctx, forgot.ResetToken, "otraClave99")
	if !errors.Is(err, domain.ErrInvalidResetToken) {
		t.Fatalf("reused token should fail, got %v", err)
	}
}

func TestForgotPasswordHidesTokenWhenDisabled(t *testing.T) {
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
	svc := usecases.NewAuthService(
		repo,
		crypto.NewBcryptHasher(),
		jwtauth.NewHS256Issuer("test-secret"),
		usecases.AuthOptions{ExposeResetToken: false},
	)
	ctx := context.Background()
	if _, err := svc.Register(ctx, "hide@example.com", "secreto12"); err != nil {
		t.Fatalf("register: %v", err)
	}
	out, err := svc.ForgotPassword(ctx, "hide@example.com")
	if err != nil {
		t.Fatalf("forgot: %v", err)
	}
	if out.ResetToken != "" {
		t.Fatal("token must stay hidden when exposure disabled")
	}
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

func TestUpdateProfile(t *testing.T) {
	svc := newTestAuth(t)
	ctx := context.Background()

	reg, err := svc.Register(ctx, "perfil@example.com", "secreto12")
	if err != nil {
		t.Fatalf("register: %v", err)
	}

	_, err = svc.UpdateProfile(ctx, "token-invalido", domain.LearnerProfile{LifePurpose: "x"})
	if !errors.Is(err, domain.ErrUnauthorized) {
		t.Fatalf("expected unauthorized, got %v", err)
	}

	_, err = svc.UpdateProfile(ctx, reg.Token, domain.LearnerProfile{})
	if !errors.Is(err, domain.ErrEmptyProfile) {
		t.Fatalf("expected empty profile, got %v", err)
	}

	got, err := svc.UpdateProfile(ctx, reg.Token, domain.LearnerProfile{
		LifePurpose:  "  Cambiar mi vida  ",
		Urgency:      "esta semana",
		Vision5Years: "liderazgo técnico",
		TechStack:    "python",
	})
	if err != nil {
		t.Fatalf("update: %v", err)
	}
	if got.LifePurpose != "Cambiar mi vida" {
		t.Fatalf("trim purpose: %q", got.LifePurpose)
	}
	if got.Urgency != "esta semana" || got.Vision5Years != "liderazgo técnico" || got.TechStack != "python" {
		t.Fatalf("unexpected profile: %+v", got)
	}

	loaded, err := svc.GetProfile(ctx, reg.Token)
	if err != nil {
		t.Fatalf("get profile: %v", err)
	}
	if loaded.LifePurpose != "Cambiar mi vida" || loaded.TechStack != "python" {
		t.Fatalf("rehydrate mismatch: %+v", loaded)
	}
}

func TestResolveExposeResetTokenEnvAndDev(t *testing.T) {
	t.Setenv("PPI_EXPOSE_RESET_TOKEN", "")
	t.Setenv("ENV", "")
	t.Setenv("APP_ENV", "")
	t.Setenv("GO_ENV", "")

	if !usecases.ResolveExposeResetToken("dev-only-change-me-ppi-jwt-secret") {
		t.Fatal("default JWT secret should expose")
	}
	if usecases.ResolveExposeResetToken("prod-super-secret-value") {
		t.Fatal("unknown JWT must not expose by default")
	}

	t.Setenv("ENV", "development")
	if !usecases.ResolveExposeResetToken("prod-super-secret-value") {
		t.Fatal("ENV=development should expose")
	}
	t.Setenv("ENV", "")

	t.Setenv("PPI_EXPOSE_RESET_TOKEN", "0")
	t.Setenv("ENV", "development")
	if usecases.ResolveExposeResetToken("dev-only-change-me-ppi-jwt-secret") {
		t.Fatal("explicit PPI_EXPOSE_RESET_TOKEN=0 must win")
	}
}

func TestCompleteProgressAdvancesLevel(t *testing.T) {
	svc := newTestAuth(t)
	ctx := context.Background()
	reg, err := svc.Register(ctx, "prog@example.com", "secreto12")
	if err != nil {
		t.Fatalf("register: %v", err)
	}

	denied, err := svc.CompleteProgress(ctx, reg.Token, 1, "py-02-variables", false)
	if err != nil {
		t.Fatalf("failed attempt: %v", err)
	}
	if denied.Advanced || denied.CurrentLevel != 1 {
		t.Fatalf("failed attempt must not advance: %#v", denied)
	}

	ok, err := svc.CompleteProgress(ctx, reg.Token, 1, "py-02-variables", true)
	if err != nil {
		t.Fatalf("pass: %v", err)
	}
	if !ok.Advanced || ok.CurrentLevel != 2 {
		t.Fatalf("expected advance to 2: %#v", ok)
	}

	idempotent, err := svc.CompleteProgress(ctx, reg.Token, 1, "py-02-variables", true)
	if err != nil {
		t.Fatalf("idempotent: %v", err)
	}
	if idempotent.Advanced || idempotent.CurrentLevel != 2 {
		t.Fatalf("second pass must not re-advance: %#v", idempotent)
	}

	_, err = svc.CompleteProgress(ctx, reg.Token, 0, "x", true)
	if !errors.Is(err, domain.ErrInvalidLevelID) {
		t.Fatalf("want ErrInvalidLevelID, got %v", err)
	}
	_, err = svc.CompleteProgress(ctx, reg.Token, 1, "  ", true)
	if !errors.Is(err, domain.ErrInvalidStepID) {
		t.Fatalf("want ErrInvalidStepID, got %v", err)
	}
}
