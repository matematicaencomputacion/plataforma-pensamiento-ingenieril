package domain

import (
	"errors"
	"slices"
	"testing"
)

func complexValidGraph() CurriculumGraph {
	return CurriculumGraph{
		Concepts: map[ConceptID]Concept{
			"print_io":  {ID: "print_io", Title: "Print"},
			"variables": {ID: "variables", Title: "Variables"},
			"control":   {ID: "control", Title: "Control de flujo"},
			"functions": {ID: "functions", Title: "Funciones"},
		},
		Lessons: map[string]LessonNode{
			"print-basics": {
				ID:       "print-basics",
				Concepts: []ConceptID{"print_io"},
			},
			"variables-and-types": {
				ID:            "variables-and-types",
				Concepts:      []ConceptID{"variables", "print_io"},
				Prerequisites: PrerequisiteList{LessonPrerequisite("print-basics")},
			},
			"conditionals": {
				ID:            "conditionals",
				Concepts:      []ConceptID{"control"},
				Prerequisites: PrerequisiteList{LessonPrerequisite("variables-and-types")},
			},
			"functions-intro": {
				ID:       "functions-intro",
				Concepts: []ConceptID{"functions"},
				Prerequisites: PrerequisiteList{
					LessonPrerequisite("conditionals"),
					ConceptPrerequisite("variables"),
				},
			},
			"refactor-lab": {
				ID:       "refactor-lab",
				Concepts: []ConceptID{"functions", "control"},
				Prerequisites: PrerequisiteList{
					ConceptPrerequisite("functions"),
					ConceptPrerequisite("control"),
				},
			},
			// Rama paralela independiente: otro entry-point del DAG.
			"git-basics": {
				ID:       "git-basics",
				Concepts: []ConceptID{},
			},
		},
	}
}

func TestHasCyclesNilAndAcyclic(t *testing.T) {
	t.Parallel()

	var nilGraph *CurriculumGraph
	if err := nilGraph.HasCycles(); err != nil {
		t.Fatalf("grafo nil no debería reportar ciclo: %v", err)
	}

	graph := complexValidGraph()
	if err := graph.HasCycles(); err != nil {
		t.Fatalf("grafo válido reportó ciclo: %v", err)
	}
}

func TestHasCyclesLessonLoop(t *testing.T) {
	t.Parallel()

	graph := CurriculumGraph{
		Lessons: map[string]LessonNode{
			"a": {ID: "a", Prerequisites: PrerequisiteList{LessonPrerequisite("b")}},
			"b": {ID: "b", Prerequisites: PrerequisiteList{LessonPrerequisite("a")}},
		},
	}

	err := graph.HasCycles()
	if err == nil {
		t.Fatal("se esperaba error por ciclo a <-> b")
	}
	if !errors.Is(err, ErrCurriculumCycle) {
		t.Fatalf("error debería envolver ErrCurriculumCycle, got %v", err)
	}
	var cycle *CycleError
	if !errors.As(err, &cycle) {
		t.Fatalf("se esperaba CycleError, got %T", err)
	}
	if len(cycle.Path) < 3 {
		t.Fatalf("path de ciclo incompleto: %+v", cycle.Path)
	}
}

func TestHasCyclesThroughConcepts(t *testing.T) {
	t.Parallel()

	// A enseña concepto X; B requiere X; A requiere B → ciclo A -> X -> B -> A
	graph := CurriculumGraph{
		Concepts: map[ConceptID]Concept{
			"x": {ID: "x", Title: "X"},
		},
		Lessons: map[string]LessonNode{
			"a": {
				ID:            "a",
				Concepts:      []ConceptID{"x"},
				Prerequisites: PrerequisiteList{LessonPrerequisite("b")},
			},
			"b": {
				ID:            "b",
				Prerequisites: PrerequisiteList{ConceptPrerequisite("x")},
			},
		},
	}

	if err := graph.HasCycles(); err == nil {
		t.Fatal("ciclo vía concepto debería detectarse")
	}
}

func TestTopologicalSortComplexValidGraph(t *testing.T) {
	t.Parallel()

	graph := complexValidGraph()
	order, err := graph.TopologicalSort()
	if err != nil {
		t.Fatalf("TopologicalSort: %v", err)
	}
	if len(order) != len(graph.Lessons) {
		t.Fatalf("orden incompleto: got %d want %d (%v)", len(order), len(graph.Lessons), order)
	}

	index := make(map[string]int, len(order))
	for i, id := range order {
		index[id] = i
	}

	assertBefore := func(before, after string) {
		t.Helper()
		if index[before] >= index[after] {
			t.Fatalf("%s debe anteceder a %s en %v", before, after, order)
		}
	}

	assertBefore("print-basics", "variables-and-types")
	assertBefore("variables-and-types", "conditionals")
	assertBefore("conditionals", "functions-intro")
	assertBefore("functions-intro", "refactor-lab")
	assertBefore("conditionals", "refactor-lab")

	if !slices.Contains(order, "git-basics") {
		t.Fatalf("rama paralela ausente del orden: %v", order)
	}
}

