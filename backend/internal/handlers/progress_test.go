package handlers_test

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/crypto"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/jwtauth"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/handlers"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories/sqlite"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/usecases"
)

func newProgressHandler(t *testing.T) (*handlers.ProgressHandler, *handlers.AuthHandler) {
	t.Helper()
	db, err := sqlite.OpenDB(":memory:")
	if err != nil {
		t.Fatalf("db: %v", err)
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
		usecases.AuthOptions{ExposeResetToken: true},
	)
	return handlers.NewProgressHandler(svc), handlers.NewAuthHandler(svc)
}

func TestProgressCompleteHTTP(t *testing.T) {
	progress, auth := newProgressHandler(t)

	regBody := []byte(`{"email":"prog-http@ppi.local","password":"clave1234"}`)
	regReq := httptest.NewRequest(http.MethodPost, "/api/auth/register", bytes.NewReader(regBody))
	regRec := httptest.NewRecorder()
	auth.Register(regRec, regReq)
	if regRec.Code != http.StatusCreated {
		t.Fatalf("register %d %s", regRec.Code, regRec.Body.String())
	}
	var regResp map[string]any
	_ = json.Unmarshal(regRec.Body.Bytes(), &regResp)
	token, _ := regResp["token"].(string)

	withCode := []byte(`{"level_id":1,"step_id":"py-02-variables","passed":true,"code":"print(1)"}`)
	codeReq := httptest.NewRequest(http.MethodPost, "/api/progress/complete", bytes.NewReader(withCode))
	codeReq.Header.Set("Authorization", "Bearer "+token)
	codeRec := httptest.NewRecorder()
	progress.Complete(codeRec, codeReq)
	if codeRec.Code != http.StatusBadRequest {
		t.Fatalf("code payload must be rejected: %d %s", codeRec.Code, codeRec.Body.String())
	}

	okBody := []byte(`{"level_id":1,"step_id":"py-02-variables","passed":true}`)
	okReq := httptest.NewRequest(http.MethodPost, "/api/progress/complete", bytes.NewReader(okBody))
	okReq.Header.Set("Authorization", "Bearer "+token)
	okRec := httptest.NewRecorder()
	progress.Complete(okRec, okReq)
	if okRec.Code != http.StatusOK {
		t.Fatalf("complete %d %s", okRec.Code, okRec.Body.String())
	}
	var out map[string]any
	_ = json.Unmarshal(okRec.Body.Bytes(), &out)
	if out["advanced"] != true {
		t.Fatalf("expected advanced: %#v", out)
	}
	if out["current_level"].(float64) != 2 {
		t.Fatalf("expected current_level=2: %#v", out)
	}

	resetReq := httptest.NewRequest(http.MethodPost, "/api/progress/reset", nil)
	resetReq.Header.Set("Authorization", "Bearer "+token)
	resetRec := httptest.NewRecorder()
	progress.Reset(resetRec, resetReq)
	if resetRec.Code != http.StatusOK {
		t.Fatalf("reset %d %s", resetRec.Code, resetRec.Body.String())
	}
	var resetOut map[string]any
	_ = json.Unmarshal(resetRec.Body.Bytes(), &resetOut)
	if resetOut["current_level"].(float64) != 1 {
		t.Fatalf("expected reset to 1: %#v", resetOut)
	}
}
