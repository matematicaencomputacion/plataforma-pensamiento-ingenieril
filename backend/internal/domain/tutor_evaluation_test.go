package domain

import "testing"

func TestParseTutorVerdictApproved(t *testing.T) {
	t.Parallel()

	verdict, err := ParseTutorVerdict(`{"is_approved": true, "feedback": "Bien", "missing_concepts": []}`)
	if err != nil {
		t.Fatalf("ParseTutorVerdict: %v", err)
	}
	if !verdict.IsApproved || verdict.Feedback != "Bien" || len(verdict.MissingConcepts) != 0 {
		t.Fatalf("veredicto inesperado: %+v", verdict)
	}
}

func TestParseTutorVerdictStripsMarkdownFence(t *testing.T) {
	t.Parallel()

	raw := "```json\n{\"is_approved\": false, \"feedback\": \"Falta string\", \"missing_concepts\": [\"strings\"]}\n```"
	verdict, err := ParseTutorVerdict(raw)
	if err != nil {
		t.Fatalf("ParseTutorVerdict: %v", err)
	}
	if verdict.IsApproved || len(verdict.MissingConcepts) != 1 || verdict.MissingConcepts[0] != "strings" {
		t.Fatalf("veredicto inesperado: %+v", verdict)
	}
}

func TestParseTutorVerdictInvalidJSON(t *testing.T) {
	t.Parallel()

	_, err := ParseTutorVerdict(`no-json`)
	if err == nil {
		t.Fatal("se esperaba error de parseo")
	}
}

func TestNormalizeTutorVerdictRejectsApprovedWithMissing(t *testing.T) {
	t.Parallel()

	normalized := NormalizeTutorVerdict(TutorVerdict{
		IsApproved:      true,
		Feedback:        "OK",
		MissingConcepts: []ConceptID{"variables", "variables", ""},
	}, []ConceptID{"variables", "strings"})

	if normalized.IsApproved {
		t.Fatal("no se puede aprobar con missing_concepts")
	}
	if len(normalized.MissingConcepts) != 1 || normalized.MissingConcepts[0] != "variables" {
		t.Fatalf("missing_concepts normalizados: %+v", normalized.MissingConcepts)
	}
}

func TestNormalizeTutorVerdictKeepsReject(t *testing.T) {
	t.Parallel()

	normalized := NormalizeTutorVerdict(TutorVerdict{
		IsApproved:      false,
		Feedback:        "Reprobado",
		MissingConcepts: []ConceptID{"integers"},
	}, []ConceptID{"integers"})

	if normalized.IsApproved {
		t.Fatal("debe permanecer reprobado")
	}
	if normalized.Feedback != "Reprobado" {
		t.Fatalf("feedback alterado: %q", normalized.Feedback)
	}
}
