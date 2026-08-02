package main

import (
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestEnableCORS(t *testing.T) {
	t.Parallel()

	nextCalled := false
	next := http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		nextCalled = true
		w.WriteHeader(http.StatusNoContent)
	})

	handler := enableCORS(next)

	t.Run("agrega headers CORS en GET", func(t *testing.T) {
		nextCalled = false
		req := httptest.NewRequest(http.MethodGet, "/api/health", nil)
		rec := httptest.NewRecorder()

		handler.ServeHTTP(rec, req)

		if !nextCalled {
			t.Fatal("se esperaba que el handler siguiente fuera invocado")
		}
		assertCORSHeaders(t, rec)
	})

	t.Run("responde OPTIONS sin continuar", func(t *testing.T) {
		nextCalled = false
		req := httptest.NewRequest(http.MethodOptions, "/api/evaluate", nil)
		rec := httptest.NewRecorder()

		handler.ServeHTTP(rec, req)

		if nextCalled {
			t.Fatal("OPTIONS no debe continuar al siguiente handler")
		}
		if rec.Code != http.StatusOK {
			t.Fatalf("código HTTP inesperado: got %d, want %d", rec.Code, http.StatusOK)
		}
		assertCORSHeaders(t, rec)
	})
}

func assertCORSHeaders(t *testing.T, rec *httptest.ResponseRecorder) {
	t.Helper()

	checks := map[string]string{
		"Access-Control-Allow-Origin":  "*",
		"Access-Control-Allow-Methods": "POST, GET, OPTIONS",
		"Access-Control-Allow-Headers": "Content-Type",
	}

	for header, want := range checks {
		got := rec.Header().Get(header)
		if got != want {
			t.Fatalf("%s inesperado: got %q, want %q", header, got, want)
		}
	}
}
