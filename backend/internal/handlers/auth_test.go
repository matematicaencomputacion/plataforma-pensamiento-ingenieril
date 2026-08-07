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

func newAuthHandler(t *testing.T) *handlers.AuthHandler {
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
	svc := usecases.NewAuthService(repo, crypto.NewBcryptHasher(), jwtauth.NewHS256Issuer("test-secret"))
	return handlers.NewAuthHandler(svc)
}

func TestAuthHTTPFlow(t *testing.T) {
	h := newAuthHandler(t)

	regBody := []byte(`{"email":"dev@ppi.local","password":"clave1234"}`)
	regReq := httptest.NewRequest(http.MethodPost, "/api/auth/register", bytes.NewReader(regBody))
	regRec := httptest.NewRecorder()
	h.Register(regRec, regReq)
	if regRec.Code != http.StatusCreated {
		t.Fatalf("register status %d body %s", regRec.Code, regRec.Body.String())
	}

	var regResp map[string]any
	if err := json.Unmarshal(regRec.Body.Bytes(), &regResp); err != nil {
		t.Fatalf("decode register: %v", err)
	}
	token, _ := regResp["token"].(string)
	if token == "" {
		t.Fatal("missing token")
	}

	dupReq := httptest.NewRequest(http.MethodPost, "/api/auth/register", bytes.NewReader(regBody))
	dupRec := httptest.NewRecorder()
	h.Register(dupRec, dupReq)
	if dupRec.Code != http.StatusConflict {
		t.Fatalf("expected 409, got %d", dupRec.Code)
	}

	loginReq := httptest.NewRequest(http.MethodPost, "/api/auth/login", bytes.NewReader(regBody))
	loginRec := httptest.NewRecorder()
	h.Login(loginRec, loginReq)
	if loginRec.Code != http.StatusOK {
		t.Fatalf("login status %d", loginRec.Code)
	}

	meReq := httptest.NewRequest(http.MethodGet, "/api/me", nil)
	meReq.Header.Set("Authorization", "Bearer "+token)
	meRec := httptest.NewRecorder()
	h.Me(meRec, meReq)
	if meRec.Code != http.StatusOK {
		t.Fatalf("me status %d body %s", meRec.Code, meRec.Body.String())
	}

	badMe := httptest.NewRequest(http.MethodGet, "/api/me", nil)
	badRec := httptest.NewRecorder()
	h.Me(badRec, badMe)
	if badRec.Code != http.StatusUnauthorized {
		t.Fatalf("expected 401, got %d", badRec.Code)
	}

	logoutReq := httptest.NewRequest(http.MethodPost, "/api/auth/logout", nil)
	logoutRec := httptest.NewRecorder()
	h.Logout(logoutRec, logoutReq)
	if logoutRec.Code != http.StatusNoContent {
		t.Fatalf("logout status %d", logoutRec.Code)
	}
}
