package postgres

import (
	"database/sql"
	"errors"
	"fmt"
	"strings"
	"time"

	"github.com/jackc/pgx/v5/pgconn"
	_ "github.com/jackc/pgx/v5/stdlib"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories"
)

// UserRepository persistencia Postgres de usuarios (Cloud SQL / DSN postgres://).
type UserRepository struct {
	db *sql.DB
}

var _ repositories.UserRepository = (*UserRepository)(nil)

func OpenDB(dsn string) (*sql.DB, error) {
	db, err := sql.Open("pgx", dsn)
	if err != nil {
		return nil, err
	}
	db.SetMaxOpenConns(5)
	db.SetMaxIdleConns(2)
	db.SetConnMaxLifetime(30 * time.Minute)
	if err := db.Ping(); err != nil {
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
  email TEXT NOT NULL,
  password_hash TEXT NOT NULL,
  current_level INTEGER NOT NULL DEFAULT 1,
  life_purpose TEXT,
  urgency TEXT,
  vision_5_years TEXT,
  tech_stack TEXT
);
CREATE UNIQUE INDEX IF NOT EXISTS users_email_lower_uidx ON users (LOWER(email));
CREATE TABLE IF NOT EXISTS password_reset_tokens (
  id TEXT PRIMARY KEY NOT NULL,
  user_id TEXT NOT NULL,
  token_hash TEXT NOT NULL UNIQUE,
  expires_at TEXT NOT NULL,
  used_at TEXT,
  FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE IF NOT EXISTS user_completed_levels (
  user_id TEXT NOT NULL,
  level_id INTEGER NOT NULL,
  PRIMARY KEY (user_id, level_id),
  FOREIGN KEY(user_id) REFERENCES users(id) ON DELETE CASCADE
);`)
	return err
}

func (r *UserRepository) Create(user domain.User) error {
	_, err := r.db.Exec(
		`INSERT INTO users (
			id, email, password_hash, current_level,
			life_purpose, urgency, vision_5_years, tech_stack
		) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)`,
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
		if isUniqueViolation(err) {
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
		 FROM users WHERE LOWER(email) = LOWER($1)`,
		strings.TrimSpace(email),
	)
}

func (r *UserRepository) GetByID(id string) (domain.User, error) {
	return r.scanOne(
		`SELECT id, email, password_hash, current_level,
			life_purpose, urgency, vision_5_years, tech_stack
		 FROM users WHERE id = $1`,
		id,
	)
}

func (r *UserRepository) UpdateProfile(userID string, profile domain.LearnerProfile) error {
	res, err := r.db.Exec(
		`UPDATE users SET
			life_purpose = $1,
			urgency = $2,
			vision_5_years = $3,
			tech_stack = $4
		 WHERE id = $5`,
		strings.TrimSpace(profile.LifePurpose),
		strings.TrimSpace(profile.Urgency),
		strings.TrimSpace(profile.Vision5Years),
		strings.TrimSpace(profile.TechStack),
		userID,
	)
	if err != nil {
		return err
	}
	return rowsAffectedUser(res)
}

func (r *UserRepository) UpdateCurrentLevel(userID string, currentLevel int) error {
	res, err := r.db.Exec(
		`UPDATE users SET current_level = $1 WHERE id = $2`,
		currentLevel,
		userID,
	)
	if err != nil {
		return err
	}
	return rowsAffectedUser(res)
}

func (r *UserRepository) MarkLevelCompleted(userID string, levelID int) error {
	if levelID <= 0 {
		return fmt.Errorf("level_id inválido: %d", levelID)
	}
	if _, err := r.GetByID(userID); err != nil {
		return err
	}
	_, err := r.db.Exec(
		`INSERT INTO user_completed_levels (user_id, level_id) VALUES ($1, $2)
		 ON CONFLICT (user_id, level_id) DO NOTHING`,
		userID,
		levelID,
	)
	return err
}

func (r *UserRepository) ClearCompletedLevels(userID string) error {
	if _, err := r.GetByID(userID); err != nil {
		return err
	}
	_, err := r.db.Exec(`DELETE FROM user_completed_levels WHERE user_id = $1`, userID)
	return err
}

