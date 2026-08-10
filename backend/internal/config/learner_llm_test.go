package config_test

import (
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/config"
)

func TestLoadLearnerLLMConfigPrefersCerebrasWhenKeyPresent(t *testing.T) {
	t.Setenv("LEARNER_PROFILE_LLM", "auto")
	t.Setenv("CEREBRAS_API_KEY", "cb-test")
	t.Setenv("GROK_API_KEY", "grok-test")
	t.Setenv("XAI_API_KEY", "")
	t.Setenv("CEREBRAS_MODEL", "")
	t.Setenv("CEREBRAS_BASE_URL", "")

	cfg := config.LoadLearnerLLMConfig()
	if cfg.Provider != "cerebras" {
		t.Fatalf("provider=%q want cerebras", cfg.Provider)
	}
	if cfg.APIKey != "cb-test" {
		t.Fatalf("api key mismatch")
	}
	if cfg.BaseURL != "https://api.cerebras.ai/v1" {
		t.Fatalf("baseURL=%q", cfg.BaseURL)
	}
	if cfg.Model != "gpt-oss-120b" {
		t.Fatalf("model=%q", cfg.Model)
	}
}

func TestLoadLearnerLLMConfigMock(t *testing.T) {
	t.Setenv("LEARNER_PROFILE_LLM", "mock")
	t.Setenv("CEREBRAS_API_KEY", "cb-test")
	cfg := config.LoadLearnerLLMConfig()
	if cfg.Provider != "mock" {
		t.Fatalf("provider=%q want mock", cfg.Provider)
	}
}

func TestLoadLearnerLLMConfigGrokFallsBackToCerebras(t *testing.T) {
	t.Setenv("LEARNER_PROFILE_LLM", "grok")
	t.Setenv("GROK_API_KEY", "")
	t.Setenv("XAI_API_KEY", "")
	t.Setenv("CEREBRAS_API_KEY", "cb-only")
	cfg := config.LoadLearnerLLMConfig()
	if cfg.Provider != "cerebras" || cfg.APIKey != "cb-only" {
		t.Fatalf("unexpected %#v", cfg)
	}
}
