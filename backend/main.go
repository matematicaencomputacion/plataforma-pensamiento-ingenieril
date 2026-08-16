package main

import (
	"context"
	"log"
	"net/http"
	"os"
	"path/filepath"
	"strings"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/crypto"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/jwtauth"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/keyword"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/smtpmail"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/adapters/xai"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/config"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/handlers"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories/jsonstore"
	sqliterepo "github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/repositories/sqlite"
	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/usecases"
)

// Set via Docker: -ldflags "-X main.ppiBuildID=<sha>".
var ppiBuildID = "dev"

func enableCORS(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		w.Header().Set("Access-Control-Allow-Origin", "*")
		w.Header().Set("Access-Control-Allow-Methods", "POST, GET, PUT, OPTIONS")
		w.Header().Set("Access-Control-Allow-Headers", "Content-Type, Authorization")

		if r.Method == http.MethodOptions {
			w.WriteHeader(http.StatusOK)
			return
		}

		next.ServeHTTP(w, r)
	})
}

func resolveDataDir() string {
	if dir := os.Getenv("DATA_DIR"); dir != "" {
		return dir
	}

	candidates := []string{
		"data",
		filepath.Join("backend", "data"),
	}
	for _, candidate := range candidates {
		if info, err := os.Stat(candidate); err == nil && info.IsDir() {
			return candidate
		}
	}

	return "data"
}

func newProfileClassifier(ctx context.Context) (domain.ProfileClassifier, string) {
	llm := config.LoadLearnerLLMConfig()
	if llm.Provider == "mock" {
		log.Printf("clasificador de perfil: mock/keywords")
		return keyword.NewClassifier(), "mock"
	}

	classifier, err := xai.NewClassifier(ctx, xai.Config{
		APIKey:  llm.APIKey,
		Model:   llm.Model,
		BaseURL: llm.BaseURL,
	})
	if err != nil {
		log.Printf("WARN: no se pudo iniciar %s (%v); usando mock keywords", llm.Provider, err)
		return keyword.NewClassifier(), "mock-fallback"
	}
	log.Printf("clasificador de perfil: %s model=%s base=%s", llm.Provider, llm.Model, llm.BaseURL)
	return classifier, llm.Provider
}

