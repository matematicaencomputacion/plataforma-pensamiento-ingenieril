package usecases

import (
	"context"
	"crypto/rand"
	"crypto/sha256"
	"encoding/hex"
	"errors"
	"net/mail"
	"os"
	"strings"
	"time"
	"unicode/utf8"

	"github.com/google/uuid"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories"
)

const minPasswordLen = 8

const defaultResetTTL = time.Hour

// AuthOptions tunables for password reset DX and TTL.
type AuthOptions struct {
	ExposeResetToken bool
	ResetTTL         time.Duration
}

// AuthService registro, login, recovery y usuario actual.
type AuthService struct {
	users            repositories.UserRepository
	hasher           domain.PasswordHasher
	tokens           domain.TokenIssuer
	exposeResetToken bool
	resetTTL         time.Duration
	now              func() time.Time
}

func NewAuthService(
	users repositories.UserRepository,
	hasher domain.PasswordHasher,
	tokens domain.TokenIssuer,
	opts ...AuthOptions,
) *AuthService {
	o := AuthOptions{}
	if len(opts) > 0 {
		o = opts[0]
	}
	ttl := o.ResetTTL
	if ttl <= 0 {
		ttl = defaultResetTTL
	}
	return &AuthService{
		users:            users,
		hasher:           hasher,
		tokens:           tokens,
		exposeResetToken: o.ExposeResetToken,
		resetTTL:         ttl,
		now:              time.Now,
	}
}

// ResolveExposeResetToken decides DX token revelation from env + JWT secret.
//
// Priority:
//  1. PPI_EXPOSE_RESET_TOKEN explicit (1/true → on, 0/false → off)
//  2. ENV / APP_ENV / GO_ENV ∈ {development,dev,local} → on
//  3. Known local/CI JWT secrets → on
//  4. otherwise off
func ResolveExposeResetToken(jwtSecret string) bool {
	if v := strings.TrimSpace(os.Getenv("PPI_EXPOSE_RESET_TOKEN")); v != "" {
		return v == "1" || strings.EqualFold(v, "true")
	}
	if isLocalDevEnv() {
		return true
	}
	switch strings.TrimSpace(jwtSecret) {
	case "dev-only-change-me-ppi-jwt-secret", "harness-jwt-secret", "test-secret", "ci-e2e-only-ppi-jwt-secret":
		return true
	default:
		return false
	}
}

func isLocalDevEnv() bool {
	for _, key := range []string{"ENV", "APP_ENV", "GO_ENV"} {
		v := strings.ToLower(strings.TrimSpace(os.Getenv(key)))
		switch v {
		case "development", "dev", "local":
			return true
		}
	}
	return false
}

type AuthResult struct {
	User  domain.PublicUser
	Token string
}

// ForgotPasswordResult is intentionally generic; ResetToken is opt-in for DX.
type ForgotPasswordResult struct {
	Message    string
	ResetToken string
}

func (s *AuthService) Register(_ context.Context, email, password string) (AuthResult, error) {
	email, err := normalizeEmail(email)
	if err != nil {
		return AuthResult{}, err
	}
	if err := validatePassword(password); err != nil {
		return AuthResult{}, err
	}

	hash, err := s.hasher.Hash(password)
	if err != nil {
		return AuthResult{}, err
	}

	user := domain.User{
		ID:           uuid.NewString(),
		Email:        email,
		PasswordHash: hash,
		CurrentLevel: 1,
	}
	if err := s.users.Create(user); err != nil {
		return AuthResult{}, err
	}

	token, err := s.tokens.Issue(user.ID, user.Email)
	if err != nil {
		return AuthResult{}, err
	}
	return AuthResult{User: user.ToPublic(), Token: token}, nil
}

func (s *AuthService) Login(_ context.Context, email, password string) (AuthResult, error) {
	emailNorm, err := normalizeEmail(email)
	if err != nil {
		return AuthResult{}, domain.ErrInvalidCredentials
	}
	user, err := s.users.GetByEmail(emailNorm)
	if err != nil {
		return AuthResult{}, domain.ErrInvalidCredentials
	}
	if err := s.hasher.Compare(user.PasswordHash, password); err != nil {
		return AuthResult{}, domain.ErrInvalidCredentials
	}
	token, err := s.tokens.Issue(user.ID, user.Email)
	if err != nil {
		return AuthResult{}, err
	}
	return AuthResult{User: user.ToPublic(), Token: token}, nil
}

func (s *AuthService) Me(_ context.Context, bearerToken string) (domain.PublicUser, error) {
	userID, _, err := s.tokens.Parse(bearerToken)
	if err != nil {
		return domain.PublicUser{}, domain.ErrUnauthorized
	}
	user, err := s.users.GetByID(userID)
	if err != nil {
		return domain.PublicUser{}, domain.ErrUnauthorized
	}
	return user.ToPublic(), nil
}

