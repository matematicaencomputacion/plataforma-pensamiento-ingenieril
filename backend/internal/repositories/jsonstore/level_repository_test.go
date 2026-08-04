package jsonstore

import (
	"path/filepath"
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

func TestLevelRepositorySeed(t *testing.T) {
	t.Parallel()

	repo := NewLevelRepository(filepath.Join("..", "..", "..", "data", "levels.json"))

	current, err := repo.GetCurrent()
	if err != nil {
		t.Fatalf("GetCurrent: %v", err)
	}
	if current.ID != 1 {
		t.Fatalf("nivel actual inesperado: got %d, want 1", current.ID)
	}
	if current.TrackType != domain.TrackMicroPaso {
		t.Fatalf("track_type inesperado: got %q", current.TrackType)
	}

	reto, err := repo.GetByID(2)
	if err != nil {
		t.Fatalf("GetByID(2): %v", err)
	}
	if reto.TrackType != domain.TrackRetoIngenieril {
		t.Fatalf("track_type inesperado para nivel 2: got %q", reto.TrackType)
	}

	levels, err := repo.List()
	if err != nil {
		t.Fatalf("List: %v", err)
	}
	if len(levels) != 2 {
		t.Fatalf("cantidad de niveles inesperada: got %d, want 2", len(levels))
	}
}
