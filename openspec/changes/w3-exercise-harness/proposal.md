# Proposal: Harness UI tipo W3Schools Exercise (Micro-Pasos)

## Why

Hoy el workspace de IngenierIA mezcla marca, stack tools, video/capítulos y editor en una misma superficie. Para los primeros micro-pasos de Python (semilla `docs/seeds/python-foundations-microsteps-v0.2.json`) necesitamos un **foco pedagógico único**: una ventana de ejercicio a pantalla completa (teoría breve + editor + chequeo), inspirada en W3Schools Exercise, sin distraer con dashboard. Sin ese harness, la semilla no se puede validar con alumnos ni iterar el contenido.

## What Changes

- Nueva ruta/superficie frontend de **Exercise Workspace** (single viewport): panel de teoría/enunciado, editor de código, consola/resultados de tests.
- Carga de micro-pasos desde la semilla JSON (primeros 10 puntos) con navegación step → next.
- Ejecución y validación **client-side** (Pyodide + pytest o asserts equivalentes); sin endpoint Go para correr código del alumno.
- Acciones pedagógicas: hint, mostrar solución bajo demanda, Continuar al siguiente paso si los checks pasan.
- Soporte opcional de MCQ (p. ej. Casting) además del coding challenge.
- Integración mínima con el workspace actual (entrada al harness desde el home / InteractiveStage, sin reemplazar el video MoureDev).

## Capabilities

### New Capabilities

- `exercise-workspace`: UI single-viewport para micro-pasos (teoría + editor + tests), navegación entre steps y estados pass/fail/hint/solution.
- `microstep-seed-runtime`: Contrato y carga de la semilla de micro-pasos (metadata, steps, checks pytest/MCQ) usable por el frontend.

### Modified Capabilities

- _(ninguna — no hay baseline en `openspec/specs/` que cambie requisitos; `learning-tracks` del change `core-learning-engine` se mantiene; este harness es una modalidad de Micro-paso concreta)_

## Alcance incluido

- Frontend Qwik: layout Exercise Workspace + carga de semilla v0.2 (o snapshot embebido).
- Runner client-side para ejecutar starter/solution del alumno y evaluar checks del step.
- Navegación secuencial de los 10 primeros steps (HOME → Casting) y frontera documentada hacia Strings.
- Tests/build frontend verdes; documentación del contrato de semilla en el change.

## Fuera de alcance

- Persistencia de progreso en backend / JWT / `POST /api/progress/complete-level` (queda para un change posterior).
- Sustituir InteractiveStage (YouTube/capítulos) ni rediseñar la marca IngenierIA.
- Ingesta masiva desde W3Schools ni scraping en runtime.
- i18n completo es/en de todos los prompts (solo estructura lista para i18n).
- Tutora IA / EvaluationUseCase en este harness (opcional más adelante).

## Impact

- Frontend: nuevas rutas/componentes bajo `frontend/src/` (exercise workspace, seed loader, runner).
- Contenido: consumo de `docs/seeds/python-foundations-microsteps-v0.2.json` (posible copia a `frontend/src/data/` o `public/`).
- Backend Go: **sin** ejecución de código de alumnos; solo impacto futuro si se añade progreso (fuera de alcance).
- ADR vigente: `docs/adr/0002-ejecucion-client-side.md` (refuerza Pyodide/pytest en cliente).

## Riesgos

- Pyodide + pytest en browser puede ser pesado al primer load (mitigar lazy-load / cache).
- Contrato `exec(open('solution.py'))` de la semilla es placeholder: el design debe definir el adapter real del runner.
- Scope creep hacia “clonar W3Schools completo” — limitar a 10 steps + harness.

## Plan de rollback

- Eliminar la ruta/feature del harness y dejar el workspace actual intacto; la semilla JSON permanece como artefacto de contenido sin UI.
- Revertir commits del change en la rama feature sin tocar `main`.
