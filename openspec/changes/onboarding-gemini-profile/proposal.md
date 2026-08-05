## Why

El onboarding ya captura el relato del alumno y confirma una síntesis de perfil, pero la clasificación aún es un mock por keywords. Eso no escala al “mapa de migas” hacia Neo4j ni refleja el contexto real del estudiante. Ya existen ADC (`GOOGLE_APPLICATION_CREDENTIALS`) y un service account listos para Gemini en Vertex AI.

## What Changes

- Backend Go: puerto de clasificación de perfil + adapter Vertex/Gemini Pro + `POST /api/learner/profile/synthesize`.
- Frontend `/exercise` onboarding: “Enviar para análisis” llama a esa API (ya no al mock automático) y muestra loading/error antes de pasar a `reviewing`.
- Config local: variables de proyecto/ubicación/modelo en `.env` / `.env.example` (sin secretos en Git).
- Persistencia Neo4j: **fuera de este hito** (sigue el `saveLearnerProfile` simulado / log).

### Alcance incluido

- Clasificación estructurada `{ purpose, urgency, vision, stack }` desde texto libre.
- Tests unitarios del use case con LLM mockeado.
- Wiring Qwik → Go existente (`API_BASE_URL`).

### Fuera de alcance

- Escritura real a Neo4j.
- Auth JWT del alumno.
- Sustituir la tutora Grok de evaluación de código.
- Pyodide / checks del harness W3 (sigue en `w3-exercise-harness`).

### Plan de rollback

- Feature flag / env `LEARNER_PROFILE_LLM=mock` para volver al clasificador por keywords.
- Remover la ruta HTTP sin tocar el resto del harness.

## Capabilities

### New Capabilities

- `learner-profile-synthesis`: Clasificación del texto de onboarding vía Gemini (Vertex) y contrato HTTP hacia el frontend.

### Modified Capabilities

- (ninguna en `openspec/specs/` principal; el harness W3 queda intacto a nivel de requirements del change previo)

## Impact

- `backend/`: domain, usecases, adapters/gemini, handlers, `main.go`, `go.mod`.
- `frontend/`: `onboarding-layout` / servicio de análisis; estados UI analyzing/error.
- Dependencias: Google Gen AI / Vertex SDK en Go; ADC en runtime local.
- Costo/latencia: llamada a Gemini Pro por cada “Enviar para análisis”.
