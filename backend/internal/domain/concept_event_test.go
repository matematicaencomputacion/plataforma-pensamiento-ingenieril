package domain

import "strings"
import "testing"

func TestValidateEventInput(t *testing.T) {
	t.Parallel()

	if err := ValidateEventInput(EventInput{Type: "page_view"}); err != ErrInvalidEventType {
		t.Fatalf("unknown type: %v", err)
	}
	if err := ValidateEventInput(EventInput{Type: EventConceptDwell, PartitionID: 0}); err != ErrInvalidPartitionID {
		t.Fatalf("dwell without partition: %v", err)
	}
	if err := ValidateEventInput(EventInput{Type: EventConceptDwell, PartitionID: 1}); err != nil {
		t.Fatalf("dwell ok: %v", err)
	}
	if err := ValidateEventInput(EventInput{
		Type: EventHeatmapDecadeOpen, PartitionID: 1, DecadeLo: 2,
	}); err != ErrInvalidDecade {
		t.Fatalf("bad decade: %v", err)
	}
	if err := ValidateEventInput(EventInput{
		Type: EventHeatmapDecadeOpen, PartitionID: 1, DecadeLo: 1,
	}); err != nil {
		t.Fatalf("decade ok: %v", err)
	}
	if err := ValidateEventInput(EventInput{Type: EventLearnValidateFail}); err != ErrInvalidStepID {
		t.Fatalf("fail without step: %v", err)
	}
	if err := ValidateEventInput(EventInput{
		Type: EventLearnValidateFail, StepID: "py-01-hello", DecadeLo: 1, PartitionID: 1,
	}); err != nil {
		t.Fatalf("fail ok: %v", err)
	}
	if err := ValidateEventInput(EventInput{Type: EventDUAFABOpen}); err != nil {
		t.Fatalf("fab optional locators: %v", err)
	}
}

func TestValidDecadeLo(t *testing.T) {
	t.Parallel()
	if !ValidDecadeLo(1) || !ValidDecadeLo(11) || !ValidDecadeLo(991) {
		t.Fatal("expected canonical decades")
	}
	if ValidDecadeLo(0) || ValidDecadeLo(10) || ValidDecadeLo(1001) {
		t.Fatal("rejected non-decades")
	}
}

func TestAggregateBottleneckPrefersFailedDecade(t *testing.T) {
	t.Parallel()
	events := []ConceptEvent{
		{Type: EventConceptDwell, PartitionID: 1},
		{Type: EventHeatmapDecadeOpen, PartitionID: 1, DecadeLo: 1},
		{Type: EventLearnValidateFail, PartitionID: 1, DecadeLo: 1, StepID: "py-01-hello"},
		{Type: EventLearnValidatePass, PartitionID: 2, DecadeLo: 51, StepID: "py-52-functions"},
		{Type: EventLearnStepEnter, PartitionID: 2, DecadeLo: 51, StepID: "py-52-functions"},
	}
	sum := Aggregate(events)
	if sum.Bottleneck == nil {
		t.Fatal("expected bottleneck")
	}
	if sum.Bottleneck.Kind != "decade" || sum.Bottleneck.DecadeLo != 1 {
		t.Fatalf("want decade 1, got %#v", sum.Bottleneck)
	}
	if sum.Bottleneck.Friction != 1+3 { // decade_open + 3*fail (dwell is partition-only here)
		// dwell has no decade_lo so decade friction = open(1)+fail*3 = 4
		t.Fatalf("friction=%d label=%s", sum.Bottleneck.Friction, sum.Bottleneck.Label)
	}
	if !strings.Contains(sum.Bottleneck.Label, "Década") {
		t.Fatalf("label: %q", sum.Bottleneck.Label)
	}
	if len(sum.Partitions) == 0 || len(sum.Decades) == 0 {
		t.Fatalf("expected rollups: %#v", sum)
	}
	var p1 PartitionCount
	for _, p := range sum.Partitions {
		if p.PartitionID == 1 {
			p1 = p
		}
	}
	if p1.Dwell != 1 || p1.ValidateFail != 1 || p1.Friction != 1+1+3 {
		t.Fatalf("partition 1: %#v", p1)
	}
}

func TestAggregateEmptyHasNilBottleneck(t *testing.T) {
	t.Parallel()
	sum := Aggregate(nil)
	if sum.Bottleneck != nil {
		t.Fatalf("expected nil bottleneck: %#v", sum.Bottleneck)
	}
	if sum.Partitions == nil || sum.Decades == nil {
		t.Fatal("arrays must be non-nil JSON []")
	}
	if len(sum.Partitions) != 0 || len(sum.Decades) != 0 {
		t.Fatalf("expected empty: %#v", sum)
	}
}

func TestAggregatePartitionFallbackWhenNoDecadeFriction(t *testing.T) {
	t.Parallel()
	sum := Aggregate([]ConceptEvent{
		{Type: EventConceptDwell, PartitionID: 3},
		{Type: EventConceptDwell, PartitionID: 3},
	})
	if sum.Bottleneck == nil || sum.Bottleneck.Kind != "partition" || sum.Bottleneck.PartitionID != 3 {
		t.Fatalf("want partition 3: %#v", sum.Bottleneck)
	}
	if sum.Bottleneck.Friction != 2 {
		t.Fatalf("friction=%d", sum.Bottleneck.Friction)
	}
}
