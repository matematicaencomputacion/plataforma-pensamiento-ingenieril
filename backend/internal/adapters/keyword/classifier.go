package keyword

import (
	"context"
	"regexp"
	"strings"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

var (
	reStudent  = regexp.MustCompile(`\bestudiante\b`)
	reParents  = regexp.MustCompile(`\bpadres\b`)
	reFast     = regexp.MustCompile(`\brapido\b`)
	reUrgency  = regexp.MustCompile(`\burgencia\b`)
	reNoSe     = regexp.MustCompile(`\bno se\b`)
	reCoursera = regexp.MustCompile(`\bcoursera\b`)
	reDenyNB   = regexp.MustCompile(`(?s)no (conozco|se|use|usei|probado|probe).{0,48}(jupyter|colab)|(jupyter|colab).{0,48}no (conozco|se|use|usei|probado)|nunca.{0,24}(jupyter|colab)|sin (conocer|saber).{0,24}(jupyter|colab)`)
)

// Classifier implementa ProfileClassifier con reglas por keywords (offline / fallback).
type Classifier struct{}

func NewClassifier() *Classifier {
	return &Classifier{}
}

func (c *Classifier) Classify(_ context.Context, rawNotes string) (domain.LearnerProfileSynthesis, error) {
	raw := strings.TrimSpace(rawNotes)
	if raw == "" {
		return domain.LearnerProfileSynthesis{}, nil
	}
	t := normalize(raw)
	out := domain.LearnerProfileSynthesis{}

	if reStudent.MatchString(t) || reParents.MatchString(t) {
		out.Purpose = "Ayudar a su familia y ganar autonomía económica."
	}
	if reFast.MatchString(t) || reUrgency.MatchString(t) {
		out.Urgency = "Extrema - Necesita resultados inmediatos."
	}
	if reNoSe.MatchString(t) {
		out.Vision = "Exploratoria. Buscando definir un camino sólido."
	}
	if reCoursera.MatchString(t) || reDenyNB.MatchString(t) {
		out.Stack = "Coursera. Primer contacto con entornos como Jupyter/Colab."
	}
	return out, nil
}

func normalize(s string) string {
	s = strings.ToLower(s)
	replacer := strings.NewReplacer(
		"á", "a", "é", "e", "í", "i", "ó", "o", "ú", "u", "ü", "u", "ñ", "n",
		"à", "a", "è", "e", "ì", "i", "ò", "o", "ù", "u",
	)
	return replacer.Replace(s)
}
