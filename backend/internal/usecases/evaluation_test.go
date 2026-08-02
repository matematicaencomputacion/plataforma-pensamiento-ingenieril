package usecases

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"
	"time"

	"github.com/tu-usuario/plataforma-edu-backend/internal/domain"
	"github.com/tu-usuario/plataforma-edu-backend/internal/repositories/jsonstore"
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
	out := make([]domain.Level, 0, len(r.levels))
	for _, level := range r.levels {
		out = append(out, level)
	}
	return out, nil
}

func seedLevel() domain.Level {
	return domain.Level{
		ID:               1,
		Title:            "Tu primer print",
		Statement:        "Imprime un saludo con print().",
		TrackType:        domain.TrackMicroPaso,
		EvaluationPrompt: "Eres un Tutor Básico.",
	}
}

func newTestProfileRepo(t *testing.T) *jsonstore.CognitiveProfileRepository {
	t.Helper()

	dir := t.TempDir()
	path := filepath.Join(dir, "cognitive_profiles.json")
	seed := []domain.CognitiveProfile{
		{
			UserID: domain.DemoUserID,
			Skills: []domain.StudentSkill{
				{
					ID:             "bucles_for",
					Status:         domain.SkillStatusMastered,
					LastReviewedAt: time.Date(2026, 7, 26, 12, 0, 0, 0, time.UTC),
				},
				{
					ID:             "print_basico",
					Status:         domain.SkillStatusLearning,
					LastReviewedAt: time.Date(2026, 8, 1, 12, 0, 0, 0, time.UTC),
				},
			},
		},
	}
	raw, err := json.MarshalIndent(seed, "", "  ")
	if err != nil {
		t.Fatalf("marshal seed: %v", err)
	}
	if err := os.WriteFile(path, append(raw, '\n'), 0o644); err != nil {
		t.Fatalf("write seed: %v", err)
	}

	return jsonstore.NewCognitiveProfileRepository(path)
}

func TestEvaluateCodeMissingAPIKey(t *testing.T) {
	t.Setenv("GROK_API_KEY", "")

	levels := &stubLevelRepo{levels: map[int]domain.Level{1: seedLevel()}}
	profiles := newTestProfileRepo(t)
	service := NewEvaluationService(levels, profiles)

	_, _, err := service.EvaluateCode("print(1)", 1, domain.DemoUserID)
	if err == nil {
		t.Fatal("se esperaba error cuando GROK_API_KEY está vacía")
	}
}

func TestEvaluateCodeSuccessUpdatesProfile(t *testing.T) {
	t.Setenv("GROK_API_KEY", "test-key")

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		var req chatCompletionRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			t.Fatalf("JSON de entrada inválido: %v", err)
		}
		if req.Model != grokModel {
			t.Fatalf("modelo inesperado: got %q, want %q", req.Model, grokModel)
		}
		if !strings.Contains(req.Messages[0].Content, "Enunciado del nivel") {
			t.Fatalf("system prompt sin enunciado: %q", req.Messages[0].Content)
		}
		if !strings.Contains(req.Messages[0].Content, "bucles_for") {
			t.Fatalf("system prompt sin perfil cognitivo: %q", req.Messages[0].Content)
		}
		if req.Messages[1].Content != "print(1)" {
			t.Fatalf("mensaje user inesperado: %+v", req.Messages[1])
		}

		w.Header().Set("Content-Type", "application/json")
		_, err := w.Write([]byte(`{"choices":[{"message":{"content":"{\"passed\": true, \"feedback\": \"Excelente trabajo\"}"}}]}`))
		if err != nil {
			t.Fatalf("error escribiendo respuesta mock: %v", err)
		}
	}))
	defer server.Close()

	levels := &stubLevelRepo{levels: map[int]domain.Level{1: seedLevel()}}
	profiles := newTestProfileRepo(t)
	service := NewEvaluationServiceForTest(server.Client(), server.URL, levels, profiles)

	got, feedback, err := service.EvaluateCode("print(1)", 1, domain.DemoUserID)
	if err != nil {
		t.Fatalf("error inesperado: %v", err)
	}
	if !got {
		t.Fatal("se esperaba passed=true")
	}
	if feedback != "Excelente trabajo" {
		t.Fatalf("feedback inesperado: got %q", feedback)
	}

	updated, err := profiles.GetByUserID(domain.DemoUserID)
	if err != nil {
		t.Fatalf("GetByUserID: %v", err)
	}

	foundLevelSkill := false
	for _, skill := range updated.Skills {
		if skill.ID == "level_1" && skill.Status == domain.SkillStatusMastered {
			foundLevelSkill = true
		}
	}
	if !foundLevelSkill {
		t.Fatalf("se esperaba skill level_1 mastered tras aprobar: %+v", updated.Skills)
	}
}

