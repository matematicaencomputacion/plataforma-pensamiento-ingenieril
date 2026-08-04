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
	if len(graph.Lessons) < 3 {
		t.Fatalf("grafo incompleto: %d lecciones", len(graph.Lessons))
	}

	lesson, err := service.GetLesson("variables-and-types")
	if err != nil {
		t.Fatalf("GetLesson: %v", err)
	}
	if lesson.TrackType != domain.TrackMicroPaso {
		t.Fatalf("track_type inesperado: %q", lesson.TrackType)
	}
	if len(lesson.Prerequisites) != 1 || lesson.Prerequisites[0] != "print-basics" {
		t.Fatalf("prerrequisitos inesperados: %+v", lesson.Prerequisites)
	}
}

func TestEmbeddedCurriculumServiceNilRepo(t *testing.T) {
	t.Parallel()

	service := NewEmbeddedCurriculumService(nil)
	if _, err := service.RawJSON(); err == nil {
		t.Fatal("se esperaba error con repo nil")
	}
}
