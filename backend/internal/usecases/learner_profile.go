package usecases

import (
	"context"
	"errors"
	"strings"
	"time"
	"unicode/utf8"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

const (
	minLearnerNotesRunes = 12
	defaultClassifyTimeout = 45 * time.Second
)

var (
	ErrLearnerNotesTooShort = errors.New("raw_notes demasiado cortas")
	ErrProfileClassify      = errors.New("error al clasificar perfil")
)

// LearnerProfileService orquesta la síntesis del perfil de onboarding.
type LearnerProfileService struct {
	classifier domain.ProfileClassifier
	timeout    time.Duration
}

func NewLearnerProfileService(classifier domain.ProfileClassifier) *LearnerProfileService {
	return &LearnerProfileService{
		classifier: classifier,
		timeout:    defaultClassifyTimeout,
	}
}

// Synthesize valida el relato y delega en el ProfileClassifier.
func (s *LearnerProfileService) Synthesize(
	ctx context.Context,
	rawNotes string,
) (domain.LearnerProfileSynthesis, error) {
	trimmed := strings.TrimSpace(rawNotes)
	if utf8.RuneCountInString(trimmed) < minLearnerNotesRunes {
		return domain.LearnerProfileSynthesis{}, ErrLearnerNotesTooShort
	}

	cctx, cancel := context.WithTimeout(ctx, s.timeout)
	defer cancel()

	out, err := s.classifier.Classify(cctx, trimmed)
	if err != nil {
		return domain.LearnerProfileSynthesis{}, errors.Join(ErrProfileClassify, err)
	}
	return out, nil
}
