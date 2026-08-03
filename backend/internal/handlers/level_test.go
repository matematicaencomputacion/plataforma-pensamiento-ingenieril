package handlers

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/tu-usuario/plataforma-edu-backend/internal/domain"
	"github.com/tu-usuario/plataforma-edu-backend/internal/usecases"
)

func TestLevelHandlerGetByID(t *testing.T) {
	t.Parallel()

	levels := &stubLevelRepo{levels: map[int]domain.Level{
		2: {
			ID:               2,
			Title:            "Inventario",
			Statement:        "Problema abierto",
			TrackType:        domain.TrackRetoIngenieril,
			EvaluationPrompt: "Arquitecto",
		},
	}}
	handler := NewLevelHandler(usecases.NewLevelService(levels))

	req := httptest.NewRequest(http.MethodGet, "/api/levels/2", nil)
	req.SetPathValue("id", "2")
	rec := httptest.NewRecorder()
	handler.GetByID(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("código HTTP inesperado: got %d body %s", rec.Code, rec.Body.String())
	}

	var level domain.Level
	if err := json.Unmarshal(rec.Body.Bytes(), &level); err != nil {
		t.Fatalf("JSON inválido: %v", err)
	}
	if level.ID != 2 || level.TrackType != domain.TrackRetoIngenieril {
		t.Fatalf("nivel inesperado: %+v", level)
	}
}

func TestLevelHandlerGetCurrent(t *testing.T) {
	t.Parallel()

	levels := &stubLevelRepo{levels: map[int]domain.Level{
		1: {
			ID:               1,
			Title:            "print",
			Statement:        "usa print",
			TrackType:        domain.TrackMicroPaso,
			EvaluationPrompt: "Tutor",
		},
	}}
	handler := NewLevelHandler(usecases.NewLevelService(levels))

	req := httptest.NewRequest(http.MethodGet, "/api/levels/current", nil)
	rec := httptest.NewRecorder()
	handler.GetCurrent(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("código HTTP inesperado: got %d", rec.Code)
	}
}

func TestLevelHandlerGetByIDInvalid(t *testing.T) {
	t.Parallel()

	handler := NewLevelHandler(usecases.NewLevelService(&stubLevelRepo{levels: map[int]domain.Level{}}))
	req := httptest.NewRequest(http.MethodGet, "/api/levels/abc", nil)
	req.SetPathValue("id", "abc")
	rec := httptest.NewRecorder()
	handler.GetByID(rec, req)

	if rec.Code != http.StatusBadRequest {
		t.Fatalf("código HTTP inesperado: got %d", rec.Code)
	}
}
