## Why

Un aprendizaje autentificado por email/password es frágil si el alumno olvida la clave:
hoy no hay camino de recuperación y el soporte sería manual. Necesitamos un flujo
seguro, modular y testeable de **forgot → token temporal → reset**, alineado al
auth existente (Clean Architecture + Leptos CSR).

## What Changes

- Backend Go: `POST /api/auth/forgot-password` y `POST /api/auth/reset-password`.
- Persistencia: tokens de reset hasheados, TTL, single-use.
- Frontend Leptos: rutas `/forgot-password` y `/reset-password`, enlace desde login.
- En entornos no-prod / harness: el forgot MAY devolver `resetToken` en JSON
  (`PPI_EXPOSE_RESET_TOKEN`) para E2E sin SMTP.
- Pruebas unitarias Go (+ smoke E2E opcional Playwright).

## Alcance incluido

- Generación/invalidación de tokens de reset.
- Actualización segura del `password_hash`.
- UX forgot + reset en `web/` (Leptos).
- Harness / tests verdes; PR atómico.

## Fuera de alcance

- SMTP / proveedor de email real.
- Cambio de contraseña autenticado (settings).
- 2FA / magic links permanentes.
- Migración del frontend Qwik.

## Plan de rollback

Revertir el PR; tokens huérfanos en SQLite son inofensivos. Sin SMTP no hay
efecto externo.
