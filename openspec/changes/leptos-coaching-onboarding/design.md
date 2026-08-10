## Context

Backend ya expone:

- `POST /api/learner/profile/synthesize` `{ raw_notes, source_step_id }` →
  `{ purpose, urgency, vision, stack }` (mock keywords o Grok).
- `GET|PUT /api/user/profile` con Bearer (campos `lifePurpose`, `urgency`,
  `vision5Years`, `techStack` — verificar wire JSON exacto en handlers).

Qwik vive en `frontend/src/components/exercise-workspace/` (onboarding-layout,
coaching-interface, profile-builder). El shell canónico post-auth es Leptos
`web/` en `:3001`.

## Goals / Non-Goals

**Goals**

- Entrada autenticada a `/onboarding` sin thrash de guards.
- UI Leptos con estados `drafting | reviewing | saved`.
- Reusar API Go; `LEARNER_PROFILE_LLM=mock` en harness/E2E.
- Rebanadas atómicas (shell → synthesize → persist → polish).

**Non-Goals**

- Port completo de Speech API en el primer PR.
- Cutover destructivo de Qwik.
- Ejecutar Python en servidor (ADR 002).

## Decisions

1. **Ruta dedicada `/onboarding`** (no embeber en `/workspace`) para poder
   deep-linkear y testear guards de forma aislada.
2. **Rebanada 1 (este hito de código):** shell + prompts estáticos + textarea +
   CTAs deshabilitados/placeholders hacia analyze/save; link desde Workspace;
   E2E “llega autenticado”.
3. **Rebanadas siguientes:** client synthesize, profile builder, PUT profile,
   advance CTA, a11y/speech opcional.
4. **source_step_id** fijo inicial `leptos-onboarding-v1` hasta cablear seed microsteps.
5. **Estilos:** extender `web/styles.css` con bloque `.onboarding` coherente al
   design system existente (sin Tailwind en Leptos shell).

## Risks / Trade-offs

- Paridad incompleta temporalmente vs Qwik (aceptado; documentado en tasks).
- Sin seed de prompts desde JSON, el copy arranca hardcodeado/rioplatense y se
  sincroniza después con `docs/seeds`.

## Migration Plan

1. Shell + ruta + E2E smoke.
2. Synthesize + reviewing UI.
3. Persist profile + continue.
4. Luego Paso 2 Pyodide como change separado.

## Open Questions

- ¿Prompts de coaching se leen de la semilla W3 o copy fijo v1? → **v1 fijo**;
  seed sync en tarea posterior.
