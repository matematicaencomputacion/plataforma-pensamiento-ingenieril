package persistence

import (
	"database/sql"
	"fmt"
	"path/filepath"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/config"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories"
	pgxrepo "github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories/postgres"
	sqliterepo "github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories/sqlite"
)

// Store is the opened user/event persistence (SQLite or Postgres).
type Store struct {
	DB     *sql.DB
	Users  repositories.UserRepository
	Events repositories.ConceptEventRepository
	Driver config.DriverKind
	Label  string
}

func (s *Store) Close() error {
	if s == nil || s.DB == nil {
		return nil
	}
	return s.DB.Close()
}

// Open selects sqlite vs postgres from DATABASE_URL. Tests stay on SQLite.
func Open(cfg config.AuthConfig, repoRoot string) (*Store, error) {
	switch cfg.Driver() {
	case config.DriverPostgres:
		db, err := pgxrepo.OpenDB(cfg.DatabaseURL)
		if err != nil {
			return nil, fmt.Errorf("postgres: %w", err)
		}
		users, err := pgxrepo.NewUserRepository(db)
		if err != nil {
			_ = db.Close()
			return nil, fmt.Errorf("postgres users: %w", err)
		}
		events, err := pgxrepo.NewConceptEventRepository(db)
		if err != nil {
			_ = db.Close()
			return nil, fmt.Errorf("postgres events: %w", err)
		}
		return &Store{
			DB:     db,
			Users:  users,
			Events: events,
			Driver: config.DriverPostgres,
			Label:  config.RedactedDatabaseURL(cfg.DatabaseURL),
		}, nil
	default:
		path := cfg.SQLitePath()
		if path != ":memory:" && !filepath.IsAbs(path) && repoRoot != "" {
			path = filepath.Join(repoRoot, path)
		}
		db, err := sqliterepo.OpenDB(path)
		if err != nil {
			return nil, fmt.Errorf("sqlite: %w", err)
		}
		users, err := sqliterepo.NewUserRepository(db)
		if err != nil {
			_ = db.Close()
			return nil, fmt.Errorf("sqlite users: %w", err)
		}
		events, err := sqliterepo.NewConceptEventRepository(db)
		if err != nil {
			_ = db.Close()
			return nil, fmt.Errorf("sqlite events: %w", err)
		}
		return &Store{
			DB:     db,
			Users:  users,
			Events: events,
			Driver: config.DriverSQLite,
			Label:  path,
		}, nil
	}
}
