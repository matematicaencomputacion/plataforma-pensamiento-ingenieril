package main

import (
	"bytes"
	"embed"
	"io"
	"io/fs"
	"log"
	"net/http"
	"os"
	"path/filepath"
	"strings"
)

//go:embed all:static
var embeddedStatic embed.FS

// resolveStaticDir returns an on-disk Trunk dist directory for local/dev.
// Cloud Run prefers the embedded copy baked at image build time.
func resolveStaticDir() string {
	if dir := strings.TrimSpace(os.Getenv("STATIC_DIR")); dir != "" {
		return dir
	}

	candidates := []string{
		"static",
		filepath.Join("web", "dist"),
		filepath.Join("..", "web", "dist"),
		"public",
	}
	for _, candidate := range candidates {
		if isTrunkBuiltDir(candidate) {
			return candidate
		}
	}
	return "static"
}

func indexPath(dir string) string {
	return filepath.Join(dir, "index.html")
}

func isTrunkBuiltDir(dir string) bool {
	body, err := os.ReadFile(indexPath(dir))
	if err != nil {
		return false
	}
	return isTrunkBuiltIndex(body)
}

// isTrunkBuiltIndex rejects the source web/index.html (data-trunk hooks) and
// requires Trunk's Wasm bootstrap injection.
func isTrunkBuiltIndex(body []byte) bool {
	if len(body) == 0 || bytes.Contains(body, []byte("data-trunk")) {
		return false
	}
	return bytes.Contains(body, []byte("import init")) ||
		bytes.Contains(body, []byte("wasmBindings"))
}

type spaRoot struct {
	fsys   http.FileSystem
	source string
}

func openSPARoot() (spaRoot, bool) {
	if body, err := embeddedStatic.ReadFile("static/index.html"); err == nil && isTrunkBuiltIndex(body) {
		sub, err := fs.Sub(embeddedStatic, "static")
		if err != nil {
			log.Printf("SPA: embed Sub(static) falló: %v", err)
		} else {
			if stamp, err := embeddedStatic.ReadFile("static/ppi-build.txt"); err == nil {
				log.Printf("SPA: embed stamp:\n%s", strings.TrimSpace(string(stamp)))
			} else {
				log.Printf("SPA: embed sin ppi-build.txt (%v)", err)
			}
			return spaRoot{fsys: http.FS(sub), source: "embed:static"}, true
		}
	}

	dir := resolveStaticDir()
	if !isTrunkBuiltDir(dir) {
		if _, err := os.Stat(indexPath(dir)); err == nil {
			log.Printf(
				"SPA: %s/index.html NO es dist de Trunk (parece fuente data-trunk o incompleto) — no se serve SPA",
				dir,
			)
		} else {
			log.Printf("SPA: sin dist Trunk en embed ni en %q — modo solo API", dir)
		}
		return spaRoot{}, false
	}
	if stamp, err := os.ReadFile(filepath.Join(dir, "ppi-build.txt")); err == nil {
		log.Printf("SPA: disk stamp (%s):\n%s", dir, strings.TrimSpace(string(stamp)))
	}
	return spaRoot{fsys: http.Dir(dir), source: dir}, true
}

// withSPA serves Trunk assets + index.html fallback for non-/api routes.
func withSPA(api http.Handler, root spaRoot) http.Handler {
	if root.fsys == nil {
		return api
	}
	log.Printf("SPA: sirviendo estáticos desde %s", root.source)
	spa := spaHandler(root.fsys)
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

func spaHandler(fsys http.FileSystem) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		name := strings.TrimPrefix(pathClean(r.URL.Path), "/")
		if name != "" && name != "." {
			if serveFSFile(w, r, fsys, name) {
				return
			}
		}
		if !serveFSFile(w, r, fsys, "index.html") {
			http.NotFound(w, r)
		}
	})
}

func serveFSFile(w http.ResponseWriter, r *http.Request, fsys http.FileSystem, name string) bool {
	f, err := fsys.Open(name)
	if err != nil {
		return false
	}
	defer f.Close()

	stat, err := f.Stat()
	if err != nil || stat.IsDir() {
		return false
	}

	rs, ok := f.(io.ReadSeeker)
	if !ok {
		return false
	}
	setStaticCacheHeaders(w, name)
	http.ServeContent(w, r, filepath.Base(name), stat.ModTime(), rs)
	return true
}

func setStaticCacheHeaders(w http.ResponseWriter, name string) {
	base := filepath.Base(name)
	switch {
	case base == "index.html" || base == "ppi-build.txt":
		w.Header().Set("Cache-Control", "no-store, max-age=0, must-revalidate")
	case strings.HasSuffix(base, ".wasm") ||
		strings.Contains(base, "-web-") ||
		strings.Contains(base, "styles-"):
		// Trunk content-hashed assets
		w.Header().Set("Cache-Control", "public, max-age=31536000, immutable")
	default:
		w.Header().Set("Cache-Control", "no-cache")
	}
}

// readSpaBuildStamp returns ppi-build.txt from embed, else disk STATIC_DIR.
func readSpaBuildStamp() string {
	if b, err := embeddedStatic.ReadFile("static/ppi-build.txt"); err == nil {
		return strings.TrimSpace(string(b))
	}
	dir := resolveStaticDir()
	if b, err := os.ReadFile(filepath.Join(dir, "ppi-build.txt")); err == nil {
		return strings.TrimSpace(string(b))
	}
	return ""
}

func pathClean(p string) string {
	return filepath.ToSlash(filepath.Clean("/" + p))
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
