package usecases

import (
	"context"
	"net/mail"
	"strings"
	"unicode/utf8"

	"github.com/google/uuid"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories"
)

const minPasswordLen = 8

// AuthService registro, login y usuario actual.
type AuthService struct {
	users  repositories.UserRepository
	hasher domain.PasswordHasher
	tokens domain.TokenIssuer
}

func NewAuthService(
	users repositories.UserRepository,
	hasher domain.PasswordHasher,
	tokens domain.TokenIssuer,
) *AuthService {
	return &AuthService{users: users, hasher: hasher, tokens: tokens}
}

type AuthResult struct {
	User  domain.PublicUser
	Token string
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
