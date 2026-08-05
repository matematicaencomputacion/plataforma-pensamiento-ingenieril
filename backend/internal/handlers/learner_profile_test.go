package handlers_test

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/keyword"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/handlers"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/usecases"
)

func TestLearnerProfileSynthesizeOK(t *testing.T) {
	svc := usecases.NewLearnerProfileService(keyword.NewClassifier())
	h := handlers.NewLearnerProfileHandler(svc)

	body := []byte(`{"raw_notes":"Soy estudiante y necesito resultados rápido por urgencia","source_step_id":"py-01-home"}`)
	req := httptest.NewRequest(http.MethodPost, "/api/learner/profile/synthesize", bytes.NewReader(body))
	rec := httptest.NewRecorder()
	h.Synthesize(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("status=%d body=%s", rec.Code, rec.Body.String())
	}
	var got map[string]string
	if err := json.Unmarshal(rec.Body.Bytes(), &got); err != nil {
		t.Fatal(err)
	}
	if got["purpose"] == "" || got["urgency"] == "" {
		t.Fatalf("unexpected synthesis: %#v", got)
	}
}

func TestLearnerProfileSynthesizeTooShort(t *testing.T) {
	svc := usecases.NewLearnerProfileService(keyword.NewClassifier())
	h := handlers.NewLearnerProfileHandler(svc)
	req := httptest.NewRequest(http.MethodPost, "/api/learner/profile/synthesize", bytes.NewReader([]byte(`{"raw_notes":"hola"}`)))
	rec := httptest.NewRecorder()
	h.Synthesize(rec, req)
	if rec.Code != http.StatusBadRequest {
		t.Fatalf("status=%d", rec.Code)
	}
}

func TestLearnerProfileServiceContext(t *testing.T) {
	// smoke: ensure synthesize accepts context cancellation path exists
	svc := usecases.NewLearnerProfileService(keyword.NewClassifier())
	_, err := svc.Synthesize(context.Background(), "texto suficientemente largo")
	if err != nil {
		t.Fatal(err)
	}
}
