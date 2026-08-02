# Proposal: Motor de Aprendizaje Dual y Perfil Cognitivo

## Why

Actualmente, la evaluación de código es binaria (aprobado/desaprobado) y carece de un hilo conductor pedagógico. Los estudiantes no tienen contexto sobre qué están resolviendo (falta de enunciados) y el sistema no tiene memoria sobre las habilidades que el usuario ya domina, lo que impide personalizar la dificultad y aplicar repetición espaciada.

## Objetivo

Transformar el motor de evaluación en una plataforma de pensamiento ingenieril híbrida. Se implementarán dos modalidades de aprendizaje (Micro-pasos de sintaxis y Retos Ingenieriles abiertos) y se integrará un "Perfil Cognitivo" que registre las habilidades adquiridas para personalizar dinámicamente los prompts enviados al motor de IA (Grok).

## What Changes

- Modelo de datos para Niveles/Retos (enunciado, tipo de reto, prompt específico por track).
- Modelo de datos Perfil Cognitivo (habilidades, estado de dominio, fechas de repaso).
- Inyección del Perfil Cognitivo en el System Prompt de evaluación.
- Interfaz Qwik para visualizar el enunciado del nivel actual según track.

## Capabilities

### New Capabilities

- `learning-tracks`: Renderizado y configuración de retos por track (Micro-paso vs Reto Ingenieril), incluyendo enunciados y prompts de evaluación asociados.
- `cognitive-profile`: Persistencia y uso del perfil cognitivo del estudiante para personalizar el feedback de la IA.

### Modified Capabilities

- _(ninguna aún — no existen specs baseline en `openspec/specs/`)_

## Alcance incluido

- Definición e implementación de modelo de datos para "Niveles/Retos" (Enunciado, Tipo de Reto, Prompt específico).
- Definición e implementación de modelo de datos "Perfil Cognitivo" (Habilidades, estado de dominio, fechas de repaso).
- Inyección del Perfil Cognitivo en el System Prompt de evaluación.
- Interfaz en el frontend (Qwik) para visualizar el enunciado del nivel actual.

## Fuera de alcance

- Sistema de autenticación de usuarios y login (se simulará un usuario estático por ahora).
- Panel de chat conversacional interactivo (botón de ayuda) — se abordará en un change posterior.
- Importación masiva de contenido de terceros (ej. repositorios open source).

## Impact

- Backend Go: `internal/domain`, `internal/usecases/evaluation.go`, nuevos repositorios/handlers para niveles y perfil.
- Frontend Qwik: `frontend/src/routes/index.tsx` (y componentes de enunciado).
- Contrato API: extensión de `/api/evaluate` y nuevos endpoints de lectura de nivel/perfil (detalle en `design.md`).
- Dependencia externa: API Grok (xAI) con prompts más ricos (riesgo de latencia/formato JSON).

## Riesgos

- Latencia en la respuesta de la IA al inyectar un System Prompt demasiado largo con el perfil cognitivo.
- Inconsistencia en la evaluación si el modelo de IA no respeta el JSON de salida ante prompts complejos.

## Plan de rollback

- Revertir al commit actual del motor de evaluación con tutoría (`3301521`).
