package jsonstore

import (
	"os"
	"path/filepath"
	"sync"
	"testing"
	"time"

	"github.com/tu-usuario/plataforma-edu-backend/internal/domain"
)

func TestCognitiveProfileRepositoryGetAndSave(t *testing.T) {
	dir := t.TempDir()
	seedSrc := filepath.Join("..", "..", "..", "data", "cognitive_profiles.json")
	seedDst := filepath.Join(dir, "cognitive_profiles.json")

	raw, err := os.ReadFile(seedSrc)
	if err != nil {
		t.Fatalf("no se pudo leer seed: %v", err)
	}
	if err := os.WriteFile(seedDst, raw, 0o644); err != nil {
		t.Fatalf("no se pudo copiar seed: %v", err)
	}

	repo := NewCognitiveProfileRepository(seedDst)

	profile, err := repo.GetByUserID(domain.DemoUserID)
	if err != nil {
		t.Fatalf("GetByUserID: %v", err)
	}
	if len(profile.Skills) == 0 {
		t.Fatal("se esperaban skills en el seed")
	}

	profile.Skills = append(profile.Skills, domain.StudentSkill{
		ID:             "variables",
		Status:         domain.SkillStatusLearning,
		LastReviewedAt: time.Date(2026, 8, 2, 12, 0, 0, 0, time.UTC),
	})

	if err := repo.Save(profile); err != nil {
		t.Fatalf("Save: %v", err)
	}

	reloaded, err := repo.GetByUserID(domain.DemoUserID)
	if err != nil {
		t.Fatalf("GetByUserID reload: %v", err)
	}
	if len(reloaded.Skills) != len(profile.Skills) {
		t.Fatalf("skills inesperadas tras Save: got %d, want %d", len(reloaded.Skills), len(profile.Skills))
	}
}

func TestCognitiveProfileRepositoryConcurrentSave(t *testing.T) {
	dir := t.TempDir()
	filePath := filepath.Join(dir, "cognitive_profiles.json")
	if err := os.WriteFile(filePath, []byte("[]\n"), 0o644); err != nil {
		t.Fatalf("no se pudo crear archivo: %v", err)
	}

	repo := NewCognitiveProfileRepository(filePath)

	var wg sync.WaitGroup
	errCh := make(chan error, 20)

	for i := 0; i < 20; i++ {
		wg.Add(1)
		go func(i int) {
			defer wg.Done()

			profile := domain.CognitiveProfile{
				UserID: domain.DemoUserID,
				Skills: []domain.StudentSkill{
					{
						ID:             "print_basico",
						Status:         domain.SkillStatusLearning,
						LastReviewedAt: time.Now().UTC(),
					},
				},
			}
			if err := repo.Save(profile); err != nil {
				errCh <- err
			}
		}(i)
	}

	wg.Wait()
	close(errCh)

	for err := range errCh {
		t.Fatalf("Save concurrente falló: %v", err)
	}

	profile, err := repo.GetByUserID(domain.DemoUserID)
	if err != nil {
		t.Fatalf("GetByUserID tras concurrencia: %v", err)
	}
	if profile.UserID != domain.DemoUserID {
		t.Fatalf("user_id inesperado: %q", profile.UserID)
	}
}
