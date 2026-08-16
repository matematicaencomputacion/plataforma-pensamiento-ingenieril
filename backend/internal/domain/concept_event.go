package domain

import (
	"strconv"
	"strings"
	"time"
)

// Closed event types for conceptual friction (Wave D.3).
const (
	EventConceptDwell      = "concept_dwell"
	EventHeatmapDecadeOpen = "heatmap_decade_open"
	EventDUAFABOpen        = "dua_fab_open"
	EventLearnStepEnter    = "learn_step_enter"
	EventLearnValidateFail = "learn_validate_fail"
	EventLearnValidatePass = "learn_validate_pass"
)

// ConceptEvent is a pedagogical-friction signal. UserID comes from the
// session; the payload MUST NOT carry email or student code (ADR 002).
type ConceptEvent struct {
	ID          string
	UserID      string
	Type        string
	PartitionID int
	DecadeLo    int
	StepID      string
	CreatedAt   time.Time
}

// EventInput is the client-supplied slice of a ConceptEvent (no user id).
type EventInput struct {
	Type        string
	PartitionID int
	DecadeLo    int
	StepID      string
}

// KnownEventType reports whether t is in the closed D.3 enum.
func KnownEventType(t string) bool {
	switch strings.TrimSpace(t) {
	case EventConceptDwell, EventHeatmapDecadeOpen, EventDUAFABOpen,
		EventLearnStepEnter, EventLearnValidateFail, EventLearnValidatePass:
		return true
	default:
		return false
	}
}

// ValidDecadeLo is the start of a coding-rail decade (1, 11, …, 991).
func ValidDecadeLo(lo int) bool {
	if lo < 1 || lo > 991 {
		return false
	}
	return (lo-1)%10 == 0
}

// ValidateEventInput checks type-specific required fields. Empty optional
// locators (0 / "") are allowed for dua_fab_open.
func ValidateEventInput(in EventInput) error {
	in.Type = strings.TrimSpace(in.Type)
	in.StepID = strings.TrimSpace(in.StepID)
	if !KnownEventType(in.Type) {
		return ErrInvalidEventType
	}
	switch in.Type {
	case EventConceptDwell:
		if in.PartitionID < 1 || in.PartitionID > 5 {
			return ErrInvalidPartitionID
		}
	case EventHeatmapDecadeOpen:
		if in.PartitionID < 1 || in.PartitionID > 5 {
			return ErrInvalidPartitionID
		}
		if !ValidDecadeLo(in.DecadeLo) {
			return ErrInvalidDecade
		}
	case EventLearnStepEnter, EventLearnValidateFail, EventLearnValidatePass:
		if in.StepID == "" {
			return ErrInvalidStepID
		}
		if in.PartitionID != 0 && (in.PartitionID < 1 || in.PartitionID > 5) {
			return ErrInvalidPartitionID
		}
		if in.DecadeLo != 0 && !ValidDecadeLo(in.DecadeLo) {
			return ErrInvalidDecade
		}
	default: // dua_fab_open
		if in.PartitionID != 0 && (in.PartitionID < 1 || in.PartitionID > 5) {
			return ErrInvalidPartitionID
		}
		if in.DecadeLo != 0 && !ValidDecadeLo(in.DecadeLo) {
			return ErrInvalidDecade
		}
	}
	return nil
}

// PartitionCount is a per-partition rollup for the session user.
type PartitionCount struct {
	PartitionID  int `json:"partition_id"`
	Dwell        int `json:"dwell"`
	DecadeOpen   int `json:"decade_open"`
	FABOpen      int `json:"fab_open"`
	ValidateFail int `json:"validate_fail"`
	ValidatePass int `json:"validate_pass"`
	Enter        int `json:"enter"`
	Friction     int `json:"friction"`
}

// DecadeCount is a per-decade rollup (optionally scoped to a partition).
type DecadeCount struct {
	DecadeLo     int `json:"decade_lo"`
	PartitionID  int `json:"partition_id"`
	Dwell        int `json:"dwell"`
	DecadeOpen   int `json:"decade_open"`
	FABOpen      int `json:"fab_open"`
	ValidateFail int `json:"validate_fail"`
	ValidatePass int `json:"validate_pass"`
	Enter        int `json:"enter"`
	Friction     int `json:"friction"`
}

// Bottleneck is the highest-friction decade (preferred) or partition.
type Bottleneck struct {
	Kind        string `json:"kind"` // "decade" | "partition"
	PartitionID int    `json:"partition_id,omitempty"`
	DecadeLo    int    `json:"decade_lo,omitempty"`
	Friction    int    `json:"friction"`
	Label       string `json:"label"`
}

// AnalyticsSummary is the tiny read model for #concept-analytics.
type AnalyticsSummary struct {
	Partitions []PartitionCount `json:"partitions"`
	Decades    []DecadeCount    `json:"decades"`
	Bottleneck *Bottleneck      `json:"bottleneck"`
}