func TestTopologicalSortRejectsCycle(t *testing.T) {
	t.Parallel()

	graph := CurriculumGraph{
		Lessons: map[string]LessonNode{
			"a": {ID: "a", Prerequisites: PrerequisiteList{LessonPrerequisite("c")}},
			"b": {ID: "b", Prerequisites: PrerequisiteList{LessonPrerequisite("a")}},
			"c": {ID: "c", Prerequisites: PrerequisiteList{LessonPrerequisite("b")}},
		},
	}

	order, err := graph.TopologicalSort()
	if err == nil {
		t.Fatalf("ciclo de 3 no debería ordenarse, got %v", order)
	}
	if !errors.Is(err, ErrCurriculumCycle) {
		t.Fatalf("error inesperado: %v", err)
	}
	if order != nil {
		t.Fatalf("ante ciclo el orden debe ser nil, got %v", order)
	}
}

func TestTopologicalSortDeterministic(t *testing.T) {
	t.Parallel()

	graph := complexValidGraph()
	first, err := graph.TopologicalSort()
	if err != nil {
		t.Fatalf("primera corrida: %v", err)
	}
	for i := 0; i < 5; i++ {
		next, err := graph.TopologicalSort()
		if err != nil {
			t.Fatalf("corrida %d: %v", i, err)
		}
		if !slices.Equal(first, next) {
			t.Fatalf("orden no determinista:\n%v\n%v", first, next)
		}
	}
}

func TestGetUnlockedNodesRootsAndProgression(t *testing.T) {
	t.Parallel()

	graph := complexValidGraph()

	roots := graph.GetUnlockedNodes(NewLearnerProgress(nil, nil))
	if !slices.Equal(roots, []string{"git-basics", "print-basics"}) {
		t.Fatalf("raíces inesperadas: %v", roots)
	}

	afterPrint := graph.GetUnlockedNodes(NewLearnerProgress([]string{"print-basics"}, nil))
	if !slices.Contains(afterPrint, "variables-and-types") {
		t.Fatalf("variables debería desbloquearse tras print-basics: %v", afterPrint)
	}
	if slices.Contains(afterPrint, "print-basics") {
		t.Fatalf("lección ya aprobada no debe listarse: %v", afterPrint)
	}
	if slices.Contains(afterPrint, "functions-intro") {
		t.Fatalf("functions-intro aún no debería estar lista: %v", afterPrint)
	}

	mid := graph.GetUnlockedNodes(NewLearnerProgress(
		[]string{"print-basics", "variables-and-types", "conditionals", "git-basics"},
		nil,
	))
	if !slices.Equal(mid, []string{"functions-intro"}) {
		t.Fatalf("tras la cadena base solo functions-intro debería abrirse: %v", mid)
	}

	afterFunctions := graph.GetUnlockedNodes(NewLearnerProgress(
		[]string{"print-basics", "variables-and-types", "conditionals", "functions-intro", "git-basics"},
		nil,
	))
	if !slices.Equal(afterFunctions, []string{"refactor-lab"}) {
		t.Fatalf("refactor-lab se desbloquea por conceptos de lecciones previas: %v", afterFunctions)
	}
}

func TestGetUnlockedNodesByExplicitConcepts(t *testing.T) {
	t.Parallel()

	graph := CurriculumGraph{
		Lessons: map[string]LessonNode{
			"capstone": {
				ID: "capstone",
				Prerequisites: PrerequisiteList{
					ConceptPrerequisite("variables"),
					ConceptPrerequisite("control"),
				},
			},
			"warmup": {ID: "warmup"},
		},
	}

	blocked := graph.GetUnlockedNodes(NewLearnerProgress(nil, []ConceptID{"variables"}))
	if slices.Contains(blocked, "capstone") {
		t.Fatalf("capstone no debe abrirse con un solo concepto: %v", blocked)
	}
	if !slices.Contains(blocked, "warmup") {
		t.Fatalf("warmup debería seguir disponible: %v", blocked)
	}

	ready := graph.GetUnlockedNodes(NewLearnerProgress(nil, []ConceptID{"variables", "control"}))
	if !slices.Contains(ready, "capstone") {
		t.Fatalf("capstone debería abrirse con ambos conceptos: %v", ready)
	}
}

func TestIsUnlockedMissingLessonAndNilGraph(t *testing.T) {
	t.Parallel()

	var graph *CurriculumGraph
	if graph.IsUnlocked("x", NewLearnerProgress(nil, nil)) {
		t.Fatal("grafo nil no desbloquea")
	}

	g := complexValidGraph()
	if g.IsUnlocked("does-not-exist", NewLearnerProgress(nil, nil)) {
		t.Fatal("lección inexistente no puede desbloquearse")
	}
	if !g.IsUnlocked("print-basics", NewLearnerProgress(nil, nil)) {
		t.Fatal("raíz debería estar desbloqueada")
	}
}

func TestTopologicalSortEmptyGraph(t *testing.T) {
	t.Parallel()

	order, err := (&CurriculumGraph{}).TopologicalSort()
	if err != nil {
		t.Fatalf("grafo vacío: %v", err)
	}
	if order != nil {
		t.Fatalf("se esperaba nil, got %v", order)
	}
}
