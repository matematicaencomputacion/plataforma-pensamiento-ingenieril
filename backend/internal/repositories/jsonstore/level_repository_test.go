package jsonstore

import (
	"path/filepath"
	"strings"
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
	if current.Title != "Variables (puente coding)" {
		t.Fatalf("título del nivel 1 desactualizado: got %q", current.Title)
	}
	blob := strings.ToLower(current.Title + " " + current.Statement)
	for _, stale := range []string{
		"tu primer print",
		"nivel operativo",
		"imprima un saludo",
		"saludo usando print",
		"declarative foundations",
		"foundations declarativas",
	} {
		if strings.Contains(blob, stale) {
			t.Fatalf("seed de niveles aún contiene copia congelada %q en %q", stale, blob)
		}
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
