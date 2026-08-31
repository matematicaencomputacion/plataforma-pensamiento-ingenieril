## Context

`main` está verificado hasta 2620. La rama histórica rotulada como Ola 28 parte de una base anterior, acumula Olas 26–30, conserva un generador declarado como Ola 24 y contiene un aplicador que puede insertar placeholders no ejecutables. Véanse `proposal.md` y la especificación de la capacidad para el contrato corregido.

## Goals / Non-Goals

**Goals:**
- Generar sesenta ejercicios originales, reproducibles y ejecutables en Pyodide.
- Enseñar la estructura de pipelines paralelizables sin introducir concurrencia real en el runtime educativo.
- Mantener una sola fuente de contenido para constantes Rust, referencias y validación.
- Probar equivalencia entre reducciones directas y combinación de resultados parciales.

**Non-Goals:**
- Reusar el generador o aplicador histórico salvo como evidencia negativa.
- Ejecutar threads, multiprocessing, red o tareas no deterministas.
- Modificar servicios Go, infraestructura o el modelo de persistencia.

## Decisions

### Organizar diez familias pedagógicas de seis pasos

Las familias cubrirán: particionado y chunks; batching; alineación con `zip`; fan-out de transformaciones puras; fan-in y merge estable; reducciones parciales; combinación asociativa; agregación agrupada; ventanas y planificación determinista; y cierre map-reduce local. Cada familia progresa de explicación a aplicación y cierre comprobable.

Alternativa descartada: renumerar el generador histórico. Aunque contiene 60 entradas, esas entradas son literalmente el temario de Ola 24 y contradicen el título de Ola 28.

### Modelar paralelismo como separación de datos

Los ejercicios representan workers como funciones puras aplicadas secuencialmente a chunks. El valor pedagógico está en demostrar independencia, orden de combinación y equivalencia de parciales, no en medir velocidad o lanzar concurrencia.

Alternativa descartada: usar `threading`, `multiprocessing` o async con I/O simulado. Añade comportamiento dependiente del runtime, no enseña la propiedad algebraica necesaria y contradice la ejecución segura en navegador.

### Generador declarativo y aplicador con anclas exactas

El generador contendrá datos de ejercicio y emitirá Rust; el aplicador solo aceptará la frontera terminal 2620, el array de 2620 referencias, particiones y conteos exactos. Una segunda aplicación debe fallar antes de escribir.

Alternativa descartada: el aplicador histórico tolerante que busca marcadores parciales e inserta `Step N` con `pytest: "..."`; puede producir un catálogo formalmente numerado pero pedagógicamente inválido.

### Validación acumulativa y autoridad de techo

El validador de Ola 27 conservará su rango y verificará el enlace hacia 2621; Ola 28 asumirá autoridad sobre el techo 2680. El nuevo validador ejecutará las 60 soluciones contra sus pruebas, comprobará rango, navegación, referencias, particiones, E2E, seguridad y no duplicación contra Olas 26 y 27.

### Orden de combinación explícito

Para operaciones no conmutativas, el ejercicio debe fijar el orden de chunks y del merge. Para reducciones declaradas asociativas, el test comparará reducción directa y composición de parciales. Así se evita enseñar que cualquier operación puede distribuirse arbitrariamente.

## Risks / Trade-offs

- [Confundir paralelismo conceptual con concurrencia real] → declarar workers simulados y prohibir APIs concurrentes.
- [Operaciones no asociativas combinadas incorrectamente] → tests de equivalencia y orden contractual explícito.
- [Contenido generado extenso] → validar las 60 definiciones y ejecutar todas las pruebas.
- [Duplicación semántica accidental] → comparar slug, prompt y solución contra Olas 26 y 27.
- [Drift de catálogo] → validar techo único 2680 en Rust, conceptos y E2E.
- [Rama histórica contaminada] → reconstrucción sobre `origin/main`, sin merge ni cherry-pick acumulativo.

## Migration Plan

1. Generar y aplicar únicamente `2621..=2680` en la rama aislada desde el merge verificado de Ola 27.
2. Ejecutar validadores acumulativos, Rust, backend y harness journeys.
3. Abrir un PR único, esperar CI completo y solicitar autorización de merge.
4. Verificar CI, Docker, E2E y Deploy post-merge antes de Ola 29.

Rollback: revertir el squash de Ola 28 para devolver frontera, particiones y conteos a 2620.
