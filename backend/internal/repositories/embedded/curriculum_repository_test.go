package embedded

import (
	"encoding/json"
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

func TestCurriculumRepositoryReadsFromMemory(t *testing.T) {
	t.Parallel()

	repo := NewCurriculumRepository()

	raw := repo.RawJSON()
	if len(raw) == 0 {
		t.Fatal("RawJSON vacío")
	}
	if !json.Valid(raw) {
		t.Fatal("RawJSON no es JSON válido")
	}

	graph, err := repo.GetGraph()
	if err != nil {
		t.Fatalf("GetGraph: %v", err)
	}
	if len(graph.Lessons) < 3 {
		t.Fatalf("lecciones insuficientes: %d", len(graph.Lessons))
	}

	lesson, err := repo.GetLesson("inventory-challenge")
	if err != nil {
		t.Fatalf("GetLesson: %v", err)
	}
	if lesson.TrackType != domain.TrackRetoIngenieril {
		t.Fatalf("track_type inesperado: %q", lesson.TrackType)
	}
	if len(lesson.Prerequisites) != 2 {
		t.Fatalf("prerrequisitos inesperados: %+v", lesson.Prerequisites)
	}
}

func TestCurriculumRepositoryGetLessonNotFound(t *testing.T) {
	t.Parallel()

	repo := NewCurriculumRepository()
	_, err := repo.GetLesson("does-not-exist")
	if err == nil {
		t.Fatal("se esperaba error para lección inexistente")
	}
}

func TestCurriculumRepositoryGraphIsolation(t *testing.T) {
	t.Parallel()

	repo := NewCurriculumRepository()
	graph, err := repo.GetGraph()
	if err != nil {
		t.Fatalf("GetGraph: %v", err)
	}

	lesson := graph.Lessons["print-basics"]
	lesson.Title = "mutado"
	graph.Lessons["print-basics"] = lesson

	reloaded, err := repo.GetGraph()
	if err != nil {
		t.Fatalf("GetGraph reload: %v", err)
	}
	if reloaded.Lessons["print-basics"].Title == "mutado" {
		t.Fatal("GetGraph debe devolver una copia aislada del grafo embebido")
	}
}
