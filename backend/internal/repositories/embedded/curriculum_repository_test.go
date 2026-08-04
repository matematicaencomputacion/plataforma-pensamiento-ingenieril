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
	if len(graph.Lessons) != 10 {
		t.Fatalf("Module 1 debe tener 10 lecciones, got %d", len(graph.Lessons))
	}

	lesson, err := repo.GetLesson("py-m01-10-declarative-studio")
	if err != nil {
		t.Fatalf("GetLesson: %v", err)
	}
	if lesson.TrackType != domain.TrackRetoIngenieril {
		t.Fatalf("track_type inesperado: %q", lesson.TrackType)
	}
	if len(lesson.Prerequisites.LessonIDs()) != 2 {
		t.Fatalf("prerrequisitos de lección inesperados: %+v", lesson.Prerequisites)
	}
	if len(lesson.Concepts) < 2 {
		t.Fatalf("se esperaban múltiples conceptos en el nodo, got %+v", lesson.Concepts)
	}
	if len(graph.Concepts) != 4 {
		t.Fatalf("catálogo base incompleto: %d", len(graph.Concepts))
	}
	if err := graph.HasCycles(); err != nil {
		t.Fatalf("curriculum embebido con ciclo: %v", err)
	}
	if _, err := graph.TopologicalSort(); err != nil {
		t.Fatalf("TopologicalSort curriculum embebido: %v", err)
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

	lesson := graph.Lessons["py-m01-01-hello-print"]
	lesson.Title = "mutado"
	graph.Lessons["py-m01-01-hello-print"] = lesson

	reloaded, err := repo.GetGraph()
	if err != nil {
		t.Fatalf("GetGraph reload: %v", err)
	}
	if reloaded.Lessons["py-m01-01-hello-print"].Title == "mutado" {
		t.Fatal("GetGraph debe devolver una copia aislada del grafo embebido")
	}
}
