## Why

El acceso actual pierde la ruta que el alumno intentaba abrir, confunde una falla transitoria de `/api/me` con una sesión cerrada y permite intentos de login sin límite. Esto degrada continuidad y seguridad justo en la puerta de entrada del producto.

## What Changes

- Limitar intentos de login por combinación de cliente y correo normalizado, con capacidad de memoria acotada, respuesta `429` y `Retry-After` sin revelar si la cuenta existe.
- Conservar y validar un destino interno `return_to` para continuar donde el alumno estaba después de autenticarse, bloqueando redirecciones externas o bucles de autenticación.
- Representar fallas transitorias de restauración de sesión como estado recuperable, conservando el token y ofreciendo reintento sin expulsar al alumno.
- Agregar pruebas unitarias, de integración y E2E para los tres comportamientos.

### Alcance incluido

- Login del backend Go y bootstrap/guards/login del frontend Leptos.
- Limitador en memoria por instancia, configurable, con máximo estricto de claves y política explícita de identidad del cliente detrás de proxy.
- Mensajería accesible y acción explícita de reintento durante indisponibilidad temporal.

### Fuera de alcance

- Migrar JWT desde `localStorage` a cookies `HttpOnly`.
- Rate limiting distribuido entre instancias de Cloud Run.
- MFA, proveedores OAuth y cambios al flujo de recuperación de contraseña.

## Capabilities

### New Capabilities

- `login-resilience`: Protege el login contra abuso, preserva destinos internos seguros y permite recuperar una sesión ante fallas transitorias.

### Modified Capabilities

Ninguna; `main` no contiene una especificación viva de autenticación bajo `openspec/specs/`.

## Impact

- Backend: handler de autenticación, configuración, composición en `backend/main.go` y nuevo limitador en memoria.
- Frontend: estado de sesión, guards de rutas protegidas y página de login.
- Contrato HTTP: `POST /api/auth/login` puede responder `429 Too Many Requests` con `Retry-After`.
- Operación: el limitador es deliberadamente local a cada instancia; una solución distribuida queda como evolución posterior.

## Plan de rollback

Revertir el commit del cambio restaura el login y los guards anteriores sin migraciones de datos. La configuración nueva tendrá valores por defecto, por lo que no deja dependencia operativa persistente.
