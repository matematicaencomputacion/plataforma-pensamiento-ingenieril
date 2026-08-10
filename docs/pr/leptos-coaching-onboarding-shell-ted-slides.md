## Slide 1 — Hook
El alumno autenticado ya puede empezar el Paso 1 de coaching dentro del shell Leptos, sin volver a Qwik.

## Slide 2 — Insight
Sin una ruta `/onboarding` canónica, el cutover quedaba partido: workspace en Leptos y relato de propósito atrapado en legacy. Primero hacía falta el cascarón (ruta + guard + drafting), no el analyze completo.

## Slide 3 — Move
- OpenSpec `leptos-coaching-onboarding` (proposal/design/spec/tasks).
- Ruta protegida `/onboarding` con prompts, textarea controlada y CTA analyze deshabilitado (siguiente rebanada).
- CTA «Empezar coaching» desde Workspace; `frontend/` intacto; sin cambios de contrato Go.
- E2E smoke: workspace → onboarding + redirect anónimo.

## Slide 4 — Proof
- `make harness` → RESULT: PASS (tras Pre-CI local).

## Slide 5 — Ask
Merge del shell para desbloquear la rebanada synthesize (`POST /api/learner/profile/synthesize`) sin romper el cimiento auth.
