## 1. Backend foundations

- [x] 1.1 Añadir config env (`JWT_SECRET`, `DATABASE_URL` sqlite) en `config/env.go` + `.env.example`
- [x] 1.2 Definir puerto `UserRepository` y errores de dominio (email duplicado, no encontrado)
- [x] 1.3 Implementar hasher bcrypt y emisor/validador JWT como adaptadores inyectables
- [x] 1.4 Implementar repositorio SQLite de users (`CREATE TABLE IF NOT EXISTS`, CRUD por email/id)
- [x] 1.5 Tests unitarios del repo SQLite (tmp DB) y del hasher/JWT

## 2. Auth use cases y HTTP

- [x] 2.1 Use case `RegisterUser` (validar email/password, hash, persistir, emitir token) + `*_test.go`
- [x] 2.2 Use case `LoginUser` (lookup, compare hash, token; mensaje genérico 401) + `*_test.go`
- [x] 2.3 Use case/handler `GET /api/me` con middleware Bearer
- [x] 2.4 Handlers `POST /api/auth/register`, `POST /api/auth/login`, `POST /api/auth/logout` + tests HTTP
- [x] 2.5 Cablear en `main.go` (abrir SQLite, inyección, rutas) sin romper CORS ni endpoints existentes

## 3. Frontend auth client

- [x] 3.1 Módulo cliente auth (register/login/me, storage de token Bearer, logout)
- [x] 3.2 Rutas `/login` y `/register` con formularios email/password y manejo de errores API
- [x] 3.3 Estado de sesión en layout/header (usuario actual o CTAs) + acción Salir

## 4. Portada y rutas de producto

- [x] 4.1 Convertir `/` en landing de marca (IngenierIA hero + headline + CTAs register/login; CTA secondary opcional “Probar sin cuenta”)
- [x] 4.2 Mover el workspace demo actual a `/workspace`
- [x] 4.3 Tras login/registro exitoso, redirigir a `/workspace`
- [x] 4.4 Estilos coherentes con `global.css` (sin tema púrpura genérico)

## 5. Verificación

- [x] 5.1 `cd backend && go test ./...` (o `make test`) en verde
- [x] 5.2 `cd frontend && npm run build` OK
- [x] 5.3 Smoke manual: registrar → login → `/api/me` → logout → landing
