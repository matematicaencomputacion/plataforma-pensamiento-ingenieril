package repositories

import (
	"context"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

// ConceptEventRepository persiste eventos de fricción conceptual por usuario.
type ConceptEventRepository interface {
	Insert(ctx context.Context, event domain.ConceptEvent) error
	ListByUser(ctx context.Context, userID string) ([]domain.ConceptEvent, error)
}
