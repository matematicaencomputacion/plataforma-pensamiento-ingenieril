package main

import (
	"log"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

// resolveStaticDir returns the Trunk/Leptos dist directory.
// Cloud Run image sets STATIC_DIR=/app/static.
func resolveStaticDir() string {
	if dir := strings.TrimSpace(os.Getenv("STATIC_DIR")); dir != "" {
		return dir
	}

	candidates := []string{
		"static",
		"public",
		filepath.Join("web", "dist"),
		filepath.Join("..", "web", "dist"),
	}
	for _, candidate := range candidates {
		if indexExists(candidate) {
			return candidate
		}
	}
	return "static"
}

func indexExists(dir string) bool {
	info, err := os.Stat(filepath.Join(dir, "index.html"))
	return err == nil && !info.IsDir()
}

// withSPA serves Trunk assets + index.html fallback for non-/api routes.
// API requests always go to the inner mux (unknown APIs stay JSON 404, not HTML).
// Implemented as a wrapper to avoid Go ServeMux conflicts between
// method-specific /api routes and a catch-all /{path...}.
func withSPA(api http.Handler, staticDir string) http.Handler {
	if !indexExists(staticDir) {
		log.Printf("SPA: sin index.html en %q — modo solo API", staticDir)
		return api
	}
	log.Printf("SPA: sirviendo estáticos desde %s", staticDir)
	spa := spaHandler(staticDir)
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api" || strings.HasPrefix(r.URL.Path, "/api/") {
			api.ServeHTTP(w, r)
			return
		}
		if r.Method != http.MethodGet && r.Method != http.MethodHead {
			http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
			return
		}
		spa.ServeHTTP(w, r)
	})
}

func spaHandler(root string) http.Handler {
	root = filepath.Clean(root)
	index := filepath.Join(root, "index.html")

	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		path := r.URL.Path
		rel := strings.TrimPrefix(filepath.Clean("/"+path), "/")
		if rel == "" || rel == "." {
			http.ServeFile(w, r, index)
			return
		}

		full := filepath.Join(root, filepath.FromSlash(rel))
		if !isUnderRoot(root, full) {
			http.NotFound(w, r)
			return
		}

		info, err := os.Stat(full)
		if err == nil && !info.IsDir() {
			http.ServeFile(w, r, full)
			return
		}

		http.ServeFile(w, r, index)
	})
}

func isUnderRoot(root, full string) bool {
	root = filepath.Clean(root)
	full = filepath.Clean(full)
	if full == root {
		return true
	}
	sep := string(os.PathSeparator)
	return strings.HasPrefix(full, root+sep)
}
