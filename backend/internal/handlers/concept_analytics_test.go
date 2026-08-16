package handlers_test

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/crypto"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/jwtauth"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/handlers"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories/sqlite"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/usecases"
)

func newAnalyticsHandler(t *testing.T) (*handlers.ConceptAnalyticsHandler, *handlers.AuthHandler) {
	t.Helper()
	db, err := sqlite.OpenDB(":memory:")
	if err != nil {
		t.Fatalf("db: %v", err)
	}
	t.Cleanup(func() { _ = db.Close() })
	users, err := sqlite.NewUserRepository(db)
	if err != nil {
		t.Fatalf("users: %v", err)
	}
	events, err := sqlite.NewConceptEventRepository(db)
	if err != nil {
		t.Fatalf("events: %v", err)
	}
	authSvc := usecases.NewAuthService(
		users,
		crypto.NewBcryptHasher(),
		jwtauth.NewHS256Issuer("test-secret"),
		usecases.AuthOptions{ExposeResetToken: true},
	)
	analytics := usecases.NewConceptAnalyticsService(authSvc, events)
	return handlers.NewConceptAnalyticsHandler(analytics), handlers.NewAuthHandler(authSvc)
}

func registerAnalyticsUser(t *testing.T, auth *handlers.AuthHandler, email string) string {
	t.Helper()
	body := []byte(`{"email":"` + email + `","password":"clave1234"}`)
	req := httptest.NewRequest(http.MethodPost, "/api/auth/register", bytes.NewReader(body))
	rec := httptest.NewRecorder()
	auth.Register(rec, req)
	if rec.Code != http.StatusCreated {
		t.Fatalf("register %d %s", rec.Code, rec.Body.String())
	}
	var resp map[string]any
	_ = json.Unmarshal(rec.Body.Bytes(), &resp)
	token, _ := resp["token"].(string)
	if token == "" {
		t.Fatal("missing token")
	}
	return token
}

func TestConceptEventsUnauthorized(t *testing.T) {
	h, _ := newAnalyticsHandler(t)
	req := httptest.NewRequest(http.MethodPost, "/api/concept-events", bytes.NewReader([]byte(`{"type":"concept_dwell","partition_id":1}`)))
	rec := httptest.NewRecorder()
	h.Record(rec, req)
	if rec.Code != http.StatusUnauthorized {
		t.Fatalf("want 401, got %d %s", rec.Code, rec.Body.String())
	}
}

