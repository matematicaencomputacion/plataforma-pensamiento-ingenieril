package usecases

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestEvaluateCodeMissingAPIKey(t *testing.T) {
	t.Setenv("GROK_API_KEY", "")

	service := NewEvaluationService()
	_, err := service.EvaluateCode("print(1)", 1)
	if err == nil {
		t.Fatal("se esperaba error cuando GROK_API_KEY está vacía")
	}
}

func TestEvaluateCodeSuccess(t *testing.T) {
	t.Setenv("GROK_API_KEY", "test-key")

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method != http.MethodPost {
			t.Fatalf("método inesperado: %s", r.Method)
		}
		if got := r.Header.Get("Authorization"); got != "Bearer test-key" {
			t.Fatalf("Authorization inesperado: %q", got)
		}

		var req chatCompletionRequest
		if err := json.NewDecoder(r.Body).Decode(&req); err != nil {
			t.Fatalf("JSON de entrada inválido: %v", err)
		}
		if req.Model != grokModel {
			t.Fatalf("modelo inesperado: got %q, want %q", req.Model, grokModel)
		}
		if len(req.Messages) != 2 {
			t.Fatalf("se esperaban 2 messages, got %d", len(req.Messages))
		}
		if req.Messages[0].Role != "system" || req.Messages[0].Content != grokSystemPrompt {
			t.Fatalf("mensaje system inesperado: %+v", req.Messages[0])
		}
		if req.Messages[1].Role != "user" || req.Messages[1].Content != "print(1)" {
			t.Fatalf("mensaje user inesperado: %+v", req.Messages[1])
		}

		w.Header().Set("Content-Type", "application/json")
		_, err := w.Write([]byte(`{"choices":[{"message":{"content":"{\"passed\": true}"}}]}`))
		if err != nil {
			t.Fatalf("error escribiendo respuesta mock: %v", err)
		}
	}))
	defer server.Close()

	service := NewEvaluationServiceForTest(server.Client(), server.URL)

	got, err := service.EvaluateCode("print(1)", 1)
	if err != nil {
		t.Fatalf("error inesperado: %v", err)
	}
	if !got {
		t.Fatal("se esperaba passed=true")
	}
}

func TestEvaluateCodeRejected(t *testing.T) {
	t.Setenv("GROK_API_KEY", "test-key")

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		_, err := w.Write([]byte(`{"choices":[{"message":{"content":"{\"passed\": false}"}}]}`))
		if err != nil {
			t.Fatalf("error escribiendo respuesta mock: %v", err)
		}
	}))
	defer server.Close()

	service := NewEvaluationServiceForTest(server.Client(), server.URL)

	got, err := service.EvaluateCode("x = 1", 1)
	if err != nil {
		t.Fatalf("error inesperado: %v", err)
	}
	if got {
		t.Fatal("se esperaba passed=false")
	}
}

func TestParsePassedFromContent(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name    string
		content string
		want    bool
	}{
		{
			name:    "json puro",
			content: `{"passed": true}`,
			want:    true,
		},
		{
			name:    "json con fences markdown",
			content: "```json\n{\"passed\": false}\n```",
			want:    false,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()

			got, err := parsePassedFromContent(tt.content)
			if err != nil {
				t.Fatalf("error inesperado: %v", err)
			}
			if got != tt.want {
				t.Fatalf("resultado inesperado: got %v, want %v", got, tt.want)
			}
		})
	}
}
