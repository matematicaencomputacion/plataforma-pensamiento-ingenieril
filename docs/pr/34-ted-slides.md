## Slide 1 — Hook
La sesión deja de mentir entre pestañas y en mobile: un logout o 401 limpia señales al instante, y el email con espacios ya no te deja fuera.

## Slide 2 — Insight
`purge_auth_storage` borraba localStorage pero el `SessionCtx` podía quedar zombie en la misma tab; `storage` no dispara en el tab origen. Forgot/reset tampoco respetaban sesión viva. En 375px el chrome se desbordaba.

## Slide 3 — Move
- Multitab `storage` + evento `ppi:auth-cleared` → clear reactivo de signals.
- Guards forgot/reset → `/workspace` si hay user.
- `sanitize_email` (trim+lower) en login/register/forgot.
- CSS `@media (max-width: 375px)` para shell/session-bar/forms.
- E2E `session.hardening.spec.ts` (trim + guards).

## Slide 4 — Proof
- `make harness` → RESULT: PASS
- Unit: `sanitize_email_trims_and_lowercases`

## Slide 5 — Ask
Merge tras CI verde. Continúa la auditoría pre-coaching (Rebanada 2 cuando indiques).