func main() {
	repoRoot := config.ResolveMonorepoRoot()
	if err := config.LoadDotEnv(filepath.Join(repoRoot, ".env")); err != nil {
		log.Printf("WARN: no se pudo cargar .env: %v", err)
	}
	config.ResolveCredentialsPath(repoRoot)

	dataDir := resolveDataDir()
	authCfg, err := config.LoadAuthConfig()
	if err != nil {
		log.Fatalf("auth config: %v", err)
	}

	sqlitePath := authCfg.SQLitePath()
	if !filepath.IsAbs(sqlitePath) && sqlitePath != ":memory:" {
		sqlitePath = filepath.Join(repoRoot, sqlitePath)
	}
	userDB, err := sqliterepo.OpenDB(sqlitePath)
	if err != nil {
		log.Fatalf("sqlite: %v", err)
	}
	defer userDB.Close()
	userRepo, err := sqliterepo.NewUserRepository(userDB)
	if err != nil {
		log.Fatalf("user repo: %v", err)
	}
	exposeReset := usecases.ResolveExposeResetToken(authCfg.JWTSecret)
	smtpCfg := config.LoadSMTPConfig()
	var mailer domain.Mailer = domain.NopMailer{}
	if smtpCfg.Enabled() {
		client, err := smtpmail.New(smtpmail.Config{
			Host:     smtpCfg.Host,
			Port:     smtpCfg.Port,
			Username: smtpCfg.Username,
			Password: smtpCfg.Password,
			From:     smtpCfg.From,
		})
		if err != nil {
			log.Printf("WARN: SMTP configurado pero inválido (%v); sin envío de correo", err)
		} else {
			mailer = client
			log.Printf("auth mailer: SMTP %s:%s from=%s public=%s", smtpCfg.Host, smtpCfg.Port, smtpCfg.From, smtpCfg.PublicAppURL)
		}
	} else {
		log.Printf("auth mailer: SMTP no configurado (forgot-password no enviará correo)")
	}
	authService := usecases.NewAuthService(
		userRepo,
		crypto.NewBcryptHasher(),
		jwtauth.NewHS256Issuer(authCfg.JWTSecret),
		usecases.AuthOptions{
			ExposeResetToken: exposeReset,
			Mailer:           mailer,
			PublicAppURL:     smtpCfg.PublicAppURL,
		},
	)
	authHandler := handlers.NewAuthHandler(authService)
	if exposeReset {
		log.Printf("auth DX: forgot-password incluye resetToken (dev/harness)")
	}

	levelRepo := jsonstore.NewLevelRepository(jsonstore.DefaultLevelsPath(dataDir))
	profileRepo := jsonstore.NewCognitiveProfileRepository(jsonstore.DefaultCognitiveProfilesPath(dataDir))

	levelService := usecases.NewLevelService(levelRepo)
	evaluationService := usecases.NewEvaluationService(levelRepo, profileRepo)

	classifier, _ := newProfileClassifier(context.Background())
	learnerProfileService := usecases.NewLearnerProfileService(classifier)

	levelHandler := handlers.NewLevelHandler(levelService)
	evaluateHandler := handlers.NewEvaluateHandler(evaluationService)
	learnerProfileHandler := handlers.NewLearnerProfileHandler(learnerProfileService)

	mux := http.NewServeMux()
	mux.HandleFunc("GET /api/health", handlers.Health)
	mux.HandleFunc("GET /api/spa-build", spaBuildHandler)
	mux.HandleFunc("POST /api/auth/register", authHandler.Register)
	mux.HandleFunc("POST /api/auth/login", authHandler.Login)
	mux.HandleFunc("POST /api/auth/logout", authHandler.Logout)
	mux.HandleFunc("POST /api/auth/forgot-password", authHandler.ForgotPassword)
	mux.HandleFunc("POST /api/auth/reset-password", authHandler.ResetPassword)
	mux.HandleFunc("GET /api/me", authHandler.Me)
	mux.HandleFunc("/api/user/profile", authHandler.Profile)
	mux.HandleFunc("GET /api/levels/current", levelHandler.GetCurrent)
	mux.HandleFunc("GET /api/levels/{id}", levelHandler.GetByID)
	mux.HandleFunc("POST /api/evaluate", evaluateHandler.Evaluate)
	mux.HandleFunc("POST /api/learner/profile/synthesize", learnerProfileHandler.Synthesize)
	progressHandler := handlers.NewProgressHandler(authService)
	mux.HandleFunc("POST /api/progress/complete", progressHandler.Complete)
	mux.HandleFunc("POST /api/progress/reset", progressHandler.Reset)

	eventRepo, err := sqliterepo.NewConceptEventRepository(userDB)
	if err != nil {
		log.Fatalf("concept events repo: %v", err)
	}
	analyticsHandler := handlers.NewConceptAnalyticsHandler(
		usecases.NewConceptAnalyticsService(authService, eventRepo),
	)
	mux.HandleFunc("POST /api/concept-events", analyticsHandler.Record)
	mux.HandleFunc("GET /api/concept-analytics", analyticsHandler.Summary)

	spa, spaOK := openSPARoot()
	handler := enableCORS(withSPA(mux, spa))
	staticLabel := "none"
	if spaOK {
		staticLabel = spa.source
	}

	addr := listenAddr()
	log.Printf(
		"servidor iniciado: escuchando en http://localhost%s (data=%s static=%s build=%s root=%s sqlite=%s)",
		addr, dataDir, staticLabel, ppiBuildID, repoRoot, sqlitePath,
	)
	if err := http.ListenAndServe(addr, handler); err != nil {
		log.Fatalf("error al iniciar el servidor: %v", err)
	}
}

func spaBuildHandler(w http.ResponseWriter, _ *http.Request) {
	w.Header().Set("Content-Type", "text/plain; charset=utf-8")
	w.Header().Set("Cache-Control", "no-store")
	stamp := readSpaBuildStamp()
	if stamp == "" {
		stamp = "id=" + ppiBuildID + "\n(source=ldflags-only; no ppi-build.txt)"
	}
	_, _ = w.Write([]byte(stamp + "\n"))
}

// listenAddr respeta PORT (Cloud Run) y cae a :8080 en local.
func listenAddr() string {
	port := strings.TrimSpace(os.Getenv("PORT"))
	if port == "" {
		return ":8080"
	}
	if strings.HasPrefix(port, ":") {
		return port
	}
	return ":" + port
}
