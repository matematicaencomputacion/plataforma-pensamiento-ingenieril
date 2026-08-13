package domain_test

import (
	"reflect"
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

func TestAdvanceCursorThroughCompleted(t *testing.T) {
	t.Parallel()

	next, moved := domain.AdvanceCursorThroughCompleted(1, []int{157})
	if moved || next != 1 {
		t.Fatalf("jump-ahead must not advance cursor: next=%d moved=%v", next, moved)
	}

	next, moved = domain.AdvanceCursorThroughCompleted(1, []int{1})
	if !moved || next != 2 {
		t.Fatalf("completing current must advance to 2: next=%d moved=%v", next, moved)
	}

	next, moved = domain.AdvanceCursorThroughCompleted(1, []int{1, 2})
	if !moved || next != 3 {
		t.Fatalf("cascade through contiguous: next=%d moved=%v", next, moved)
	}

	next, moved = domain.AdvanceCursorThroughCompleted(2, []int{1})
	if moved || next != 2 {
		t.Fatalf("cursor already past earned set: next=%d moved=%v", next, moved)
	}
}

func TestWithCompletedLevelIdempotent(t *testing.T) {
	t.Parallel()

	got := domain.WithCompletedLevel([]int{2}, 157)
	want := []int{2, 157}
	if !reflect.DeepEqual(got, want) {
		t.Fatalf("got %#v want %#v", got, want)
	}
	again := domain.WithCompletedLevel(got, 157)
	if !reflect.DeepEqual(again, want) {
		t.Fatalf("idempotent got %#v want %#v", again, want)
	}
	if !domain.HasCompletedLevel(again, 157) || domain.HasCompletedLevel(again, 1) {
		t.Fatalf("membership mismatch: %#v", again)
	}
}

func TestPublicUserIncludesCompletedLevels(t *testing.T) {
	t.Parallel()

	u := domain.User{
		ID:              "u1",
		Email:           "a@b.com",
		CurrentLevel:    2,
		CompletedLevels: []int{1},
	}
	pub := u.ToPublic()
	if pub.CurrentLevel != 2 || !reflect.DeepEqual(pub.CompletedLevels, []int{1}) {
		t.Fatalf("unexpected public user: %#v", pub)
	}

	empty := domain.User{ID: "u2", Email: "c@d.com", CurrentLevel: 1}.ToPublic()
	if empty.CompletedLevels == nil || len(empty.CompletedLevels) != 0 {
		t.Fatalf("nil completed must serialize as empty slice: %#v", empty.CompletedLevels)
	}
}
