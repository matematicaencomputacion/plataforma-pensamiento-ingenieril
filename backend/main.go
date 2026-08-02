package main

import (
	"log"
	"net/http"

	"github.com/tu-usuario/plataforma-edu-backend/internal/handlers"
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

func main() {
	evaluationService := usecases.NewEvaluationService()
	evaluateHandler := handlers.NewEvaluateHandler(evaluationService)

	mux := http.NewServeMux()
	mux.HandleFunc("GET /api/health", handlers.Health)
	mux.HandleFunc("POST /api/evaluate", evaluateHandler.Evaluate)

	addr := ":8080"
	log.Printf("servidor iniciado: escuchando en http://localhost%s", addr)
	if err := http.ListenAndServe(addr, enableCORS(mux)); err != nil {
		log.Fatalf("error al iniciar el servidor: %v", err)
	}
}
