## Slide 1 — Hook
Forgot-password decía que mandaría mail, pero no había mailer: el mensaje era cosmético.

## Slide 2 — Insight
El change original dejó el correo como non-goal; en prod con `PPI_EXPOSE_RESET_TOKEN=0` no había canal para el link.

## Slide 3 — Move
- `domain.Mailer` + adapter SMTP genérico
- `ForgotPassword` envía link con `APP_PUBLIC_URL`
- Env: `SMTP_HOST/PORT/USERNAME/PASSWORD/FROM`

## Slide 4 — Proof
- `go test ./internal/usecases ./internal/config`
- Configurar secrets en Cloud Run y probar forgot

## Slide 5 — Ask
Merge + setear SMTP en Cloud Run (y `APP_PUBLIC_URL=https://ingenieria.wecgat.com.ar`).
