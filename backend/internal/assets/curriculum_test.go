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
		if !concept.HasMedia() {
			continue
		}
		withMedia++
		for _, lang := range concept.AvailableMediaLocales() {
			media, ok := concept.MediaFor(lang)
			if !ok || !media.HasURL() {
				t.Fatalf("MediaFor(%s) inválido en %s", lang, concept.ID)
			}
			for i, seg := range media.Transcript {
				if seg.EndSec <= seg.StartSec || strings.TrimSpace(seg.Text) == "" {
					t.Fatalf("segmento inválido en %s/%s[%d]: %+v", concept.ID, lang, i, seg)
				}
			}
		}
	}
	if withMedia < 2 {
		t.Fatalf("Module 1 debe sembrar al menos 2 conceptos con media OCW/DUA, got %d", withMedia)
	}

	seed := graph.Concepts["concept:string-literals"]
	if !seed.HasMedia() {
		t.Fatal("concept:string-literals debe incluir media")
	}
	if len(seed.Resources) < 2 {
		t.Fatalf("string-literals debe ser bilingüe, resources=%d", len(seed.Resources))
	}
	es, ok := seed.MediaFor("es")
	if !ok || !es.HasContent() {
		t.Fatal("MediaFor(es) incompleto")
	}
	en, ok := seed.MediaFor("en")
	if !ok || !en.HasContent() {
		t.Fatal("MediaFor(en) incompleto")
	}
	if es.ResourceURL == en.ResourceURL {
		t.Fatal("ES y EN deberían apuntar a videos distintos en el seed bilingüe")
	}
	if _, ok := seed.ActiveTranscriptSegmentFor("en", 20); !ok {
		t.Fatal("ActiveTranscriptSegmentFor(en) debe resolver un bloque en t=20s")
	}
	if _, ok := seed.ActiveTranscriptSegment(20); !ok {
		t.Fatal("ActiveTranscriptSegment (legado/es) debe resolver un bloque en t=20s")
	}

	esMedia, ok := seed.MediaFor("es")
	if !ok || !esMedia.HasChapters() {
		t.Fatal("string-literals/es debe exponer chapters del arnés pedagógico")
	}
	if len(esMedia.Chapters) != 18 {
		t.Fatalf("curso MoureDev debe exponer 18 capítulos oficiales, got %d", len(esMedia.Chapters))
	}

	wantBounds := []struct {
		atSec float64
		id    string
		title string
	}{
		{0, "ch-01-introduccion", "Capítulo 1: Introducción"},
		{244, "ch-02-contexto", "Capítulo 2: Contexto"},
		{850, "ch-03-configuracion", "Capítulo 3: 01 - Configuración"},
		{2938, "ch-05-variables", "Capítulo 5: 03 - Variables"},
		{8645, "ch-07-strings", "Capítulo 7: 05 - Strings"},
		{26619, "ch-14-funciones", "Capítulo 14: 12 - Funciones"},
		{36391, "ch-18-proximos-pasos", "Capítulo 18: Próximos pasos"},
	}
	for _, want := range wantBounds {
		ch, ok := esMedia.ChapterAt(want.atSec)
		if !ok || ch.ID != want.id {
			t.Fatalf("ChapterAt(%.0f): got ok=%v id=%q want %q", want.atSec, ok, ch.ID, want.id)
		}
		if ch.Title != want.title {
			t.Fatalf("ChapterAt(%.0f) title: got %q want %q", want.atSec, ch.Title, want.title)
		}
		if ch.EndSec <= ch.StartSec {
			t.Fatalf("capítulo %s con rango inválido: %.0f-%.0f", ch.ID, ch.StartSec, ch.EndSec)
		}
	}

	last := esMedia.Chapters[len(esMedia.Chapters)-1]
	if last.EndSec != 36454 {
		t.Fatalf("fin del video/capítulo 18: got %.0f want 36454", last.EndSec)
	}
	if len(esMedia.TranscriptForChapter(esMedia.Chapters[0])) == 0 {
		t.Fatal("el capítulo 1 debe aportar transcripción propia o filtrada")
	}

	enMedia, ok := seed.MediaFor("en")
	if !ok {
		t.Fatal("MediaFor(en) debe seguir disponible")
	}
	if enMedia.HasChapters() {
		t.Fatal("EN corto no debe requerir chapters (retrocompatibilidad)")
	}
}
