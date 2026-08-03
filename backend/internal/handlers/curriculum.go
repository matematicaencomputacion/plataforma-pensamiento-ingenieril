package handlers

import (
	"encoding/json"
	"log"
	"net/http"
	"strings"

	"github.com/tu-usuario/plataforma-edu-backend/internal/domain"
	"github.com/tu-usuario/plataforma-edu-backend/internal/usecases"
)

type curriculumListResponse struct {
	Lessons []usecases.LessonProgressView `json:"lessons"`
}

// CurriculumHandler expone la malla curricular y el desbloqueo dinámico.
type CurriculumHandler struct {
	service *usecases.CurriculumService
}

// NewCurriculumHandler inyecta el servicio de curriculum.
func NewCurriculumHandler(service *usecases.CurriculumService) *CurriculumHandler {
	return &CurriculumHandler{service: service}
}

// List procesa GET /api/curriculum?student_id=...
func (h *CurriculumHandler) List(w http.ResponseWriter, r *http.Request) {
	studentID := r.URL.Query().Get("student_id")
	if studentID == "" {
		studentID = domain.DemoUserID
	}

	lessons, err := h.service.ListForStudent(studentID)
	if err != nil {
		log.Printf("Error detallado en Curriculum.List: %v", err)
		http.Error(w, "error al obtener el curriculum", http.StatusInternalServerError)
		return
	}

	writeJSONBytes(w, curriculumListResponse{Lessons: lessons})
}

// GetLesson procesa GET /api/curriculum/lessons/{id}?student_id=...
func (h *CurriculumHandler) GetLesson(w http.ResponseWriter, r *http.Request) {
	lessonID := r.PathValue("id")
	studentID := r.URL.Query().Get("student_id")
	if studentID == "" {
		studentID = domain.DemoUserID
	}

	lesson, err := h.service.GetLessonForStudent(lessonID, studentID)
	if err != nil {
		log.Printf("Error detallado en Curriculum.GetLesson: %v", err)
		if strings.Contains(err.Error(), "no encontrada") {
			http.Error(w, "lección no encontrada", http.StatusNotFound)
			return
		}
		http.Error(w, "error al obtener la lección", http.StatusInternalServerError)
		return
	}

	writeLessonJSON(w, lesson)
}

func writeJSONBytes(w http.ResponseWriter, payload curriculumListResponse) {
	body, err := json.Marshal(payload)
	if err != nil {
		http.Error(w, "error al codificar la respuesta", http.StatusInternalServerError)
		return
	}
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	if _, err := w.Write(body); err != nil {
		log.Printf("error al escribir respuesta de curriculum: %v", err)
	}
}

func writeLessonJSON(w http.ResponseWriter, lesson usecases.LessonProgressView) {
	body, err := json.Marshal(lesson)
	if err != nil {
		http.Error(w, "error al codificar la respuesta", http.StatusInternalServerError)
		return
	}
	w.Header().Set("Content-Type", "application/json")
	w.WriteHeader(http.StatusOK)
	if _, err := w.Write(body); err != nil {
		log.Printf("error al escribir respuesta de lección: %v", err)
	}
}