func TestEvaluateCodeRejectedDoesNotPersistMastery(t *testing.T) {
	t.Setenv("GROK_API_KEY", "test-key")

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		_, err := w.Write([]byte(`{"choices":[{"message":{"content":"{\"passed\": false, \"feedback\": \"Falta un print válido\"}"}}]}`))
		if err != nil {
			t.Fatalf("error escribiendo respuesta mock: %v", err)
		}
	}))
	defer server.Close()

	levels := &stubLevelRepo{levels: map[int]domain.Level{1: seedLevel()}}
	profiles := newTestProfileRepo(t)
	service := NewEvaluationServiceForTest(server.Client(), server.URL, levels, profiles)

	got, feedback, err := service.EvaluateCode("x = 1", 1, domain.DemoUserID)
	if err != nil {
		t.Fatalf("error inesperado: %v", err)
	}
	if got {
		t.Fatal("se esperaba passed=false")
	}
	if feedback != "Falta un print válido" {
		t.Fatalf("feedback inesperado: got %q", feedback)
	}

	updated, err := profiles.GetByUserID(domain.DemoUserID)
	if err != nil {
		t.Fatalf("GetByUserID: %v", err)
	}
	for _, skill := range updated.Skills {
		if skill.ID == "level_1" {
			t.Fatal("no se debe crear level_1 si desaprueba")
		}
	}
}

func TestEvaluateCodeNonOKStatus(t *testing.T) {
	t.Setenv("GROK_API_KEY", "test-key")

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusUnauthorized)
		_, err := w.Write([]byte(`{"error":"invalid api key"}`))
		if err != nil {
			t.Fatalf("error escribiendo respuesta mock: %v", err)
		}
	}))
	defer server.Close()

	levels := &stubLevelRepo{levels: map[int]domain.Level{1: seedLevel()}}
	profiles := newTestProfileRepo(t)
	service := NewEvaluationServiceForTest(server.Client(), server.URL, levels, profiles)

	_, _, err := service.EvaluateCode("print(1)", 1, domain.DemoUserID)
	if err == nil {
		t.Fatal("se esperaba error por status != 200")
	}

	wantFragment := `xAI API error: status 401, body: {"error":"invalid api key"}`
	if err.Error() != wantFragment {
		t.Fatalf("mensaje de error inesperado:\ngot:  %q\nwant: %q", err.Error(), wantFragment)
	}
}

func TestParseVerdictFromContent(t *testing.T) {
	t.Parallel()

	gotPassed, gotFeedback, err := parseVerdictFromContent(`{"passed": true, "feedback": "Bien hecho"}`)
	if err != nil {
		t.Fatalf("error inesperado: %v", err)
	}
	if !gotPassed || gotFeedback != "Bien hecho" {
		t.Fatalf("veredicto inesperado: %v %q", gotPassed, gotFeedback)
	}
}

func TestBuildSystemPrompt(t *testing.T) {
	t.Parallel()

	prompt, err := buildSystemPrompt(seedLevel(), domain.CognitiveProfile{
		UserID: domain.DemoUserID,
		Skills: []domain.StudentSkill{{ID: "bucles_for", Status: domain.SkillStatusMastered}},
	})
	if err != nil {
		t.Fatalf("error inesperado: %v", err)
	}
	if !strings.Contains(prompt, "Tutor Básico") || !strings.Contains(prompt, "bucles_for") {
		t.Fatalf("prompt incompleto: %q", prompt)
	}
}
