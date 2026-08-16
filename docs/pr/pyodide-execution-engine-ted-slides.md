## Slide 1 — Hook
Ejecutar Python ya no es un botón borroso: la consola muestra stdout/stderr claros y el motor no toca Go.

## Slide 2 — Insight
#39 abrió `/learn` y Validar; faltaba cerrar la rebanada de ejecución interactiva (Run + consola + enunciado del micro-paso vivo) con E2E explícito de `print`.

## Slide 3 — Move
- Interop movido a `web/src/interop/pyodide.rs`; paneles `#learn-stdout` / `#learn-stderr`.
- «Ejecutar código» + busy «Ejecutando Python en tu navegador…».
- Enunciado desde el micro-paso embebido; E2E `exercise.spec.ts` (ADR 002).

## Slide 4 — Proof
- `make harness` → RESULT: PASS.

## Slide 5 — Ask
Merge para fijar el motor de ejecución; siguiente: loader multi-step de la semilla.
