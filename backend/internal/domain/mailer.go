package domain

import "context"

// MailMessage is a single outbound email (password reset, etc.).
type MailMessage struct {
	To      string
	Subject string
	BodyText string
}

// Mailer sends transactional email. Implementations must be safe for concurrent use.
type Mailer interface {
	Send(ctx context.Context, msg MailMessage) error
}

// NopMailer discards messages (tests / SMTP unconfigured).
type NopMailer struct{}

func (NopMailer) Send(context.Context, MailMessage) error { return nil }
