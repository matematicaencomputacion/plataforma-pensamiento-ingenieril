package domain

// TrackType identifica la modalidad pedagógica del nivel.
type TrackType string

const (
	TrackMicroPaso      TrackType = "micro_paso"
	TrackRetoIngenieril TrackType = "reto_ingenieril"
)

// Level representa un nivel/reto del recorrido de aprendizaje.
type Level struct {
	ID               int       `json:"id"`
	Title            string    `json:"title"`
	Statement        string    `json:"statement"`
	TrackType        TrackType `json:"track_type"`
	EvaluationPrompt string    `json:"evaluation_prompt"`
}
