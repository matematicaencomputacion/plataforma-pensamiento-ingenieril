# Journeys de prueba — Auth + Hub (Leptos CSR)

Documento operativo del **ADR 003**. Describe las páginas del shell Leptos, los
caminos felices/fallidos y qué suite Playwright los vigila.

## Mapa de páginas

| # | Ruta | Rol | Señales de “está vivo” |
|---|---|---|---|
| P1 | `/` | Portada | título `IngenierIA`; CTAs register/login **o** `Ir al workspace` si hay sesión |
| P2a | `/register` | Alta | `#register-email`, botón `Crear cuenta` |
| P2b | `/login` | Entrada | `#login-email`, botón `Entrar`, link forgot |
| P2c | `/forgot-password` | Recovery DX | `#forgot-email`; en dev navega con `resetToken` |
| P2d | `/reset-password` | Nueva clave | `#reset-password` (+ token en query/`#reset-token`) |
| P3 | `/workspace` | Hub operativo | `Workspace`, `Nivel actual`, nav `Portada` |

## Journey A — Auth (identidad → hub)

```mermaid
flowchart TD
  A[P1 Portada anónima] -->|Crear cuenta| B[P2a Register]
  A -->|Iniciar sesión| C[P2b Login]
  B -->|200 + JWT| D[P3 Workspace]
  C -->|200 + JWT| D
  C -->|Olvidé mi contraseña| E[P2c Forgot]
  E -->|DX resetToken| F[P2d Reset]
  E -->|sin exposición| G[Mensaje genérico sin token]
  F -->|password OK| D
  B -->|409 email taken| H[Alert + purge sesión]
  C -->|401 bad password| H
  D -->|sesión viva| I[Session bar: Portada / Workspace / Salir]
```

## Journey B — Hub (sesión sin expulsión)

```mermaid
flowchart LR
  W[P3 Workspace] -->|link Portada / brand| L[P1 Portada autenticada]
  L -->|Ir al workspace / Session bar| W
  L -->|Salir| A[P1 Portada anónima]
  W -->|Salir| A
  W -->|sin user hidratado| Login[P2b Login replace]
```

## Opciones / fallos que el harness debe contemplar

```mermaid
flowchart TD
  Start([Persona intenta entrar]) --> Q1{¿Email ya en SQLite?}
  Q1 -->|No| Reg[Register → Workspace]
  Q1 -->|Sí| Q2{¿Password correcta en ESA DB?}
  Q2 -->|Sí| Login[Login → Workspace]
  Q2 -->|No| Q3{¿Dev reset disponible?}
  Q3 -->|ppi-authctl / forgot+token| Reset[Reset password → Login]
  Q3 -->|SMTP only / token oculto| Block[Bloqueo percibido]
  Block --> Tip["Verificar DATABASE_URL\n+ PPI_EXPOSE_RESET_TOKEN\n+ make dev-set-password"]

  subgraph DBs[Riesgo operativo]
    DB1[data/ppi.db — make run default]
    DB2[data/ppi-harness.db — make harness]
  end
  Q2 -.-> DBs
```

## Matriz página ↔ test

| Journey | Spec Playwright | Estado |
|---|---|---|
| Auth P1→P2b→P3 | `auth.login.spec.ts` | Activo |
| Auth recovery P2b→P2c→P2d→P3 | `auth.reset.spec.ts` | Activo |
| Auth errores | `auth.validation.spec.ts` | Activo |
| Hub P3↔P1 + orphan JWT | `session.navigation.spec.ts` | Activo |
| Auth+Hub transversal (páginas 1→3 oiladas) | `journey.auth-hub.spec.ts` | Activo |
| Compás conceptual P2 (Wave A) | `journey.concepts.spec.ts` | Activo |
| Hub conceptual + analytics D.3 | `concepts.partitions.spec.ts` | Activo |

## Journey C — Compás conceptual (partición 2)

```mermaid
flowchart TD
  A[Register + login] --> B["Workspace · data-mastery 2 = 0"]
  B --> C[Unlock rail 1..51]
  C --> D["/learn/py-52-functions"]
  D --> E["Clic #partition-nav-2"]
  E --> F["/concepts/2"]
  F --> G["Clic drill 52"]
  G --> D
  D --> H[Validar solution_example]
  H --> I["#learn-progress-check"]
  I --> J["data-mastery 2 > 0"]
  J --> F
  F --> K["drill 52 · Completado"]
```

## Cómo ejecutar

```bash
# Journey autenticación + hub (stack efímero)
make harness-journeys

# Suite E2E completa del harness
make harness-e2e

# Suite integral (unit + e2e)
make harness
```

Reset local de una cuenta humana (solo desarrollo):

```bash
make dev-set-password EMAIL=vos@example.com PASSWORD=secreto12
# Si estabas en harness:
make dev-set-password EMAIL=vos@example.com PASSWORD=secreto12 DB=sqlite://./data/ppi-harness.db
```
