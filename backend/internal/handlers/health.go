package handlers

import (
	"encoding/json"
	"log"
	"net/http"
)

type healthResponse struct {
	Status  string `json:"status"`
	Message string `json:"message"`
}

// Health responde con el estado de disponibilidad de la API.
func Health(w http.ResponseWriter, r *http.Request) {
	response := healthResponse{
		Status:  "ok",
		Message: "Plataforma Educativa API funcionando",
	}

	payload, err := json.Marshal(response)
	if err != nil {
		http.Error(w, "error al codificar la respuesta", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)

	if _, err := w.Write(payload); err != nil {
		log.Printf("error al escribir la respuesta de health: %v", err)
	}
}
