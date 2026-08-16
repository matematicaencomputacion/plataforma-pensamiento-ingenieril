## Why

El hub conceptual ya muestra cobertura (heatmap D) y rutas (facetas D.2),
pero no hay señal de **fricción pedagógica**: dónde el alumno se queda
(dwell), abre décadas o falla un validate. Sin un pipeline mínimo de
eventos, no se puede apuntar el siguiente drill al cuello de botella.

## What Changes

**Telemetría y analytics conceptual — primer slice** (Wave D.3):

1. El cliente Leptos emite un set cerrado de eventos (dwell en
   `/concepts/:id`, apertura de década del heatmap, apertura DUA/FAB,
   enter/validate fail-or-pass en `/learn`).
2. La API Go los persiste (Clean Architecture + SQLite). Solo usuarios
   autenticados; el único identificador es el `user_id` de la sesión.
3. Un GET de resumen + widget `#concept-analytics` en el hub: conteos
   por década/partición y un hint de cuello de botella (“esta década
   tiene alta fricción”). No es un dashboard ni un warehouse.

## Capabilities

### New Capabilities

- `concept-analytics`: ingestión autenticada de eventos de fricción y
  resumen por década/partición para el usuario de la sesión.

### Modified Capabilities

- `coding-conceptual-partitions`: el hub `/concepts/:id` SHALL mostrar
  `#concept-analytics` con el hint de bottleneck del usuario (sin
  desmontar heatmap, facetas, FAB ni drawer).

## Alcance incluido

- Eventos: `concept_dwell`, `heatmap_decade_open`, `dua_fab_open`,
  `learn_step_enter`, `learn_validate_fail`, `learn_validate_pass`.
- `POST /api/concept-events` + `GET /api/concept-analytics` (auth Bearer).
- SQLite `concept_events`; ADR 002: el body NUNCA acepta código Python.
- Widget `#concept-analytics` en el hub.
- Go tests + `make web-test` + un smoke Playwright (acción en el hub
  → hint visible **o** contrato POST+GET). Spec + código en el mismo PR.

## Fuera de alcance

- #4 DAG / prerrequisitos / Neo4j.
- #5 visual-regression mega-suite.
- Retaguear `1..=1000`.
- Romper FAB / heatmap / búsqueda facetada.
- Warehouse, roles teacher, PII extra, ejecución de alumnos en Go.

## Impact

- Backend: domain + usecase + handler + repo SQLite; `main.go`.
- Web: `api.rs` / `auth.rs`, `concepts.rs`, `learn.rs`, FAB, CSS, e2e.
- Base = `origin/main` @ `57c7d8a` (Wave D.2 #248).
- Python del alumno sigue solo en Pyodide (ADR 002).

## Plan de rollback

Revertir este PR. El hub D.2 y el progreso `POST /api/progress/complete`
siguen. La tabla SQLite nueva queda vacía e inofensiva si se deja.
