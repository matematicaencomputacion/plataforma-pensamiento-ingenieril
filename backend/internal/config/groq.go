package config

import (
	"os"
	"strings"
)

// GroqConfig parámetros del clasificador de perfil vía Groq.
type GroqConfig struct {
	APIKey  string
	Model   string
	BaseURL string
}

// LoadGroqConfig lee GROQ_API_KEY / GROQ_MODEL / GROQ_BASE_URL.
func LoadGroqConfig() GroqConfig {
	return GroqConfig{
		APIKey:  strings.TrimSpace(os.Getenv("GROQ_API_KEY")),
		Model:   strings.TrimSpace(os.Getenv("GROQ_MODEL")),
		BaseURL: strings.TrimSpace(os.Getenv("GROQ_BASE_URL")),
	}
}
