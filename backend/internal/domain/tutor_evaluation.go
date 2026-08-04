package domain

import (
	"context"
	"encoding/json"
	"fmt"
	"strings"
)

// TutorVerdict es el esquema JSON determinista que la IA tutora debe retornar.
type TutorVerdict struct {
	IsApproved      bool        `json:"is_approved"`
	Feedback        string      `json:"feedback"`
	MissingConcepts []ConceptID `json:"missing_concepts"`
}

// TutorEvaluationInput agrupa el código del alumno y el LessonNode a evaluar.
type TutorEvaluationInput struct {
	LessonID string
	Code     string
}

// LLMTutor es el puerto de salida hacia el proveedor LLM (Grok u otro).
// La infraestructura implementa este contrato; los tests usan mocks.
type LLMTutor interface {
	Complete(ctx context.Context, systemPrompt, userContent string) (string, error)
}

// ParseTutorVerdict interpreta la respuesta cruda del LLM al esquema endurecido.
func ParseTutorVerdict(content string) (TutorVerdict, error) {
	cleaned := strings.TrimSpace(content)
	cleaned = strings.TrimPrefix(cleaned, "```json")
	cleaned = strings.TrimPrefix(cleaned, "```")
	cleaned = strings.TrimSuffix(cleaned, "```")
	cleaned = strings.TrimSpace(cleaned)

	var verdict TutorVerdict
	if err := json.Unmarshal([]byte(cleaned), &verdict); err != nil {
		return TutorVerdict{}, fmt.Errorf("veredicto de tutora inválido (%q): %w", content, err)
	}
	if verdict.MissingConcepts == nil {
		verdict.MissingConcepts = []ConceptID{}
	}
	return verdict, nil
}

// NormalizeTutorVerdict aplica reglas locales de consistencia sobre la respuesta del LLM.
// Si faltan conceptos requeridos, la evaluación no puede aprobarse.
func NormalizeTutorVerdict(verdict TutorVerdict, required []ConceptID) TutorVerdict {
	missing := uniqueConceptIDs(verdict.MissingConcepts)
	if verdict.IsApproved && len(missing) > 0 {
		verdict.IsApproved = false
		if strings.TrimSpace(verdict.Feedback) == "" {
			verdict.Feedback = "La solución no demuestra todos los conceptos requeridos por la lección."
		}
	}
	verdict.MissingConcepts = missing

	// Defensa adicional: si aprueba sin reportar faltantes, no inventamos missing;
	// la validación semántica sigue en el LLM. required se usa para documentar
	// el contrato y futuros chequeos heurísticos.
	_ = required
	return verdict
}

func uniqueConceptIDs(ids []ConceptID) []ConceptID {
	if len(ids) == 0 {
		return []ConceptID{}
	}
	seen := make(map[ConceptID]struct{}, len(ids))
	out := make([]ConceptID, 0, len(ids))
	for _, id := range ids {
		if id == "" {
			continue
		}
		if _, ok := seen[id]; ok {
			continue
		}
		seen[id] = struct{}{}
		out = append(out, id)
	}
	return out
}
