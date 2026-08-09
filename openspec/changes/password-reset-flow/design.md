## Context

Auth actual: registro/login JWT + bcrypt en SQLite (`UserRepository`). Leptos CSR
en `web/` habla same-origin `/api/*` vía proxy Trunk. ADR 002 intacto (sin ejecución
de código de alumnos en servidor).

## Goals / Non-Goals

**Goals**

- Flujo completo forgot → reset sin revelar existencia de email en prod.
- Tokens: secreto aleatorio, guardar solo hash, TTL (~1h), uso único.
- Dev/E2E: exposición opt-in del token en la respuesta forgot.
- Tests Go del usecase/handler; smoke E2E opcional.

**Non-Goals**

- Envío de correo real.
- Invalidar todos los JWT existentes al reset (stateless HS256).

## Decisions

1. **Endpoints**
   - `POST /api/auth/forgot-password` `{ "email" }` → siempre `200`
     `{ "message": "..." }` (+ `resetToken` si exposición habilitada y el usuario existe).
   - `POST /api/auth/reset-password` `{ "token", "password" }` → `200` `{ user, token }`
     (sesión nueva) o `400` token/password inválidos.

2. **Persistencia**: tabla `password_reset_tokens` en el mismo SQLite que users.

3. **Exposición del token**: `PPI_EXPOSE_RESET_TOKEN=1`, o default cuando
   `JWT_SECRET` es el secreto de desarrollo/harness. Prod MUST setear
   `PPI_EXPOSE_RESET_TOKEN=0` (o un JWT de producción).

4. **Frontend**: páginas dedicadas (no modal) para deep-link `?token=`.

## Risks / Trade-offs

- Sin email real, prod necesitará un Mailer en un change posterior.
- Exponer token en JSON es solo DX; abuso si se deja activo en prod.

## Migration Plan

Migración SQLite `CREATE TABLE IF NOT EXISTS` en arranque del repo de usuarios.
