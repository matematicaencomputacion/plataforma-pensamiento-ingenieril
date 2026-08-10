# PR TED slides — Progress checkmarks + reset

## Slide 1 — Hook
Pasar `test_variables` no dejaba una marca visible y persistente en learn/workspace.

## Slide 2 — Insight
`current_level` ya existía en SQLite, pero no llegaba al cliente (`/api/me`) ni había reset.

## Slide 3 — Move
- Check verde bajo Run/Validar y junto al statement del workspace
- `current_level` en PublicUser + `POST /api/progress/reset` («Volver a empezar»)

## Slide 4 — Proof
`make harness` → `RESULT: PASS` (incluye `progress.check.spec.ts`)

## Slide 5 — Ask
Merge a `main` con CI verde.
