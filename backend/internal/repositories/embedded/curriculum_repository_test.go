package embedded

import (
	"encoding/json"
	"strings"
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
	if len(graph.Concepts) != 20 {
		t.Fatalf("se esperaban 20 conceptos, got %d", len(graph.Concepts))
	}
	if len(graph.Edges) != 20 {
		t.Fatalf("se esperaban 20 aristas, got %d", len(graph.Edges))
	}
	if len(graph.Lessons) != 20 {
		t.Fatalf("proyección de lessons incompleta: %d", len(graph.Lessons))
	}

	lesson, err := repo.GetLesson("concept:function-parameters")
	if err != nil {
		t.Fatalf("GetLesson: %v", err)
	}
	if lesson.TrackType != domain.TrackMicroPaso {
		t.Fatalf("track_type inesperado: %q", lesson.TrackType)
	}
	prereqs := lesson.Prerequisites.LessonIDs()
	if len(prereqs) != 1 || prereqs[0] != "concept:variables-scope" {
		t.Fatalf("prerrequisitos inesperados: %+v", lesson.Prerequisites)
	}
	if len(lesson.Concepts) != 1 {
		t.Fatalf("nodo proyectado debe mapear su concepto, got %+v", lesson.Concepts)
	}

	foundCurated := false
	for _, edge := range graph.Edges {
		if edge.Source == "curated" && strings.TrimSpace(edge.RationaleES) != "" &&
			!strings.Contains(edge.RationaleES, "BORRADOR") {
			foundCurated = true
			break
		}
	}
	if !foundCurated {
		t.Fatal("se esperaba al menos una arista con rationale_es curado")
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

	const rootID = "concept:string-literals"
	lesson := graph.Lessons[rootID]
	lesson.Title = "mutado"
	graph.Lessons[rootID] = lesson

	reloaded, err := repo.GetGraph()
	if err != nil {
		t.Fatalf("GetGraph reload: %v", err)
	}
	if reloaded.Lessons[rootID].Title == "mutado" {
		t.Fatal("GetGraph debe devolver una copia aislada del grafo embebido")
	}
}
