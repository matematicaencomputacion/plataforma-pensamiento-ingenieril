package config

import (
	"bufio"
	"os"
	"path/filepath"
	"strings"
)

// LoadDotEnv lee KEY=VALUE de un archivo .env sin sobrescribir variables ya exportadas.
func LoadDotEnv(path string) error {
	f, err := os.Open(path)
	if err != nil {
		if os.IsNotExist(err) {
			return nil
		}
		return err
	}
	defer f.Close()

	scanner := bufio.NewScanner(f)
	for scanner.Scan() {
		line := strings.TrimSpace(scanner.Text())
		if line == "" || strings.HasPrefix(line, "#") {
			continue
		}
		key, val, ok := strings.Cut(line, "=")
		if !ok {
			continue
		}
		key = strings.TrimSpace(key)
		val = strings.TrimSpace(val)
		val = strings.Trim(val, `"'`)
		if key == "" {
			continue
		}
		if _, exists := os.LookupEnv(key); exists {
			continue
		}
		_ = os.Setenv(key, val)
	}
	return scanner.Err()
}

// ResolveMonorepoRoot busca la raíz que contiene .env / Makefile subiendo desde cwd.
func ResolveMonorepoRoot() string {
	cwd, err := os.Getwd()
	if err != nil {
		return "."
	}
	dir := cwd
	for i := 0; i < 6; i++ {
		if fileExists(filepath.Join(dir, ".env")) || fileExists(filepath.Join(dir, "Makefile")) {
			return dir
		}
		parent := filepath.Dir(dir)
		if parent == dir {
			break
		}
		dir = parent
	}
	return cwd
}

// ResolveCredentialsPath hace absolutas las rutas relativas de GOOGLE_APPLICATION_CREDENTIALS.
func ResolveCredentialsPath(repoRoot string) {
	creds := strings.TrimSpace(os.Getenv("GOOGLE_APPLICATION_CREDENTIALS"))
	if creds == "" || filepath.IsAbs(creds) {
		return
	}
	abs := filepath.Join(repoRoot, creds)
	_ = os.Setenv("GOOGLE_APPLICATION_CREDENTIALS", abs)
}

func fileExists(path string) bool {
	info, err := os.Stat(path)
	return err == nil && !info.IsDir()
}
