## 1. Planificación

- [x] 1.1 Change `coding-compass-wave-d3-concept-analytics` (proposal /
      design / spec / tasks) anclado a `origin/main` @ `57c7d8a`
- [x] 1.2 Fuera de alcance explícito: DAG, visual-regression suite,
      retags `1..=1000`, romper FAB / heatmap / facetas

## 2. Backend Go

- [x] 2.1 Domain `ConceptEvent` + tipos cerrados + `Aggregate` (tests)
- [x] 2.2 Puerto `ConceptEventRepository` + SQLite `concept_events`
- [x] 2.3 Usecase Record + Summary vía `AuthService.Me`
- [x] 2.4 Handlers `POST /api/concept-events` y
      `GET /api/concept-analytics` (401, ADR 002 `code`, 204, summary)
- [x] 2.5 Cablear `main.go` y mux de integration; `go test ./...`

## 3. Cliente Leptos

- [x] 3.1 URLs + tipos wire; emit best-effort (dwell, decade, FAB,
      learn enter/validate) **sin** enviar código
- [x] 3.2 Widget `#concept-analytics` / `#concept-analytics-hint` en
      el hub; heatmap D.1 y facetas D.2 intactos

## 4. Tests y proof

- [x] 4.1 Playwright: login → `/concepts/1` → abrir década → hint
      visible (`data-hint` ≠ `none`); FAB `/learn` sigue
- [ ] 4.2 `make web-test` + `make test`; PR TED; squash-merge cuando
      Backend + Frontend + Playwright aggregator estén verdes
