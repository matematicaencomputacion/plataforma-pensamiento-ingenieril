package main

import (
	"context"
	"log"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/gemini"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/keyword"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/config"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/handlers"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories/jsonstore"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/usecases"
)

func enableCORS(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Methods", "POST, GET, OPTIONS")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type")

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

func newProfileClassifier(ctx context.Context) (domain.ProfileClassifier, string) {
	mode := strings.ToLower(strings.TrimSpace(os.Getenv("LEARNER_PROFILE_LLM")))
	if mode == "" {
		mode = "gemini"
	}
	if mode == "mock" || mode == "keyword" {
		log.Printf("clasificador de perfil: mock/keywords")
		return keyword.NewClassifier(), "mock"
	}

	classifier, err := gemini.NewClassifier(ctx, gemini.Config{
		Project:  os.Getenv("GOOGLE_CLOUD_PROJECT"),
		Location: os.Getenv("VERTEX_LOCATION"),
		Model:    os.Getenv("GEMINI_MODEL"),
		Backend:  os.Getenv("LEARNER_PROFILE_BACKEND"),
		APIKey:   os.Getenv("GEMINI_API_KEY"),
	})
	if err != nil {
		log.Printf("WARN: no se pudo iniciar Gemini (%v); usando mock keywords", err)
		return keyword.NewClassifier(), "mock-fallback"
	}
	log.Printf(
		"clasificador de perfil: gemini model=%s backend=%s project=%s",
		firstNonEmpty(os.Getenv("GEMINI_MODEL"), "gemini-2.0-pro"),
		firstNonEmpty(os.Getenv("LEARNER_PROFILE_BACKEND"), "auto"),
		os.Getenv("GOOGLE_CLOUD_PROJECT"),
	)
	return classifier, "gemini"
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
	levelRepo := jsonstore.NewLevelRepository(jsonstore.DefaultLevelsPath(dataDir))
	profileRepo := jsonstore.NewCognitiveProfileRepository(jsonstore.DefaultCognitiveProfilesPath(dataDir))

	levelService := usecases.NewLevelService(levelRepo)
	evaluationService := usecases.NewEvaluationService(levelRepo, profileRepo)

	classifier, _ := newProfileClassifier(context.Background())
	learnerProfileService := usecases.NewLearnerProfileService(classifier)

	levelHandler := handlers.NewLevelHandler(levelService)
	evaluateHandler := handlers.NewEvaluateHandler(evaluationService)
	learnerProfileHandler := handlers.NewLearnerProfileHandler(learnerProfileService)

	mux := http.NewServeMux()
	mux.HandleFunc("GET /api/health", handlers.Health)
	mux.HandleFunc("GET /api/levels/current", levelHandler.GetCurrent)
	mux.HandleFunc("GET /api/levels/{id}", levelHandler.GetByID)
	mux.HandleFunc("POST /api/evaluate", evaluateHandler.Evaluate)
	mux.HandleFunc("POST /api/learner/profile/synthesize", learnerProfileHandler.Synthesize)

	addr := ":8080"
	log.Printf("servidor iniciado: escuchando en http://localhost%s (data=%s root=%s)", addr, dataDir, repoRoot)
	if err := http.ListenAndServe(addr, enableCORS(mux)); err != nil {
		log.Fatalf("error al iniciar el servidor: %v", err)
	}
}
