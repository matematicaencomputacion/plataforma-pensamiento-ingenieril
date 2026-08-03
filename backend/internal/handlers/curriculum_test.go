package handlers

import (
	"encoding/json"
	"fmt"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/tu-usuario/plataforma-edu-backend/internal/domain"
	"github.com/tu-usuario/plataforma-edu-backend/internal/usecases"
)

type stubCurriculumRepo struct {
	graph domain.CurriculumGraph
}

func (r *stubCurriculumRepo) GetGraph() (domain.CurriculumGraph, error) {
	return r.graph, nil
}

func (r *stubCurriculumRepo) GetLesson(id string) (domain.LessonNode, error) {
	lesson, ok := r.graph.Lessons[id]
	if !ok {
		return domain.LessonNode{}, fmt.Errorf("lección %q no encontrada", id)
	}
	return lesson, nil
}

type stubProfileRepoForCurriculum struct {
	profile domain.CognitiveProfile
}

func (r *stubProfileRepoForCurriculum) GetByUserID(userID string) (domain.CognitiveProfile, error) {
	if r.profile.UserID != "" && r.profile.UserID != userID {
		return domain.CognitiveProfile{}, fmt.Errorf("perfil cognitivo no encontrado para usuario %q", userID)
	}
	return r.profile, nil
}

func (r *stubProfileRepoForCurriculum) Save(profile domain.CognitiveProfile) error {
	r.profile = profile
	return nil
}

func TestCurriculumHandlerList(t *testing.T) {
	t.Parallel()

	graph := domain.CurriculumGraph{
		Lessons: map[string]domain.LessonNode{
			"print-basics": {
				ID:          "print-basics",
				Title:       "Print",
				TrackType:   domain.TrackMicroPaso,
				SkillTarget: "print_basico",
			},
		},
	}
	service := usecases.NewCurriculumService(
		&stubCurriculumRepo{graph: graph},
		&stubProfileRepoForCurriculum{profile: domain.CognitiveProfile{UserID: domain.DemoUserID}},
	)
	handler := NewCurriculumHandler(service)

	req := httptest.NewRequest(http.MethodGet, "/api/curriculum?student_id=demo-user", nil)
	rec := httptest.NewRecorder()
	handler.List(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("código HTTP inesperado: %d body %s", rec.Code, rec.Body.String())
	}

	var body curriculumListResponse
	if err := json.Unmarshal(rec.Body.Bytes(), &body); err != nil {
		t.Fatalf("JSON inválido: %v", err)
	}
	if len(body.Lessons) != 1 || !body.Lessons[0].Unlocked {
		t.Fatalf("respuesta inesperada: %+v", body)
	}
}

func TestCurriculumHandlerGetLessonNotFound(t *testing.T) {
	t.Parallel()

	service := usecases.NewCurriculumService(
		&stubCurriculumRepo{graph: domain.CurriculumGraph{Lessons: map[string]domain.LessonNode{}}},
		&stubProfileRepoForCurriculum{profile: domain.CognitiveProfile{UserID: domain.DemoUserID}},
	)
	handler := NewCurriculumHandler(service)

	req := httptest.NewRequest(http.MethodGet, "/api/curriculum/lessons/missing", nil)
	req.SetPathValue("id", "missing")
	rec := httptest.NewRecorder()
	handler.GetLesson(rec, req)

	if rec.Code != http.StatusNotFound {
		t.Fatalf("código HTTP inesperado: %d", rec.Code)
	}
}
