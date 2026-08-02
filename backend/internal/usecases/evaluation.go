package usecases

import "strings"

// EvaluationService orquesta la validación de ejercicios del estudiante.
// Nota: esta implementación es un mock temporal. La ejecución real de Python
// ocurre en el cliente (Wasm); aquí solo simulamos el resultado de evaluación.
type EvaluationService struct{}

// NewEvaluationService crea una instancia del servicio de evaluación.
func NewEvaluationService() *EvaluationService {
	return &EvaluationService{}
}

// EvaluateCode simula la evaluación de un ejercicio.
// Por ahora aprueba si el código contiene la palabra "print".
func (s *EvaluationService) EvaluateCode(code string, levelID int) (bool, error) {
	_ = levelID // reservado para reglas por nivel en iteraciones futuras

	passed := strings.Contains(code, "print")
	return passed, nil
}
