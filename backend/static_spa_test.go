package main

import (
	"io"
	"net/http"
	"net/http/httptest"
	"os"
	"path/filepath"
	"strings"
	"testing"
)

func TestSPAHandler_RootAndFallback(t *testing.T) {
	t.Parallel()
	dir := t.TempDir()
	mustWrite(t, filepath.Join(dir, "index.html"), `<!doctype html><title>spa</title>
<script type="module">
import init, * as bindings from '/app.js';
window.wasmBindings = bindings;
</script>`)
	mustWrite(t, filepath.Join(dir, "app.js"), "console.log(1)")
	mustWrite(t, filepath.Join(dir, "favicon.ico"), "ico")

	mux := http.NewServeMux()
	mux.HandleFunc("GET /api/health", func(w http.ResponseWriter, _ *http.Request) {
		w.WriteHeader(http.StatusOK)
		_, _ = w.Write([]byte(`{"ok":true}`))
	})
	handler := withSPA(mux, spaRoot{fsys: http.Dir(dir), source: dir})

	t.Run("root serves index", func(t *testing.T) {
		rec := httptest.NewRecorder()
		handler.ServeHTTP(rec, httptest.NewRequest(http.MethodGet, "/", nil))
		if rec.Code != http.StatusOK {
			t.Fatalf("code=%d body=%s", rec.Code, rec.Body.String())
		}
		if !strings.Contains(rec.Body.String(), "<title>spa</title>") {
			t.Fatalf("body=%q", rec.Body.String())
		}
		if !strings.Contains(rec.Body.String(), "import init") {
			t.Fatalf("expected trunk bootstrap, body=%q", rec.Body.String())
		}
	})

	t.Run("existing asset", func(t *testing.T) {
		rec := httptest.NewRecorder()
		handler.ServeHTTP(rec, httptest.NewRequest(http.MethodGet, "/app.js", nil))
		if rec.Code != http.StatusOK {
			t.Fatalf("code=%d", rec.Code)
		}
		if rec.Body.String() != "console.log(1)" {
			t.Fatalf("body=%q", rec.Body.String())
		}
	})

	t.Run("spa route fallback", func(t *testing.T) {
		rec := httptest.NewRecorder()
		handler.ServeHTTP(rec, httptest.NewRequest(http.MethodGet, "/learn", nil))
		if rec.Code != http.StatusOK {
			t.Fatalf("code=%d", rec.Code)
		}
		if !strings.Contains(rec.Body.String(), "<title>spa</title>") {
			t.Fatalf("body=%q", rec.Body.String())
		}
	})

	t.Run("api health still works", func(t *testing.T) {
		rec := httptest.NewRecorder()
		handler.ServeHTTP(rec, httptest.NewRequest(http.MethodGet, "/api/health", nil))
		if rec.Code != http.StatusOK {
			t.Fatalf("code=%d body=%s", rec.Code, rec.Body.String())
		}
		if !strings.Contains(rec.Body.String(), `"ok":true`) {
			t.Fatalf("body=%q", rec.Body.String())
		}
	})

	t.Run("unknown api stays 404 not html", func(t *testing.T) {
		rec := httptest.NewRecorder()
		handler.ServeHTTP(rec, httptest.NewRequest(http.MethodGet, "/api/does-not-exist", nil))
		if rec.Code != http.StatusNotFound {
			t.Fatalf("code=%d body=%s", rec.Code, rec.Body.String())
		}
		body, _ := io.ReadAll(rec.Body)
		if strings.Contains(string(body), "<title>spa</title>") {
			t.Fatalf("API 404 must not return SPA html: %q", body)
		}
	})

	t.Run("post non-api not allowed", func(t *testing.T) {
		rec := httptest.NewRecorder()
		handler.ServeHTTP(rec, httptest.NewRequest(http.MethodPost, "/learn", nil))
		if rec.Code != http.StatusMethodNotAllowed {
			t.Fatalf("code=%d", rec.Code)
		}
	})
}

func TestIsTrunkBuiltIndex(t *testing.T) {
	t.Parallel()
	source := []byte(`<link data-trunk rel="rust" /><body></body>`)
	if isTrunkBuiltIndex(source) {
		t.Fatal("source index with data-trunk must be rejected")
	}
	built := []byte(`<script type="module">import init, * as bindings from '/x.js';window.wasmBindings = bindings;</script><body></body>`)
	if !isTrunkBuiltIndex(built) {
		t.Fatal("trunk dist index must be accepted")
	}
}

func TestWithSPA_MissingRoot(t *testing.T) {
	t.Parallel()
	mux := http.NewServeMux()
	mux.HandleFunc("GET /api/health", func(w http.ResponseWriter, _ *http.Request) {
		w.WriteHeader(http.StatusNoContent)
	})
	h := withSPA(mux, spaRoot{})
	rec := httptest.NewRecorder()
	h.ServeHTTP(rec, httptest.NewRequest(http.MethodGet, "/", nil))
	if rec.Code != http.StatusNotFound {
		t.Fatalf("solo-API root should 404, got %d", rec.Code)
	}
}

func TestOpenSPARoot_RejectsSourceDir(t *testing.T) {
	dir := t.TempDir()
	mustWrite(t, filepath.Join(dir, "index.html"), `<link data-trunk rel="css" href="styles.css" /><body></body>`)
	t.Setenv("STATIC_DIR", dir)
	body, _ := embeddedStatic.ReadFile("static/index.html")
	if isTrunkBuiltIndex(body) {
		t.Skip("test binary already embeds trunk dist")
	}
	if _, ok := openSPARoot(); ok {
		t.Fatal("openSPARoot must not accept data-trunk source index")
	}
}

func TestIsUnderRoot(t *testing.T) {
	t.Parallel()
	root := filepath.Clean("/app/static")
	if !isUnderRoot(root, filepath.Join(root, "a.wasm")) {
		t.Fatal("child should be under root")
	}
	if isUnderRoot(root, filepath.Clean("/app/static-evil/x")) {
		t.Fatal("sibling prefix must not count as under root")
	}
}

func mustWrite(t *testing.T, path, body string) {
	t.Helper()
	if err := os.WriteFile(path, []byte(body), 0o644); err != nil {
		t.Fatalf("write %s: %v", path, err)
	}
}