// ForgotPassword always returns a generic message; may include reset token for DX.
func (s *AuthService) ForgotPassword(_ context.Context, email string) (ForgotPasswordResult, error) {
	const generic = "Si el correo está registrado, recibirás instrucciones para restablecer la contraseña."
	out := ForgotPasswordResult{Message: generic}

	emailNorm, err := normalizeEmail(email)
	if err != nil {
		// Same message — do not leak validation nuance beyond format for empty/invalid.
		return out, nil
	}

	user, err := s.users.GetByEmail(emailNorm)
	if err != nil {
		return out, nil
	}

	raw, err := randomToken(32)
	if err != nil {
		return ForgotPasswordResult{}, err
	}
	hash := hashToken(raw)
	tok := repositories.PasswordResetToken{
		ID:        uuid.NewString(),
		UserID:    user.ID,
		TokenHash: hash,
		ExpiresAt: s.now().UTC().Add(s.resetTTL),
	}
	if err := s.users.CreatePasswordResetToken(tok); err != nil {
		return ForgotPasswordResult{}, err
	}
	if s.exposeResetToken {
		out.ResetToken = raw
	}
	return out, nil
}

// ResetPassword validates a reset token and replaces the password hash.
func (s *AuthService) ResetPassword(_ context.Context, rawToken, password string) (AuthResult, error) {
	rawToken = strings.TrimSpace(rawToken)
	if rawToken == "" {
		return AuthResult{}, domain.ErrInvalidResetToken
	}
	if err := validatePassword(password); err != nil {
		return AuthResult{}, err
	}

	stored, err := s.users.GetPasswordResetTokenByHash(hashToken(rawToken))
	if err != nil {
		if errors.Is(err, repositories.ErrResetTokenNotFound) {
			return AuthResult{}, domain.ErrInvalidResetToken
		}
		return AuthResult{}, err
	}
	if stored.UsedAt != nil || !s.now().UTC().Before(stored.ExpiresAt) {
		return AuthResult{}, domain.ErrInvalidResetToken
	}

	hash, err := s.hasher.Hash(password)
	if err != nil {
		return AuthResult{}, err
	}
	if err := s.users.UpdatePasswordHash(stored.UserID, hash); err != nil {
		if errors.Is(err, repositories.ErrUserNotFound) {
			return AuthResult{}, domain.ErrInvalidResetToken
		}
		return AuthResult{}, err
	}
	if err := s.users.MarkPasswordResetTokenUsed(stored.ID, s.now().UTC()); err != nil {
		return AuthResult{}, err
	}

	user, err := s.users.GetByID(stored.UserID)
	if err != nil {
		return AuthResult{}, err
	}
	token, err := s.tokens.Issue(user.ID, user.Email)
	if err != nil {
		return AuthResult{}, err
	}
	return AuthResult{User: user.ToPublic(), Token: token}, nil
}

// GetProfile devuelve el coaching persistido del usuario autenticado (puede estar vacío).
func (s *AuthService) GetProfile(_ context.Context, bearerToken string) (domain.LearnerProfile, error) {
	userID, _, err := s.tokens.Parse(bearerToken)
	if err != nil {
		return domain.LearnerProfile{}, domain.ErrUnauthorized
	}
	user, err := s.users.GetByID(userID)
	if err != nil {
		return domain.LearnerProfile{}, domain.ErrUnauthorized
	}
	return user.Profile, nil
}

// UpdateProfile persiste el coaching de onboarding del usuario autenticado.
func (s *AuthService) UpdateProfile(
	_ context.Context,
	bearerToken string,
	profile domain.LearnerProfile,
) (domain.LearnerProfile, error) {
	userID, _, err := s.tokens.Parse(bearerToken)
	if err != nil {
		return domain.LearnerProfile{}, domain.ErrUnauthorized
	}

	normalized := domain.LearnerProfile{
		LifePurpose:  strings.TrimSpace(profile.LifePurpose),
		Urgency:      strings.TrimSpace(profile.Urgency),
		Vision5Years: strings.TrimSpace(profile.Vision5Years),
		TechStack:    strings.TrimSpace(profile.TechStack),
	}
	if normalized.IsEmpty() {
		return domain.LearnerProfile{}, domain.ErrEmptyProfile
	}

	if err := s.users.UpdateProfile(userID, normalized); err != nil {
		if errors.Is(err, repositories.ErrUserNotFound) {
			return domain.LearnerProfile{}, domain.ErrUnauthorized
		}
		return domain.LearnerProfile{}, err
	}
	return normalized, nil
}

func normalizeEmail(email string) (string, error) {
	email = strings.TrimSpace(strings.ToLower(email))
	if email == "" {
		return "", domain.ErrInvalidEmail
	}
	addr, err := mail.ParseAddress(email)
	if err != nil || addr.Address != email {
		return "", domain.ErrInvalidEmail
	}
	return email, nil
}

func validatePassword(password string) error {
	if utf8.RuneCountInString(password) < minPasswordLen {
		return domain.ErrInvalidPassword
	}
	return nil
}

func randomToken(nBytes int) (string, error) {
	buf := make([]byte, nBytes)
	if _, err := rand.Read(buf); err != nil {
		return "", err
	}
	return hex.EncodeToString(buf), nil
}

func hashToken(raw string) string {
	sum := sha256.Sum256([]byte(raw))
	return hex.EncodeToString(sum[:])
}
