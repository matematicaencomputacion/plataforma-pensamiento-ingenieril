package domain

import (
	"encoding/json"
	"fmt"
	"strings"
)

// ConceptID es la clave estable de un concepto de aprendizaje compartido.
type ConceptID string

// TranscriptSegment es un bloque de transcripción con marcas de tiempo (segundos).
type TranscriptSegment struct {
	StartSec float64 `json:"start_sec"`
	EndSec   float64 `json:"end_sec"`
	Text     string  `json:"text"`
}

// MediaResource agrupa video + transcripción para un idioma (Hito 8 i18n).
type MediaResource struct {
	ResourceURL string              `json:"resource_url"`
	Transcript  []TranscriptSegment `json:"transcript"`
}

// HasURL indica si hay un recurso de video.
func (m MediaResource) HasURL() bool {
	return strings.TrimSpace(m.ResourceURL) != ""
}

// HasTranscript indica si hay segmentos de transcripción utilizables.
func (m MediaResource) HasTranscript() bool {
	return len(m.Transcript) > 0
}

// HasContent indica video + transcripción (stage completo).
func (m MediaResource) HasContent() bool {
	return m.HasURL() && m.HasTranscript()
}

// Concept describe una unidad cognitiva reutilizable en el curriculum.
type Concept struct {
	ID      ConceptID `json:"id"`
	Title   string    `json:"title"`
	Summary string    `json:"summary"`
	Track   string    `json:"track,omitempty"`
	Tags    []string  `json:"tags,omitempty"`
	Source  string    `json:"source,omitempty"`
	// ResourceURL (legado) apunta a un recurso multimedia opcional (p. ej. YouTube).
	ResourceURL string `json:"resource_url,omitempty"`
	// Transcript (legado) es la transcripción sincronizable del recurso.
	Transcript []TranscriptSegment `json:"transcript,omitempty"`
	// Resources mapea idioma ("es"|"en") → recurso multimedia (preferido sobre legado).
	Resources map[string]MediaResource `json:"resources,omitempty"`
}

// HasMedia indica si el concepto declara algún recurso usable en InteractiveStage.
func (c Concept) HasMedia() bool {
	if _, ok := c.MediaFor("es"); ok {
		return true
	}
	if _, ok := c.MediaFor("en"); ok {
		return true
	}
	for lang := range c.Resources {
		if m, ok := c.MediaFor(lang); ok && m.HasURL() {
			return true
		}
	}
	return strings.TrimSpace(c.ResourceURL) != ""
}

// MediaFor resuelve el recurso para un idioma con fallback legado y entre idiomas.
func (c Concept) MediaFor(lang string) (MediaResource, bool) {
	lang = strings.ToLower(strings.TrimSpace(lang))
	if lang == "" {
		lang = "es"
	}

	if c.Resources != nil {
		if m, ok := c.Resources[lang]; ok && m.HasURL() {
			return cloneMediaResource(m), true
		}
	}

	// Legado: resource_url/transcript se interpreta como locale "es".
	if lang == "es" && strings.TrimSpace(c.ResourceURL) != "" {
		return MediaResource{
			ResourceURL: c.ResourceURL,
			Transcript:  append([]TranscriptSegment(nil), c.Transcript...),
		}, true
	}

	for _, fallback := range []string{"es", "en"} {
		if fallback == lang {
			continue
		}
		if c.Resources != nil {
			if m, ok := c.Resources[fallback]; ok && m.HasURL() {
				return cloneMediaResource(m), true
			}
		}
		if fallback == "es" && strings.TrimSpace(c.ResourceURL) != "" {
			return MediaResource{
				ResourceURL: c.ResourceURL,
				Transcript:  append([]TranscriptSegment(nil), c.Transcript...),
			}, true
		}
	}

	return MediaResource{}, false
}

// AvailableMediaLocales retorna los idiomas con al menos URL de video.
func (c Concept) AvailableMediaLocales() []string {
	seen := map[string]struct{}{}
	var out []string
	add := func(lang string) {
		lang = strings.ToLower(strings.TrimSpace(lang))
		if lang == "" {
			return
		}
		if _, ok := seen[lang]; ok {
			return
		}
		seen[lang] = struct{}{}
		out = append(out, lang)
	}
	for lang, m := range c.Resources {
		if m.HasURL() {
			add(lang)
		}
	}
	if strings.TrimSpace(c.ResourceURL) != "" {
		add("es")
	}
	return out
}

// ActiveTranscriptSegment retorna el bloque activo (locale por defecto: es).
func (c Concept) ActiveTranscriptSegment(atSec float64) (TranscriptSegment, bool) {
	return c.ActiveTranscriptSegmentFor("es", atSec)
}

