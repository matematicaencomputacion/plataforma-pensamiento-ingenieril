## Context

Hoy el backend evalúa código vía Grok (`EvaluateCode → passed + feedback`) y el frontend muestra editor + resultado. No hay enunciados por nivel ni memoria de habilidades. Este change introduce el motor pedagógico dual (tracks) y el Perfil Cognitivo, sin autenticación real (usuario estático).

Stack vigente (ADR 001/002): Qwik + Go Clean Architecture; la ejecución interactiva de Python sigue en el cliente; el backend orquesta evaluación/progreso y prompts.

## Goals / Non-Goals

**Goals:**

- Modelar Niveles/Retos con enunciado, tipo (`micro_paso` | `reto_ingenieril`) y prompt de evaluación.
- Modelar Perfil Cognitivo (habilidades, estado, fechas de repaso) y usarlo al construir el system prompt.
- Exponer enunciado del nivel actual en la UI Qwik antes del editor.
- Mantener respuesta JSON de evaluación (`passed`, `feedback`) con tutoría personalizada.

**Non-Goals:**

- Auth/login real.
- Chat conversacional de ayuda.
- Ingesta masiva de contenido externo.
- Ejecutar código del alumno en el servidor.

## Decisions

1. **Domain-first en Go**
   - Extender `internal/domain` con `Challenge`/`Level` (enunciado, track/tipo, evaluationPrompt) y `CognitiveProfile` + `Skill` (id, status, lastReviewedAt).
   - Casos de uso: obtener nivel actual, cargar perfil, evaluar con contexto (componer system prompt = prompt del track + JSON del perfil + reglas JSON de salida).

2. **Usuario estático temporal**
   - Constante `demo-user` en backend/repos en memoria o SQLite local.
   - Facilita demos sin bloquear el change por HU04.

3. **API mínima**
   - `GET /api/levels/current` → enunciado, tipo, track, metadata UI.
   - `POST /api/evaluate` → además de `code`/`level_id`, el use case carga perfil + prompt del nivel; respuesta sigue `{"passed","feedback"}`.
   - Persistencia inicial: repositorio in-memory o SQLite (alineado a PRD dev); interfaz de repo para no acoplar use cases.

4. **Prompts por track**
   - `micro_paso` → persona "Tutor Básico" (instrucciones cortas, foco sintaxis/`print`).
   - `reto_ingenieril` → persona "Arquitecto de Software" (problema abierto, criterios de diseño).
   - Ambos exigen JSON puro `{passed, feedback}` sin markdown.

5. **Frontend Qwik**
   - Al montar la vista, `fetch` del nivel actual; render de tarjeta de enunciado (micro vs reto) encima del editor.
   - Mantener `useSignal`/`useStore` y Resumability; estilos coherentes con `global.css` actual (Tailwind pendiente de adopción global).

6. **Observabilidad**
   - Conservar logging detallado de errores de evaluación (`Error detallado en EvaluateCode`) para diagnosticar fallos de JSON/latencia xAI.

## Risks / Trade-offs

- **Latencia**: perfil + prompt largo → mitigar truncando habilidades a las N más relevantes / recientes.
- **Drift JSON**: modelos verbosos → reforzar regla anti-markdown y parser defensivo existente.
- **Repos in-memory**: suficiente para MVP SDD; migración a Postgres queda para change posterior.
- **Acoplamiento a Grok**: mantener puerto/cliente HTTP inyectable (ya usado en tests con `httptest`).
