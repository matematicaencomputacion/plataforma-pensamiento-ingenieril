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
	t.Parallel()

	handler := NewEvaluateHandler(usecases.NewEvaluationService())

	tests := []struct {
		name       string
		body       evaluateRequest
		wantPassed bool
		wantStatus int
	}{
		{
			name:       "aprobado",
			body:       evaluateRequest{Code: "print(42)", LevelID: 1},
			wantPassed: true,
			wantStatus: http.StatusOK,
		},
		{
			name:       "desaprobado",
			body:       evaluateRequest{Code: "return 42", LevelID: 1},
			wantPassed: false,
			wantStatus: http.StatusOK,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()

			payload, err := json.Marshal(tt.body)
			if err != nil {
				t.Fatalf("no se pudo serializar request: %v", err)
			}

			req := httptest.NewRequest(http.MethodPost, "/api/evaluate", bytes.NewReader(payload))
			rec := httptest.NewRecorder()

			handler.Evaluate(rec, req)

			if rec.Code != tt.wantStatus {
				t.Fatalf("código HTTP inesperado: got %d, want %d", rec.Code, tt.wantStatus)
			}

			var resp evaluateResponse
			if err := json.Unmarshal(rec.Body.Bytes(), &resp); err != nil {
				t.Fatalf("JSON inválido: %v", err)
			}

			if resp.Passed != tt.wantPassed {
				t.Fatalf("passed inesperado: got %v, want %v", resp.Passed, tt.wantPassed)
			}
		})
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
