package handlers

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHealth(t *testing.T) {
	t.Parallel()

	req := httptest.NewRequest(http.MethodGet, "/api/health", nil)
	rec := httptest.NewRecorder()

	Health(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("código HTTP inesperado: got %d, want %d", rec.Code, http.StatusOK)
	}

	contentType := rec.Header().Get("Content-Type")
	if contentType != "application/json" {
		t.Fatalf("Content-Type inesperado: got %q, want %q", contentType, "application/json")
	}

	var body healthResponse
	if err := json.Unmarshal(rec.Body.Bytes(), &body); err != nil {
		t.Fatalf("JSON inválido: %v", err)
	}

	if body.Status != "ok" {
		t.Fatalf("status inesperado: got %q, want %q", body.Status, "ok")
	}

	wantMessage := "Plataforma Educativa API funcionando"
	if body.Message != wantMessage {
		t.Fatalf("message inesperado: got %q, want %q", body.Message, wantMessage)
	}
}
