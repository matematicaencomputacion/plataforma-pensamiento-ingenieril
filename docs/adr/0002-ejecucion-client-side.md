# ADR 002: Ejecución de Código en el Cliente
## Estado: Aceptado
## Contexto
Los estudiantes enviarán código Python para resolver ejercicios. Ejecutar este código del lado del servidor requiere infraestructura costosa, contenedores aislados (sandboxing) y mantenimiento complejo.
## Decisión
Delegaremos la ejecución del código interactivo (Python) enteramente al cliente (navegador del estudiante) utilizando tecnologías basadas en WebAssembly, como Pyodide/JupyterLite integradas con Monaco Editor.
## Consecuencias Positivas
- El costo de infraestructura de computación (CPU/RAM) para correr el código de los alumnos se reduce a cero.
- Riesgo de seguridad en el servidor mitigado (no hay RCE posible).
## Flujo de Trabajo
- El frontend ejecuta los tests (ej. Pytest localmente vía Wasm).
- El backend en Go **solo** recibe los resultados finales firmados/validados para actualizar la base de datos y desbloquear el siguiente nivel, sin evaluar el código crudo.
