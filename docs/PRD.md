# Product Requirements Document (PRD)
## Nombre del Proyecto: Plataforma Educativa de Pensamiento Ingenieril

> **Spec-Driven Development (OpenSpec):** este PRD conserva la visión fundacional.
> Los requisitos vivos, deltas y el plan de implementación se gestionan en `openspec/`.
> Change activo inicial: `openspec/changes/core-learning-engine/` (`proposal.md`, `specs/`, `design.md`, `tasks.md`).

### 1. Visión y Objetivo
Desarrollar una plataforma web de aprendizaje autogestionado para enseñar programación (Python y Go), matemáticas e inglés técnico. El diferenciador principal es el enfoque pedagógico: enseñar "Pensamiento Ingenieril" (Abstracción → Diseño → Implementación → Pruebas). 
La plataforma debe ser ultrarrápida, de bajo costo operativo y altamente gamificada mediante el desbloqueo automático de niveles.
### 2. Público Objetivo
Estudiantes que buscan aprender a programar resolviendo problemas del mundo real mediante fundamentos sólidos, algoritmos y buenas prácticas de ingeniería, sin tener que configurar entornos complejos en sus máquinas locales.
### 3. Stack Tecnológico Core
- **Frontend:** Qwik (para carga instantánea mediante Resumability) + Tailwind CSS.
- **Backend:** Go (Golang) siguiendo Clean Architecture para una API robusta y determinista.
- **Base de Datos:** PostgreSQL (Producción) / SQLite (Desarrollo local).
- **Entorno de Ejecución del Estudiante:** JupyterLite / Monaco Editor (Client-Side con WebAssembly, evitando costos de infraestructura de servidores para ejecución de código).
### 4. Historias de Usuario Principales (MVP)
- **HU01 - Entorno Integrado:** Como estudiante, quiero escribir y ejecutar código Python/Go directamente en el navegador de la plataforma para no lidiar con instalaciones locales.
- **HU02 - Evaluación Automatizada:** Como estudiante, quiero que mi código sea evaluado contra una suite de pruebas unitarias (ej. Pytest) de forma transparente para saber si mi lógica es correcta.
- **HU03 - Gamificación y Niveles:** Como estudiante, quiero que el "Nivel 2" se desbloquee únicamente y de forma automática cuando el backend registre que he pasado el 100% de las pruebas del "Nivel 1".
- **HU04 - Gestión de Identidad:** Como usuario, quiero un sistema de registro/login propio (usuario y contraseña) para mantener mi progreso guardado, con una interfaz de marca 100% personalizada.
- **HU05 - Ingesta de Lecciones:** Como administrador, quiero poder subir lecciones estructuradas en formato `.ipynb` o Markdown interactivo al repositorio, y que la plataforma las parsee para los estudiantes.
### 5. Flujo de Validación de Ejercicios
1. El estudiante completa el código en el editor web (Qwik + Monaco/JupyterLite).
2. El código se ejecuta y se testea en el navegador (WebAssembly).
3. Si los tests pasan, el frontend envía un Webhook/Petición REST al backend en Go: `POST /api/progress/complete-level`.
4. El backend de Go valida el token JWT, registra el avance en la base de datos y retorna el acceso al siguiente nivel.
### 6. Métricas de Éxito
- **Rendimiento:** Tiempo de carga del dashboard del alumno inferior a 1.5 segundos (gracias a Qwik).
- **Infraestructura:** Costo de computación cercano a cero por ejecución de código de alumnos (trasladado al cliente).
- **Engagement:** Tasa de finalización de módulos superior al promedio mediante la fricción cero del entorno integrado.
