package usecases_test

import (
	"context"
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/crypto"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/jwtauth"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories/sqlite"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/usecases"
)

func newAnalyticsService(t *testing.T) (*usecases.ConceptAnalyticsService, *usecases.AuthService) {
	t.Helper()
	db, err := sqlite.OpenDB(":memory:")
	if err != nil {
		t.Fatalf("db: %v", err)
	}
	t.Cleanup(func() { _ = db.Close() })
	users, err := sqlite.NewUserRepository(db)
	if err != nil {
		t.Fatalf("users: %v", err)
	}
	events, err := sqlite.NewConceptEventRepository(db)
	if err != nil {
		t.Fatalf("events: %v", err)
	}
	auth := usecases.NewAuthService(
		users,
		crypto.NewBcryptHasher(),
		jwtauth.NewHS256Issuer("test-secret"),
		usecases.AuthOptions{ExposeResetToken: true},
	)
	return usecases.NewConceptAnalyticsService(auth, events), auth
}

func TestConceptAnalyticsRecordAndSummary(t *testing.T) {
	svc, auth := newAnalyticsService(t)
	ctx := context.Background()

	out, err := auth.Register(ctx, "ana@ppi.local", "clave1234")
	if err != nil {
		t.Fatalf("register: %v", err)
	}

	if err := svc.Record(ctx, out.Token, domain.EventInput{
		Type: domain.EventHeatmapDecadeOpen, PartitionID: 1, DecadeLo: 1,
	}); err != nil {
		t.Fatalf("record open: %v", err)
	}
	if err := svc.Record(ctx, out.Token, domain.EventInput{
		Type:        domain.EventLearnValidateFail,
		PartitionID: 1,
		DecadeLo:    1,
		StepID:      "py-01-hello",
	}); err != nil {
		t.Fatalf("record fail: %v", err)
	}

	sum, err := svc.Summary(ctx, out.Token)
	if err != nil {
		t.Fatalf("summary: %v", err)
	}
	if sum.Bottleneck == nil || sum.Bottleneck.Kind != "decade" || sum.Bottleneck.DecadeLo != 1 {
		t.Fatalf("bottleneck: %#v", sum.Bottleneck)
	}
	if sum.Bottleneck.Friction < 4 {
		t.Fatalf("expected fail-weighted friction: %#v", sum.Bottleneck)
	}

	if _, err := svc.Summary(ctx, "bad-token"); err != domain.ErrUnauthorized {
		t.Fatalf("bad token: %v", err)
	}
	if err := svc.Record(ctx, out.Token, domain.EventInput{Type: "page_view"}); err != domain.ErrInvalidEventType {
		t.Fatalf("unknown type: %v", err)
	}
}

func TestConceptAnalyticsEmptySummary(t *testing.T) {
	svc, auth := newAnalyticsService(t)
	ctx := context.Background()
	out, err := auth.Register(ctx, "empty@ppi.local", "clave1234")
	if err != nil {
		t.Fatalf("register: %v", err)
	}
	sum, err := svc.Summary(ctx, out.Token)
	if err != nil {
		t.Fatalf("summary: %v", err)
	}
	if sum.Bottleneck != nil {
		t.Fatalf("want nil bottleneck: %#v", sum.Bottleneck)
	}
	if sum.Partitions == nil || sum.Decades == nil {
		t.Fatal("JSON arrays must be non-nil")
	}
}
