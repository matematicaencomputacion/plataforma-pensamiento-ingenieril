package handlers

import (
	"encoding/json"
	"log"
	"net/http"

	"github.com/tu-usuario/plataforma-edu-backend/internal/usecases"
)

type evaluateRequest struct {
	Code    string `json:"code"`
	LevelID int    `json:"level_id"`
}

type evaluateResponse struct {
	Passed bool `json:"passed"`
}

// EvaluateHandler expone el caso de uso de evaluación vía HTTP.
type EvaluateHandler struct {
	service *usecases.EvaluationService
}

// NewEvaluateHandler inyecta el servicio de evaluación en el handler.
func NewEvaluateHandler(service *usecases.EvaluationService) *EvaluateHandler {
	return &EvaluateHandler{service: service}
}

// Evaluate procesa POST /api/evaluate.
func (h *EvaluateHandler) Evaluate(w http.ResponseWriter, r *http.Request) {
	var req evaluateRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "JSON de entrada inválido", http.StatusBadRequest)
		return
	}

	passed, err := h.service.EvaluateCode(req.Code, req.LevelID)
	if err != nil {
		log.Printf("Error detallado en EvaluateCode: %v", err)
		http.Error(w, "error al evaluar el código", http.StatusInternalServerError)
		return
	}

	response := evaluateResponse{Passed: passed}
	payload, err := json.Marshal(response)
	if err != nil {
		http.Error(w, "error al codificar la respuesta", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)

	if _, err := w.Write(payload); err != nil {
		log.Printf("error al escribir la respuesta de evaluate: %v", err)
	}
}
