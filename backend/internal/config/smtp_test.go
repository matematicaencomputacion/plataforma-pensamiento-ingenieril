package config

import "testing"

func TestLoadSMTPConfig_Enabled(t *testing.T) {
	t.Setenv("SMTP_HOST", "smtp.example.com")
	t.Setenv("SMTP_PORT", "587")
	t.Setenv("SMTP_FROM", "noreply@example.com")
	t.Setenv("APP_PUBLIC_URL", "https://ingenieria.example/")
	cfg := LoadSMTPConfig()
	if !cfg.Enabled() {
		t.Fatal("expected enabled")
	}
	if cfg.PublicAppURL != "https://ingenieria.example" {
		t.Fatalf("public url: %q", cfg.PublicAppURL)
	}
}

func TestLoadSMTPConfig_DisabledWithoutHost(t *testing.T) {
	t.Setenv("SMTP_HOST", "")
	t.Setenv("SMTP_FROM", "noreply@example.com")
	if LoadSMTPConfig().Enabled() {
		t.Fatal("expected disabled")
	}
}
