package domain

import "context"

// LearnerProfileSynthesis es la síntesis estructurada del onboarding (migas de pan).
type LearnerProfileSynthesis struct {
	Purpose string `json:"purpose"`
	Urgency string `json:"urgency"`
	Vision  string `json:"vision"`
	Stack   string `json:"stack"`
}

// ProfileClassifier clasifica el relato libre del alumno hacia un perfil inicial.
type ProfileClassifier interface {
	Classify(ctx context.Context, rawNotes string) (LearnerProfileSynthesis, error)
}
