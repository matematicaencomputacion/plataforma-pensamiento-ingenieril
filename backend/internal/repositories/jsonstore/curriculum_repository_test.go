package jsonstore

import (
	"path/filepath"
	"testing"

	"github.com/tu-usuario/plataforma-edu-backend/internal/domain"
)

func TestCurriculumRepositorySeed(t *testing.T) {
	t.Parallel()

	repo := NewCurriculumRepository(filepath.Join("..", "..", "..", "data", "curriculum.json"))

	graph, err := repo.GetGraph()
	if err != nil {
		t.Fatalf("GetGraph: %v", err)
	}
	if len(graph.Lessons) < 3 {
		t.Fatalf("se esperaban al menos 3 lecciones, got %d", len(graph.Lessons))
	}

	lesson, err := repo.GetLesson("inventory-challenge")
	if err != nil {
		t.Fatalf("GetLesson: %v", err)
	}
	if lesson.TrackType != domain.TrackRetoIngenieril {
		t.Fatalf("track inesperado: %q", lesson.TrackType)
	}
	if len(lesson.Prerequisites) != 2 {
		t.Fatalf("prerrequisitos inesperados: %+v", lesson.Prerequisites)
	}
}
