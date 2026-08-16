// Command ppi-authctl: local/dev auth maintenance against the same SQLite as ppi-api.
//
// Example:
//
//	cd backend && go run ./cmd/ppi-authctl set-password \
//	  -email=alum@example.com -password=secreto12
//
// Honours JWT_SECRET / DATABASE_URL via config.LoadAuthConfig (defaults to data/ppi.db).
package main

import (
	"flag"
	"fmt"
	"log"
	"os"
	"path/filepath"
	"strings"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/crypto"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/config"
	sqliterepo "github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories/sqlite"
)

func main() {
	log.SetFlags(0)
	if len(os.Args) < 2 {
		usage()
		os.Exit(2)
	}
	switch os.Args[1] {
	case "set-password":
		if err := setPassword(os.Args[2:]); err != nil {
			log.Fatalf("set-password: %v", err)
		}
	case "help", "-h", "--help":
		usage()
	default:
		log.Printf("comando desconocido: %s", os.Args[1])
		usage()
		os.Exit(2)
	}
}

func usage() {
	fmt.Fprintf(os.Stderr, `ppi-authctl — herramientas de auth para desarrollo local

Usage:
  ppi-authctl set-password -email=USER -password=PASS [-db=sqlite://./data/ppi.db]

Environment (same as ppi-api):
  DATABASE_URL   default sqlite://./data/ppi.db
  JWT_SECRET     unused for hashing; bcrypt cost is fixed in adapters/crypto

`)
}

func setPassword(args []string) error {
	fs := flag.NewFlagSet("set-password", flag.ContinueOnError)
	email := fs.String("email", "", "correo del usuario (requerido)")
	password := fs.String("password", "", "nueva contraseña (mín. 8)")
	dbURL := fs.String("db", "", "override DATABASE_URL (sqlite://...)")
	if err := fs.Parse(args); err != nil {
		return err
	}
	*email = strings.TrimSpace(strings.ToLower(*email))
	if *email == "" || *password == "" {
		return fmt.Errorf("se requieren -email y -password")
	}
	if len(*password) < 8 {
		return fmt.Errorf("password debe tener al menos 8 caracteres")
	}

	repoRoot := config.ResolveMonorepoRoot()
	_ = config.LoadDotEnv(filepath.Join(repoRoot, ".env"))
	authCfg, err := config.LoadAuthConfig()
	if err != nil {
		return fmt.Errorf("auth config: %w", err)
	}
	if strings.TrimSpace(*dbURL) != "" {
		authCfg.DatabaseURL = strings.TrimSpace(*dbURL)
	}

	sqlitePath := authCfg.SQLitePath()
	if !filepath.IsAbs(sqlitePath) && sqlitePath != ":memory:" {
		sqlitePath = filepath.Join(repoRoot, sqlitePath)
	}

	db, err := sqliterepo.OpenDB(sqlitePath)
	if err != nil {
		return fmt.Errorf("open %s: %w", sqlitePath, err)
	}
	defer db.Close()

	repo, err := sqliterepo.NewUserRepository(db)
	if err != nil {
		return err
	}
	user, err := repo.GetByEmail(*email)
	if err != nil {
		return fmt.Errorf("usuario %q no encontrado en %s: %w", *email, sqlitePath, err)
	}

	hasher := crypto.NewBcryptHasher()
	hash, err := hasher.Hash(*password)
	if err != nil {
		return err
	}
	if err := repo.UpdatePasswordHash(user.ID, hash); err != nil {
		return err
	}
	// Verify round-trip with the same hasher instance semantics.
	if err := hasher.Compare(hash, *password); err != nil {
		return fmt.Errorf("verificación bcrypt falló tras escribir: %w", err)
	}

	fmt.Printf("OK: password actualizada para %s (sqlite=%s)\n", user.Email, sqlitePath)
	return nil
}
