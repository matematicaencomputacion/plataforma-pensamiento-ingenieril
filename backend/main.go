package main

import (
	"log"
	"net/http"

	"github.com/tu-usuario/plataforma-edu-backend/internal/handlers"
)

func main() {
	mux := http.NewServeMux()
	mux.HandleFunc("GET /api/health", handlers.Health)

	addr := ":8080"
	log.Printf("servidor iniciado: escuchando en http://localhost%s", addr)
	if err := http.ListenAndServe(addr, mux); err != nil {
		log.Fatalf("error al iniciar el servidor: %v", err)
	}
}
