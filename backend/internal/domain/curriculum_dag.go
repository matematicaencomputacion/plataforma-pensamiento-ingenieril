package domain

import (
	"errors"
	"fmt"
	"sort"
	"strings"
)

// ErrCurriculumCycle indica que el digrafo curricular no es un DAG.
var ErrCurriculumCycle = errors.New("curriculum graph contains a cycle")

// CycleError detalla un ciclo encontrado en el digrafo.
type CycleError struct {
	Path []string
}

func (e *CycleError) Error() string {
	if e == nil || len(e.Path) == 0 {
		return ErrCurriculumCycle.Error()
	}
	return fmt.Sprintf("%v: %s", ErrCurriculumCycle, strings.Join(e.Path, " -> "))
}

func (e *CycleError) Unwrap() error {
	return ErrCurriculumCycle
}

// LearnerProgress modela lecciones y conceptos ya aprobados por un alumno.
type LearnerProgress struct {
	Lessons  map[string]struct{}
	Concepts map[ConceptID]struct{}
}

// NewLearnerProgress construye un set de progreso a partir de slices.
func NewLearnerProgress(lessons []string, concepts []ConceptID) LearnerProgress {
	p := LearnerProgress{
		Lessons:  make(map[string]struct{}, len(lessons)),
		Concepts: make(map[ConceptID]struct{}, len(concepts)),
	}
	for _, id := range lessons {
		if id != "" {
			p.Lessons[id] = struct{}{}
		}
	}
	for _, id := range concepts {
		if id != "" {
			p.Concepts[id] = struct{}{}
		}
	}
	return p
}

// HasLesson indica si la lección figura como aprobada.
func (p LearnerProgress) HasLesson(lessonID string) bool {
	_, ok := p.Lessons[lessonID]
	return ok
}

// HasConcept indica si el concepto figura como aprobado explícitamente.
func (p LearnerProgress) HasConcept(conceptID ConceptID) bool {
	_, ok := p.Concepts[conceptID]
	return ok
}

// HasCycles recorre el digrafo de lecciones (incl. dependencias vía conceptos) y
// retorna error si detecta una dependencia circular.
func (g *CurriculumGraph) HasCycles() error {
	if g == nil {
		return nil
	}

	adj, order := g.buildLessonDependencyGraph()
	state := make(map[string]int, len(order)) // 0=unseen, 1=visiting, 2=done
	stack := make([]string, 0, len(order))

	var dfs func(string) error
	dfs = func(v string) error {
		state[v] = 1
		stack = append(stack, v)
		for _, next := range adj[v] {
			switch state[next] {
			case 1:
				return &CycleError{Path: cyclePath(stack, next)}
			case 0:
				if err := dfs(next); err != nil {
					return err
				}
			}
		}
		stack = stack[:len(stack)-1]
		state[v] = 2
		return nil
	}

	for _, v := range order {
		if state[v] == 0 {
			if err := dfs(v); err != nil {
				return err
			}
		}
	}
	return nil
}

// TopologicalSort aplana el DAG y retorna una secuencia lineal de IDs de lección.
// Respeta prerrequisitos de lección y de concepto (vía otras lecciones que los enseñan).
func (g *CurriculumGraph) TopologicalSort() ([]string, error) {
	if g == nil || len(g.Lessons) == 0 {
		return nil, nil
	}
	if err := g.HasCycles(); err != nil {
		return nil, err
	}

	adj, order := g.buildLessonDependencyGraph()
	indegree := make(map[string]int, len(order))
	for _, v := range order {
		if _, ok := indegree[v]; !ok {
			indegree[v] = 0
		}
		for _, next := range adj[v] {
			indegree[next]++
		}
	}

	ready := make([]string, 0, len(order))
	for _, v := range order {
		if indegree[v] == 0 {
			ready = append(ready, v)
		}
	}
	sort.Strings(ready)

	sortedLessons := make([]string, 0, len(g.Lessons))
	for len(ready) > 0 {
		v := ready[0]
		ready = ready[1:]
		sortedLessons = append(sortedLessons, v)

		nextReady := make([]string, 0)
		for _, next := range adj[v] {
			indegree[next]--
			if indegree[next] == 0 {
				nextReady = append(nextReady, next)
			}
		}
		sort.Strings(nextReady)
		ready = mergeSorted(ready, nextReady)
	}

	if len(sortedLessons) != len(g.Lessons) {
		return nil, ErrCurriculumCycle
	}
	return sortedLessons, nil
}

// GetUnlockedNodes calcula las lecciones listas para cursarse ahora mismo.
// Excluye lecciones ya aprobadas. Un nodo está listo si todos sus prerrequisitos
// (lección o concepto) están satisfechos por el progreso efectivo del alumno.
func (g *CurriculumGraph) GetUnlockedNodes(progress LearnerProgress) []string {
	if g == nil || g.Lessons == nil {
		return nil
	}

	effective := g.effectiveProgress(progress)
	unlocked := make([]string, 0)
	for id := range g.Lessons {
		if effective.HasLesson(id) {
			continue
		}
		if g.isUnlockedWithProgress(id, effective) {
			unlocked = append(unlocked, id)
		}
	}
	sort.Strings(unlocked)
	return unlocked
}

