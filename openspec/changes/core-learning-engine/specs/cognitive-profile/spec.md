## ADDED Requirements

### Requirement: Evaluación con Memoria Contextual (Perfil Cognitivo)

El sistema SHALL inyectar las habilidades previas del estudiante en el prompt de la IA para generar un feedback personalizado basado en su progreso histórico.

#### Scenario: Alumno comete un error en un tema que ya dominaba

- GIVEN un estudiante con la habilidad "bucles_for" marcada como "mastered" hace 7 días
- AND un reto actual que requiere iteración
- WHEN el estudiante envía código con un error de sintaxis en el bucle
- THEN el backend inyecta el JSON de habilidades en el contexto de Grok
- AND la IA responde con un feedback que le recuerda su conocimiento previo para forzar la recuperación de memoria
