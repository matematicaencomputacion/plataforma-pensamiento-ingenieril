# PR TED slides — Coaching voice + Cerebras synthesize + persist

## Slide 1 — Hook
El relato del alumno no llegaba a los 4 campos; “Guardar perfil” fallaba con “perfil vacío”, y faltaba el micrófono.

## Slide 2 — Insight
Solo había `CEREBRAS_API_KEY` en `.env`, pero el clasificador buscaba Grok → fallback keywords (a menudo vacío). SpeechRecognition estaba en el backlog OpenSpec sin UI.

## Slide 3 — Move
- `LEARNER_PROFILE_LLM=auto` prefiere Cerebras; botones Analizar/Guardar + mic Dictar (`ppi-speech.js`)
- Síntesis vacía → error claro; E2E `coaching.spec.ts`

## Slide 4 — Proof
`make harness` → `RESULT: PASS`

## Slide 5 — Ask
Merge cuando CI verde. Validar Cerebras localmente (API key activa) fuera de mock.
