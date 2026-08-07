## Context

Hoy el dominio ya modela `User` (`ID`, `Email`, `PasswordHash`, `CurrentLevel`), pero no hay registro, login ni sesión. El frontend abre directo a un workspace demo y al harness de micro-pasos; el onboarding de coaching guarda el perfil solo vía `server$` simulado (log). El PRD (HU04) pide identidad propia con marca personalizada. Este change introduce **portada + auth email/password + JWT** como base para, en changes posteriores, persistir progreso y reanudar en el micro-paso 2.

Stack vigente (ADR 001/002): Qwik + Go Clean Architecture; código de alumnos solo en cliente.

## Goals / Non-Goals

**Goals:**

- Portada de marca como primera vista pública.
- Registro e inicio de sesión con correo y contraseña.
- Emisión y uso de JWT para identificar al usuario autenticado.
- Rutas Qwik de login/registro y redirección post-login al camino de aprendizaje.
- Tests unitarios del use case de auth (hash, credenciales inválidas, email duplicado).

**Non-Goals:**

- OAuth / social login.
- Verificación obligatoria de email / magic links.
- Reset de contraseña por correo (puede quedar mensaje “próximamente”).
- Persistir perfil de coaching / progreso de micro-pasos / Neo4j.
- Saltar automático al paso 2 según onboarding completado.
- Proteger todavía todos los endpoints pedagógicos (`evaluate`, `levels`, etc.) — solo auth + `/api/me` mínimo; endurecer rutas en change de persistencia.
- Migración inmediata a PostgreSQL de producción.

## Decisions

1. **Persistencia de usuarios: SQLite (dev) detrás de un puerto de repositorio**
   - Interfaz `UserRepository` en domain/ports; implementación SQLite en `internal/repositories/sqlite` (o similar).
   - Evita el “solo demo-user” de `core-learning-engine` sin imponer Postgres aún.
   - **Alternativa considerada:** jsonstore como levels — descartada porque auth exige unicidad de email, hashing y queries simples; SQLite es el default del PRD para desarrollo.

2. **Hash de contraseña: bcrypt (cost razonable, p. ej. 12)**
   - Maduro en ecosistema Go (`golang.org/x/crypto/bcrypt`).
   - **Alternativa:** argon2id — más moderna; se puede migrar después; bcrypt alcanza para MVP con menos superficie.

3. **Sesión: JWT firmado (HS256) en header `Authorization: Bearer`**
   - Claims mínimos: `sub` (user id), `email`, `exp`.
   - Secret vía env (`JWT_SECRET`); nunca hardcodeado.
   - Frontend: `localStorage` (o cookie httpOnly en iteración futura). Para Qwik SSR, preferir cookie httpOnly si el esfuerzo cabe; si no, bearer en `localStorage` documentado como limitación XSS aceptada en MVP.
   - **Alternativa:** sesiones server-side en SQLite — más inválables; JWT es suficiente para HU04 inicial y alinea al flujo del PRD (§5 menciona JWT).

4. **API auth mínima**
   - `POST /api/auth/register` `{ email, password }` → `201` + token + user público (`id`, `email`).
   - `POST /api/auth/login` `{ email, password }` → `200` + token + user.
   - `POST /api/auth/logout` → `204` (cliente descarta token; server-stateless).
   - `GET /api/me` (Bearer requerido) → user público.
   - Errores: `400` validación, `401` credenciales, `409` email duplicado. Sin filtrar si el email existe en mensajes de login genéricos (“credenciales inválidas”).

5. **Clean Architecture**
   - Use cases: `RegisterUser`, `LoginUser`, `GetCurrentUser`.
   - Handlers HTTP delgados; dominio sin deps de JWT/bcrypt (puertos `PasswordHasher`, `TokenIssuer` si hace falta tipar).

6. **Frontend: portada + auth routes**
   - `/` → landing de marca (hero: nombre IngenierIA, una headline, CTA registro/login). El workspace demo actual se mueve a `/workspace` (o `/app`) para no mezclar portada con editor.
   - `/login`, `/register` formularios simples, mismos tokens visuales que el producto (global.css / acentos existentes; sin tema púrpura genérico).
   - Tras login/registro OK → `/exercise?step=py-01-home` (o `/workspace` si se prefiere; default: harness, donde está el coaching).
   - Link “Salir” limpia sesión y vuelve a `/`.

7. **CORS / cookies**
   - Mantener CORS actual del API; si se elige cookie httpOnly, configurar `SameSite=Lax` y origen del front (`localhost:5173`). Decisión de implementación: **Bearer + localStorage** en v1 para menos fricción SSR/CORS; anotar cookie httpOnly como follow-up de seguridad.

## Risks / Trade-offs

- **[XSS roba JWT en localStorage]** → Mitigar con CSP básica cuando exista; planear cookie httpOnly en change de seguridad; no guardar password en ningún storage.
- **[SQLite concurrente en multi-instancia]** → Aceptable en monousuario/dev; Postgres cuando haya deploy real.
- **[Rutas pedagógicas abiertas]** → El hito no endurece evaluate/levels; riesgo bajo en local; documentar que persistencia + auth-gated APIs van después.
- **[Fricción UX: forzar cuenta antes de probar]** → La landing puede incluir CTA secundario “Probar sin cuenta” opcional que apunte al harness sin sesión; si se incluye, marcar sesión anónima como no persistente (recomendado: **sí**, CTA secondary para no frenar demos).

## Migration Plan

1. Implementar en rama `feat/auth-login-portada`.
2. Migración SQLite automática al arranque (`CREATE TABLE IF NOT EXISTS users`).
3. Variables nuevas en `.env.example`: `JWT_SECRET`, `DATABASE_URL` (p. ej. `sqlite://./data/ppi.db`).
4. Pre-CI: `make test` + build frontend.
5. Rollback: revert PR; borrar archivo SQLite local no rompe el resto de la API.

## Open Questions

- ¿Bloquear `/exercise` sin sesión desde este hito, o dejar “Probar sin cuenta” hasta el change de persistencia? **Recomendación:** permitir guest al harness; exigir login solo cuando se persista progreso.
- ¿Mover exactamente el workspace actual a `/workspace` vs dejarlo embebido post-login? **Recomendación:** `/` = landing; `/workspace` = demo legacy; `/exercise` = harness.
