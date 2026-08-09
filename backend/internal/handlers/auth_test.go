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
	svc := usecases.NewAuthService(
		repo,
		crypto.NewBcryptHasher(),
		jwtauth.NewHS256Issuer("test-secret"),
		usecases.AuthOptions{ExposeResetToken: true},
	)
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

func TestUpdateProfileHTTP(t *testing.T) {
	h := newAuthHandler(t)

	regBody := []byte(`{"email":"perfil@ppi.local","password":"clave1234"}`)
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

	unauthorized := httptest.NewRequest(
		http.MethodPut,
		"/api/user/profile",
		bytes.NewReader([]byte(`{"lifePurpose":"x"}`)),
	)
	unauthRec := httptest.NewRecorder()
	h.UpdateProfile(unauthRec, unauthorized)
	if unauthRec.Code != http.StatusUnauthorized {
		t.Fatalf("expected 401, got %d", unauthRec.Code)
	}

	emptyReq := httptest.NewRequest(
		http.MethodPut,
		"/api/user/profile",
		bytes.NewReader([]byte(`{}`)),
	)
	emptyReq.Header.Set("Authorization", "Bearer "+token)
	emptyRec := httptest.NewRecorder()
	h.UpdateProfile(emptyRec, emptyReq)
	if emptyRec.Code != http.StatusBadRequest {
		t.Fatalf("expected 400 empty, got %d body %s", emptyRec.Code, emptyRec.Body.String())
	}

	body := []byte(`{
		"lifePurpose":"Construir productos",
		"urgency":"ahora",
		"vision5Years":"ser staff engineer",
		"techStack":"go y python"
	}`)
	okReq := httptest.NewRequest(http.MethodPut, "/api/user/profile", bytes.NewReader(body))
	okReq.Header.Set("Authorization", "Bearer "+token)
	okRec := httptest.NewRecorder()
	h.UpdateProfile(okRec, okReq)
	if okRec.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d body %s", okRec.Code, okRec.Body.String())
	}

	var profile map[string]string
	if err := json.Unmarshal(okRec.Body.Bytes(), &profile); err != nil {
		t.Fatalf("decode profile: %v", err)
	}
	if profile["lifePurpose"] != "Construir productos" || profile["techStack"] != "go y python" {
		t.Fatalf("unexpected profile response: %#v", profile)
	}

	getReq := httptest.NewRequest(http.MethodGet, "/api/user/profile", nil)
	getReq.Header.Set("Authorization", "Bearer "+token)
	getRec := httptest.NewRecorder()
	h.GetProfile(getRec, getReq)
	if getRec.Code != http.StatusOK {
		t.Fatalf("GET profile expected 200, got %d body %s", getRec.Code, getRec.Body.String())
	}
	var loaded map[string]string
	if err := json.Unmarshal(getRec.Body.Bytes(), &loaded); err != nil {
		t.Fatalf("decode get profile: %v", err)
	}
	if loaded["lifePurpose"] != "Construir productos" {
		t.Fatalf("GET profile mismatch: %#v", loaded)
	}

	emptyGet := httptest.NewRequest(http.MethodGet, "/api/user/profile", nil)
	emptyGetRec := httptest.NewRecorder()
	h.GetProfile(emptyGetRec, emptyGet)
	if emptyGetRec.Code != http.StatusUnauthorized {
		t.Fatalf("GET without token expected 401, got %d", emptyGetRec.Code)
	}

	postReq := httptest.NewRequest(
		http.MethodPost,
		"/api/user/profile",
		bytes.NewReader([]byte(`{"lifePurpose":"vía POST","urgency":"hoy","vision5Years":"x","techStack":"go"}`)),
	)
	postReq.Header.Set("Authorization", "Bearer "+token)
	postRec := httptest.NewRecorder()
	h.Profile(postRec, postReq)
	if postRec.Code != http.StatusOK {
		t.Fatalf("POST profile expected 200, got %d body %s", postRec.Code, postRec.Body.String())
	}

	aliasReq := httptest.NewRequest(
		http.MethodPut,
		"/api/user/profile",
		bytes.NewReader([]byte(`{"purpose":"alias purpose","urgency":"u","vision":"v","stack":"s"}`)),
	)
	aliasReq.Header.Set("Authorization", "Bearer "+token)
	aliasRec := httptest.NewRecorder()
	h.UpdateProfile(aliasRec, aliasReq)
	if aliasRec.Code != http.StatusOK {
		t.Fatalf("alias PUT expected 200, got %d body %s", aliasRec.Code, aliasRec.Body.String())
	}
	var aliased map[string]string
	if err := json.Unmarshal(aliasRec.Body.Bytes(), &aliased); err != nil {
		t.Fatalf("decode alias: %v", err)
	}
	if aliased["lifePurpose"] != "alias purpose" || aliased["techStack"] != "s" {
		t.Fatalf("alias mapping failed: %#v", aliased)
	}

	badMethod := httptest.NewRequest(http.MethodDelete, "/api/user/profile", nil)
	badMethod.Header.Set("Authorization", "Bearer "+token)
	badRec := httptest.NewRecorder()
	h.Profile(badRec, badMethod)
	if badRec.Code != http.StatusMethodNotAllowed {
		t.Fatalf("DELETE expected 405, got %d", badRec.Code)
	}
}

