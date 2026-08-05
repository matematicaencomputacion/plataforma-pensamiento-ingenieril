package gemini

import (
	"context"
	"encoding/json"
	"fmt"
	"os"
	"strings"

	"google.golang.org/genai"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

const defaultModel = "gemini-2.5-pro"

// Classifier clasifica el relato del alumno usando Gemini (Vertex ADC o API key).
type Classifier struct {
	client *genai.Client
	model  string
}

type Config struct {
	Project  string
	Location string
	Model    string
	// Backend: "vertex" (ADC) | "gemini-api" (GEMINI_API_KEY / GOOGLE_API_KEY)
	Backend string
	APIKey  string
}

func NewClassifier(ctx context.Context, cfg Config) (*Classifier, error) {
	model := firstNonEmpty(cfg.Model, os.Getenv("GEMINI_MODEL"), defaultModel)
	backend := strings.ToLower(firstNonEmpty(cfg.Backend, os.Getenv("LEARNER_PROFILE_BACKEND"), "auto"))

	var (
		client *genai.Client
		err    error
	)

	switch backend {
	case "gemini-api", "api", "developer":
		client, err = newGeminiAPIClient(ctx, cfg)
	case "vertex", "vertexai":
		client, err = newVertexClient(ctx, cfg)
	default: // auto: prefer Vertex si hay project+creds; si no, API key
		if hasVertexConfig(cfg) {
			client, err = newVertexClient(ctx, cfg)
			if err != nil && hasAPIKey(cfg) {
				client, err = newGeminiAPIClient(ctx, cfg)
			}
		} else if hasAPIKey(cfg) {
			client, err = newGeminiAPIClient(ctx, cfg)
		} else {
			return nil, fmt.Errorf("config incompleta: definí Vertex (GOOGLE_CLOUD_PROJECT + ADC) o GEMINI_API_KEY")
		}
	}
	if err != nil {
		return nil, err
	}

	return &Classifier{client: client, model: model}, nil
}

func newVertexClient(ctx context.Context, cfg Config) (*genai.Client, error) {
	project := firstNonEmpty(cfg.Project, os.Getenv("GOOGLE_CLOUD_PROJECT"))
	location := firstNonEmpty(
		cfg.Location,
		os.Getenv("VERTEX_LOCATION"),
		os.Getenv("GOOGLE_CLOUD_LOCATION"),
		"us-central1",
	)
	if project == "" {
		return nil, fmt.Errorf("GOOGLE_CLOUD_PROJECT es obligatorio para Vertex AI")
	}
	client, err := genai.NewClient(ctx, &genai.ClientConfig{
		Project:  project,
		Location: location,
		Backend:  genai.BackendVertexAI,
	})
	if err != nil {
		return nil, fmt.Errorf("crear cliente Vertex/Gemini: %w", err)
	}
	return client, nil
}

func newGeminiAPIClient(ctx context.Context, cfg Config) (*genai.Client, error) {
	apiKey := firstNonEmpty(cfg.APIKey, os.Getenv("GEMINI_API_KEY"), os.Getenv("GOOGLE_API_KEY"))
	if apiKey == "" {
		return nil, fmt.Errorf("GEMINI_API_KEY es obligatorio para Gemini Developer API")
	}
	client, err := genai.NewClient(ctx, &genai.ClientConfig{
		APIKey:  apiKey,
		Backend: genai.BackendGeminiAPI,
	})
	if err != nil {
		return nil, fmt.Errorf("crear cliente Gemini API: %w", err)
	}
	return client, nil
}

func hasVertexConfig(cfg Config) bool {
	project := firstNonEmpty(cfg.Project, os.Getenv("GOOGLE_CLOUD_PROJECT"))
	creds := os.Getenv("GOOGLE_APPLICATION_CREDENTIALS")
	return project != "" && creds != ""
}

func hasAPIKey(cfg Config) bool {
	return firstNonEmpty(cfg.APIKey, os.Getenv("GEMINI_API_KEY"), os.Getenv("GOOGLE_API_KEY")) != ""
}

func (c *Classifier) Classify(ctx context.Context, rawNotes string) (domain.LearnerProfileSynthesis, error) {
	schema := &genai.Schema{
		Type: genai.TypeObject,
		Properties: map[string]*genai.Schema{
			"purpose": {Type: genai.TypeString},
			"urgency": {Type: genai.TypeString},
			"vision":  {Type: genai.TypeString},
			"stack":   {Type: genai.TypeString},
		},
		Required: []string{"purpose", "urgency", "vision", "stack"},
	}

	system := strings.TrimSpace(`
Sos el coach de onboarding de IngenierIA (español rioplatense, tono cercano y preciso).
A partir del relato del alumno, sintetizá cuatro campos cortos (1-2 oraciones cada uno):
- purpose: motivación / para qué quiere aprender
- urgency: qué tan urgente es y en qué horizonte temporal
- vision: visión a ~5 años o trayectoria deseada
- stack: herramientas / entornos previos (Jupyter, Cursor, Coursera, etc.)
Si falta evidencia para un campo, devolvé string vacío "".
No inventes biografía. No uses markdown. Solo JSON.
`)

	user := "Relato del alumno:\n\n" + strings.TrimSpace(rawNotes)

	result, err := c.client.Models.GenerateContent(ctx, c.model, genai.Text(user), &genai.GenerateContentConfig{
		SystemInstruction: &genai.Content{
			Parts: []*genai.Part{{Text: system}},
		},
		ResponseMIMEType: "application/json",
		ResponseSchema:   schema,
		Temperature:      genai.Ptr[float32](0.2),
	})
	if err != nil {
		return domain.LearnerProfileSynthesis{}, fmt.Errorf("GenerateContent: %w", err)
	}

	raw := strings.TrimSpace(result.Text())
	if raw == "" {
		return domain.LearnerProfileSynthesis{}, fmt.Errorf("respuesta vacía del modelo")
	}
	raw = stripFences(raw)

	var parsed struct {
		Purpose string `json:"purpose"`
		Urgency string `json:"urgency"`
		Vision  string `json:"vision"`
		Stack   string `json:"stack"`
	}
	if err := json.Unmarshal([]byte(raw), &parsed); err != nil {
		return domain.LearnerProfileSynthesis{}, fmt.Errorf("parsear JSON del modelo: %w (raw=%q)", err, truncate(raw, 200))
	}

	return domain.LearnerProfileSynthesis{
		Purpose: strings.TrimSpace(parsed.Purpose),
		Urgency: strings.TrimSpace(parsed.Urgency),
		Vision:  strings.TrimSpace(parsed.Vision),
		Stack:   strings.TrimSpace(parsed.Stack),
	}, nil
}

func firstNonEmpty(values ...string) string {
	for _, v := range values {
		if strings.TrimSpace(v) != "" {
			return strings.TrimSpace(v)
		}
	}
	return ""
}

func stripFences(s string) string {
	s = strings.TrimSpace(s)
	if strings.HasPrefix(s, "```") {
		s = strings.TrimPrefix(s, "```json")
		s = strings.TrimPrefix(s, "```JSON")
		s = strings.TrimPrefix(s, "```")
		s = strings.TrimSuffix(s, "```")
		s = strings.TrimSpace(s)
	}
	return s
}

func truncate(s string, n int) string {
	if len(s) <= n {
		return s
	}
	return s[:n] + "…"
}
