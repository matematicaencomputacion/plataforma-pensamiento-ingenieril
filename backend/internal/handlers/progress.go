package handlers

import (
	"context"
	"encoding/json"
	"errors"
	"io"
	"net/http"
	"strings"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/usecases"
)

type progressCompleteRequest struct {
	LevelID int    `json:"level_id"`
	StepID  string `json:"step_id"`
	Passed  bool   `json:"passed"`
}

// ProgressHandler registra avance de ejercicios evaluados en el cliente (ADR 002).
type ProgressHandler struct {
	service *usecases.AuthService
}

func NewProgressHandler(service *usecases.AuthService) *ProgressHandler {
	return &ProgressHandler{service: service}
}

// Complete procesa POST /api/progress/complete.
// Nunca acepta código Python del alumno — solo el resultado del harness client-side.
func (h *ProgressHandler) Complete(w http.ResponseWriter, r *http.Request) {
	token, ok := bearerToken(r)
	if !ok {
		writeJSONError(w, "no autorizado", http.StatusUnauthorized)
		return
	}

	raw, err := io.ReadAll(io.LimitReader(r.Body, 1<<20))
	if err != nil {
		writeJSONError(w, "JSON de entrada inválido", http.StatusBadRequest)
		return
	}

	var probe map[string]json.RawMessage
	if err := json.Unmarshal(raw, &probe); err != nil {
		writeJSONError(w, "JSON de entrada inválido", http.StatusBadRequest)
		return
	}
	if _, hasCode := probe["code"]; hasCode {
		writeJSONError(
			w,
			"no se acepta código del alumno (ADR 002: evaluación solo en el browser)",
			http.StatusBadRequest,
		)
		return
	}

	var req progressCompleteRequest
	if err := json.Unmarshal(raw, &req); err != nil {
		writeJSONError(w, "JSON de entrada inválido", http.StatusBadRequest)
		return
	}
	req.StepID = strings.TrimSpace(req.StepID)

	out, err := h.service.CompleteProgress(
		context.Background(),
		token,
		req.LevelID,
		req.StepID,
		req.Passed,
	)
	if err != nil {
		switch {
		case errors.Is(err, domain.ErrUnauthorized):
			writeJSONError(w, err.Error(), http.StatusUnauthorized)
		case errors.Is(err, domain.ErrInvalidLevelID), errors.Is(err, domain.ErrInvalidStepID):
			writeJSONError(w, err.Error(), http.StatusBadRequest)
		default:
			writeAuthError(w, err)
		}
		return
	}
	writeJSON(w, http.StatusOK, out)
}
