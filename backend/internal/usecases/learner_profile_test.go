package usecases

import (
	"context"
	"errors"
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/keyword"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

type stubClassifier struct {
	out domain.LearnerProfileSynthesis
	err error
	calls int
}

func (s *stubClassifier) Classify(_ context.Context, _ string) (domain.LearnerProfileSynthesis, error) {
	s.calls++
	return s.out, s.err
}

func TestLearnerProfileServiceTooShort(t *testing.T) {
	stub := &stubClassifier{}
	svc := NewLearnerProfileService(stub)
	_, err := svc.Synthesize(context.Background(), "corto")
	if !errors.Is(err, ErrLearnerNotesTooShort) {
		t.Fatalf("want ErrLearnerNotesTooShort, got %v", err)
	}
	if stub.calls != 0 {
		t.Fatalf("classifier no debería invocarse: %d", stub.calls)
	}
}

func TestLearnerProfileServiceOK(t *testing.T) {
	stub := &stubClassifier{
		out: domain.LearnerProfileSynthesis{Purpose: "p", Urgency: "u", Vision: "v", Stack: "s"},
	}
	svc := NewLearnerProfileService(stub)
	got, err := svc.Synthesize(context.Background(), "texto suficientemente largo para analizar")
	if err != nil {
		t.Fatalf("unexpected err: %v", err)
	}
	if got.Purpose != "p" || stub.calls != 1 {
		t.Fatalf("got=%+v calls=%d", got, stub.calls)
	}
}

func TestLearnerProfileServiceClassifierError(t *testing.T) {
	stub := &stubClassifier{err: errors.New("boom")}
	svc := NewLearnerProfileService(stub)
	_, err := svc.Synthesize(context.Background(), "texto suficientemente largo para analizar")
	if !errors.Is(err, ErrProfileClassify) {
		t.Fatalf("want ErrProfileClassify, got %v", err)
	}
}

func TestKeywordClassifierSignals(t *testing.T) {
	c := keyword.NewClassifier()
	got, err := c.Classify(context.Background(), "Soy estudiante, necesito resultados rápido; de mi futuro no sé; hice Coursera.")
	if err != nil {
		t.Fatal(err)
	}
	if got.Purpose == "" || got.Urgency == "" || got.Vision == "" || got.Stack == "" {
		t.Fatalf("expected all signals, got %+v", got)
	}
}
