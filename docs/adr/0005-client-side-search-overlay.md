# ADR 0005: Client-Side Command Palette & Global Search Overlay

## Estado
Aceptado (Diferido a UI Polish — se implementa tras completar el catálogo de ejercicios hasta el micro-step 5000).

## Contexto
La plataforma requiere un mecanismo de navegación rápida y descubrimiento conceptual transversal a los 5 portales, los términos del glosario y el catálogo de micro-pasos de ingeniería.

## Decisión
Implementar un componente de búsqueda global tipo *Command Palette* (patrón Google Cloud Console) desplegable sobre el shell de la aplicación (Leptos SPA / WebAssembly) con las siguientes características:

1. **Activación y Layout:**
    - Barra compacta en el header con placeholder y badge de atajo.
    - Panel modal flotante centrado con backdrop semitransparente (*dimming*).
    - Atajos globales unificados (`Cmd/Ctrl + K`, `/`, `Escape`) reconciliando el Floating Action Button (FAB).
    - Navegación accesible por teclado (`ArrowUp`, `ArrowDown`, `Enter`).

2. **Alcance e Indexación en Cliente (Client-Side Only):**
    - El índice residirá en memoria (Wasm), extrayendo datos de `curriculum.rs` y el glosario.
    - Categorización de resultados: *Portales (1–5)*, *Pasos del Rail*, *Keywords del Glosario* y *Recientes*.
    - **No Goal:** Se descarta el uso de bases de datos de grafos (Neo4j) o endpoints de búsqueda pesados en backend para esta funcionalidad.

3. **Plan de Ejecución:**
    - La implementación en código se posterga hasta completar la generación y consolidación del catálogo de ejercicios (meta: micro-step 5000), evitando retrabajos sobre índices incompletos y manteniendo el foco en el pipeline de contenido.

## Consecuencias
- **Positivas:** Cero latencia de red en búsquedas, navegación instantánea, desacoplamiento total del backend.
- **Negativas / Trade-offs:** El índice debe ser liviano para no penalizar el tiempo de carga ni el consumo de memoria en el navegador.