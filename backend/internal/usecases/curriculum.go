package usecases

import (
	"fmt"

	"github.com/tu-usuario/plataforma-edu-backend/internal/domain"
	"github.com/tu-usuario/plataforma-edu-backend/internal/repositories"
)

// LessonProgressView expone una lección con su estado de desbloqueo para un estudiante.
type LessonProgressView struct {
	ID            string           `json:"id"`
	Title         string           `json:"title"`
	TrackType     domain.TrackType `json:"track_type"`
	Description   string           `json:"description"`
	Prerequisites []string         `json:"prerequisites"`
	SkillTarget   string           `json:"skill_target"`
	Unlocked      bool             `json:"unlocked"`
}

// CurriculumService orquesta consultas de malla curricular y desbloqueo.
type CurriculumService struct {
	curriculum repositories.CurriculumRepository
	profiles   repositories.CognitiveProfileRepository
}

// NewCurriculumService crea el servicio de curriculum.
func NewCurriculumService(
	curriculum repositories.CurriculumRepository,
	profiles repositories.CognitiveProfileRepository,
) *CurriculumService {
	return &CurriculumService{
		curriculum: curriculum,
		profiles:   profiles,
	}
}

// ListForStudent retorna la malla con flags de desbloqueo para el estudiante.
func (s *CurriculumService) ListForStudent(studentID string) ([]LessonProgressView, error) {
	if studentID == "" {
		studentID = domain.DemoUserID
	}

	graph, err := s.curriculum.GetGraph()
	if err != nil {
		return nil, err
	}

	profile, err := s.profiles.GetByUserID(studentID)
	if err != nil {
		return nil, fmt.Errorf("error al cargar perfil cognitivo de %q: %w", studentID, err)
	}

	views := make([]LessonProgressView, 0, len(graph.Lessons))
	for _, lesson := range graph.Lessons {
		views = append(views, toLessonProgressView(lesson, graph.IsUnlocked(lesson.ID, &profile)))
	}

	return views, nil
}

// GetLessonForStudent retorna una lección y si está desbloqueada para el estudiante.
func (s *CurriculumService) GetLessonForStudent(lessonID, studentID string) (LessonProgressView, error) {
	if lessonID == "" {
		return LessonProgressView{}, fmt.Errorf("lesson_id es obligatorio")
	}
	if studentID == "" {
		studentID = domain.DemoUserID
	}

	graph, err := s.curriculum.GetGraph()
	if err != nil {
		return LessonProgressView{}, err
	}

	lesson, ok := graph.Lessons[lessonID]
	if !ok {
		return LessonProgressView{}, fmt.Errorf("lección %q no encontrada", lessonID)
	}

	profile, err := s.profiles.GetByUserID(studentID)
	if err != nil {
		return LessonProgressView{}, fmt.Errorf("error al cargar perfil cognitivo de %q: %w", studentID, err)
	}

	return toLessonProgressView(lesson, graph.IsUnlocked(lessonID, &profile)), nil
}

func toLessonProgressView(lesson domain.LessonNode, unlocked bool) LessonProgressView {
	prereqs := make([]string, len(lesson.Prerequisites))
	copy(prereqs, lesson.Prerequisites)

	return LessonProgressView{
		ID:            lesson.ID,
		Title:         lesson.Title,
		TrackType:     lesson.TrackType,
		Description:   lesson.Description,
		Prerequisites: prereqs,
		SkillTarget:   lesson.SkillTarget,
		Unlocked:      unlocked,
	}
}