// IsUnlocked indica si una lección concreta está lista según el progreso dado.
func (g *CurriculumGraph) IsUnlocked(lessonID string, progress LearnerProgress) bool {
	if g == nil || g.Lessons == nil {
		return false
	}
	if _, ok := g.Lessons[lessonID]; !ok {
		return false
	}
	return g.isUnlockedWithProgress(lessonID, g.effectiveProgress(progress))
}

func (g *CurriculumGraph) isUnlockedWithProgress(lessonID string, progress LearnerProgress) bool {
	node, ok := g.Lessons[lessonID]
	if !ok {
		return false
	}

	for _, prereq := range node.Prerequisites {
		kind := prereq.Kind
		if kind == "" {
			kind = PrerequisiteKindLesson
		}
		switch kind {
		case PrerequisiteKindLesson:
			if prereq.RefID == "" || !progress.HasLesson(prereq.RefID) {
				return false
			}
		case PrerequisiteKindConcept:
			if prereq.RefID == "" || !progress.HasConcept(ConceptID(prereq.RefID)) {
				return false
			}
		default:
			return false
		}
	}
	return true
}

// effectiveProgress une conceptos explícitos con los enseñados por lecciones aprobadas.
func (g *CurriculumGraph) effectiveProgress(progress LearnerProgress) LearnerProgress {
	out := LearnerProgress{
		Lessons:  make(map[string]struct{}, len(progress.Lessons)),
		Concepts: make(map[ConceptID]struct{}, len(progress.Concepts)+len(g.Lessons)),
	}
	for id := range progress.Lessons {
		out.Lessons[id] = struct{}{}
	}
	for id := range progress.Concepts {
		out.Concepts[id] = struct{}{}
	}
	for lessonID := range out.Lessons {
		lesson, ok := g.Lessons[lessonID]
		if !ok {
			continue
		}
		for _, conceptID := range lesson.Concepts {
			out.Concepts[conceptID] = struct{}{}
		}
	}
	return out
}

// buildLessonDependencyGraph proyecta dependencias a un digrafo solo de lecciones.
//
// Aristas (prerrequisito → dependiente):
//   - prerrequisito de lección P en L  ⇒  P → L
//   - prerrequisito de concepto C en L ⇒  T → L para cada lección T≠L que enseña C
//
// Así se detectan ciclos mediados por conceptos sin falsos positivos cuando una
// lección refuerza (enseña) un concepto que también declara como prerrequisito.
func (g *CurriculumGraph) buildLessonDependencyGraph() (map[string][]string, []string) {
	producers := make(map[ConceptID][]string)
	for _, lesson := range g.Lessons {
		for _, conceptID := range lesson.Concepts {
			producers[conceptID] = append(producers[conceptID], lesson.ID)
		}
	}
	for conceptID, ids := range producers {
		producers[conceptID] = uniqueSorted(ids)
	}

	adj := make(map[string][]string, len(g.Lessons))
	order := make([]string, 0, len(g.Lessons))
	for id := range g.Lessons {
		adj[id] = nil
		order = append(order, id)
	}
	sort.Strings(order)

	addEdge := func(from, to string) {
		if from == "" || to == "" || from == to {
			return
		}
		if _, ok := g.Lessons[from]; !ok {
			return
		}
		if _, ok := g.Lessons[to]; !ok {
			return
		}
		adj[from] = append(adj[from], to)
	}

	for _, lesson := range g.Lessons {
		for _, prereq := range lesson.Prerequisites {
			kind := prereq.Kind
			if kind == "" {
				kind = PrerequisiteKindLesson
			}
			if prereq.RefID == "" {
				continue
			}
			switch kind {
			case PrerequisiteKindLesson:
				addEdge(prereq.RefID, lesson.ID)
			case PrerequisiteKindConcept:
				for _, producerID := range producers[ConceptID(prereq.RefID)] {
					addEdge(producerID, lesson.ID)
				}
			}
		}
	}

	for from, tos := range adj {
		adj[from] = uniqueSorted(tos)
	}
	return adj, order
}

func cyclePath(stack []string, backEdgeTo string) []string {
	start := -1
	for i, v := range stack {
		if v == backEdgeTo {
			start = i
			break
		}
	}
	if start < 0 {
		return []string{backEdgeTo}
	}
	path := append([]string{}, stack[start:]...)
	path = append(path, backEdgeTo)
	return path
}

func uniqueSorted(values []string) []string {
	if len(values) == 0 {
		return nil
	}
	sorted := append([]string(nil), values...)
	sort.Strings(sorted)
	out := make([]string, 0, len(sorted))
	var prev string
	first := true
	for _, v := range sorted {
		if first || v != prev {
			out = append(out, v)
			prev = v
			first = false
		}
	}
	return out
}

func mergeSorted(a, b []string) []string {
	out := make([]string, 0, len(a)+len(b))
	i, j := 0, 0
	for i < len(a) && j < len(b) {
		if a[i] == b[j] {
			out = append(out, a[i])
			i++
			j++
			continue
		}
		if a[i] < b[j] {
			out = append(out, a[i])
			i++
			continue
		}
		out = append(out, b[j])
		j++
	}
	out = append(out, a[i:]...)
	out = append(out, b[j:]...)
	return out
}
