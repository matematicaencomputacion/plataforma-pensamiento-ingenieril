package domain

import "testing"

func sampleGraph() CurriculumGraph {
	return CurriculumGraph{
		Lessons: map[string]LessonNode{
			"print-basics": {
				ID:            "print-basics",
				Title:         "Tu primer print",
				TrackType:     TrackMicroPaso,
				Description:   "Base",
				Prerequisites: nil,
				SkillTarget:   "print_basico",
			},
			"variables-and-types": {
				ID:            "variables-and-types",
				Title:         "Variables",
				TrackType:     TrackMicroPaso,
				Description:   "Avanzada",
				Prerequisites: []string{"print-basics"},
				SkillTarget:   "variables",
			},
			"inventory-challenge": {
				ID:            "inventory-challenge",
				Title:         "Inventario",
				TrackType:     TrackRetoIngenieril,
				Description:   "Reto",
				Prerequisites: []string{"print-basics", "variables-and-types"},
				SkillTarget:   "inventario_basico",
			},
		},
	}
}

func TestIsUnlockedRootLessonAlwaysOpen(t *testing.T) {
	t.Parallel()

	graph := sampleGraph()
	profile := &CognitiveProfile{UserID: DemoUserID}

	if !graph.IsUnlocked("print-basics", profile) {
		t.Fatal("la lección raíz debe estar desbloqueada sin prerrequisitos")
	}
}

func TestIsUnlockedBlockedWhenPrerequisiteNotMastered(t *testing.T) {
	t.Parallel()

	graph := sampleGraph()
	profile := &CognitiveProfile{
		UserID: DemoUserID,
		Skills: []StudentSkill{
			{ID: "print_basico", Status: SkillStatusLearning},
		},
	}

	if graph.IsUnlocked("variables-and-types", profile) {
		t.Fatal("variables no debe desbloquearse si print_basico no está mastered")
	}
	if graph.IsUnlocked("inventory-challenge", profile) {
		t.Fatal("el reto no debe desbloquearse sin prerrequisitos mastered")
	}
}

func TestIsUnlockedWhenDirectPrerequisiteMastered(t *testing.T) {
	t.Parallel()

	graph := sampleGraph()
	profile := &CognitiveProfile{
		UserID: DemoUserID,
		Skills: []StudentSkill{
			{ID: "print_basico", Status: SkillStatusMastered},
		},
	}

	if !graph.IsUnlocked("variables-and-types", profile) {
		t.Fatal("variables debe desbloquearse cuando print_basico está mastered")
	}
	if graph.IsUnlocked("inventory-challenge", profile) {
		t.Fatal("el reto sigue bloqueado sin variables mastered")
	}
}

func TestIsUnlockedWhenAllPrerequisitesMastered(t *testing.T) {
	t.Parallel()

	graph := sampleGraph()
	profile := &CognitiveProfile{
		UserID: DemoUserID,
		Skills: []StudentSkill{
			{ID: "print_basico", Status: SkillStatusMastered},
			{ID: "variables", Status: SkillStatusMastered},
		},
	}

	if !graph.IsUnlocked("inventory-challenge", profile) {
		t.Fatal("el reto debe desbloquearse cuando ambos prerrequisitos están mastered")
	}
}

func TestIsUnlockedMissingLesson(t *testing.T) {
	t.Parallel()

	graph := sampleGraph()
	profile := &CognitiveProfile{UserID: DemoUserID}

	if graph.IsUnlocked("does-not-exist", profile) {
		t.Fatal("una lección inexistente no puede estar desbloqueada")
	}
}

func TestIsUnlockedNilGraph(t *testing.T) {
	t.Parallel()

	var graph *CurriculumGraph
	if graph.IsUnlocked("print-basics", &CognitiveProfile{}) {
		t.Fatal("grafo nil no debe desbloquear lecciones")
	}
}

func TestIsUnlockedDetectsCycle(t *testing.T) {
	t.Parallel()

	graph := CurriculumGraph{
		Lessons: map[string]LessonNode{
			"a": {
				ID:            "a",
				Prerequisites: []string{"b"},
				SkillTarget:   "skill_a",
			},
			"b": {
				ID:            "b",
				Prerequisites: []string{"a"},
				SkillTarget:   "skill_b",
			},
		},
	}
	profile := &CognitiveProfile{
		Skills: []StudentSkill{
			{ID: "skill_a", Status: SkillStatusMastered},
			{ID: "skill_b", Status: SkillStatusMastered},
		},
	}

	if graph.IsUnlocked("a", profile) {
		t.Fatal("un ciclo en el grafo no debe reportarse como desbloqueado")
	}
}

func TestHasMastered(t *testing.T) {
	t.Parallel()

	profile := &CognitiveProfile{
		Skills: []StudentSkill{
			{ID: "print_basico", Status: SkillStatusMastered},
			{ID: "variables", Status: SkillStatusLearning},
		},
	}

	if !profile.HasMastered("print_basico") {
		t.Fatal("se esperaba print_basico mastered")
	}
	if profile.HasMastered("variables") {
		t.Fatal("variables en learning no cuenta como mastered")
	}
	if profile.HasMastered("") {
		t.Fatal("skill vacío no puede estar mastered")
	}
}
