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

// HasMastered indica si el perfil domina la habilidad indicada.
func (p *CognitiveProfile) HasMastered(skillID string) bool {
	if p == nil || skillID == "" {
		return false
	}

	for _, skill := range p.Skills {
		if skill.ID == skillID && skill.Status == SkillStatusMastered {
			return true
		}
	}

	return false
}

// IsUnlocked indica si una lección está desbloqueada según el perfil cognitivo.
// Una lección sin prerrequisitos siempre está desbloqueada.
// Si tiene prerrequisitos, cada uno debe:
//  1. existir en el grafo,
//  2. tener su skill_target en estado mastered en el perfil,
//  3. estar a su vez desbloqueada (cierre transitivo del DAG).
func (g *CurriculumGraph) IsUnlocked(lessonID string, profile *CognitiveProfile) bool {
	if g == nil || g.Lessons == nil {
		return false
	}

	return g.isUnlocked(lessonID, profile, make(map[string]bool))
}

func (g *CurriculumGraph) isUnlocked(
	lessonID string,
	profile *CognitiveProfile,
	visiting map[string]bool,
) bool {
	if visiting[lessonID] {
		// Ciclo detectado: el grafo debe ser acíclico; se considera bloqueado.
		return false
	}

	node, ok := g.Lessons[lessonID]
	if !ok {
		return false
	}

	if len(node.Prerequisites) == 0 {
		return true
	}

	visiting[lessonID] = true
	defer delete(visiting, lessonID)

	for _, prereqID := range node.Prerequisites {
		prereq, exists := g.Lessons[prereqID]
		if !exists {
			return false
		}
		if !profile.HasMastered(prereq.SkillTarget) {
			return false
		}
		if !g.isUnlocked(prereqID, profile, visiting) {
			return false
		}
	}

	return true
}
