## Context

Wave D.2 (#248, `57c7d8a`) dejó heatmap + drawer por década + facetas
en `/concepts/:id`. El progreso de ejercicios ya viaja a Go como
`passed` (ADR 002: sin código). No hay telemetría de fricción
(permanencia, apertura de década, FAB, fallos de validate).

Stack: Go Clean Architecture, SQLite de usuarios (`ppi.db`), Leptos CSR.
Python del alumno solo en Pyodide.

## Goals / Non-Goals

**Goals**

- Pipeline mínimo: 6 tipos de evento, persistidos por `user_id` de sesión.
- Resumen por década y por partición + un bottleneck hint.
- Widget `#concept-analytics` en el hub (señal, no dashboard).
- Auth obligatoria; sin email ni PII extra en el evento.
- Playwright: acción en el hub → hint visible; Go tests del contrato.

**Non-Goals**

- DAG, visual-regression suite, retags `1..=1000`.
- Roles teacher, warehouse, agregados globales entre alumnos.
- Rebind FAB / facetas / heatmap.

## Decisions

1. **Set cerrado de tipos**
   - `concept_dwell` — requiere `partition_id` 1..=5.
   - `heatmap_decade_open` — `partition_id` + `decade_lo` (1, 11, …, 991).
   - `dua_fab_open` — opcional `step_id` / `partition_id`.
   - `learn_step_enter` / `learn_validate_fail` / `learn_validate_pass` —
     requieren `step_id`; `partition_id` y `decade_lo` si el cliente los tiene.
   - Tipos desconocidos → 400. Campo `code` → 400 (ADR 002).

2. **Clean Architecture**
   - `domain.ConceptEvent` + `Aggregate`.
   - `repositories.ConceptEventRepository` (Insert + ListByUser).
   - SQLite en el mismo `*sql.DB` de usuarios.
   - Usecase resuelve usuario con `AuthService.Me`.
   - `POST /api/concept-events` (204) y `GET /api/concept-analytics`.

3. **Fricción y bottleneck**
   - Score = dwell + decade_open + dua_fab_open + 3×validate_fail.
   - Enter/pass no suman fricción (contexto).
   - Bottleneck = década con mayor score; si no hay décadas, partición.
   - Resumen **solo del usuario autenticado** (no es un panel docente).

4. **Cliente best-effort**
   - Emit fire-and-forget; un 4xx de analytics no cierra sesión.
   - Dwell al montar `/concepts/:id`; decade_open al abrir el drawer;
     FAB al salir de Collapsed; learn enter al cambiar de step;
     validate fail/pass junto al harness existente (sin enviar código).

5. **Gates**
   - `go test ./...` (domain, usecase, handler, sqlite).
   - `make web-test`.
   - Playwright: login → `/concepts/1` → clic década → `#concept-analytics`
     con `data-hint` distinto de `none`.
   - No retag `STEP_PARTITIONS`.

## Risks / Trade-offs

| Riesgo | Mitigación |
|---|---|
| Doble dwell por Effect Leptos | conteos, no unicidad; slice 1 no deduplica |
| Confundir con warehouse docente | spec: resumen del usuario de la sesión |
| Payload con código | rechazo explícito como progress |
| Widget vacío en e2e lento | esperar `data-hint` after POST implícito |

## Migration Plan

1. OpenSpec + implementación en
   `feat/coding-compass-wave-d3-concept-analytics`.
2. Tabla SQLite `concept_events` (IF NOT EXISTS).
3. Cliente + widget + smoke.
4. PR TED. Rollback: revert; hub D.2 intacto.

## Open Questions

- ¿Agregados cross-user para docentes? → **No** en D.3.
