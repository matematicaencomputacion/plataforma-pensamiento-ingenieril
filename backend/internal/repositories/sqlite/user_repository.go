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
  current_level INTEGER NOT NULL DEFAULT 1,
  life_purpose TEXT,
  urgency TEXT,
  vision_5_years TEXT,
  tech_stack TEXT
);`)
	if err != nil {
		return err
	}
	return r.ensureProfileColumns()
}

func (r *UserRepository) ensureProfileColumns() error {
	cols, err := r.columnNames("users")
	if err != nil {
		return err
	}
	needed := []string{
		"life_purpose",
		"urgency",
		"vision_5_years",
		"tech_stack",
	}
	for _, col := range needed {
		if cols[col] {
			continue
		}
		if _, err := r.db.Exec(`ALTER TABLE users ADD COLUMN ` + col + ` TEXT`); err != nil {
			return fmt.Errorf("alter users add %s: %w", col, err)
		}
	}
	return nil
}

func (r *UserRepository) columnNames(table string) (map[string]bool, error) {
	rows, err := r.db.Query(`PRAGMA table_info(` + table + `)`)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	out := make(map[string]bool)
	for rows.Next() {
		var cid int
		var name, ctype string
		var notnull, pk int
		var dflt sql.NullString
		if err := rows.Scan(&cid, &name, &ctype, &notnull, &dflt, &pk); err != nil {
			return nil, err
		}
		out[name] = true
	}
	return out, rows.Err()
}

func (r *UserRepository) Create(user domain.User) error {
	_, err := r.db.Exec(
		`INSERT INTO users (
			id, email, password_hash, current_level,
			life_purpose, urgency, vision_5_years, tech_stack
		) VALUES (?, ?, ?, ?, ?, ?, ?, ?)`,
		user.ID,
		strings.TrimSpace(user.Email),
		user.PasswordHash,
		user.CurrentLevel,
		nullIfEmpty(user.Profile.LifePurpose),
		nullIfEmpty(user.Profile.Urgency),
		nullIfEmpty(user.Profile.Vision5Years),
		nullIfEmpty(user.Profile.TechStack),
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
		`SELECT id, email, password_hash, current_level,
			life_purpose, urgency, vision_5_years, tech_stack
		 FROM users WHERE email = ? COLLATE NOCASE`,
		strings.TrimSpace(email),
	)
}

func (r *UserRepository) GetByID(id string) (domain.User, error) {
	return r.scanOne(
		`SELECT id, email, password_hash, current_level,
			life_purpose, urgency, vision_5_years, tech_stack
		 FROM users WHERE id = ?`,
		id,
	)
}

func (r *UserRepository) UpdateProfile(userID string, profile domain.LearnerProfile) error {
	res, err := r.db.Exec(
		`UPDATE users SET
			life_purpose = ?,
			urgency = ?,
			vision_5_years = ?,
			tech_stack = ?
		 WHERE id = ?`,
		strings.TrimSpace(profile.LifePurpose),
		strings.TrimSpace(profile.Urgency),
		strings.TrimSpace(profile.Vision5Years),
		strings.TrimSpace(profile.TechStack),
		userID,
	)
	if err != nil {
		return err
	}
	n, err := res.RowsAffected()
	if err != nil {
		return err
	}
	if n == 0 {
		return repositories.ErrUserNotFound
	}
	return nil
}

func (r *UserRepository) scanOne(query string, arg any) (domain.User, error) {
	var u domain.User
	var life, urgency, vision, stack sql.NullString
	err := r.db.QueryRow(query, arg).Scan(
		&u.ID,
		&u.Email,
		&u.PasswordHash,
		&u.CurrentLevel,
		&life,
		&urgency,
		&vision,
		&stack,
	)
	if errors.Is(err, sql.ErrNoRows) {
		return domain.User{}, repositories.ErrUserNotFound
	}
	if err != nil {
		return domain.User{}, err
	}
	u.Profile = domain.LearnerProfile{
		LifePurpose:  life.String,
		Urgency:      urgency.String,
		Vision5Years: vision.String,
		TechStack:    stack.String,
	}
	return u, nil
}

func nullIfEmpty(s string) any {
	s = strings.TrimSpace(s)
	if s == "" {
		return nil
	}
	return s
}
