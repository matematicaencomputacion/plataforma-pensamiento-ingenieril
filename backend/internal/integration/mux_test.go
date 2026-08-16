//go:build integration

package integration_test

import (
	"net/http"
	"path/filepath"
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/crypto"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/jwtauth"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/handlers"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories/jsonstore"
	sqliterepo "github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories/sqlite"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/usecases"
)

// newTestMux wires a minimal production-like ServeMux for integration tests.
// Prefer this over main.go's ListenAndServe for deterministic CI.
func newTestMux(t *testing.T) http.Handler {
	t.Helper()

	dbPath := filepath.Join(t.TempDir(), "ppi-integration.db")
	db, err := sqliterepo.OpenDB(dbPath)
	if err != nil {
		t.Fatalf("sqlite open: %v", err)
	}
	t.Cleanup(func() { _ = db.Close() })

	users, err := sqliterepo.NewUserRepository(db)
	if err != nil {
		t.Fatalf("users repo: %v", err)
	}

	authSvc := usecases.NewAuthService(
		users,
		crypto.NewBcryptHasher(),
		jwtauth.NewHS256Issuer("integration-test-secret"),
		usecases.AuthOptions{ExposeResetToken: true},
	)
	authHandler := handlers.NewAuthHandler(authSvc)

	// Resolve seeds relative to backend/ when tests run with cwd=backend.
	dataDir := "data"
	levelRepo := jsonstore.NewLevelRepository(jsonstore.DefaultLevelsPath(dataDir))
	levelSvc := usecases.NewLevelService(levelRepo)
	levelHandler := handlers.NewLevelHandler(levelSvc)

	mux := http.NewServeMux()
	mux.HandleFunc("GET /api/health", handlers.Health)
	mux.HandleFunc("POST /api/auth/register", authHandler.Register)
	mux.HandleFunc("POST /api/auth/login", authHandler.Login)
	mux.HandleFunc("POST /api/auth/logout", authHandler.Logout)
	mux.HandleFunc("POST /api/auth/forgot-password", authHandler.ForgotPassword)
	mux.HandleFunc("POST /api/auth/reset-password", authHandler.ResetPassword)
	mux.HandleFunc("GET /api/me", authHandler.Me)
	progressHandler := handlers.NewProgressHandler(authSvc)
	mux.HandleFunc("POST /api/progress/complete", progressHandler.Complete)
	mux.HandleFunc("POST /api/progress/reset", progressHandler.Reset)
	eventRepo, err := sqliterepo.NewConceptEventRepository(db)
	if err != nil {
		t.Fatalf("events repo: %v", err)
	}
	analyticsHandler := handlers.NewConceptAnalyticsHandler(
		usecases.NewConceptAnalyticsService(authSvc, eventRepo),
	)
	mux.HandleFunc("POST /api/concept-events", analyticsHandler.Record)
	mux.HandleFunc("GET /api/concept-analytics", analyticsHandler.Summary)
	mux.HandleFunc("GET /api/levels/current", levelHandler.GetCurrent)
	mux.HandleFunc("GET /api/levels/{id}", levelHandler.GetByID)
	return mux
}
