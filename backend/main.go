package main

import (
	"log"
	"net/http"

	"github.com/tu-usuario/plataforma-edu-backend/internal/handlers"
	"github.com/tu-usuario/plataforma-edu-backend/internal/usecases"
)

func main() {
	evaluationService := usecases.NewEvaluationService()
	evaluateHandler := handlers.NewEvaluateHandler(evaluationService)

	mux := http.NewServeMux()
	mux.HandleFunc("GET /api/health", handlers.Health)
	mux.HandleFunc("POST /api/evaluate", evaluateHandler.Evaluate)

	addr := ":8080"
	log.Printf("servidor iniciado: escuchando en http://localhost%s", addr)
	if err := http.ListenAndServe(addr, mux); err != nil {
		log.Fatalf("error al iniciar el servidor: %v", err)
	}
}
