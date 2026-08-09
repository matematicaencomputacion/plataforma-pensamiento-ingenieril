//go:build integration

package integration_test

import (
	"net/http"
	"net/http/httptest"
	"os"
	"testing"
)

func TestLevelsCurrent(t *testing.T) {
	if _, err := os.Stat("data/levels.json"); err != nil {
		t.Skip("data/levels.json not present in checkout — seed required for levels integration")
	}

	mux := newTestMux(t)

	req := httptest.NewRequest(http.MethodGet, "/api/levels/current", nil)
	rec := httptest.NewRecorder()
	mux.ServeHTTP(rec, req)

	if rec.Code != http.StatusOK && rec.Code != http.StatusNotFound {
		t.Fatalf("levels/current status=%d body=%s", rec.Code, rec.Body.String())
	}
}
