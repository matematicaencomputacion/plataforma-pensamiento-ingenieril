package config

import (
	"os"
	"strings"
)

const (
	defaultCerebrasBaseURL = "https://api.cerebras.ai/v1"
	defaultCerebrasModel   = "gpt-oss-120b"
	defaultGrokBaseURL     = "https://api.x.ai/v1"
	defaultGrokModel       = "grok-4.5"
)

// LearnerLLMMode is LEARNER_PROFILE_LLM (mock|keyword|grok|cerebras|auto).
type LearnerLLMMode string

const (
	LearnerLLMAuto     LearnerLLMMode = "auto"
	LearnerLLMMock     LearnerLLMMode = "mock"
	LearnerLLMKeyword  LearnerLLMMode = "keyword"
	LearnerLLMGrok     LearnerLLMMode = "grok"
	LearnerLLMCerebras LearnerLLMMode = "cerebras"
)

// LearnerLLMConfig resolved provider for onboarding profile synthesis.
type LearnerLLMConfig struct {
	Mode    LearnerLLMMode
	APIKey  string
	Model   string
	BaseURL string
	// Provider is the concrete backend selected (mock|grok|cerebras).
	Provider string
}

// LoadLearnerLLMConfig picks mock/keywords, Cerebras, or Grok.
// When Mode is auto (default) and CEREBRAS_API_KEY is set, Cerebras wins.
func LoadLearnerLLMConfig() LearnerLLMConfig {
	modeRaw := strings.ToLower(strings.TrimSpace(os.Getenv("LEARNER_PROFILE_LLM")))
	if modeRaw == "" {
		modeRaw = string(LearnerLLMAuto)
	}
	mode := LearnerLLMMode(modeRaw)

	cerebrasKey := strings.TrimSpace(os.Getenv("CEREBRAS_API_KEY"))
	cerebrasModel := firstNonEmptyEnv("CEREBRAS_MODEL", defaultCerebrasModel)
	cerebrasBase := firstNonEmptyEnv("CEREBRAS_BASE_URL", defaultCerebrasBaseURL)

	grok := LoadGrokConfig()
	grokModel := firstNonEmpty(grok.Model, defaultGrokModel)
	grokBase := firstNonEmpty(grok.BaseURL, defaultGrokBaseURL)

	cfg := LearnerLLMConfig{Mode: mode}

	switch mode {
	case LearnerLLMMock, LearnerLLMKeyword:
		cfg.Provider = "mock"
		return cfg
	case LearnerLLMCerebras:
		cfg.Provider = "cerebras"
		cfg.APIKey = cerebrasKey
		cfg.Model = cerebrasModel
		cfg.BaseURL = cerebrasBase
		return cfg
	case LearnerLLMGrok:
		if grok.APIKey == "" && cerebrasKey != "" {
			// Friendly fallback: requested grok but only Cerebras key present.
			cfg.Provider = "cerebras"
			cfg.APIKey = cerebrasKey
			cfg.Model = cerebrasModel
			cfg.BaseURL = cerebrasBase
			return cfg
		}
		cfg.Provider = "grok"
		cfg.APIKey = grok.APIKey
		cfg.Model = grokModel
		cfg.BaseURL = grokBase
		return cfg
	default: // auto
		if cerebrasKey != "" {
			cfg.Mode = LearnerLLMAuto
			cfg.Provider = "cerebras"
			cfg.APIKey = cerebrasKey
			cfg.Model = cerebrasModel
			cfg.BaseURL = cerebrasBase
			return cfg
		}
		if grok.APIKey != "" {
			cfg.Provider = "grok"
			cfg.APIKey = grok.APIKey
			cfg.Model = grokModel
			cfg.BaseURL = grokBase
			return cfg
		}
		cfg.Provider = "mock"
		return cfg
	}
}

func firstNonEmptyEnv(key, fallback string) string {
	if v := strings.TrimSpace(os.Getenv(key)); v != "" {
		return v
	}
	return fallback
}

func firstNonEmpty(values ...string) string {
	for _, v := range values {
		if strings.TrimSpace(v) != "" {
			return strings.TrimSpace(v)
		}
	}
	return ""
}
