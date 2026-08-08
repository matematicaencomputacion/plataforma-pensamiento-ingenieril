## 1. Backend — dominio y use case

- [x] 1.1 Tipos `LearnerProfileSynthesis` + puerto `ProfileClassifier` en domain
- [x] 1.2 Use case `SynthesizeLearnerProfile` (validación ≥12 chars, timeout context)
- [x] 1.3 Clasificador mock por keywords (mismo contrato que el frontend actual)
- [x] 1.4 Tests unitarios del use case con classifier fake / mock

## 2. Adapter LLM (implementado con xAI Grok; supersede al plan Vertex/Gemini)

- [x] 2.1 Adapter xAI Grok (`internal/adapters/xai`) con env (`GROK_API_KEY`/`XAI_API_KEY`, `GROK_MODEL`, `XAI_BASE_URL`)
- [x] 2.2 Prompt de sistema + parseo JSON estricto (strip fences + `response_format: json_object`)
- [x] 2.3 Selección mock vs grok vía `LEARNER_PROFILE_LLM` (con fallback a mock si falla el arranque de Grok)
- [x] 2.4 Carga `.env` desde raíz del monorepo y resolución de path relativo de credenciales

## 3. HTTP + frontend

- [x] 3.1 Handler `POST /api/learner/profile/synthesize` + registro en `main.go`
- [x] 3.2 Cliente frontend + wire “Enviar para análisis” (loading/error; sin mock silencioso)
- [x] 3.3 Actualizar `.env.example` con variables del proveedor LLM (`GROK_API_KEY`, `GROK_MODEL`, `LEARNER_PROFILE_LLM`)
- [x] 3.4 `go test ./...` y `cd frontend && npm run build` en verde
