package domain

// LessonNode representa un nodo del grafo curricular (DAG).
type LessonNode struct {
	ID            string    `json:"id"`
	Title         string    `json:"title"`
	TrackType     TrackType `json:"track_type"`
	Description   string    `json:"description"`
	Prerequisites []string  `json:"prerequisites"`
	SkillTarget   string    `json:"skill_target"`
}

// CurriculumGraph modela la malla de lecciones como un mapa indexado por ID.
type CurriculumGraph struct {
	Lessons map[string]LessonNode `json:"lessons"`
}
