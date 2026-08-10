// Package smtpmail sends transactional mail via net/smtp.
package smtpmail

import (
	"context"
	"fmt"
	"net"
	"net/smtp"
	"strings"

	"github.com/matematicaencomputacion/plataforma-pensamiento-ingenieril/backend/internal/domain"
)

// Config holds SMTP connection settings.
type Config struct {
	Host     string
	Port     string
	Username string
	Password string
	From     string
}

// Client implements domain.Mailer using smtp.SendMail (STARTTLS when offered).
type Client struct {
	cfg Config
}

func New(cfg Config) (*Client, error) {
	cfg.Host = strings.TrimSpace(cfg.Host)
	cfg.Port = strings.TrimSpace(cfg.Port)
	cfg.From = strings.TrimSpace(cfg.From)
	cfg.Username = strings.TrimSpace(cfg.Username)
	if cfg.Host == "" || cfg.From == "" {
		return nil, fmt.Errorf("smtp: host and from are required")
	}
	if cfg.Port == "" {
		cfg.Port = "587"
	}
	return &Client{cfg: cfg}, nil
}

func (c *Client) Send(ctx context.Context, msg domain.MailMessage) error {
	select {
	case <-ctx.Done():
		return ctx.Err()
	default:
	}

	to := strings.TrimSpace(msg.To)
	if to == "" {
		return fmt.Errorf("smtp: empty recipient")
	}
	addr := net.JoinHostPort(c.cfg.Host, c.cfg.Port)
	envelope := []byte(buildMIME(c.cfg.From, to, msg.Subject, msg.BodyText))

	var auth smtp.Auth
	if c.cfg.Username != "" {
		auth = smtp.PlainAuth("", c.cfg.Username, c.cfg.Password, c.cfg.Host)
	}
	if err := smtp.SendMail(addr, auth, c.cfg.From, []string{to}, envelope); err != nil {
		return fmt.Errorf("smtp send: %w", err)
	}
	return nil
}

func buildMIME(from, to, subject, body string) string {
	var b strings.Builder
	b.WriteString("From: ")
	b.WriteString(from)
	b.WriteString("\r\nTo: ")
	b.WriteString(to)
	b.WriteString("\r\nSubject: ")
	b.WriteString(sanitizeHeader(subject))
	b.WriteString("\r\nMIME-Version: 1.0\r\nContent-Type: text/plain; charset=UTF-8\r\n\r\n")
	b.WriteString(body)
	if !strings.HasSuffix(body, "\n") {
		b.WriteString("\r\n")
	}
	return b.String()
}

func sanitizeHeader(s string) string {
	return strings.Map(func(r rune) rune {
		if r == '\r' || r == '\n' {
			return -1
		}
		return r
	}, s)
}
