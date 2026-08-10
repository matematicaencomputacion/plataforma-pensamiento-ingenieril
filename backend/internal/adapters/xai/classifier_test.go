package xai_test

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/xai"
)

func TestClassifierClassifyOK(t *testing.T) {
	t.Parallel()

	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path != "/v1/chat/completions" {
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

	clf, err := xai.NewClassifier(context.Background(), xai.Config{
		APIKey:  "test-key",
		Model:   "grok-4.5",
		BaseURL: srv.URL + "/v1",
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
	t.Setenv("GROK_API_KEY", "")
	t.Setenv("XAI_API_KEY", "")
	t.Setenv("CEREBRAS_API_KEY", "")
	_, err := xai.NewClassifier(context.Background(), xai.Config{})
	if err == nil {
		t.Fatal("expected error without API key")
	}
}
