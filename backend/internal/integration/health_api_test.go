//go:build integration

package integration_test

import (
	"net/http"
	"net/http/httptest"
	"testing"
)

// TestHealthEndpoint is a scaffold for full-mux integration runs.
// Enable: PPI_HARNESS_INTEGRATION=1 make harness-integration
func TestHealthEndpoint(t *testing.T) {
	mux := newTestMux(t)

	req := httptest.NewRequest(http.MethodGet, "/api/health", nil)
	rec := httptest.NewRecorder()
	mux.ServeHTTP(rec, req)

	if rec.Code != http.StatusOK {
		t.Fatalf("health status=%d body=%s", rec.Code, rec.Body.String())
	}
}
