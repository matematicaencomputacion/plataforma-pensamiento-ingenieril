package assets

import (
	"encoding/json"
	"strings"
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

func TestCurriculumJSONEmbedded(t *testing.T) {
	t.Parallel()

	if len(CurriculumJSON) == 0 {
		t.Fatal("CurriculumJSON embebido está vacío")
	}

	var payload struct {
		Version  int `json:"version"`
		Concepts []struct {
			ID string `json:"id"`
		} `json:"concepts"`
		Edges []struct {
			From        string `json:"from"`
			To          string `json:"to"`
			RationaleES string `json:"rationale_es"`
			Source      string `json:"source"`
		} `json:"edges"`
	}
	if err := json.Unmarshal(CurriculumJSON, &payload); err != nil {
		t.Fatalf("JSON embebido inválido: %v", err)
	}

	if payload.Version != 1 {
		t.Fatalf("version inesperada: %d", payload.Version)
	}
	if len(payload.Concepts) != 20 {
		t.Fatalf("se esperaban 20 conceptos, got %d", len(payload.Concepts))
	}
	if len(payload.Edges) != 20 {
		t.Fatalf("se esperaban 20 aristas, got %d", len(payload.Edges))
	}

	for i, edge := range payload.Edges {
		if edge.From == "" || edge.To == "" {
			t.Fatalf("arista %d incompleta: %+v", i, edge)
		}
		if strings.TrimSpace(edge.RationaleES) == "" {
			t.Fatalf("arista %s->%s sin rationale_es", edge.From, edge.To)
		}
		if edge.Source != "curated" {
			t.Fatalf("rationale debe estar curado (source=curated), got %q en %s->%s", edge.Source, edge.From, edge.To)
		}
		if strings.Contains(edge.RationaleES, "BORRADOR") {
			t.Fatalf("rationale_es no debe quedar en borrador: %q", edge.RationaleES)
		}
	}
}

func TestUnifiedCurriculumDAGIsAcyclicAndSortable(t *testing.T) {
	t.Parallel()

	var graph domain.CurriculumGraph
	if err := json.Unmarshal(CurriculumJSON, &graph); err != nil {
		t.Fatalf("unmarshal CurriculumGraph: %v", err)
	}

	if len(graph.Concepts) != 20 {
		t.Fatalf("catálogo: got %d want 20", len(graph.Concepts))
	}
	if len(graph.Edges) != 20 {
		t.Fatalf("edges: got %d want 20", len(graph.Edges))
	}
	if len(graph.Lessons) != 20 {
		t.Fatalf("proyección a lessons: got %d want 20", len(graph.Lessons))
	}

	if err := graph.HasCycles(); err != nil {
		t.Fatalf("curriculum unificado no debe tener ciclos: %v", err)
	}

	order, err := graph.TopologicalSort()
	if err != nil {
		t.Fatalf("TopologicalSort: %v", err)
	}
	if len(order) != 20 {
		t.Fatalf("orden topológico incompleto: got %d want 20 (%v)", len(order), order)
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

	// requires: from requiere to ⇒ to antecede a from
	assertBefore("concept:string-literals", "concept:variables-scope")
	assertBefore("concept:variables-scope", "concept:function-parameters")
	assertBefore("concept:env-file", "concept:env-secrets")
	assertBefore("concept:coordinate-geometry", "concept:distance-on-plane")
	assertBefore("concept:distance-on-plane", "concept:postgis-stdwithin")
	assertBefore("concept:dua-dimensions", "concept:interactive-stage")
	assertBefore("concept:interactive-stage", "concept:live-station")
}

func TestModule1ConceptsOptionalMedia(t *testing.T) {
	t.Parallel()

	var graph domain.CurriculumGraph
	if err := json.Unmarshal(CurriculumJSON, &graph); err != nil {
		t.Fatalf("unmarshal: %v", err)
	}

	withMedia := 0
	for _, concept := range graph.Concepts {
		if concept.ResourceURL == "" && len(concept.Transcript) == 0 {
			continue
		}
		if !concept.HasMedia() {
			t.Fatalf("media parcial inválida en %s", concept.ID)
		}
		withMedia++
		for i, seg := range concept.Transcript {
			if seg.EndSec <= seg.StartSec || strings.TrimSpace(seg.Text) == "" {
				t.Fatalf("segmento inválido en %s[%d]: %+v", concept.ID, i, seg)
			}
		}
	}
	if withMedia < 2 {
		t.Fatalf("Module 1 debe sembrar al menos 2 conceptos con media OCW/DUA, got %d", withMedia)
	}

	seed := graph.Concepts["concept:string-literals"]
	if !seed.HasMedia() {
		t.Fatal("concept:string-literals debe incluir resource_url y transcript")
	}
	if _, ok := seed.ActiveTranscriptSegment(20); !ok {
		t.Fatal("ActiveTranscriptSegment debe resolver un bloque en t=20s")
	}
}
