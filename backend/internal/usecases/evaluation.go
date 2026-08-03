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

	"github.com/tu-usuario/plataforma-edu-backend/internal/domain"
	"github.com/tu-usuario/plataforma-edu-backend/internal/repositories"
)

const (
	grokChatCompletionsURL = "https://api.x.ai/v1/chat/completions"
	// grok-4.5: modelo chat actual usado por el motor de evaluación.
	grokModel         = "grok-4.5"
	baseProfessorRole = "Eres un profesor de programación empático, alentador y claro. El usuario enviará un código en Python. Evalúa si el código es correcto respecto del enunciado y si ejecuta al menos un print() válido. Devuelve EXCLUSIVAMENTE un JSON puro con esta estructura: {\"passed\": true/false, \"feedback\": \"Tu explicación aquí\"}. Si es correcto, felicítalo brevemente. Si hay un error, explícale el porqué de forma didáctica para guiarlo, sin darle el código resuelto. REGLA ESTRICTA: NO uses bloques Markdown (```json)."
)

// EvaluationService orquesta la evaluación de ejercicios usando la API de Grok (xAI).
type EvaluationService struct {
	httpClient *http.Client
	apiURL     string
	levels     repositories.LevelRepository
	profiles   repositories.CognitiveProfileRepository
}

// NewEvaluationService crea una instancia del servicio de evaluación.
func NewEvaluationService(
	levels repositories.LevelRepository,
	profiles repositories.CognitiveProfileRepository,
) *EvaluationService {
	return &EvaluationService{
		httpClient: &http.Client{Timeout: 60 * time.Second},
		apiURL:     grokChatCompletionsURL,
		levels:     levels,
		profiles:   profiles,
	}
}

// NewEvaluationServiceForTest permite inyectar cliente HTTP, URL y repositorios en pruebas.
func NewEvaluationServiceForTest(
	client *http.Client,
	apiURL string,
	levels repositories.LevelRepository,
	profiles repositories.CognitiveProfileRepository,
) *EvaluationService {
	return &EvaluationService{
		httpClient: client,
		apiURL:     apiURL,
		levels:     levels,
		profiles:   profiles,
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
	Passed   bool   `json:"passed"`
	Feedback string `json:"feedback"`
}

// EvaluateCode carga nivel + perfil, evalúa con Grok y actualiza el perfil si aprueba.
func (s *EvaluationService) EvaluateCode(code string, levelID int, studentID string) (bool, string, error) {
	if s.levels == nil || s.profiles == nil {
		return false, "", fmt.Errorf("repositorios de nivel/perfil no configurados")
	}
	if levelID <= 0 {
		return false, "", fmt.Errorf("level_id inválido: %d", levelID)
	}
	if studentID == "" {
		studentID = domain.DemoUserID
	}

	apiKey := os.Getenv("GROK_API_KEY")
	if apiKey == "" {
		return false, "", fmt.Errorf("GROK_API_KEY no está configurada")
	}

	level, err := s.levels.GetByID(levelID)
	if err != nil {
		return false, "", fmt.Errorf("error al cargar nivel %d: %w", levelID, err)
	}

	profile, err := s.profiles.GetByUserID(studentID)
	if err != nil {
		return false, "", fmt.Errorf("error al cargar perfil cognitivo de %q: %w", studentID, err)
	}

	systemPrompt, err := buildSystemPrompt(level, profile)
	if err != nil {
		return false, "", err
	}

	payload := chatCompletionRequest{
		Model: grokModel,
		Messages: []chatMessage{
			{Role: "system", Content: systemPrompt},
			{Role: "user", Content: code},
		},
	}

	body, err := json.Marshal(payload)
	if err != nil {
		return false, "", fmt.Errorf("error al codificar la petición a Grok: %w", err)
	}

	req, err := http.NewRequest(http.MethodPost, s.apiURL, bytes.NewReader(body))
	if err != nil {
		return false, "", fmt.Errorf("error al crear la petición a Grok: %w", err)
	}
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+apiKey)

	resp, err := s.httpClient.Do(req)
	if err != nil {
		return false, "", fmt.Errorf("xAI API error: fallo en http.Client: %w", err)
	}
	defer resp.Body.Close()

	bodyBytes, err := io.ReadAll(resp.Body)
	if err != nil {
		return false, "", fmt.Errorf("xAI API error: no se pudo leer el cuerpo de la respuesta: %w", err)
	}

	if resp.StatusCode != http.StatusOK {
		return false, "", fmt.Errorf("xAI API error: status %d, body: %s", resp.StatusCode, string(bodyBytes))
	}

	var completion chatCompletionResponse
	if err := json.Unmarshal(bodyBytes, &completion); err != nil {
		return false, "", fmt.Errorf("error al unmarshal de la respuesta de xAI: %w; body: %s", err, string(bodyBytes))
	}

	if len(completion.Choices) == 0 {
		return false, "", fmt.Errorf("xAI API error: la respuesta no contiene choices; body: %s", string(bodyBytes))
	}

	passed, feedback, err := parseVerdictFromContent(completion.Choices[0].Message.Content)
	if err != nil {
		return false, "", fmt.Errorf("error al unmarshal del veredicto de evaluación: %w", err)
	}

	if passed {
		updated := promoteProfileOnPass(profile, level, time.Now().UTC())
		if err := s.profiles.Save(updated); err != nil {
			return false, "", fmt.Errorf("evaluación aprobada pero falló persistir perfil cognitivo: %w", err)
		}
	}

	return passed, feedback, nil
}

