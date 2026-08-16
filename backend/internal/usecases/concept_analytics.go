package usecases

import (
	"context"
	"strings"
	"time"

	"github.com/google/uuid"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories"
)

// ConceptAnalyticsService ingestión y resumen de fricción conceptual.
type ConceptAnalyticsService struct {
	auth   *AuthService
	events repositories.ConceptEventRepository
	now    func() time.Time
	newID  func() string
}

func NewConceptAnalyticsService(auth *AuthService, events repositories.ConceptEventRepository) *ConceptAnalyticsService {
	return &ConceptAnalyticsService{
		auth:   auth,
		events: events,
		now:    time.Now,
		newID:  uuid.NewString,
	}
}

// Record persiste un evento para el usuario del Bearer. Nunca acepta código.
func (s *ConceptAnalyticsService) Record(ctx context.Context, bearerToken string, in domain.EventInput) error {
	user, err := s.auth.Me(ctx, bearerToken)
	if err != nil {
		return err
	}
	in.Type = strings.TrimSpace(in.Type)
	in.StepID = strings.TrimSpace(in.StepID)
	if err := domain.ValidateEventInput(in); err != nil {
		return err
	}
	ev := domain.ConceptEvent{
		ID:          s.newID(),
		UserID:      user.ID,
		Type:        in.Type,
		PartitionID: in.PartitionID,
		DecadeLo:    in.DecadeLo,
		StepID:      in.StepID,
		CreatedAt:   s.now().UTC(),
	}
	return s.events.Insert(ctx, ev)
}

// Summary agrega los eventos del usuario autenticado (no es un warehouse docente).
func (s *ConceptAnalyticsService) Summary(ctx context.Context, bearerToken string) (domain.AnalyticsSummary, error) {
	user, err := s.auth.Me(ctx, bearerToken)
	if err != nil {
		return domain.AnalyticsSummary{}, err
	}
	events, err := s.events.ListByUser(ctx, user.ID)
	if err != nil {
		return domain.AnalyticsSummary{}, err
	}
	sum := domain.Aggregate(events)
	if sum.Partitions == nil {
		sum.Partitions = []domain.PartitionCount{}
	}
	if sum.Decades == nil {
		sum.Decades = []domain.DecadeCount{}
	}
	return sum, nil
}
