package usecases

import (
	"fmt"
	"testing"

	"github.com/tu-usuario/plataforma-edu-backend/internal/domain"
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

type stubProfileRepo struct {
	profile domain.CognitiveProfile
}

func (r *stubProfileRepo) GetByUserID(userID string) (domain.CognitiveProfile, error) {
	if r.profile.UserID != userID {
		return domain.CognitiveProfile{}, fmt.Errorf("perfil cognitivo no encontrado para usuario %q", userID)
	}
	return r.profile, nil
}

func (r *stubProfileRepo) Save(profile domain.CognitiveProfile) error {
	r.profile = profile
	return nil
}

func TestCurriculumServiceListForStudent(t *testing.T) {
	t.Parallel()

	graph := domain.CurriculumGraph{
		Lessons: map[string]domain.LessonNode{
			"print-basics": {
				ID:          "print-basics",
				Title:       "Print",
				TrackType:   domain.TrackMicroPaso,
				SkillTarget: "print_basico",
			},
			"variables-and-types": {
				ID:            "variables-and-types",
				Title:         "Variables",
				TrackType:     domain.TrackMicroPaso,
				Prerequisites: []string{"print-basics"},
				SkillTarget:   "variables",
			},
		},
	}
	profiles := &stubProfileRepo{profile: domain.CognitiveProfile{
		UserID: domain.DemoUserID,
		Skills: []domain.StudentSkill{
			{ID: "print_basico", Status: domain.SkillStatusMastered},
		},
	}}
	service := NewCurriculumService(&stubCurriculumRepo{graph: graph}, profiles)

	views, err := service.ListForStudent(domain.DemoUserID)
	if err != nil {
		t.Fatalf("ListForStudent: %v", err)
	}
	if len(views) != 2 {
		t.Fatalf("cantidad inesperada: %d", len(views))
	}

	byID := map[string]LessonProgressView{}
	for _, view := range views {
		byID[view.ID] = view
	}
	if !byID["print-basics"].Unlocked {
		t.Fatal("print-basics debe estar unlocked")
	}
	if !byID["variables-and-types"].Unlocked {
		t.Fatal("variables debe estar unlocked con print mastered")
	}
}

func TestCurriculumServiceGetLessonForStudentBlocked(t *testing.T) {
	t.Parallel()

	graph := domain.CurriculumGraph{
		Lessons: map[string]domain.LessonNode{
			"print-basics": {
				ID:          "print-basics",
				SkillTarget: "print_basico",
			},
			"variables-and-types": {
				ID:            "variables-and-types",
				Prerequisites: []string{"print-basics"},
				SkillTarget:   "variables",
			},
		},
	}
	profiles := &stubProfileRepo{profile: domain.CognitiveProfile{
		UserID: domain.DemoUserID,
		Skills: []domain.StudentSkill{},
	}}
	service := NewCurriculumService(&stubCurriculumRepo{graph: graph}, profiles)

	view, err := service.GetLessonForStudent("variables-and-types", domain.DemoUserID)
	if err != nil {
		t.Fatalf("GetLessonForStudent: %v", err)
	}
	if view.Unlocked {
		t.Fatal("variables debe estar bloqueada sin print mastered")
	}
}
