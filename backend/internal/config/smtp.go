package config

import (
	"os"
	"strings"
)

// SMTPConfig outbound mail for password reset (generic SMTP).
type SMTPConfig struct {
	Host     string
	Port     string
	Username string
	Password string
	From     string
	// PublicAppURL is the HTTPS origin used in reset links (no trailing slash).
	PublicAppURL string
}

// Enabled reports whether enough env is present to attempt SMTP delivery.
func (c SMTPConfig) Enabled() bool {
	return strings.TrimSpace(c.Host) != "" && strings.TrimSpace(c.From) != ""
}

// LoadSMTPConfig reads SMTP_* and APP_PUBLIC_URL.
func LoadSMTPConfig() SMTPConfig {
	port := strings.TrimSpace(os.Getenv("SMTP_PORT"))
	if port == "" {
		port = "587"
	}
	public := strings.TrimSpace(os.Getenv("APP_PUBLIC_URL"))
	if public == "" {
		public = strings.TrimSpace(os.Getenv("PUBLIC_APP_URL"))
	}
	public = strings.TrimRight(public, "/")

	return SMTPConfig{
		Host:         strings.TrimSpace(os.Getenv("SMTP_HOST")),
		Port:         port,
		Username:     strings.TrimSpace(os.Getenv("SMTP_USERNAME")),
		Password:     os.Getenv("SMTP_PASSWORD"), // may contain spaces — do not Trim aggressively
		From:         strings.TrimSpace(os.Getenv("SMTP_FROM")),
		PublicAppURL: public,
	}
}
