package crypto

import (
	"golang.org/x/crypto/bcrypt"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

const defaultCost = 12

// BcryptHasher implementa domain.PasswordHasher.
//
// Each Hash() call embeds a fresh random salt inside the `$2a$…` string; the
// salt is not derived from process state, so hashes remain comparable across
// server restarts with CompareHashAndPassword.
type BcryptHasher struct {
	cost int
}

func NewBcryptHasher() *BcryptHasher {
	return &BcryptHasher{cost: defaultCost}
}

func (h *BcryptHasher) Hash(plain string) (string, error) {
	hashed, err := bcrypt.GenerateFromPassword([]byte(plain), h.cost)
	if err != nil {
		return "", err
	}
	return string(hashed), nil
}

func (h *BcryptHasher) Compare(hash, plain string) error {
	err := bcrypt.CompareHashAndPassword([]byte(hash), []byte(plain))
	if err != nil {
		return domain.ErrInvalidCredentials
	}
	return nil
}
