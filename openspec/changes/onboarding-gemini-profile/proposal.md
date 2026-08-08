## Why

El onboarding ya captura el relato del alumno y confirma una síntesis de perfil, pero la clasificación aún es un mock por keywords. Eso no escala al “mapa de migas” hacia Neo4j ni refleja el contexto real del estudiante.

> **Nota de sincronización (2026-08-08):** el proveedor LLM implementado es **xAI Grok**
> (adapter `backend/internal/adapters/xai`, API OpenAI-compatible), no Gemini/Vertex como
> proponía el borrador original. El pivote se hizo durante la implementación
> (commit `fix(ai): usar xAI Grok 4.5 en vez de Groq`) para reusar la misma key/proveedor
> que la tutora de evaluación. Estos artefactos quedan actualizados a esa realidad.

## What Changes

- Backend Go: puerto de clasificación de perfil + adapter xAI Grok (chat completions con `response_format: json_object`) + `POST /api/learner/profile/synthesize`.
- Frontend `/exercise` onboarding: “Enviar para análisis” llama a esa API (ya no al mock automático) y muestra loading/error antes de pasar a `reviewing`.
- Config local: `GROK_API_KEY` (alias `XAI_API_KEY`), `GROK_MODEL`, `XAI_BASE_URL` en `.env` / `.env.example` (sin secretos en Git).
- Persistencia Neo4j: **fuera de este hito** (sigue el `saveLearnerProfile` simulado / log).

### Alcance incluido

- Clasificación estructurada `{ purpose, urgency, vision, stack }` desde texto libre.
- Tests unitarios del use case con LLM mockeado.
- Wiring Qwik → Go existente (`API_BASE_URL`).

### Fuera de alcance

- Escritura real a Neo4j.
- Auth JWT del alumno.
- Cambiar la tutora Grok de evaluación de código (comparte proveedor, no implementación).
- Pyodide / checks del harness W3 (sigue en `w3-exercise-harness`).

### Plan de rollback

- Feature flag / env `LEARNER_PROFILE_LLM=mock` para volver al clasificador por keywords.
- Remover la ruta HTTP sin tocar el resto del harness.

## Capabilities

### New Capabilities

- `learner-profile-synthesis`: Clasificación del texto de onboarding vía xAI Grok y contrato HTTP hacia el frontend.

### Modified Capabilities

- (ninguna en `openspec/specs/` principal; el harness W3 queda intacto a nivel de requirements del change previo)

## Impact

- `backend/`: domain, usecases, `adapters/xai` (+ `adapters/keyword` como mock), handlers, `main.go`, `internal/config/grok.go`.
- `frontend/`: `onboarding-layout` / servicio de análisis; estados UI analyzing/error.
- Dependencias: solo `net/http` estándar (API OpenAI-compatible de xAI); sin SDK de Google.
- Costo/latencia: llamada a Grok por cada “Enviar para análisis” (timeout 45s).
