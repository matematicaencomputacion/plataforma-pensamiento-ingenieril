package sqlite

import (
	"context"
	"testing"
	"time"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

func TestConceptEventRepositoryInsertAndList(t *testing.T) {
	db, err := OpenDB(":memory:")
	if err != nil {
		t.Fatalf("db: %v", err)
	}
	t.Cleanup(func() { _ = db.Close() })

	users, err := NewUserRepository(db)
	if err != nil {
		t.Fatalf("users: %v", err)
	}
	if err := users.Create(domain.User{
		ID:           "u-events",
		Email:        "events@ppi.local",
		PasswordHash: "hash",
		CurrentLevel: 1,
	}); err != nil {
		t.Fatalf("create user: %v", err)
	}

	repo, err := NewConceptEventRepository(db)
	if err != nil {
		t.Fatalf("events repo: %v", err)
	}

	ev := domain.ConceptEvent{
		ID:          "e1",
		UserID:      "u-events",
		Type:        domain.EventHeatmapDecadeOpen,
		PartitionID: 1,
		DecadeLo:    1,
		CreatedAt:   time.Date(2026, 8, 16, 12, 0, 0, 0, time.UTC),
	}
	if err := repo.Insert(context.Background(), ev); err != nil {
		t.Fatalf("insert: %v", err)
	}

	other := domain.ConceptEvent{
		ID:          "e2",
		UserID:      "nobody",
		Type:        domain.EventConceptDwell,
		PartitionID: 2,
		CreatedAt:   time.Now().UTC(),
	}
	if err := repo.Insert(context.Background(), other); err == nil {
		t.Fatal("FK should reject unknown user")
	}

	got, err := repo.ListByUser(context.Background(), "u-events")
	if err != nil {
		t.Fatalf("list: %v", err)
	}
	if len(got) != 1 {
		t.Fatalf("want 1 event, got %#v", got)
	}
	if got[0].Type != domain.EventHeatmapDecadeOpen || got[0].DecadeLo != 1 {
		t.Fatalf("roundtrip: %#v", got[0])
	}
	empty, err := repo.ListByUser(context.Background(), "missing")
	if err != nil {
		t.Fatalf("list missing: %v", err)
	}
	if len(empty) != 0 {
		t.Fatalf("want empty, got %#v", empty)
	}
}
