## 1. Backend domain & persistence

- [x] 1.1 Añadir errores de dominio para reset inválido/expirado
- [x] 1.2 Extender `UserRepository` con update de password hash + tokens de reset
- [x] 1.3 Migrar tabla `password_reset_tokens` en SQLite

## 2. Use cases & HTTP

- [x] 2.1 `AuthService.ForgotPassword` / `ResetPassword` (+ opciones de exposición/TTL)
- [x] 2.2 Handlers `ForgotPassword` / `ResetPassword` + rutas en `main.go` y mux de integration
- [x] 2.3 Tests unitarios usecase + handler HTTP
- [x] 2.4 Wire `PPI_EXPOSE_RESET_TOKEN` / secreto dev en `main.go`

## 3. Frontend Leptos

- [x] 3.1 URLs + tipos API + cliente `forgot`/`reset`
- [x] 3.2 Páginas `/forgot-password` y `/reset-password` + link en login
- [x] 3.3 Tests unitarios de contratos URL/JSON en `web/`

## 4. Validación

- [x] 4.1 Smoke E2E Playwright (forgot → reset con token expuesto)
- [x] 4.2 `make harness` (o `make test` + `web-test` + e2e) verde
- [x] 4.3 Commit + PR atómico
