## Context

`main` está verificado hasta 2560. La rama histórica rotulada como Ola 27 contiene ancestry de Olas 28–30 y su generador duplica campo por campo Ola 26. Véanse `proposal.md` y la especificación de la capacidad para el contrato corregido.

## Goals / Non-Goals

**Goals:**
- Generar sesenta ejercicios originales, reproducibles y ejecutables en Pyodide.
- Mantener una sola fuente de contenido para constantes Rust, referencias y validación.
- Probar explícitamente no duplicación respecto de Ola 26.

**Non-Goals:**
- Reusar el generador histórico salvo como evidencia negativa.
- Introducir recursión profunda, algoritmos exponenciales sin cotas o dependencias externas.
- Modificar servicios Go o infraestructura.

## Decisions

### Organizar diez familias pedagógicas de seis pasos

Las familias cubrirán: casos base; suma/producto recursivos; strings; listas; estructuras anidadas; comprehensions de listas; filtrado/transformación; dict comprehensions; set comprehensions; y combinación/revisión. Cada familia progresa de explicación a aplicación y cierre comprobable.

Alternativa descartada: renumerar el generador histórico. Aunque compila, duplica Ola 26 y contradice el propósito declarado.

### Generador declarativo y aplicador con anclas exactas

El generador contendrá datos de ejercicio y emitirá Rust; el aplicador solo aceptará la frontera terminal 2560, el array de 2560 referencias, particiones y conteos exactos. Una segunda aplicación debe fallar antes de escribir.

Alternativa descartada: editar el bloque Rust manualmente, por riesgo de drift entre seis superficies.

### Validación acumulativa y autoridad de techo

El validador de Ola 26 conserva su rango y verifica el enlace hacia 2561; Ola 27 asume autoridad sobre el techo 2620. El nuevo validador ejecutará soluciones/pruebas, comprobará rango, navegación, referencias, particiones, E2E y no duplicación pedagógica.

### Recursión acotada

Los ejemplos usarán entradas pequeñas y casos base explícitos. Se evitan recursiones sobre datos no limitados para conservar seguridad y velocidad en Wasm.

## Risks / Trade-offs

- [Contenido generado extenso] → validar las 60 definiciones y ejecutar todas las pruebas.
- [Duplicación semántica accidental] → comparar slug, prompt y solución contra Ola 26.
- [Recursión sin terminación] → casos base obligatorios y entradas de prueba acotadas.
- [Drift de catálogo] → validar techo único 2620 en Rust, conceptos y E2E.
- [Rama histórica contaminada] → reconstrucción sobre `main`, sin merge ni cherry-pick acumulativo.

## Migration Plan

1. Generar y aplicar únicamente `2561..=2620` en una rama nueva desde `main`.
2. Ejecutar validadores acumulativos, Rust, backend y harness journeys.
3. Abrir un PR único, esperar CI completo y solicitar autorización de merge.
4. Verificar CI, Docker, E2E y Deploy post-merge antes de Ola 28.

Rollback: revertir el squash de Ola 27 para devolver frontera, particiones y conteos a 2560.
