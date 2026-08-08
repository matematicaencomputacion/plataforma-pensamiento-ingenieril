package main

import (
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/crypto"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/jwtauth"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/handlers"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories/sqlite"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/usecases"
)

// Verifica que el mux registra GET y PUT/POST en /api/user/profile (no 405 en GET).
func TestUserProfileRouteMethods(t *testing.T) {
	db, err := sqlite.OpenDB(":memory:")
	if err != nil {
		t.Fatalf("db: %v", err)
	}
	t.Cleanup(func() { _ = db.Close() })
	repo, err := sqlite.NewUserRepository(db)
	if err != nil {
		t.Fatalf("repo: %v", err)
	}
	svc := usecases.NewAuthService(repo, crypto.NewBcryptHasher(), jwtauth.NewHS256Issuer("test-secret"))
	authHandler := handlers.NewAuthHandler(svc)

	mux := http.NewServeMux()
	mux.HandleFunc("/api/user/profile", authHandler.Profile)
	handler := enableCORS(mux)

	getReq := httptest.NewRequest(http.MethodGet, "/api/user/profile", nil)
	getRec := httptest.NewRecorder()
	handler.ServeHTTP(getRec, getReq)
	if getRec.Code == http.StatusMethodNotAllowed {
		t.Fatalf("GET /api/user/profile no debe ser 405; Allow=%q body=%s",
			getRec.Header().Get("Allow"), getRec.Body.String())
	}
	if getRec.Code != http.StatusUnauthorized {
		t.Fatalf("GET sin token: got %d want 401", getRec.Code)
	}

	putReq := httptest.NewRequest(http.MethodPut, "/api/user/profile", nil)
	putRec := httptest.NewRecorder()
	handler.ServeHTTP(putRec, putReq)
	if putRec.Code == http.StatusMethodNotAllowed {
		t.Fatal("PUT no debe ser 405")
	}

	postReq := httptest.NewRequest(http.MethodPost, "/api/user/profile", nil)
	postRec := httptest.NewRecorder()
	handler.ServeHTTP(postRec, postReq)
	if postRec.Code == http.StatusMethodNotAllowed {
		t.Fatal("POST no debe ser 405")
	}

	optReq := httptest.NewRequest(http.MethodOptions, "/api/user/profile", nil)
	optRec := httptest.NewRecorder()
	handler.ServeHTTP(optRec, optReq)
	if optRec.Code != http.StatusOK {
		t.Fatalf("OPTIONS got %d want 200", optRec.Code)
	}
}
