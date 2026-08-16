package handlers

import (
	"encoding/json"
	"errors"
	"io"
	"net/http"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/usecases"
)

type conceptEventRequest struct {
	Type        string `json:"type"`
	PartitionID int    `json:"partition_id"`
	DecadeLo    int    `json:"decade_lo"`
	StepID      string `json:"step_id"`
}

// ConceptAnalyticsHandler ingestión y resumen de fricción conceptual.
type ConceptAnalyticsHandler struct {
	service *usecases.ConceptAnalyticsService
}

func NewConceptAnalyticsHandler(service *usecases.ConceptAnalyticsService) *ConceptAnalyticsHandler {
	return &ConceptAnalyticsHandler{service: service}
}

// Record procesa POST /api/concept-events.
// Nunca acepta código Python del alumno (ADR 002).
func (h *ConceptAnalyticsHandler) Record(w http.ResponseWriter, r *http.Request) {
	token, ok := bearerToken(r)
	if !ok {
		writeJSONError(w, "no autorizado", http.StatusUnauthorized)
		return
	}

	raw, err := io.ReadAll(io.LimitReader(r.Body, 1<<16))
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

	var req conceptEventRequest
	if err := json.Unmarshal(raw, &req); err != nil {
		writeJSONError(w, "JSON de entrada inválido", http.StatusBadRequest)
		return
	}

	err = h.service.Record(r.Context(), token, domain.EventInput{
		Type:        req.Type,
		PartitionID: req.PartitionID,
		DecadeLo:    req.DecadeLo,
		StepID:      req.StepID,
	})
	if err != nil {
		writeConceptAnalyticsError(w, err)
		return
	}
	w.WriteHeader(http.StatusNoContent)
}

// Summary procesa GET /api/concept-analytics (solo el usuario de la sesión).
func (h *ConceptAnalyticsHandler) Summary(w http.ResponseWriter, r *http.Request) {
	token, ok := bearerToken(r)
	if !ok {
		writeJSONError(w, "no autorizado", http.StatusUnauthorized)
		return
	}
	sum, err := h.service.Summary(r.Context(), token)
	if err != nil {
		writeConceptAnalyticsError(w, err)
		return
	}
	writeJSON(w, http.StatusOK, sum)
}

func writeConceptAnalyticsError(w http.ResponseWriter, err error) {
	switch {
	case errors.Is(err, domain.ErrUnauthorized):
		writeJSONError(w, err.Error(), http.StatusUnauthorized)
	case errors.Is(err, domain.ErrInvalidEventType),
		errors.Is(err, domain.ErrInvalidPartitionID),
		errors.Is(err, domain.ErrInvalidDecade),
		errors.Is(err, domain.ErrInvalidStepID):
		writeJSONError(w, err.Error(), http.StatusBadRequest)
	default:
		writeAuthError(w, err)
	}
}
