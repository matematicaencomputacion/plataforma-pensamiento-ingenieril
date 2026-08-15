## Context

`web/src/concepts.rs` mantiene hoy una tabla ordenada de pares
`(micro_step, &[partition_id])`. Es segura y simple, pero extenderla hasta 1000
pasos fila por fila aumenta el costo de revisión y favorece errores mecánicos.
El catálogo ya agrupa los pasos posteriores en familias contiguas de seis.

## Goals / Non-Goals

**Goals**

- Representar tags por familias/rangos con excepciones explícitas.
- Conservar búsqueda determinista y soporte multi-tag.
- Mantener los hubs y el cálculo de dominio sin cambios de contrato.
- Hacer auditable qué pasos quedan sin tag y por qué.

**Non-Goals**

- Inferir tags en runtime desde títulos o texto libre.
- Exigir cobertura del 100%: “sin tag” es válido si la lente no es material.
- Introducir una jerarquía conceptual nueva.

## Decisions

1. **Índice sparse explícito**: cada micro-paso etiquetado aparece en
   `STEP_PARTITIONS`. Los pasos sin fila quedan sin tag a propósito.
2. **Semántica estricta**:
   - P1 cuando identidad, mutación, representación o estructura de datos es parte
     central del objetivo.
   - P2 cuando resolución de nombres, ámbito, closures, parámetros o namespaces
     es parte central.
   - P3 cuando el ejercicio practica estructura imperativa, funcional, OOP o
     recursiva como decisión de diseño.
3. **No default DSA→P3**: una familia algorítmica recibe P3 solo si la técnica de
   diseño (recursión, backtracking, DP, etc.) es pedagógicamente explícita.
4. **Validación**: tests verifican orden, tags válidos, existencia en catálogo,
   correcciones Foundations y huecos intencionales (p. ej. 517, 721–1000).
5. **Cobertura**: ~330 pasos etiquetados de 1000; el resto permanece sin lente
   conceptual hasta que el enunciado la haga material.

## Risks / Trade-offs

- Los rangos pueden ocultar diferencias internas; se mitiga con excepciones y
  tests representativos por familia.
- El porcentaje de dominio cambia al crecer el denominador. Es intencional: pasa
  de una muestra Foundations a la cobertura curricular real.
- Tags pedagógicos admiten juicio editorial; el OpenSpec documenta la regla para
  evitar deriva.

## Migration Plan

1. Auditar y documentar familias.
2. Reemplazar la tabla fila-a-fila por reglas compactas equivalentes.
3. Agregar cobertura del catálogo completo.
4. Ejecutar `make web-test`; no se toca el backend ni Pyodide.
