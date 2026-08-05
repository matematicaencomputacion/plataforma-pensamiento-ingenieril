## Context

IngenierIA ya tiene workspace con marca, tool-stack, InteractiveStage (YouTube + capítulos) y editor/evaluación. La semilla pedagógica `docs/seeds/python-foundations-microsteps-v0.2.json` define 10 micro-pasos (HOME→Casting) con patrón explain→try→check inspirado en W3Schools Exercise, pero **no existe aún una UI de foco único** para recorrerlos.

Restricciones vigentes:
- ADR 002: ejecución Python del alumno **solo en cliente** (Pyodide/JupyterLite).
- Qwik resumability; no bloquear el home actual.
- Semilla v0.2 usa placeholders `exec(open('solution.py'))` que el runner debe adaptar.

Stakeholders: alumno (flujo de aprendizaje), instructional design (iterar semilla), frontend (harness).

## Goals / Non-Goals

**Goals:**

- Superficie **single-viewport** theory | editor | checks, navegable step a step.
- Cargar y tipar la semilla de micro-pasos en el frontend.
- Correr código + validar checks en browser; hint / solución bajo demanda; avanzar si pasa.
- Entrada clara desde el home sin romper InteractiveStage.

**Non-Goals:**

- Persistencia de progreso en Go / JWT.
- Clonar todo el catálogo W3Schools.
- Sustituir video MoureDev / capítulos.
- Tutora IA dentro del harness en este change.

## Decisions

### D1 — Ruta dedicada `/exercise` (o `/microsteps`)

- **Decisión:** Nueva ruta Qwik City para el harness, no embeber el layout W3 dentro del header del home.
- **Por qué:** Single composition / foco total (como la ventana Exercise de W3). El home sigue siendo hub.
- **Alternativa:** Modal fullscreen sobre index → más fricción con HMR/estado y peor deep-link.
- **Entrada:** CTA “Micro-pasos Python” en home → `/exercise?step=py-01-home`.

### D2 — Semilla como asset frontend versionado

- **Decisión:** Copiar/sincronizar la semilla a `frontend/src/data/python-foundations-microsteps.json` (fuente de verdad de runtime). Mantener `docs/seeds/...-v0.2.json` como documento de curaduría.
- **Por qué:** Import tipado + tree-shake friendly; sin fetch a Google Docs.
- **Alternativa:** Fetch desde `/public` → válido, pero peor para tipos y tests unitarios del loader.

### D3 — Contrato de runner: adapter sobre checks

- **Decisión:** El runtime **no** ejecuta literalmente `exec(open('solution.py'))`. El loader normaliza cada step a:
  - `studentCode: string`
  - `testSource: string` (pytest o suite de asserts)
  - El runner inyecta el código del alumno como módulo/archivo virtual en Pyodide y corre los tests.
- **Por qué:** Compatible con browser FS de Pyodide; desacopla curaduría de implementación.
- **Alternativa:** Solo comparar stdout con golden string → frágil para steps con variables/`type()`.

### D4 — Layout de tres bandas en un viewport

```
┌─────────────────────────────────────────────┐
│ Step N/10 · Title              [Hint] [Sol] │
├──────────────────┬──────────────────────────┤
│ prompt_md        │  editor (starter_code)   │
│ (teoría breve)   │                          │
├──────────────────┼──────────────────────────┤
│ MCQ (opcional)   │  Run · Check · Continuar │
│                  │  consola / pytest output │
└──────────────────┴──────────────────────────┘
```

- Desktop: theory izquierda / editor derecha; checks abajo.
- Mobile: stack vertical theory → editor → checks.
- **Alternativa:** Solo editor + drawer de teoría → menos parecido a W3 “previo + problema”.

### D5 — Pyodide lazy-load

- **Decisión:** Cargar Pyodide al entrar a `/exercise` (o al primer Run), con indicador de “Preparando motor Python…”.
- **Por qué:** No penalizar el home.
- **Alternativa:** Preload en index → peor TTI del hub.

### D6 — Estado local primero

- **Decisión:** `useStore` por sesión (step actual, código, pass/fail, hint visible). Sin backend en este change.
- **Por qué:** Alcance del proposal; desbloqueo DAG/progress queda para change posterior.
- **Opcional futuro:** `localStorage` keyed by seed version + step id (no bloqueante).

### D7 — MCQ opcional en el mismo viewport

- **Decisión:** Si `content.mcq_bank` o `checks.mcq` existe, renderizar opciones; Continuar puede exigir MCQ correcto **o** pytest según `checks.mode`.
- **Por qué:** Casting W3 combina teoría + exercise MCQ + coding challenge del Doc.

## Risks / Trade-offs

| Risk | Mitigation |
|------|------------|
| Peso/latencia Pyodide | Lazy-load + spinner; cache CDN |
| Tests de semilla no portables | Adapter documentado; suite de smoke por step en CI frontend (sin browser completo al inicio: unit tests del normalizer) |
| Scope creep a 20+ capítulos | Freeze a 10 steps + frontera Strings |
| Confusión home vs exercise | CTA explícita; breadcrumb “Salir al workspace” |

## Migration Plan

1. Aterrizar change en rama `feat/w3-exercise-harness`.
2. Añadir data + tipos + loader + UI + runner stub→Pyodide.
3. Pre-CI: `cd frontend && npm run build` (+ tests unitarios del loader).
4. PR al completar harness usable con steps 1–10.
5. **Rollback:** quitar ruta y CTA; home intacto; semilla docs se conserva.

## Open Questions

1. ¿Editor mínimo (textarea) en MVP del harness o Monaco desde el día 1? → **Recomendación MVP:** textarea tipado + upgrade Monaco en tarea opcional si el tiempo aprieta.
2. ¿Sincronizar semilla docs↔frontend con script `make seed-sync` o copia manual en este change? → **Recomendación:** copia versionada + nota en tasks; script después.
3. ¿Mapear steps a `Concept.id` del curriculum en este change o en iter-02 del backlog de semilla? → **Fuera de este design** (backlog semilla); harness solo consume step ids.
