package jwtauth_test

import (
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/jwtauth"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

func TestHS256Issuer(t *testing.T) {
	iss := jwtauth.NewHS256Issuer("secret")
	tok, err := iss.Issue("uid-1", "a@b.c")
	if err != nil {
		t.Fatalf("issue: %v", err)
	}
	uid, email, err := iss.Parse(tok)
	if err != nil {
		t.Fatalf("parse: %v", err)
	}
	if uid != "uid-1" || email != "a@b.c" {
		t.Fatalf("got %s %s", uid, email)
	}
	if _, _, err := iss.Parse("not-a-token"); err != domain.ErrUnauthorized {
		t.Fatalf("expected unauthorized, got %v", err)
	}
}
