package sqlite

import (
	"database/sql"
	"errors"
	"fmt"
	"os"
	"path/filepath"
	"strings"

	_ "modernc.org/sqlite"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories"
)

// UserRepository persistencia SQLite de usuarios.
type UserRepository struct {
	db *sql.DB
}

func OpenDB(path string) (*sql.DB, error) {
	dsn := path
	if path != ":memory:" && !strings.HasPrefix(path, "file:") {
		if err := os.MkdirAll(filepath.Dir(path), 0o755); err != nil {
			return nil, fmt.Errorf("crear dir sqlite: %w", err)
		}
		dsn = path
	}
	db, err := sql.Open("sqlite", dsn)
	if err != nil {
		return nil, err
	}
	if _, err := db.Exec(`PRAGMA foreign_keys = ON`); err != nil {
		_ = db.Close()
		return nil, err
	}
	return db, nil
}

func NewUserRepository(db *sql.DB) (*UserRepository, error) {
	repo := &UserRepository{db: db}
	if err := repo.migrate(); err != nil {
		return nil, err
	}
	return repo, nil
}

func (r *UserRepository) migrate() error {
	_, err := r.db.Exec(`
CREATE TABLE IF NOT EXISTS users (
  id TEXT PRIMARY KEY NOT NULL,
  email TEXT NOT NULL UNIQUE COLLATE NOCASE,
  password_hash TEXT NOT NULL,
  current_level INTEGER NOT NULL DEFAULT 1
);`)
	return err
}

func (r *UserRepository) Create(user domain.User) error {
	_, err := r.db.Exec(
		`INSERT INTO users (id, email, password_hash, current_level) VALUES (?, ?, ?, ?)`,
		user.ID,
		strings.TrimSpace(user.Email),
		user.PasswordHash,
		user.CurrentLevel,
	)
	if err != nil {
		if strings.Contains(strings.ToLower(err.Error()), "unique") {
			return repositories.ErrEmailTaken
		}
		return err
	}
	return nil
}

func (r *UserRepository) GetByEmail(email string) (domain.User, error) {
	return r.scanOne(
		`SELECT id, email, password_hash, current_level FROM users WHERE email = ? COLLATE NOCASE`,
		strings.TrimSpace(email),
	)
}

func (r *UserRepository) GetByID(id string) (domain.User, error) {
	return r.scanOne(
		`SELECT id, email, password_hash, current_level FROM users WHERE id = ?`,
		id,
	)
}

func (r *UserRepository) scanOne(query string, arg any) (domain.User, error) {
	var u domain.User
	err := r.db.QueryRow(query, arg).Scan(&u.ID, &u.Email, &u.PasswordHash, &u.CurrentLevel)
	if errors.Is(err, sql.ErrNoRows) {
		return domain.User{}, repositories.ErrUserNotFound
	}
	if err != nil {
		return domain.User{}, err
	}
	return u, nil
}
