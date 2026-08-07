package crypto_test

import (
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/crypto"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

func TestBcryptHasher(t *testing.T) {
	h := crypto.NewBcryptHasher()
	hash, err := h.Hash("secreto12")
	if err != nil {
		t.Fatalf("hash: %v", err)
	}
	if err := h.Compare(hash, "secreto12"); err != nil {
		t.Fatalf("compare ok: %v", err)
	}
	if err := h.Compare(hash, "otra"); err != domain.ErrInvalidCredentials {
		t.Fatalf("expected invalid credentials, got %v", err)
	}
}
