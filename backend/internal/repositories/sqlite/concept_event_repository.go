package sqlite

import (
	"context"
	"database/sql"
	"time"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories"
)

// ConceptEventRepository persistencia SQLite de eventos de fricción conceptual.
type ConceptEventRepository struct {
	db *sql.DB
}

var _ repositories.ConceptEventRepository = (*ConceptEventRepository)(nil)

func NewConceptEventRepository(db *sql.DB) (*ConceptEventRepository, error) {
	repo := &ConceptEventRepository{db: db}
	if err := repo.migrate(); err != nil {
		return nil, err
	}
	return repo, nil
}

func (r *ConceptEventRepository) migrate() error {
	_, err := r.db.Exec(`
CREATE TABLE IF NOT EXISTS concept_events (
  id TEXT PRIMARY KEY NOT NULL,
  user_id TEXT NOT NULL,
  event_type TEXT NOT NULL,
  partition_id INTEGER NOT NULL DEFAULT 0,
  decade_lo INTEGER NOT NULL DEFAULT 0,
  step_id TEXT NOT NULL DEFAULT '',
  created_at TEXT NOT NULL,
  FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE INDEX IF NOT EXISTS idx_concept_events_user ON concept_events(user_id);
`)
	return err
}

func (r *ConceptEventRepository) Insert(ctx context.Context, event domain.ConceptEvent) error {
	created := event.CreatedAt.UTC().Format(time.RFC3339Nano)
	_, err := r.db.ExecContext(
		ctx,
		`INSERT INTO concept_events (id, user_id, event_type, partition_id, decade_lo, step_id, created_at)
		 VALUES (?, ?, ?, ?, ?, ?, ?)`,
		event.ID,
		event.UserID,
		event.Type,
		event.PartitionID,
		event.DecadeLo,
		event.StepID,
		created,
	)
	return err
}

func (r *ConceptEventRepository) ListByUser(ctx context.Context, userID string) ([]domain.ConceptEvent, error) {
	rows, err := r.db.QueryContext(
		ctx,
		`SELECT id, user_id, event_type, partition_id, decade_lo, step_id, created_at
		 FROM concept_events WHERE user_id = ? ORDER BY created_at ASC`,
		userID,
	)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	out := make([]domain.ConceptEvent, 0)
	for rows.Next() {
		var ev domain.ConceptEvent
		var created string
		if err := rows.Scan(
			&ev.ID,
			&ev.UserID,
			&ev.Type,
			&ev.PartitionID,
			&ev.DecadeLo,
			&ev.StepID,
			&created,
		); err != nil {
			return nil, err
		}
		if ts, err := time.Parse(time.RFC3339Nano, created); err == nil {
			ev.CreatedAt = ts
		} else if ts, err := time.Parse(time.RFC3339, created); err == nil {
			ev.CreatedAt = ts
		}
		out = append(out, ev)
	}
	return out, rows.Err()
}