func frictionScore(dwell, decadeOpen, fabOpen, fail int) int {
	return dwell + decadeOpen + fabOpen + 3*fail
}

type locKey struct {
	partition int
	decade    int
}

// Aggregate rolls events of one user into counts + bottleneck. Enter/pass
// are counted but do not add friction.
func Aggregate(events []ConceptEvent) AnalyticsSummary {
	parts := map[int]*PartitionCount{}
	decs := map[locKey]*DecadeCount{}

	bump := func(ev ConceptEvent) {
		if ev.PartitionID >= 1 && ev.PartitionID <= 5 {
			p := parts[ev.PartitionID]
			if p == nil {
				p = &PartitionCount{PartitionID: ev.PartitionID}
				parts[ev.PartitionID] = p
			}
			applyCount(p, ev.Type)
		}
		if ValidDecadeLo(ev.DecadeLo) {
			k := locKey{partition: ev.PartitionID, decade: ev.DecadeLo}
			d := decs[k]
			if d == nil {
				d = &DecadeCount{DecadeLo: ev.DecadeLo, PartitionID: ev.PartitionID}
				decs[k] = d
			}
			applyDecadeCount(d, ev.Type)
		}
	}

	for _, ev := range events {
		bump(ev)
	}

	out := AnalyticsSummary{
		Partitions: make([]PartitionCount, 0, len(parts)),
		Decades:    make([]DecadeCount, 0, len(decs)),
	}
	for id := 1; id <= 5; id++ {
		if p, ok := parts[id]; ok {
			p.Friction = frictionScore(p.Dwell, p.DecadeOpen, p.FABOpen, p.ValidateFail)
			out.Partitions = append(out.Partitions, *p)
		}
	}
	// Stable decade order: partition then decade_lo.
	for pid := 0; pid <= 5; pid++ {
		for lo := 1; lo <= 991; lo += 10 {
			if d, ok := decs[locKey{partition: pid, decade: lo}]; ok {
				d.Friction = frictionScore(d.Dwell, d.DecadeOpen, d.FABOpen, d.ValidateFail)
				out.Decades = append(out.Decades, *d)
			}
		}
	}
	out.Bottleneck = pickBottleneck(out.Partitions, out.Decades)
	return out
}

func applyCount(p *PartitionCount, t string) {
	switch t {
	case EventConceptDwell:
		p.Dwell++
	case EventHeatmapDecadeOpen:
		p.DecadeOpen++
	case EventDUAFABOpen:
		p.FABOpen++
	case EventLearnValidateFail:
		p.ValidateFail++
	case EventLearnValidatePass:
		p.ValidatePass++
	case EventLearnStepEnter:
		p.Enter++
	}
}

func applyDecadeCount(d *DecadeCount, t string) {
	switch t {
	case EventConceptDwell:
		d.Dwell++
	case EventHeatmapDecadeOpen:
		d.DecadeOpen++
	case EventDUAFABOpen:
		d.FABOpen++
	case EventLearnValidateFail:
		d.ValidateFail++
	case EventLearnValidatePass:
		d.ValidatePass++
	case EventLearnStepEnter:
		d.Enter++
	}
}

func pickBottleneck(parts []PartitionCount, decs []DecadeCount) *Bottleneck {
	var bestDec *DecadeCount
	for i := range decs {
		d := &decs[i]
		if d.Friction <= 0 {
			continue
		}
		if bestDec == nil || d.Friction > bestDec.Friction ||
			(d.Friction == bestDec.Friction && (d.PartitionID < bestDec.PartitionID ||
				(d.PartitionID == bestDec.PartitionID && d.DecadeLo < bestDec.DecadeLo))) {
			bestDec = d
		}
	}
	if bestDec != nil {
		hi := bestDec.DecadeLo + 9
		return &Bottleneck{
			Kind:        "decade",
			PartitionID: bestDec.PartitionID,
			DecadeLo:    bestDec.DecadeLo,
			Friction:    bestDec.Friction,
			Label:       "Década " + strconv.Itoa(bestDec.DecadeLo) + "–" + strconv.Itoa(hi) + " · alta fricción",
		}
	}
	var bestPart *PartitionCount
	for i := range parts {
		p := &parts[i]
		if p.Friction <= 0 {
			continue
		}
		if bestPart == nil || p.Friction > bestPart.Friction ||
			(p.Friction == bestPart.Friction && p.PartitionID < bestPart.PartitionID) {
			bestPart = p
		}
	}
	if bestPart == nil {
		return nil
	}
	return &Bottleneck{
		Kind:        "partition",
		PartitionID: bestPart.PartitionID,
		Friction:    bestPart.Friction,
		Label:       "Partición " + strconv.Itoa(bestPart.PartitionID) + " · alta fricción",
	}
}
