package domain

import "time"

// DemoUserID es el usuario estático temporal (sin auth real).
const DemoUserID = "demo-user"

// SkillStatus representa el estado de dominio de una habilidad.
type SkillStatus string

const (
	SkillStatusLearning SkillStatus = "learning"
	SkillStatusMastered SkillStatus = "mastered"
)

// StudentSkill representa una habilidad del estudiante en el perfil cognitivo.
type StudentSkill struct {
	ID             string      `json:"id"`
	Status         SkillStatus `json:"status"`
	LastReviewedAt time.Time   `json:"last_reviewed_at"`
}

// CognitiveProfile agrupa las habilidades adquiridas de un estudiante.
type CognitiveProfile struct {
	UserID string         `json:"user_id"`
	Skills []StudentSkill `json:"skills"`
}