// ActiveTranscriptSegmentFor retorna el bloque activo para un idioma.
func (c Concept) ActiveTranscriptSegmentFor(lang string, atSec float64) (TranscriptSegment, bool) {
	media, ok := c.MediaFor(lang)
	if !ok || !media.HasTranscript() {
		return TranscriptSegment{}, false
	}
	return activeTranscriptSegment(media.Transcript, atSec)
}

func activeTranscriptSegment(transcript []TranscriptSegment, atSec float64) (TranscriptSegment, bool) {
	for _, seg := range transcript {
		if atSec >= seg.StartSec && atSec < seg.EndSec {
			return seg, true
		}
	}
	if len(transcript) > 0 && atSec >= transcript[len(transcript)-1].EndSec {
		return transcript[len(transcript)-1], true
	}
	return TranscriptSegment{}, false
}

func cloneMediaResource(m MediaResource) MediaResource {
	out := MediaResource{ResourceURL: m.ResourceURL}
	if len(m.Transcript) > 0 {
		out.Transcript = append([]TranscriptSegment(nil), m.Transcript...)
	}
	return out
}

// CurriculumEdge es una arista del grafo unificado de conceptos (con rationale curable).
type CurriculumEdge struct {
	From        string  `json:"from"`
	To          string  `json:"to"`
	Kind        string  `json:"kind"`
	Strength    float64 `json:"strength"`
	RationaleES string  `json:"rationale_es"`
	Source      string  `json:"source"`
}

// Edge kinds del esquema unificado vectorial / PPI.
const (
	EdgeKindRequires    = "requires"
	EdgeKindDeepens     = "deepens"
	EdgeKindContinues   = "continues"
	EdgeKindAlternative = "alternative"
)

// PrerequisiteKind distingue el destino de un prerrequisito.
type PrerequisiteKind string

const (
	PrerequisiteKindLesson  PrerequisiteKind = "lesson"
	PrerequisiteKindConcept PrerequisiteKind = "concept"
)

// Prerequisite expresa una dependencia hacia una lección o un concepto.
type Prerequisite struct {
	Kind  PrerequisiteKind `json:"kind"`
	RefID string           `json:"ref_id"`
}

// LessonPrerequisite construye un prerrequisito de lección.
func LessonPrerequisite(lessonID string) Prerequisite {
	return Prerequisite{Kind: PrerequisiteKindLesson, RefID: lessonID}
}

// ConceptPrerequisite construye un prerrequisito de concepto.
func ConceptPrerequisite(conceptID ConceptID) Prerequisite {
	return Prerequisite{Kind: PrerequisiteKindConcept, RefID: string(conceptID)}
}

// CompetencyLevel indica el grado de dominio esperado sobre un concepto.
type CompetencyLevel string

const (
	CompetencyIntroductory CompetencyLevel = "introductory"
	CompetencyDeveloping   CompetencyLevel = "developing"
	CompetencyProficient   CompetencyLevel = "proficient"
)

// Competency vincula un ConceptID con un nivel de competencia esperado.
type Competency struct {
	ConceptID ConceptID       `json:"concept_id"`
	Level     CompetencyLevel `json:"level"`
}

// ConceptSet es el contrato compartido para entidades que portan conceptos.
type ConceptSet interface {
	ConceptIDs() []ConceptID
}

// PrerequisiteList soporta JSON estructurado y el formato legado []string de lecciones.
type PrerequisiteList []Prerequisite

// UnmarshalJSON acepta [{"kind":"lesson","ref_id":"..."}] o ["lesson-id"].
func (p *PrerequisiteList) UnmarshalJSON(data []byte) error {
	var structured []Prerequisite
	if err := json.Unmarshal(data, &structured); err == nil {
		*p = structured
		return nil
	}

	var legacy []string
	if err := json.Unmarshal(data, &legacy); err != nil {
		return fmt.Errorf("prerequisites: formato inválido: %w", err)
	}

	out := make(PrerequisiteList, len(legacy))
	for i, id := range legacy {
		out[i] = LessonPrerequisite(id)
	}
	*p = out
	return nil
}

// LessonIDs retorna solo los prerrequisitos de tipo lección.
func (p PrerequisiteList) LessonIDs() []string {
	ids := make([]string, 0, len(p))
	for _, item := range p {
		kind := item.Kind
		if kind == "" {
			kind = PrerequisiteKindLesson
		}
		if kind == PrerequisiteKindLesson && item.RefID != "" {
			ids = append(ids, item.RefID)
		}
	}
	return ids
}

// ConceptIDs retorna solo los prerrequisitos de tipo concepto.
func (p PrerequisiteList) ConceptIDs() []ConceptID {
	ids := make([]ConceptID, 0, len(p))
	for _, item := range p {
		if item.Kind == PrerequisiteKindConcept && item.RefID != "" {
			ids = append(ids, ConceptID(item.RefID))
		}
	}
	return ids
}
