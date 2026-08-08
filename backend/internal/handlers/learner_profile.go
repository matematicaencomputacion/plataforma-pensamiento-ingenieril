package handlers

import (
	"encoding/json"
	"errors"
	"log"
	"net/http"
	"strings"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/usecases"
)

type learnerProfileRequest struct {
	RawNotes     string `json:"raw_notes"`
	SourceStepID string `json:"source_step_id"`
}

type learnerProfileResponse struct {
	Purpose string `json:"purpose"`
	Urgency string `json:"urgency"`
	Vision  string `json:"vision"`
	Stack   string `json:"stack"`
}

// LearnerProfileHandler expone la síntesis de perfil de onboarding.
type LearnerProfileHandler struct {
	service *usecases.LearnerProfileService
}

func NewLearnerProfileHandler(service *usecases.LearnerProfileService) *LearnerProfileHandler {
	return &LearnerProfileHandler{service: service}
}

// Synthesize procesa POST /api/learner/profile/synthesize.
func (h *LearnerProfileHandler) Synthesize(w http.ResponseWriter, r *http.Request) {
	var req learnerProfileRequest
	if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
		http.Error(w, "JSON de entrada inválido", http.StatusBadRequest)
		return
	}

	out, err := h.service.Synthesize(r.Context(), req.RawNotes)
	if errors.Is(err, usecases.ErrLearnerNotesTooShort) {
		http.Error(w, "raw_notes demasiado cortas", http.StatusBadRequest)
		return
	}
	if err != nil {
		log.Printf(
			"síntesis de perfil falló (step=%s notes_len=%d): %v",
			strings.TrimSpace(req.SourceStepID),
			len(strings.TrimSpace(req.RawNotes)),
			err,
		)
		w.Header().Set("Content-Type", "application/json")
		w.WriteHeader(http.StatusBadGateway)
		msg := "error al sintetizar el perfil"
		errText := err.Error()
		switch {
		case strings.Contains(errText, "401"), strings.Contains(errText, "Invalid API Key"):
			msg = "xAI rechazó la API key. Revisá GROK_API_KEY en .env."
		case strings.Contains(errText, "429"), strings.Contains(errText, "rate_limit"):
			msg = "Cuota/rate limit de Grok. Esperá un momento e intentá de nuevo."
		case strings.Contains(errText, "model"), strings.Contains(errText, "404"):
			msg = "El modelo Grok configurado no está disponible. Probá otro GROK_MODEL en .env."
		}
		_ = json.NewEncoder(w).Encode(map[string]string{"error": msg})
		return
	}

	payload, err := json.Marshal(learnerProfileResponse{
		Purpose: out.Purpose,
		Urgency: out.Urgency,
		Vision:  out.Vision,
		Stack:   out.Stack,
	})
	if err != nil {
		http.Error(w, "error al codificar la respuesta", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	if _, err := w.Write(payload); err != nil {
		log.Printf("error al escribir respuesta de síntesis: %v", err)
	}
}
