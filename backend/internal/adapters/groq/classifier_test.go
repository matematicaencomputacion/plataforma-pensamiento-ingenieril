package groq_test

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/groq"
)

func TestClassifierClassifyOK(t *testing.T) {
	t.Parallel()

	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/openai/v1/chat/completions" {
			t.Fatalf("path: %s", r.URL.Path)
		}
		if !strings.HasPrefix(r.Header.Get("Authorization"), "Bearer test-key") {
			t.Fatalf("auth header: %q", r.Header.Get("Authorization"))
		}
		_ = json.NewEncoder(w).Encode(map[string]any{
			"choices": []map[string]any{
				{
					"message": map[string]string{
						"content": `{"purpose":"aprender","urgency":"ya","vision":"staff","stack":"python"}`,
					},
				},
			},
		})
	}))
	t.Cleanup(srv.Close)

	clf, err := groq.NewClassifier(context.Background(), groq.Config{
		APIKey:  "test-key",
		Model:   "llama-3.1-8b-instant",
		BaseURL: srv.URL + "/openai/v1",
	})
	if err != nil {
		t.Fatalf("new: %v", err)
	}

	out, err := clf.Classify(context.Background(), "Quiero aprender Python rápido para trabajar.")
	if err != nil {
		t.Fatalf("classify: %v", err)
	}
	if out.Purpose != "aprender" || out.Stack != "python" {
		t.Fatalf("unexpected synthesis: %+v", out)
	}
}

func TestNewClassifierRequiresAPIKey(t *testing.T) {
	t.Setenv("GROQ_API_KEY", "")
	_, err := groq.NewClassifier(context.Background(), groq.Config{})
	if err == nil {
		t.Fatal("expected error without GROQ_API_KEY")
	}
}
