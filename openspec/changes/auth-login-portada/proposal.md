## Why

Sin identidad, el estudiante no puede recuperar su sesión ni su progreso; el onboarding y los micro-pasos viven en estado efímero del navegador. El PRD (HU04) exige registro/login propio con marca personalizada. Ahora, antes de persistir perfil/coaching o saltar al micro-paso 2, hace falta anclar al usuario a una cuenta (correo + contraseña) y una portada de entrada clara.

## What Changes

- Portada (landing) de marca **IngenierIA** como puerta de entrada: identidad visual, propuesta de valor breve y CTAs hacia registro/login.
- Registro e inicio de sesión con **correo + contraseña** (sin OAuth en este hito).
- Backend Go: alta de usuario, autenticación, emisión/validación de sesión (JWT) y endpoints mínimos de auth.
- Frontend Qwik: rutas `/`, `/login`, `/register` (o equivalentes) y cliente que envía credenciales y conserva la sesión.
- Tras login exitoso: redirección al workspace / camino de aprendizaje (p. ej. micro-pasos), sin exigir aún que el progreso del coaching esté hidratado desde DB.

## Capabilities

### New Capabilities

- `user-auth`: Registro, login, logout y sesión autenticada (correo + contraseña + JWT) expuesta por la API Go y consumida por Qwik.
- `brand-landing`: Portada de marca como primera vista para usuarios no autenticados, con CTAs a registro/login y acceso al producto.

### Modified Capabilities

- _(ninguna — `openspec/specs/` aún no tiene specs main; este change introduce capacidades nuevas)_

## Impact

- **Backend:** `internal/domain.User` (ya tiene `Email`/`PasswordHash`), nuevos use cases/handlers/repos de auth; hash de contraseña; middleware JWT; posibles dependencias (bcrypt/argon2, JWT). Persistencia usuarios: SQLite local (alineado al PRD de desarrollo) o jsonstore si el hito debe permanecer mínimo — decisión en `design.md`.
- **Frontend:** nuevas rutas Qwik City; formularios de auth; guarda de token/sesión; la `/` actual (workspace demo) se reordena: portada primero, workspace detrás de sesión o CTA post-login.
- **Fuera de alcance (explícito):** OAuth/social login; reset de contraseña por email (puede quedar stub); persistencia Neo4j del perfil de coaching; hidratar/saltar automático al micro-paso 2 según progreso; roles admin; verificación de email obligatoria.
- **Rollback:** feature flags o revert del PR; la API existente (`/api/health`, levels, evaluate, profile synthesize) sigue usable sin auth hasta que se protejan rutas en un change posterior.
- **Orden posterior acordado:** (1) este change → (2) persistencia de progreso/perfil → (3) reanudar/saltar al paso 2 tras onboarding guardado.
