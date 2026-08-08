## Context

El Paso 1 (`/exercise?step=py-01-home`) ya tiene máquina `drafting → reviewing → saved` y un mock `simulateAISynthesis`. El backend Go expone `:8080` con CORS abierto y Clean Architecture (handlers → usecases → domain). La tutora de evaluación ya usa xAI Grok, por lo que existe key y proveedor provisionados.

## Goals / Non-Goals

**Goals:**

- Clasificar texto de onboarding con xAI Grok (API OpenAI-compatible).
- Endpoint HTTP estable para el frontend.
- Fallback opcional a mock por keywords vía `LEARNER_PROFILE_LLM=mock`.
- Mantener el contrato UI de tarjetas `{ purpose, urgency, vision, stack }`.

**Non-Goals:**

- Neo4j write path.
- Auth de usuario.
- Ejecutar código de alumnos en servidor (ADR 002).

## Decisions

### D1 — xAI Grok vía API OpenAI-compatible (supersede a Vertex/Gemini)

- **Decisión:** Adapter en `internal/adapters/xai` con `net/http` estándar contra `https://api.x.ai/v1/chat/completions`. Env: `GROK_API_KEY` (alias `XAI_API_KEY`), `GROK_MODEL` (default `grok-4.5`), `XAI_BASE_URL` override. `response_format: json_object` + temperatura 0.2.
- **Por qué:** misma key/proveedor que la tutora de evaluación (una sola integración LLM que operar); sin SDK externo ni ADC; la key nunca llega al browser.
- **Historia:** el borrador original proponía Vertex/Gemini con ADC (`internal/adapters/gemini`). Se pivoteó durante la implementación — commits `feat(profile,ai): rehidratación GET profile y síntesis vía Groq` y `fix(ai): usar xAI Grok 4.5 en vez de Groq`.
- **Alternativas:** Qwik `server$` → rechazada (ADR 001: API core en Go); Vertex/Gemini → descartada para no sumar un segundo proveedor y credenciales GCP.

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
- Se mantiene la resolución de rutas relativas de credenciales (`config.ResolveCredentialsPath`) por si un adapter futuro vuelve a necesitar service accounts.

### D6 — Fallback resiliente en arranque

- Si `NewClassifier` de xAI falla (key ausente/mal formada), `main` cae al clasificador mock por keywords con un WARN en logs, en vez de abortar el arranque.

## Risks / Trade-offs

- [Cuotas/billing xAI] → Mitigación: modelo configurable (`GROK_MODEL`); errores claros al alumno (502 genérico).
- [Latencia del modelo] → Mitigación: UI “Analizando…”; timeout HTTP 45s.
- [PII en logs] → Mitigación: no loguear `raw_notes` completas en prod; solo length + step id.
- [Acoplamiento a un solo proveedor LLM] → Mitigación: puerto `ProfileClassifier` en domain; swap por config.

## Migration Plan

1. Levantar backend con `GROK_API_KEY` en `.env`.
2. Frontend apunta a endpoint nuevo.
3. Rollback: `LEARNER_PROFILE_LLM=mock` o revert del wire en onboarding.

## Open Questions

- Persistencia Neo4j en change posterior.
