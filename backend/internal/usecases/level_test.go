package usecases

import (
	"testing"

	"github.com/tu-usuario/plataforma-edu-backend/internal/domain"
)

func TestLevelServiceGetByID(t *testing.T) {
	t.Parallel()

	service := NewLevelService(&stubLevelRepo{levels: map[int]domain.Level{1: seedLevel()}})
	level, err := service.GetByID(1)
	if err != nil {
		t.Fatalf("GetByID: %v", err)
	}
	if level.TrackType != domain.TrackMicroPaso {
		t.Fatalf("track inesperado: %q", level.TrackType)
	}
}

func TestLevelServiceGetByIDInvalid(t *testing.T) {
	t.Parallel()

	service := NewLevelService(&stubLevelRepo{levels: map[int]domain.Level{}})
	_, err := service.GetByID(0)
	if err == nil {
		t.Fatal("se esperaba error para id inválido")
	}
}
