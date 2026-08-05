## Context

El Paso 1 (`/exercise?step=py-01-home`) ya tiene máquina `drafting → reviewing → saved` y un mock `simulateAISynthesis`. ADC y service account (`qwik-gemini@…`) están en la raíz del monorepo. El backend Go expone `:8080` con CORS abierto y Clean Architecture (handlers → usecases → domain).

## Goals / Non-Goals

**Goals:**

- Clasificar texto de onboarding con Gemini Pro vía Vertex AI (ADC).
- Endpoint HTTP estable para el frontend.
- Fallback opcional a mock por keywords vía `LEARNER_PROFILE_LLM=mock`.
- Mantener el contrato UI de tarjetas `{ purpose, urgency, vision, stack }`.

**Non-Goals:**

- Neo4j write path.
- Auth de usuario.
- Ejecutar código de alumnos en servidor (ADR 002).

## Decisions

### D1 — Vertex AI + Gen AI Go SDK

- **Decisión:** Adapter en `internal/adapters/gemini` usando ADC (`GOOGLE_APPLICATION_CREDENTIALS`) y backend Vertex (`GOOGLE_CLOUD_PROJECT`, `VERTEX_LOCATION`, `GEMINI_MODEL=gemini-2.0-pro`).
- **Por qué:** Service account ya provisionado; alineado a GCP; no API key en el browser.
- **Alternativa:** Qwik `server$` → rechazada (ADR 001: API core en Go).

### D2 — Puerto de dominio `ProfileClassifier`

- **Decisión:** `Classify(ctx, rawNotes) (LearnerProfileSynthesis, error)` en domain; use case valida input y normaliza JSON.
- **Por qué:** Testeable sin red (mock en tests); swappable mock/keywords.

### D3 — Contrato HTTP

- `POST /api/learner/profile/synthesize`
- Body: `{ "raw_notes": "...", "source_step_id": "py-01-home" }`
- 200: `{ "purpose", "urgency", "vision", "stack" }`
- 400 si notes vacías / < umbral; 502 si falló el proveedor.

### D4 — Prompt + JSON estricto

- System prompt pide solo JSON con las 4 claves, en español rioplatense pedagógico, sin markdown.
- Parser tolerante: strip fences si el modelo las agrega.

### D5 — Carga de `.env` en arranque

- `main` resuelve monorepo root y carga `.env` (parser mínimo, sin nueva dep crítica si godotenv complica).
- Resuelve rutas relativas de `GOOGLE_APPLICATION_CREDENTIALS` desde la raíz del repo.

## Risks / Trade-offs

- [Cuotas/billing Vertex] → Mitigación: modelo configurable; errores claros al alumno.
- [Modelo `gemini-2.0-pro` no habilitado en la región] → Mitigación: `GEMINI_MODEL` override; documentar en `.env.example`.
- [Latencia Pro] → Mitigación: UI “Analizando…”; timeout ~45s.
- [PII en logs] → Mitigación: no loguear `raw_notes` completas en prod; solo length + step id.

## Migration Plan

1. Levantar backend con ADC.
2. Frontend apunta a endpoint nuevo.
3. Rollback: `LEARNER_PROFILE_LLM=mock` o revert del wire en onboarding.

## Open Questions

- Región Vertex definitiva (`us-central1` por defecto).
- Persistencia Neo4j en change posterior.
