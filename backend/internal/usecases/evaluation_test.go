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
	_, _, err := service.EvaluateCode("print(1)", 1)
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
		_, err := w.Write([]byte(`{"choices":[{"message":{"content":"{\"passed\": true, \"feedback\": \"Excelente trabajo\"}"}}]}`))
		if err != nil {
			t.Fatalf("error escribiendo respuesta mock: %v", err)
		}
	}))
	defer server.Close()

	service := NewEvaluationServiceForTest(server.Client(), server.URL)

	got, feedback, err := service.EvaluateCode("print(1)", 1)
	if err != nil {
		t.Fatalf("error inesperado: %v", err)
	}
	if !got {
		t.Fatal("se esperaba passed=true")
	}
	if feedback != "Excelente trabajo" {
		t.Fatalf("feedback inesperado: got %q", feedback)
	}
}

func TestEvaluateCodeRejected(t *testing.T) {
	t.Setenv("GROK_API_KEY", "test-key")

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		_, err := w.Write([]byte(`{"choices":[{"message":{"content":"{\"passed\": false, \"feedback\": \"Falta un print válido\"}"}}]}`))
		if err != nil {
			t.Fatalf("error escribiendo respuesta mock: %v", err)
		}
	}))
	defer server.Close()

	service := NewEvaluationServiceForTest(server.Client(), server.URL)

	got, feedback, err := service.EvaluateCode("x = 1", 1)
	if err != nil {
		t.Fatalf("error inesperado: %v", err)
	}
	if got {
		t.Fatal("se esperaba passed=false")
	}
	if feedback != "Falta un print válido" {
		t.Fatalf("feedback inesperado: got %q", feedback)
	}
}

func TestEvaluateCodeNonOKStatus(t *testing.T) {
	t.Setenv("GROK_API_KEY", "test-key")

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.WriteHeader(http.StatusUnauthorized)
		_, err := w.Write([]byte(`{"error":"invalid api key"}`))
		if err != nil {
			t.Fatalf("error escribiendo respuesta mock: %v", err)
		}
	}))
	defer server.Close()

	service := NewEvaluationServiceForTest(server.Client(), server.URL)

	_, _, err := service.EvaluateCode("print(1)", 1)
	if err == nil {
		t.Fatal("se esperaba error por status != 200")
	}

	wantFragment := `xAI API error: status 401, body: {"error":"invalid api key"}`
	if err.Error() != wantFragment {
		t.Fatalf("mensaje de error inesperado:\ngot:  %q\nwant: %q", err.Error(), wantFragment)
	}
}

func TestEvaluateCodeInvalidCompletionJSON(t *testing.T) {
	t.Setenv("GROK_API_KEY", "test-key")

	server := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Content-Type", "application/json")
		_, err := w.Write([]byte(`{"choices":`))
		if err != nil {
			t.Fatalf("error escribiendo respuesta mock: %v", err)
		}
	}))
	defer server.Close()

	service := NewEvaluationServiceForTest(server.Client(), server.URL)

	_, _, err := service.EvaluateCode("print(1)", 1)
	if err == nil {
		t.Fatal("se esperaba error de unmarshal")
	}
}

func TestParseVerdictFromContent(t *testing.T) {
	t.Parallel()

	tests := []struct {
		name         string
		content      string
		wantPassed   bool
		wantFeedback string
	}{
		{
			name:         "json puro",
			content:      `{"passed": true, "feedback": "Bien hecho"}`,
			wantPassed:   true,
			wantFeedback: "Bien hecho",
		},
		{
			name:         "json con fences markdown",
			content:      "```json\n{\"passed\": false, \"feedback\": \"Revisa el print\"}\n```",
			wantPassed:   false,
			wantFeedback: "Revisa el print",
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			t.Parallel()

			gotPassed, gotFeedback, err := parseVerdictFromContent(tt.content)
			if err != nil {
				t.Fatalf("error inesperado: %v", err)
			}
			if gotPassed != tt.wantPassed {
				t.Fatalf("passed inesperado: got %v, want %v", gotPassed, tt.wantPassed)
			}
			if gotFeedback != tt.wantFeedback {
				t.Fatalf("feedback inesperado: got %q, want %q", gotFeedback, tt.wantFeedback)
			}
		})
	}
}
