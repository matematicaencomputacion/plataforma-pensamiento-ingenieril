package main

import (
	"log"
	"net/http"
	"os"
	"path/filepath"

	"github.com/tu-usuario/plataforma-edu-backend/internal/handlers"
	"github.com/tu-usuario/plataforma-edu-backend/internal/repositories/jsonstore"
	"github.com/tu-usuario/plataforma-edu-backend/internal/usecases"
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

func main() {
	dataDir := resolveDataDir()
	levelRepo := jsonstore.NewLevelRepository(jsonstore.DefaultLevelsPath(dataDir))
	profileRepo := jsonstore.NewCognitiveProfileRepository(jsonstore.DefaultCognitiveProfilesPath(dataDir))
	curriculumRepo := jsonstore.NewCurriculumRepository(jsonstore.DefaultCurriculumPath(dataDir))

	levelService := usecases.NewLevelService(levelRepo)
	evaluationService := usecases.NewEvaluationService(levelRepo, profileRepo)
	curriculumService := usecases.NewCurriculumService(curriculumRepo, profileRepo)

	levelHandler := handlers.NewLevelHandler(levelService)
	evaluateHandler := handlers.NewEvaluateHandler(evaluationService)
	curriculumHandler := handlers.NewCurriculumHandler(curriculumService)

	mux := http.NewServeMux()
	mux.HandleFunc("GET /api/health", handlers.Health)
	mux.HandleFunc("GET /api/levels/current", levelHandler.GetCurrent)
	mux.HandleFunc("GET /api/levels/{id}", levelHandler.GetByID)
	mux.HandleFunc("GET /api/curriculum", curriculumHandler.List)
	mux.HandleFunc("GET /api/curriculum/lessons/{id}", curriculumHandler.GetLesson)
	mux.HandleFunc("POST /api/evaluate", evaluateHandler.Evaluate)

	addr := ":8080"
	log.Printf("servidor iniciado: escuchando en http://localhost%s (data=%s)", addr, dataDir)
	if err := http.ListenAndServe(addr, enableCORS(mux)); err != nil {
		log.Fatalf("error al iniciar el servidor: %v", err)
	}
}
