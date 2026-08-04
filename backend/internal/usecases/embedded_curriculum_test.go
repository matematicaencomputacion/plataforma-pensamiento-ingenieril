package usecases

import (
	"encoding/json"
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories/embedded"
)

func TestEmbeddedCurriculumServiceFromMemory(t *testing.T) {
	t.Parallel()

	service := NewEmbeddedCurriculumService(embedded.NewCurriculumRepository())

	raw, err := service.RawJSON()
	if err != nil {
		t.Fatalf("RawJSON: %v", err)
	}
	if !json.Valid(raw) {
		t.Fatal("RawJSON inválido")
	}

	graph, err := service.GetGraph()
	if err != nil {
		t.Fatalf("GetGraph: %v", err)
	}
	if len(graph.Concepts) != 20 || len(graph.Edges) != 20 {
		t.Fatalf("grafo unificado incompleto: concepts=%d edges=%d", len(graph.Concepts), len(graph.Edges))
	}
	if len(graph.Lessons) != 20 {
		t.Fatalf("proyección de lessons incompleta: %d", len(graph.Lessons))
	}

	lesson, err := service.GetLesson("concept:variables-scope")
	if err != nil {
		t.Fatalf("GetLesson: %v", err)
	}
	if lesson.TrackType != domain.TrackMicroPaso {
		t.Fatalf("track_type inesperado: %q", lesson.TrackType)
	}
	prereqs := lesson.Prerequisites.LessonIDs()
	if len(prereqs) != 1 || prereqs[0] != "concept:string-literals" {
		t.Fatalf("prerrequisitos inesperados: %+v", lesson.Prerequisites)
	}
	if len(lesson.ConceptIDs()) != 1 || lesson.ConceptIDs()[0] != "concept:variables-scope" {
		t.Fatalf("conceptos del nodo inesperados: %+v", lesson.Concepts)
	}
}

func TestEmbeddedCurriculumServiceNilRepo(t *testing.T) {
	t.Parallel()

	service := NewEmbeddedCurriculumService(nil)
	if _, err := service.RawJSON(); err == nil {
		t.Fatal("se esperaba error con repo nil")
	}
}
