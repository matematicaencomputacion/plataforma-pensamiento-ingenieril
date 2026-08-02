package usecases

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"os"
	"strings"
	"time"
)

const (
	grokChatCompletionsURL = "https://api.x.ai/v1/chat/completions"
	// latest apunta al modelo chat más reciente recomendado por xAI.
	grokModel        = "latest"
	grokSystemPrompt = "Eres un evaluador estricto de código Python. El usuario enviará un código. Tu única respuesta debe ser un JSON puro con la estructura {\"passed\": true} si el código resuelve un problema básico, o {\"passed\": false} si es incorrecto. No agregues markdown ni explicaciones."
)

// EvaluationService orquesta la evaluación de ejercicios usando la API de Grok (xAI).
type EvaluationService struct {
	httpClient *http.Client
	apiURL     string
}

// NewEvaluationService crea una instancia del servicio de evaluación.
func NewEvaluationService() *EvaluationService {
	return &EvaluationService{
		httpClient: &http.Client{Timeout: 60 * time.Second},
		apiURL:     grokChatCompletionsURL,
	}
}

// NewEvaluationServiceForTest permite inyectar cliente HTTP y URL en pruebas.
func NewEvaluationServiceForTest(client *http.Client, apiURL string) *EvaluationService {
	return &EvaluationService{
		httpClient: client,
		apiURL:     apiURL,
	}
}

type chatMessage struct {
	Role    string `json:"role"`
	Content string `json:"content"`
}

type chatCompletionRequest struct {
	Model    string        `json:"model"`
	Messages []chatMessage `json:"messages"`
}

type chatCompletionResponse struct {
	Choices []struct {
		Message struct {
			Content string `json:"content"`
		} `json:"message"`
	} `json:"choices"`
}

type evaluationVerdict struct {
	Passed bool `json:"passed"`
}

// EvaluateCode envía el código a Grok y retorna si el ejercicio fue aprobado.
func (s *EvaluationService) EvaluateCode(code string, levelID int) (bool, error) {
	_ = levelID // reservado para prompts por nivel en iteraciones futuras

	apiKey := os.Getenv("GROK_API_KEY")
	if apiKey == "" {
		return false, fmt.Errorf("GROK_API_KEY no está configurada")
	}

	payload := chatCompletionRequest{
		Model: grokModel,
		Messages: []chatMessage{
			{Role: "system", Content: grokSystemPrompt},
			{Role: "user", Content: code},
		},
	}

	body, err := json.Marshal(payload)
	if err != nil {
		return false, fmt.Errorf("error al codificar la petición a Grok: %w", err)
	}

	req, err := http.NewRequest(http.MethodPost, s.apiURL, bytes.NewReader(body))
	if err != nil {
		return false, fmt.Errorf("error al crear la petición a Grok: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+apiKey)

	resp, err := s.httpClient.Do(req)
	if err != nil {
		return false, fmt.Errorf("error al llamar a la API de Grok: %w", err)
	}
	defer resp.Body.Close()

	respBody, err := io.ReadAll(resp.Body)
	if err != nil {
		return false, fmt.Errorf("error al leer la respuesta de Grok: %w", err)
	}

	if resp.StatusCode < http.StatusOK || resp.StatusCode >= http.StatusMultipleChoices {
		return false, fmt.Errorf("API de Grok respondió con estado %d: %s", resp.StatusCode, string(respBody))
	}

	var completion chatCompletionResponse
	if err := json.Unmarshal(respBody, &completion); err != nil {
		return false, fmt.Errorf("error al parsear la respuesta de Grok: %w", err)
	}

	if len(completion.Choices) == 0 {
		return false, fmt.Errorf("la API de Grok no devolvió choices")
	}

	passed, err := parsePassedFromContent(completion.Choices[0].Message.Content)
	if err != nil {
		return false, err
	}

	return passed, nil
}

func parsePassedFromContent(content string) (bool, error) {
	cleaned := strings.TrimSpace(content)
	cleaned = strings.TrimPrefix(cleaned, "```json")
	cleaned = strings.TrimPrefix(cleaned, "```")
	cleaned = strings.TrimSuffix(cleaned, "```")
	cleaned = strings.TrimSpace(cleaned)

	var verdict evaluationVerdict
	if err := json.Unmarshal([]byte(cleaned), &verdict); err != nil {
		return false, fmt.Errorf("no se pudo parsear el veredicto de Grok (%q): %w", content, err)
	}

	return verdict.Passed, nil
}
