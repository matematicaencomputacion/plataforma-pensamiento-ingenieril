package handlers

import (
	"encoding/json"
	"log"
	"net/http"
	"strconv"
	"strings"

	"github.com/tu-usuario/plataforma-edu-backend/internal/domain"
	"github.com/tu-usuario/plataforma-edu-backend/internal/usecases"
)

// LevelHandler expone endpoints de niveles.
type LevelHandler struct {
	service *usecases.LevelService
}

// NewLevelHandler inyecta el servicio de niveles.
func NewLevelHandler(service *usecases.LevelService) *LevelHandler {
	return &LevelHandler{service: service}
}

// GetByID procesa GET /api/levels/{id}.
func (h *LevelHandler) GetByID(w http.ResponseWriter, r *http.Request) {
	id, err := strconv.Atoi(r.PathValue("id"))
	if err != nil || id <= 0 {
		http.Error(w, "id de nivel inválido", http.StatusBadRequest)
		return
	}

	level, err := h.service.GetByID(id)
	if err != nil {
		log.Printf("Error detallado en GetByID: %v", err)
		if strings.Contains(err.Error(), "no encontrado") {
			http.Error(w, "nivel no encontrado", http.StatusNotFound)
			return
		}
		http.Error(w, "error al obtener el nivel", http.StatusInternalServerError)
		return
	}

	writeLevelJSON(w, level)
}

// GetCurrent procesa GET /api/levels/current.
func (h *LevelHandler) GetCurrent(w http.ResponseWriter, r *http.Request) {
	level, err := h.service.GetCurrent()
	if err != nil {
		log.Printf("Error detallado en GetCurrent: %v", err)
		if strings.Contains(err.Error(), "no hay niveles") {
			http.Error(w, "no hay niveles disponibles", http.StatusNotFound)
			return
		}
		http.Error(w, "error al obtener el nivel actual", http.StatusInternalServerError)
		return
	}

	writeLevelJSON(w, level)
}

func writeLevelJSON(w http.ResponseWriter, level domain.Level) {
	payload, err := json.Marshal(level)
	if err != nil {
		http.Error(w, "error al codificar la respuesta", http.StatusInternalServerError)
		return
	}

	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	if _, err := w.Write(payload); err != nil {
		log.Printf("error al escribir respuesta de nivel: %v", err)
	}
}
