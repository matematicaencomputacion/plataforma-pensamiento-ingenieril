## Context

`main` termina en 2860 con Ola 31 verificada. La extensión toca el catálogo Rust, sus particiones conceptuales y tres contratos E2E; debe ejecutarse en Pyodide y preservar los validadores acumulativos. El PR histórico #294 parte de una base obsoleta, mezcla cambios ajenos y enseña primitivas elementales ya cubiertas, por lo que se usa solo como evidencia de descarte.

## Goals / Non-Goals

**Goals:**

- Producir 60 ejercicios originales y deterministas en diez familias 10x6.
- Representar distribución mediante datos ordenados y funciones puras, manteniendo ejecución client-side.
- Hacer que aplicación, catálogo y validación sean exactos, acumulativos e idempotentes.
- Mantener un diff limitado a Ola 32 y su contrato OpenSpec.

**Non-Goals:**

- Ejecutar workers reales, concurrencia, red o servicios distribuidos.
- Introducir dependencias o modificar backend e infraestructura.
- Corregir o reabrir el PR histórico #294.

## Decisions

### D1 — Nueva generación sobre `main`, no cherry-pick histórico

Se crearán `gen_wave32.py`, `apply_wave32.py` y `validate_wave32.py` siguiendo el contrato robusto de Ola 31. Cherry-pick del PR #294 se descarta porque arrastraría un diff contaminado y contenido pedagógico solapado.

### D2 — Distribución simulada como transformaciones deterministas

Shards, mensajes, agregados parciales y planes de reequilibrio serán listas, tuplas y diccionarios ordenados. Las claves numéricas o strings se asignarán con reglas explícitas, nunca con `hash()` de Python, cuyo resultado puede variar entre procesos.

Alternativa descartada: threads, multiprocessing o colas reales. No son Wasm-safe, añaden no determinismo y enseñan operación de infraestructura antes que razonamiento algorítmico.

### D3 — Diez familias con cierre integrador

Las familias serán: particionado estable; sharding por clave; fan-out; fan-in ordenado; map-reduce; agregados parciales; ventanas por ticks; detección y mitigación de skew; reequilibrio como plan de datos; y capstone de agregación distribuida. Cada sexta actividad integra las cinco anteriores.

### D4 — Generador con datos estructurados y pruebas ejecutables

Cada definición declarará familia, objetivo, solución y valor esperado. El generador derivará starter, prompt, pytest y constantes Rust, reduciendo divergencias. El validador ejecutará las 60 soluciones, verificará 10x6, unicidad de slugs, restricciones prohibidas y presencia suficiente de ejercicios de combinación/agregación.

### D5 — Aplicador idempotente y guardado por baseline

El aplicador exigirá el techo exacto 2860, insertará una sola vez el rango nuevo, actualizará los registros y conteos mediante reemplazos únicos, y luego aceptará una segunda ejecución sin cambios. Cualquier baseline ambiguo abortará antes de escribir.

### D6 — Integración acumulativa

La verificación ejecutará validadores 26–32, `cargo fmt --check`, `cargo test`, `make test`, journeys del harness y OpenSpec estricto. GitHub será autoridad para Docker y E2E completo si el entorno local no puede ejecutarlos de forma fiable.

## Risks / Trade-offs

- [Simulación demasiado abstracta] → usar dominios pequeños, resultados visibles y capstones que combinen partición, parciales y merge.
- [No determinismo por orden de diccionarios o hashing] → ordenar explícitamente claves y prohibir `hash()`.
- [Colisión pedagógica con olas previas] → comparar firmas y slugs contra el catálogo 1–2860.
- [Drift de techo] → validar 2920 en catálogo, conceptos, Rust y los tres E2E.
- [Aplicador parcial] → calcular todos los outputs antes de escribir y exigir anclas únicas.

## Migration Plan

1. Generar y aplicar únicamente `2861..=2920` desde el merge verificado de Ola 31.
2. Ejecutar validación local completa y comprobar un segundo apply sin diff.
3. Publicar una sola rama/PR y esperar todos los checks efectivos.
4. Hacer squash merge con la autorización ya otorgada y verificar CI, Docker, E2E y Deploy por el merge SHA exacto.
5. Ante fallo, corregir en la misma rama; ante regresión post-merge, revertir el squash de Ola 32.
