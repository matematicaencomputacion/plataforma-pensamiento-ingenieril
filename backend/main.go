package main

import (
	"context"
	"log"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/crypto"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/groq"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/jwtauth"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/keyword"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/config"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/handlers"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories/jsonstore"
	sqliterepo "github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories/sqlite"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/usecases"
)

func enableCORS(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Methods", "POST, GET, PUT, OPTIONS")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type, Authorization")

		if r.Method == http.MethodOptions {
			w.WriteHeader(http.StatusOK)
			return
		}

		next.ServeHTTP(w, r)
	})
}

func resolveDataDir() string {
	if dir := os.Getenv("DATA_DIR"); dir != "" {
		return dir
	}

	candidates := []string{
		"data",
		filepath.Join("backend", "data"),
	}
	for _, candidate := range candidates {
		if info, err := os.Stat(candidate); err == nil && info.IsDir() {
			return candidate
		}
	}

	return "data"
}

func newProfileClassifier(ctx context.Context, groqCfg config.GroqConfig) (domain.ProfileClassifier, string) {
	mode := strings.ToLower(strings.TrimSpace(os.Getenv("LEARNER_PROFILE_LLM")))
	if mode == "" {
		mode = "groq"
	}
	if mode == "mock" || mode == "keyword" {
		log.Printf("clasificador de perfil: mock/keywords")
		return keyword.NewClassifier(), "mock"
	}

	classifier, err := groq.NewClassifier(ctx, groq.Config{
		APIKey:  groqCfg.APIKey,
		Model:   groqCfg.Model,
		BaseURL: groqCfg.BaseURL,
	})
	if err != nil {
		log.Printf("WARN: no se pudo iniciar Groq (%v); usando mock keywords", err)
		return keyword.NewClassifier(), "mock-fallback"
	}
	log.Printf(
		"clasificador de perfil: groq model=%s",
		firstNonEmpty(groqCfg.Model, "llama-3.1-8b-instant"),
	)
	return classifier, "groq"
}

func firstNonEmpty(values ...string) string {
	for _, v := range values {
		if strings.TrimSpace(v) != "" {
			return strings.TrimSpace(v)
		}
	}
	return ""
}

func main() {
	repoRoot := config.ResolveMonorepoRoot()
	if err := config.LoadDotEnv(filepath.Join(repoRoot, ".env")); err != nil {
		log.Printf("WARN: no se pudo cargar .env: %v", err)
	}
	config.ResolveCredentialsPath(repoRoot)

	dataDir := resolveDataDir()
	authCfg := config.LoadAuthConfig()
	groqCfg := config.LoadGroqConfig()

	sqlitePath := authCfg.SQLitePath()
	if !filepath.IsAbs(sqlitePath) && sqlitePath != ":memory:" {
		sqlitePath = filepath.Join(repoRoot, sqlitePath)
	}
	userDB, err := sqliterepo.OpenDB(sqlitePath)
	if err != nil {
		log.Fatalf("sqlite: %v", err)
	}
	defer userDB.Close()
	userRepo, err := sqliterepo.NewUserRepository(userDB)
	if err != nil {
		log.Fatalf("user repo: %v", err)
	}
	authService := usecases.NewAuthService(
		userRepo,
		crypto.NewBcryptHasher(),
		jwtauth.NewHS256Issuer(authCfg.JWTSecret),
	)
	authHandler := handlers.NewAuthHandler(authService)

	levelRepo := jsonstore.NewLevelRepository(jsonstore.DefaultLevelsPath(dataDir))
	profileRepo := jsonstore.NewCognitiveProfileRepository(jsonstore.DefaultCognitiveProfilesPath(dataDir))

	levelService := usecases.NewLevelService(levelRepo)
	evaluationService := usecases.NewEvaluationService(levelRepo, profileRepo)

	classifier, _ := newProfileClassifier(context.Background(), groqCfg)
	learnerProfileService := usecases.NewLearnerProfileService(classifier)

	levelHandler := handlers.NewLevelHandler(levelService)
	evaluateHandler := handlers.NewEvaluateHandler(evaluationService)
	learnerProfileHandler := handlers.NewLearnerProfileHandler(learnerProfileService)

	mux := http.NewServeMux()
	mux.HandleFunc("GET /api/health", handlers.Health)
	mux.HandleFunc("POST /api/auth/register", authHandler.Register)
	mux.HandleFunc("POST /api/auth/login", authHandler.Login)
	mux.HandleFunc("POST /api/auth/logout", authHandler.Logout)
	mux.HandleFunc("GET /api/me", authHandler.Me)
	mux.HandleFunc("GET /api/user/profile", authHandler.GetProfile)
	mux.HandleFunc("PUT /api/user/profile", authHandler.UpdateProfile)
	mux.HandleFunc("GET /api/levels/current", levelHandler.GetCurrent)
	mux.HandleFunc("GET /api/levels/{id}", levelHandler.GetByID)
	mux.HandleFunc("POST /api/evaluate", evaluateHandler.Evaluate)
	mux.HandleFunc("POST /api/learner/profile/synthesize", learnerProfileHandler.Synthesize)

	addr := ":8080"
	log.Printf(
		"servidor iniciado: escuchando en http://localhost%s (data=%s root=%s sqlite=%s)",
		addr, dataDir, repoRoot, sqlitePath,
	)
	if err := http.ListenAndServe(addr, enableCORS(mux)); err != nil {
		log.Fatalf("error al iniciar el servidor: %v", err)
	}
}
