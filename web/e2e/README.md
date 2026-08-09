# E2E smoke — Leptos CSR (`web/`) con Playwright + Chromium

## Alcance

Automatiza el **login email/password** real de la app (rutas `/login` o `/register` → `/workspace`).

**No** automatiza Google Sign-In / OAuth: ese proveedor **no existe** en el backend Go actual. Podés usar un `@gmail.com` solo como *dirección de correo* del usuario local (`PPI_E2E_EMAIL`), no como flujo de cuentas Google.

## Prerrequisitos

1. API Go en `:8080`:

   ```bash
   make run
   ```

2. Shell Leptos en `:3001` (no uses `:8080` para Trunk; choca con Go):

   ```bash
   cd web && env -u NO_COLOR trunk serve --port 3001
   ```

3. Node 20+ y browsers:

   ```bash
   cd web/e2e
   npm install
   npm run install:browsers
   ```

## Credenciales (solo env / `.env.local`)

Nunca commits de passwords. En la raíz del monorepo (o exportá en el shell):

```bash
# .env.local  (ya está en .gitignore)
export PPI_E2E_EMAIL='tu-usuario@example.com'
export PPI_E2E_PASSWORD='secreto12'
# opcional: register la primera vez
# export PPI_E2E_MODE=register
# export PPI_E2E_HEADED=1
# export PPI_E2E_BASE_URL=http://127.0.0.1:3001
```

Luego:

```bash
cd web/e2e
# carga vars si usás un helper; o exportá antes
npm test
# depuración visual:
npm run test:headed
```

Desde la raíz del monorepo:

```bash
make web-e2e
```

## Cloud / agente remoto

1. Secrets del entorno: `PPI_E2E_EMAIL`, `PPI_E2E_PASSWORD` (y opcionalmente `PPI_E2E_BASE_URL` si el preview no es local).
2. Instalar deps + Chromium: `cd web/e2e && npm ci && npx playwright install --with-deps chromium`.
3. Levantar API + Trunk (o apuntar `PPI_E2E_BASE_URL` a un deploy de preview).
4. `npm test`.

En GitHub Actions, usá repository secrets — no pongas el password del correo en el YAML.

## Selectores

Formularios Leptos actuales:

| Campo | Selector |
|---|---|
| Login email | `#login-email` |
| Login password | `#login-password` |
| Register email | `#register-email` |
| Register password | `#register-password` |