func TestConceptEventsRejectsStudentCode(t *testing.T) {
	h, auth := newAnalyticsHandler(t)
	token := registerAnalyticsUser(t, auth, "code@ppi.local")
	body := []byte(`{"type":"learn_validate_fail","step_id":"py-01","code":"print(1)"}`)
	req := httptest.NewRequest(http.MethodPost, "/api/concept-events", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer "+token)
	rec := httptest.NewRecorder()
	h.Record(rec, req)
	if rec.Code != http.StatusBadRequest {
		t.Fatalf("want 400, got %d %s", rec.Code, rec.Body.String())
	}
	if !bytes.Contains(rec.Body.Bytes(), []byte("ADR 002")) {
		t.Fatalf("ADR 002 mention: %s", rec.Body.String())
	}
}

func TestConceptEventsUnknownType(t *testing.T) {
	h, auth := newAnalyticsHandler(t)
	token := registerAnalyticsUser(t, auth, "type@ppi.local")
	body := []byte(`{"type":"page_view"}`)
	req := httptest.NewRequest(http.MethodPost, "/api/concept-events", bytes.NewReader(body))
	req.Header.Set("Authorization", "Bearer "+token)
	rec := httptest.NewRecorder()
	h.Record(rec, req)
	if rec.Code != http.StatusBadRequest {
		t.Fatalf("want 400, got %d %s", rec.Code, rec.Body.String())
	}
}

func TestConceptEventsRecordAndSummary(t *testing.T) {
	h, auth := newAnalyticsHandler(t)
	token := registerAnalyticsUser(t, auth, "sum@ppi.local")

	open := []byte(`{"type":"heatmap_decade_open","partition_id":1,"decade_lo":1}`)
	openReq := httptest.NewRequest(http.MethodPost, "/api/concept-events", bytes.NewReader(open))
	openReq.Header.Set("Authorization", "Bearer "+token)
	openRec := httptest.NewRecorder()
	h.Record(openRec, openReq)
	if openRec.Code != http.StatusNoContent {
		t.Fatalf("open %d %s", openRec.Code, openRec.Body.String())
	}

	fail := []byte(`{"type":"learn_validate_fail","partition_id":1,"decade_lo":1,"step_id":"py-01-hello"}`)
	failReq := httptest.NewRequest(http.MethodPost, "/api/concept-events", bytes.NewReader(fail))
	failReq.Header.Set("Authorization", "Bearer "+token)
	failRec := httptest.NewRecorder()
	h.Record(failRec, failReq)
	if failRec.Code != http.StatusNoContent {
		t.Fatalf("fail %d %s", failRec.Code, failRec.Body.String())
	}

	sumReq := httptest.NewRequest(http.MethodGet, "/api/concept-analytics", nil)
	sumReq.Header.Set("Authorization", "Bearer "+token)
	sumRec := httptest.NewRecorder()
	h.Summary(sumRec, sumReq)
	if sumRec.Code != http.StatusOK {
		t.Fatalf("summary %d %s", sumRec.Code, sumRec.Body.String())
	}
	var payload struct {
		Bottleneck *struct {
			Kind     string `json:"kind"`
			DecadeLo int    `json:"decade_lo"`
			Friction int    `json:"friction"`
		} `json:"bottleneck"`
		Partitions []any `json:"partitions"`
		Decades    []any `json:"decades"`
	}
	if err := json.Unmarshal(sumRec.Body.Bytes(), &payload); err != nil {
		t.Fatalf("decode: %v %s", err, sumRec.Body.String())
	}
	if payload.Bottleneck == nil || payload.Bottleneck.Kind != "decade" || payload.Bottleneck.DecadeLo != 1 {
		t.Fatalf("bottleneck: %#v body=%s", payload.Bottleneck, sumRec.Body.String())
	}
	if payload.Bottleneck.Friction <= 0 {
		t.Fatalf("friction: %#v", payload.Bottleneck)
	}
	if len(payload.Decades) == 0 {
		t.Fatalf("expected decade counts: %s", sumRec.Body.String())
	}

	emptyAuth := httptest.NewRequest(http.MethodGet, "/api/concept-analytics", nil)
	emptyRec := httptest.NewRecorder()
	h.Summary(emptyRec, emptyAuth)
	if emptyRec.Code != http.StatusUnauthorized {
		t.Fatalf("summary without token %d", emptyRec.Code)
	}
}

func TestConceptAnalyticsEmptyJSONArrays(t *testing.T) {
	h, auth := newAnalyticsHandler(t)
	token := registerAnalyticsUser(t, auth, "empty-sum@ppi.local")
	req := httptest.NewRequest(http.MethodGet, "/api/concept-analytics", nil)
	req.Header.Set("Authorization", "Bearer "+token)
	rec := httptest.NewRecorder()
	h.Summary(rec, req)
	if rec.Code != http.StatusOK {
		t.Fatalf("summary %d %s", rec.Code, rec.Body.String())
	}
	var payload map[string]any
	if err := json.Unmarshal(rec.Body.Bytes(), &payload); err != nil {
		t.Fatalf("decode: %v", err)
	}
	if payload["bottleneck"] != nil {
		t.Fatalf("want null bottleneck: %#v", payload["bottleneck"])
	}
	if _, ok := payload["partitions"].([]any); !ok {
		t.Fatalf("partitions array: %#v", payload["partitions"])
	}
}
