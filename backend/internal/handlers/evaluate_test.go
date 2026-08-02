package handlers

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/tu-usuario/plataforma-edu-backend/internal/usecases"
)

func TestEvaluateHandler(t *testing.T) {
	t.Setenv("GROK_API_KEY", "test-key")

	grokServer := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		_, err := w.Write([]byte(`{"choices":[{"message":{"content":"{\"passed\": true, \"feedback\": \"Excelente print\"}"}}]}`))
		if err != nil {
			t.Fatalf("error escribiendo respuesta mock: %v", err)
		}
	}))
	defer grokServer.Close()

	service := usecases.NewEvaluationServiceForTest(grokServer.Client(), grokServer.URL)
	handler := NewEvaluateHandler(service)

	payload, err := json.Marshal(evaluateRequest{Code: "print(42)", LevelID: 1})
	if err != nil {
		t.Fatalf("no se pudo serializar request: %v", err)
	}

	req := httptest.NewRequest(http.MethodPost, "/api/evaluate", bytes.NewReader(payload))
	rec := httptest.NewRecorder()

	handler.Evaluate(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("código HTTP inesperado: got %d, want %d", rec.Code, http.StatusOK)
	}

	var resp evaluateResponse
	if err := json.Unmarshal(rec.Body.Bytes(), &resp); err != nil {
		t.Fatalf("JSON inválido: %v", err)
	}
	if !resp.Passed {
		t.Fatal("se esperaba passed=true")
	}
	if resp.Feedback != "Excelente print" {
		t.Fatalf("feedback inesperado: got %q", resp.Feedback)
	}
}

func TestEvaluateHandlerInvalidJSON(t *testing.T) {
	t.Parallel()

	handler := NewEvaluateHandler(usecases.NewEvaluationService())
	req := httptest.NewRequest(http.MethodPost, "/api/evaluate", bytes.NewBufferString("{invalid"))
	rec := httptest.NewRecorder()

	handler.Evaluate(rec, req)

	if rec.Code != http.StatusBadRequest {
		t.Fatalf("código HTTP inesperado: got %d, want %d", rec.Code, http.StatusBadRequest)
	}
}

func TestEvaluateHandlerMissingAPIKey(t *testing.T) {
	t.Setenv("GROK_API_KEY", "")

	handler := NewEvaluateHandler(usecases.NewEvaluationService())
	payload, err := json.Marshal(evaluateRequest{Code: "print(1)", LevelID: 1})
	if err != nil {
		t.Fatalf("no se pudo serializar request: %v", err)
	}

	req := httptest.NewRequest(http.MethodPost, "/api/evaluate", bytes.NewReader(payload))
	rec := httptest.NewRecorder()

	handler.Evaluate(rec, req)

	if rec.Code != http.StatusInternalServerError {
		t.Fatalf("código HTTP inesperado: got %d, want %d", rec.Code, http.StatusInternalServerError)
	}
}
