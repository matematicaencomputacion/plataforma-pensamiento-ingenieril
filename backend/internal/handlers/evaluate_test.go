package handlers

import (
	"bytes"
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"testing"
	"time"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories/jsonstore"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/usecases"
)

type stubLevelRepo struct {
	levels map[int]domain.Level
}

func (r *stubLevelRepo) GetByID(id int) (domain.Level, error) {
	level, ok := r.levels[id]
	if !ok {
		return domain.Level{}, fmt.Errorf("nivel %d no encontrado", id)
	}
	return level, nil
}

func (r *stubLevelRepo) GetCurrent() (domain.Level, error) {
	return r.GetByID(1)
}

func (r *stubLevelRepo) List() ([]domain.Level, error) {
	return []domain.Level{r.levels[1]}, nil
}

func testProfileRepo(t *testing.T) *jsonstore.CognitiveProfileRepository {
	t.Helper()
	dir := t.TempDir()
	path := filepath.Join(dir, "cognitive_profiles.json")
	seed := []domain.CognitiveProfile{{
		UserID: domain.DemoUserID,
		Skills: []domain.StudentSkill{{
			ID:             "print_basico",
			Status:         domain.SkillStatusLearning,
			LastReviewedAt: time.Now().UTC(),
		}},
	}}
	raw, err := json.MarshalIndent(seed, "", "  ")
	if err != nil {
		t.Fatalf("marshal: %v", err)
	}
	if err := os.WriteFile(path, append(raw, '\n'), 0o644); err != nil {
		t.Fatalf("write: %v", err)
	}
	return jsonstore.NewCognitiveProfileRepository(path)
}

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

	levels := &stubLevelRepo{levels: map[int]domain.Level{
		1: {
			ID:               1,
			Title:            "print",
			Statement:        "usa print",
			TrackType:        domain.TrackMicroPaso,
			EvaluationPrompt: "Tutor Básico",
		},
	}}
	profiles := testProfileRepo(t)
	service := usecases.NewEvaluationServiceForTest(grokServer.Client(), grokServer.URL, levels, profiles)
	handler := NewEvaluateHandler(service)

	payload, err := json.Marshal(evaluateRequest{
		Code:      "print(42)",
		LevelID:   1,
		StudentID: domain.DemoUserID,
	})
	if err != nil {
		t.Fatalf("no se pudo serializar request: %v", err)
	}

	req := httptest.NewRequest(http.MethodPost, "/api/evaluate", bytes.NewReader(payload))
	rec := httptest.NewRecorder()
	handler.Evaluate(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("código HTTP inesperado: got %d body %s", rec.Code, rec.Body.String())
	}

	var resp evaluateResponse
	if err := json.Unmarshal(rec.Body.Bytes(), &resp); err != nil {
		t.Fatalf("JSON inválido: %v", err)
	}
	if !resp.Passed || resp.Feedback != "Excelente print" {
		t.Fatalf("respuesta inesperada: %+v", resp)
	}
}

func TestEvaluateHandlerInvalidJSON(t *testing.T) {
	t.Parallel()

	levels := &stubLevelRepo{levels: map[int]domain.Level{}}
	profiles := testProfileRepo(t)
	handler := NewEvaluateHandler(usecases.NewEvaluationService(levels, profiles))

	req := httptest.NewRequest(http.MethodPost, "/api/evaluate", bytes.NewBufferString("{invalid"))
	rec := httptest.NewRecorder()
	handler.Evaluate(rec, req)

	if rec.Code != http.StatusBadRequest {
		t.Fatalf("código HTTP inesperado: got %d, want %d", rec.Code, http.StatusBadRequest)
	}
}

func TestEvaluateHandlerMissingAPIKey(t *testing.T) {
	t.Setenv("GROK_API_KEY", "")

	levels := &stubLevelRepo{levels: map[int]domain.Level{
		1: {ID: 1, Title: "t", Statement: "s", TrackType: domain.TrackMicroPaso, EvaluationPrompt: "p"},
	}}
	profiles := testProfileRepo(t)
	handler := NewEvaluateHandler(usecases.NewEvaluationService(levels, profiles))

	payload, err := json.Marshal(evaluateRequest{Code: "print(1)", LevelID: 1, StudentID: domain.DemoUserID})
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
