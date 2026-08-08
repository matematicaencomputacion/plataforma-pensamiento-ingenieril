package xai

import (
	"bytes"
	"context"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
	"time"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

const (
	defaultModel   = "grok-4.5"
	defaultBaseURL = "https://api.x.ai/v1"
	httpTimeout    = 45 * time.Second
)

// Classifier implementa domain.ProfileClassifier vía xAI Grok (OpenAI-compatible).
type Classifier struct {
	apiKey     string
	model      string
	baseURL    string
	httpClient *http.Client
}

// Config mantiene API key + model (misma forma que el adaptador previo).
type Config struct {
	APIKey  string
	Model   string
	BaseURL string
}

func NewClassifier(_ context.Context, cfg Config) (*Classifier, error) {
	apiKey := firstNonEmpty(cfg.APIKey, os.Getenv("GROK_API_KEY"), os.Getenv("XAI_API_KEY"))
	if apiKey == "" {
		return nil, fmt.Errorf("GROK_API_KEY (o XAI_API_KEY) es obligatorio para Grok")
	}
	model := firstNonEmpty(cfg.Model, os.Getenv("GROK_MODEL"), defaultModel)
	baseURL := strings.TrimRight(firstNonEmpty(cfg.BaseURL, os.Getenv("XAI_BASE_URL"), defaultBaseURL), "/")

	return &Classifier{
		apiKey:  apiKey,
		model:   model,
		baseURL: baseURL,
		httpClient: &http.Client{
			Timeout: httpTimeout,
		},
	}, nil
}

func (c *Classifier) Classify(ctx context.Context, rawNotes string) (domain.LearnerProfileSynthesis, error) {
	system := strings.TrimSpace(`
Sos el coach de onboarding de IngenierIA (español rioplatense, tono cercano y preciso).
A partir del relato del alumno, sintetizá cuatro campos cortos (1-2 oraciones cada uno):
- purpose: motivación / para qué quiere aprender
- urgency: qué tan urgente es y en qué horizonte temporal
- vision: visión a ~5 años o trayectoria deseada
- stack: herramientas / entornos previos (Jupyter, Cursor, Coursera, etc.)
Si falta evidencia para un campo, devolvé string vacío "".
No inventes biografía. No uses markdown. Solo JSON con claves purpose, urgency, vision, stack.
`)

	body := chatRequest{
		Model: c.model,
		Messages: []chatMessage{
			{Role: "system", Content: system},
			{Role: "user", Content: "Relato del alumno:\n\n" + strings.TrimSpace(rawNotes)},
		},
		Temperature: 0.2,
		ResponseFormat: &responseFormat{
			Type: "json_object",
		},
	}

	payload, err := json.Marshal(body)
	if err != nil {
		return domain.LearnerProfileSynthesis{}, fmt.Errorf("marshal request: %w", err)
	}

	req, err := http.NewRequestWithContext(
		ctx,
		http.MethodPost,
		c.baseURL+"/chat/completions",
		bytes.NewReader(payload),
	)
	if err != nil {
		return domain.LearnerProfileSynthesis{}, err
	}
	req.Header.Set("Authorization", "Bearer "+c.apiKey)
	req.Header.Set("Content-Type", "application/json")

	res, err := c.httpClient.Do(req)
	if err != nil {
		return domain.LearnerProfileSynthesis{}, fmt.Errorf("xAI request: %w", err)
	}
	defer res.Body.Close()

	rawBody, err := io.ReadAll(io.LimitReader(res.Body, 1<<20))
	if err != nil {
		return domain.LearnerProfileSynthesis{}, fmt.Errorf("leer respuesta xAI: %w", err)
	}
	if res.StatusCode < 200 || res.StatusCode >= 300 {
		return domain.LearnerProfileSynthesis{}, fmt.Errorf(
			"xAI HTTP %d: %s",
			res.StatusCode,
			truncate(string(rawBody), 300),
		)
	}

	var parsed chatResponse
	if err := json.Unmarshal(rawBody, &parsed); err != nil {
		return domain.LearnerProfileSynthesis{}, fmt.Errorf("decode xAI: %w", err)
	}
	if len(parsed.Choices) == 0 || strings.TrimSpace(parsed.Choices[0].Message.Content) == "" {
		return domain.LearnerProfileSynthesis{}, fmt.Errorf("respuesta vacía del modelo")
	}

	content := stripFences(parsed.Choices[0].Message.Content)
	var fields struct {
		Purpose string `json:"purpose"`
		Urgency string `json:"urgency"`
		Vision  string `json:"vision"`
		Stack   string `json:"stack"`
	}
	if err := json.Unmarshal([]byte(content), &fields); err != nil {
		return domain.LearnerProfileSynthesis{}, fmt.Errorf(
			"parsear JSON del modelo: %w (raw=%q)",
			err,
			truncate(content, 200),
		)
	}

	return domain.LearnerProfileSynthesis{
		Purpose: strings.TrimSpace(fields.Purpose),
		Urgency: strings.TrimSpace(fields.Urgency),
		Vision:  strings.TrimSpace(fields.Vision),
		Stack:   strings.TrimSpace(fields.Stack),
	}, nil
}

type chatMessage struct {
	Role    string `json:"role"`
	Content string `json:"content"`
}

type responseFormat struct {
	Type string `json:"type"`
}

type chatRequest struct {
	Model          string          `json:"model"`
	Messages       []chatMessage   `json:"messages"`
	Temperature    float32         `json:"temperature"`
	ResponseFormat *responseFormat `json:"response_format,omitempty"`
}

type chatResponse struct {
	Choices []struct {
		Message struct {
			Content string `json:"content"`
		} `json:"message"`
	} `json:"choices"`
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