func (r *UserRepository) listCompletedLevels(userID string) ([]int, error) {
	rows, err := r.db.Query(
		`SELECT level_id FROM user_completed_levels WHERE user_id = $1 ORDER BY level_id ASC`,
		userID,
	)
	if err != nil {
		return nil, err
	}
	defer rows.Close()

	out := make([]int, 0)
	for rows.Next() {
		var levelID int
		if err := rows.Scan(&levelID); err != nil {
			return nil, err
		}
		out = append(out, levelID)
	}
	return out, rows.Err()
}

func (r *UserRepository) UpdatePasswordHash(userID, passwordHash string) error {
	res, err := r.db.Exec(
		`UPDATE users SET password_hash = $1 WHERE id = $2`,
		passwordHash,
		userID,
	)
	if err != nil {
		return err
	}
	return rowsAffectedUser(res)
}

func (r *UserRepository) CreatePasswordResetToken(token repositories.PasswordResetToken) error {
	_, err := r.db.Exec(
		`INSERT INTO password_reset_tokens (id, user_id, token_hash, expires_at, used_at)
		 VALUES ($1, $2, $3, $4, $5)`,
		token.ID,
		token.UserID,
		token.TokenHash,
		token.ExpiresAt.UTC().Format(time.RFC3339Nano),
		nullTime(token.UsedAt),
	)
	return err
}

func (r *UserRepository) GetPasswordResetTokenByHash(tokenHash string) (repositories.PasswordResetToken, error) {
	var tok repositories.PasswordResetToken
	var expires string
	var used sql.NullString
	err := r.db.QueryRow(
		`SELECT id, user_id, token_hash, expires_at, used_at
		 FROM password_reset_tokens WHERE token_hash = $1`,
		tokenHash,
	).Scan(&tok.ID, &tok.UserID, &tok.TokenHash, &expires, &used)
	if errors.Is(err, sql.ErrNoRows) {
		return repositories.PasswordResetToken{}, repositories.ErrResetTokenNotFound
	}
	if err != nil {
		return repositories.PasswordResetToken{}, err
	}
	exp, err := time.Parse(time.RFC3339Nano, expires)
	if err != nil {
		exp, err = time.Parse(time.RFC3339, expires)
		if err != nil {
			return repositories.PasswordResetToken{}, fmt.Errorf("parse expires_at: %w", err)
		}
	}
	tok.ExpiresAt = exp
	if used.Valid && strings.TrimSpace(used.String) != "" {
		u, err := time.Parse(time.RFC3339Nano, used.String)
		if err != nil {
			u, err = time.Parse(time.RFC3339, used.String)
			if err != nil {
				return repositories.PasswordResetToken{}, fmt.Errorf("parse used_at: %w", err)
			}
		}
		tok.UsedAt = &u
	}
	return tok, nil
}

func (r *UserRepository) MarkPasswordResetTokenUsed(id string, usedAt time.Time) error {
	res, err := r.db.Exec(
		`UPDATE password_reset_tokens SET used_at = $1 WHERE id = $2 AND used_at IS NULL`,
		usedAt.UTC().Format(time.RFC3339Nano),
		id,
	)
	if err != nil {
		return err
	}
	n, err := res.RowsAffected()
	if err != nil {
		return err
	}
	if n == 0 {
		return repositories.ErrResetTokenNotFound
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
	completed, err := r.listCompletedLevels(u.ID)
	if err != nil {
		return domain.User{}, err
	}
	u.CompletedLevels = completed
	return u, nil
}

func rowsAffectedUser(res sql.Result) error {
	n, err := res.RowsAffected()
	if err != nil {
		return err
	}
	if n == 0 {
		return repositories.ErrUserNotFound
	}
	return nil
}

func isUniqueViolation(err error) bool {
	var pgErr *pgconn.PgError
	return errors.As(err, &pgErr) && pgErr.Code == "23505"
}

func nullTime(t *time.Time) any {
	if t == nil {
		return nil
	}
	return t.UTC().Format(time.RFC3339Nano)
}

func nullIfEmpty(s string) any {
	s = strings.TrimSpace(s)
	if s == "" {
		return nil
	}
	return s
}
