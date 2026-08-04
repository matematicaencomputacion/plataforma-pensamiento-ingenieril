package assets

import (
	"encoding/json"
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

func TestCurriculumJSONEmbedded(t *testing.T) {
	t.Parallel()

	if len(CurriculumJSON) == 0 {
		t.Fatal("CurriculumJSON embebido está vacío")
	}

	var payload map[string]json.RawMessage
	if err := json.Unmarshal(CurriculumJSON, &payload); err != nil {
		t.Fatalf("JSON embebido inválido: %v", err)
	}

	lessonsRaw, ok := payload["lessons"]
	if !ok {
		t.Fatal("el JSON embebido debe contener la clave lessons")
	}

	var lessons map[string]struct {
		ID       string   `json:"id"`
		Concepts []string `json:"concepts"`
	}
	if err := json.Unmarshal(lessonsRaw, &lessons); err != nil {
		t.Fatalf("no se pudo parsear lessons: %v", err)
	}

	if len(lessons) != 10 {
		t.Fatalf("Module 1 debe exponer 10 lecciones, got %d", len(lessons))
	}

	if _, ok := lessons["py-m01-01-hello-print"]; !ok {
		t.Fatal("falta lección raíz py-m01-01-hello-print")
	}
	if _, ok := lessons["py-m01-10-declarative-studio"]; !ok {
		t.Fatal("falta reto final py-m01-10-declarative-studio")
	}

	var concepts map[string]struct {
		ID string `json:"id"`
	}
	if err := json.Unmarshal(payload["concepts"], &concepts); err != nil {
		t.Fatalf("no se pudo parsear concepts: %v", err)
	}
	for _, id := range []string{"variables", "integers", "strings", "basic_declarations"} {
		if _, ok := concepts[id]; !ok {
			t.Fatalf("falta concepto base %q", id)
		}
	}

	multi := lessons["py-m01-08-types-conversion"]
	if len(multi.Concepts) < 3 {
		t.Fatalf("types-conversion debe mapear múltiples conceptos, got %+v", multi.Concepts)
	}
}

func TestPythonModule1DAGIsAcyclicAndSortable(t *testing.T) {
	t.Parallel()

	var graph domain.CurriculumGraph
	if err := json.Unmarshal(CurriculumJSON, &graph); err != nil {
		t.Fatalf("unmarshal CurriculumGraph: %v", err)
	}

	if err := graph.HasCycles(); err != nil {
		t.Fatalf("Module 1 no debe tener ciclos: %v", err)
	}

	order, err := graph.TopologicalSort()
	if err != nil {
		t.Fatalf("TopologicalSort Module 1: %v", err)
	}
	if len(order) != 10 {
		t.Fatalf("orden topológico incompleto: got %d want 10 (%v)", len(order), order)
	}

	index := make(map[string]int, len(order))
	for i, id := range order {
		index[id] = i
	}

	assertBefore := func(before, after string) {
		t.Helper()
		bi, okB := index[before]
		ai, okA := index[after]
		if !okB || !okA {
			t.Fatalf("IDs ausentes en orden: %s(%v) %s(%v) — %v", before, okB, after, okA, order)
		}
		if bi >= ai {
			t.Fatalf("%s debe anteceder a %s en %v", before, after, order)
		}
	}

	assertBefore("py-m01-01-hello-print", "py-m01-02-assignment")
	assertBefore("py-m01-02-assignment", "py-m01-04-integer-literals")
	assertBefore("py-m01-02-assignment", "py-m01-06-string-literals")
	assertBefore("py-m01-04-integer-literals", "py-m01-05-integer-arithmetic")
	assertBefore("py-m01-06-string-literals", "py-m01-07-string-operations")
	assertBefore("py-m01-05-integer-arithmetic", "py-m01-08-types-conversion")
	assertBefore("py-m01-07-string-operations", "py-m01-08-types-conversion")
	assertBefore("py-m01-03-naming", "py-m01-09-composed-expressions")
	assertBefore("py-m01-08-types-conversion", "py-m01-10-declarative-studio")
	assertBefore("py-m01-09-composed-expressions", "py-m01-10-declarative-studio")
}
