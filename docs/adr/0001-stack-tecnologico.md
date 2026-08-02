# ADR 001: Elección de Stack Tecnológico Core
## Estado: Aceptado
## Contexto
Necesitamos construir una plataforma educativa que ofrezca tiempos de carga casi instantáneos y una API robusta capaz de manejar validaciones rápidas, manteniendo el código escalable y limpio.
## Decisión
- **Frontend:** Utilizaremos Qwik. Su arquitectura basada en "Resumability" y cero hidratación inmediata garantiza una carga inicial óptima, crucial para la retención del estudiante.
- **Backend:** Utilizaremos Go (Golang) estructurado bajo principios de Clean Architecture. Esto nos brinda tipado fuerte, concurrencia eficiente y binarios rápidos.
## Consecuencias Positivas
- Tiempos de respuesta mínimos y bajo consumo de recursos en el servidor.
## Restricciones para el Agente IA
- Queda estrictamente prohibido sugerir migraciones o implementaciones en frameworks SPA tradicionales (React, Vue, Angular) para el frontend, o entornos como Node.js/Python para la API core.
