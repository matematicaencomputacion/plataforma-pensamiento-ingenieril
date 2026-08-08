package config

import (
	"os"
	"strings"
)

// GrokConfig parámetros del clasificador de perfil vía xAI Grok.
type GrokConfig struct {
	APIKey  string
	Model   string
	BaseURL string
}

// LoadGrokConfig lee GROK_API_KEY (o XAI_API_KEY) / GROK_MODEL / XAI_BASE_URL.
func LoadGrokConfig() GrokConfig {
	apiKey := strings.TrimSpace(os.Getenv("GROK_API_KEY"))
	if apiKey == "" {
		apiKey = strings.TrimSpace(os.Getenv("XAI_API_KEY"))
	}
	return GrokConfig{
		APIKey:  apiKey,
		Model:   strings.TrimSpace(os.Getenv("GROK_MODEL")),
		BaseURL: strings.TrimSpace(os.Getenv("XAI_BASE_URL")),
	}
}