func buildSystemPrompt(level domain.Level, profile domain.CognitiveProfile) (string, error) {
	skillsJSON, err := json.Marshal(profile.Skills)
	if err != nil {
		return "", fmt.Errorf("error al serializar perfil cognitivo: %w", err)
	}

	var b strings.Builder
	b.WriteString(baseProfessorRole)
	b.WriteString("\n\n## Rol del track (")
	b.WriteString(string(level.TrackType))
	b.WriteString(")\n")
	b.WriteString(level.EvaluationPrompt)
	b.WriteString("\n\n## Enunciado del nivel\nTítulo: ")
	b.WriteString(level.Title)
	b.WriteString("\n")
	b.WriteString(level.Statement)
	b.WriteString("\n\n## Perfil cognitivo del estudiante (JSON)\n")
	b.Write(skillsJSON)
	b.WriteString("\nSi el estudiante ya dominaba una habilidad relacionada y vuelve a fallar, recuérdaselo con amabilidad para forzar recuperación de memoria. No inventes habilidades ausentes del JSON.")

	return b.String(), nil
}

func promoteProfileOnPass(profile domain.CognitiveProfile, level domain.Level, now time.Time) domain.CognitiveProfile {
	skillID := fmt.Sprintf("level_%d", level.ID)
	found := false

	for i := range profile.Skills {
		profile.Skills[i].LastReviewedAt = now
		if profile.Skills[i].ID == skillID {
			// Subir confianza: learning/mastered consolidado al aprobar.
			profile.Skills[i].Status = domain.SkillStatusMastered
			found = true
		}
	}

	if !found {
		profile.Skills = append(profile.Skills, domain.StudentSkill{
			ID:             skillID,
			Status:         domain.SkillStatusMastered,
			LastReviewedAt: now,
		})
	}

	return profile
}

func parseVerdictFromContent(content string) (bool, string, error) {
	cleaned := strings.TrimSpace(content)
	cleaned = strings.TrimPrefix(cleaned, "```json")
	cleaned = strings.TrimPrefix(cleaned, "```")
	cleaned = strings.TrimSuffix(cleaned, "```")
	cleaned = strings.TrimSpace(cleaned)

	var verdict evaluationVerdict
	if err := json.Unmarshal([]byte(cleaned), &verdict); err != nil {
		return false, "", fmt.Errorf("no se pudo parsear el veredicto de Grok (%q): %w", content, err)
	}

	return verdict.Passed, verdict.Feedback, nil
}
