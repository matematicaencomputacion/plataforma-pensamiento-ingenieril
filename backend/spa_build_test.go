package main

import (
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"
)

func TestSpaBuildHandler_NoStamp(t *testing.T) {
	rec := httptest.NewRecorder()
	ppiBuildID = "test-build-id"
	spaBuildHandler(rec, httptest.NewRequest(http.MethodGet, "/api/spa-build", nil))
	if rec.Code != http.StatusOK {
		t.Fatalf("code=%d", rec.Code)
	}
	body := rec.Body.String()
	// Without embed stamp, handler still reports ldflags id.
	if !strings.Contains(body, "test-build-id") && !strings.Contains(body, "id=") {
		t.Fatalf("unexpected body=%q", body)
	}
	if cc := rec.Header().Get("Cache-Control"); cc != "no-store" {
		t.Fatalf("Cache-Control=%q", cc)
	}
}

func TestSetStaticCacheHeaders(t *testing.T) {
	t.Parallel()
	rec := httptest.NewRecorder()
	setStaticCacheHeaders(rec, "index.html")
	if !strings.Contains(rec.Header().Get("Cache-Control"), "no-store") {
		t.Fatalf("index cache: %q", rec.Header().Get("Cache-Control"))
	}
	rec = httptest.NewRecorder()
	setStaticCacheHeaders(rec, "plataforma-pensamiento-ingenieril-web-abc_bg.wasm")
	if !strings.Contains(rec.Header().Get("Cache-Control"), "immutable") {
		t.Fatalf("wasm cache: %q", rec.Header().Get("Cache-Control"))
	}
}
