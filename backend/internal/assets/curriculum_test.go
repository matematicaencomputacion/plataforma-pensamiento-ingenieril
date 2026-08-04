package assets

import (
	"encoding/json"
	"testing"
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
		ID string `json:"id"`
	}
	if err := json.Unmarshal(lessonsRaw, &lessons); err != nil {
		t.Fatalf("no se pudo parsear lessons: %v", err)
	}

	if len(lessons) < 3 {
		t.Fatalf("se esperaban al menos 3 lecciones embebidas, got %d", len(lessons))
	}

	if _, ok := lessons["print-basics"]; !ok {
		t.Fatal("falta lección print-basics en el embed")
	}
}
