package jwtauth

import (
	"fmt"
	"time"

	"github.com/golang-jwt/jwt/v5"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

const defaultTTL = 24 * time.Hour

type Claims struct {
	Email string `json:"email"`
	jwt.RegisteredClaims
}

// HS256Issuer emite y valida JWT HS256.
type HS256Issuer struct {
	secret []byte
	ttl    time.Duration
}

func NewHS256Issuer(secret string) *HS256Issuer {
	return &HS256Issuer{
		secret: []byte(secret),
		ttl:    defaultTTL,
	}
}

func (i *HS256Issuer) Issue(userID, email string) (string, error) {
	now := time.Now()
	claims := Claims{
		Email: email,
		RegisteredClaims: jwt.RegisteredClaims{
			Subject:   userID,
			IssuedAt:  jwt.NewNumericDate(now),
			ExpiresAt: jwt.NewNumericDate(now.Add(i.ttl)),
		},
	}
	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	return token.SignedString(i.secret)
}

func (i *HS256Issuer) Parse(tokenStr string) (userID string, email string, err error) {
	token, err := jwt.ParseWithClaims(tokenStr, &Claims{}, func(t *jwt.Token) (any, error) {
		if t.Method != jwt.SigningMethodHS256 {
			return nil, fmt.Errorf("alg unexpected: %v", t.Header["alg"])
		}
		return i.secret, nil
	})
	if err != nil || !token.Valid {
		return "", "", domain.ErrUnauthorized
	}
	claims, ok := token.Claims.(*Claims)
	if !ok || claims.Subject == "" {
		return "", "", domain.ErrUnauthorized
	}
	return claims.Subject, claims.Email, nil
}