func TestForgotResetPasswordHTTP(t *testing.T) {
	h := newAuthHandler(t)

	regBody := []byte(`{"email":"recover@ppi.local","password":"clave1234"}`)
	regReq := httptest.NewRequest(http.MethodPost, "/api/auth/register", bytes.NewReader(regBody))
	regRec := httptest.NewRecorder()
	h.Register(regRec, regReq)
	if regRec.Code != http.StatusCreated {
		t.Fatalf("register status %d body %s", regRec.Code, regRec.Body.String())
	}

	forgotBody := []byte(`{"email":"recover@ppi.local"}`)
	forgotReq := httptest.NewRequest(http.MethodPost, "/api/auth/forgot-password", bytes.NewReader(forgotBody))
	forgotRec := httptest.NewRecorder()
	h.ForgotPassword(forgotRec, forgotReq)
	if forgotRec.Code != http.StatusOK {
		t.Fatalf("forgot status %d body %s", forgotRec.Code, forgotRec.Body.String())
	}
	var forgotResp map[string]any
	if err := json.Unmarshal(forgotRec.Body.Bytes(), &forgotResp); err != nil {
		t.Fatalf("decode forgot: %v", err)
	}
	token, _ := forgotResp["resetToken"].(string)
	if token == "" {
		t.Fatal("expected resetToken in DX response")
	}

	unknownBody := []byte(`{"email":"missing@ppi.local"}`)
	unknownReq := httptest.NewRequest(http.MethodPost, "/api/auth/forgot-password", bytes.NewReader(unknownBody))
	unknownRec := httptest.NewRecorder()
	h.ForgotPassword(unknownRec, unknownReq)
	if unknownRec.Code != http.StatusOK {
		t.Fatalf("unknown forgot status %d", unknownRec.Code)
	}
	var unknownResp map[string]any
	if err := json.Unmarshal(unknownRec.Body.Bytes(), &unknownResp); err != nil {
		t.Fatalf("decode unknown: %v", err)
	}
	if _, ok := unknownResp["resetToken"]; ok {
		t.Fatal("unknown email must omit resetToken")
	}

	resetPayload, _ := json.Marshal(map[string]string{
		"token":    token,
		"password": "nuevaClave9",
	})
	resetReq := httptest.NewRequest(http.MethodPost, "/api/auth/reset-password", bytes.NewReader(resetPayload))
	resetRec := httptest.NewRecorder()
	h.ResetPassword(resetRec, resetReq)
	if resetRec.Code != http.StatusOK {
		t.Fatalf("reset status %d body %s", resetRec.Code, resetRec.Body.String())
	}

	oldLogin := httptest.NewRequest(http.MethodPost, "/api/auth/login", bytes.NewReader(regBody))
	oldRec := httptest.NewRecorder()
	h.Login(oldRec, oldLogin)
	if oldRec.Code != http.StatusUnauthorized {
		t.Fatalf("old password expected 401, got %d", oldRec.Code)
	}

	newBody := []byte(`{"email":"recover@ppi.local","password":"nuevaClave9"}`)
	newLogin := httptest.NewRequest(http.MethodPost, "/api/auth/login", bytes.NewReader(newBody))
	newRec := httptest.NewRecorder()
	h.Login(newRec, newLogin)
	if newRec.Code != http.StatusOK {
		t.Fatalf("new password login status %d", newRec.Code)
	}

	badReset := httptest.NewRequest(http.MethodPost, "/api/auth/reset-password", bytes.NewReader(resetPayload))
	badRec := httptest.NewRecorder()
	h.ResetPassword(badRec, badReset)
	if badRec.Code != http.StatusBadRequest {
		t.Fatalf("reused token expected 400, got %d", badRec.Code)
	}
}
