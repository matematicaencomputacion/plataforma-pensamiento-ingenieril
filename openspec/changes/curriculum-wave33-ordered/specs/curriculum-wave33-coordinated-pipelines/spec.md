## Purpose

Define una extensión educativa determinista para razonar sobre coordinación, garantías de entrega, observabilidad y recuperación de pipelines distribuidos.

## ADDED Requirements

### Requirement: Catálogo exacto y original de Ola 33
El catálogo SHALL incorporar exactamente 60 ejercicios originales, únicos y contiguos entre 2921 y 2980, sin duplicar firmas completas de Olas 26–32.

#### Scenario: Rango y continuidad
- **WHEN** se inspecciona el catálogo integrado
- **THEN** existen una vez `py-2921` a `py-2980`, 2920 enlaza con 2921 y 2980 es el único terminal nuevo

### Requirement: Progresión de coordinación 10x6
La ola MUST organizarse en diez familias de seis ejercicios sobre offsets, acknowledgements, deduplicación, checkpoints, barreras, backpressure, lineage, replay, reconciliación de estado y coordinación integral.

#### Scenario: Familias completas
- **WHEN** se agrupan los ejercicios por familia
- **THEN** hay diez familias con seis ejercicios y cada sexta actividad integra las cinco anteriores

### Requirement: Semánticas observables y deterministas
Cada ejercicio MUST modelar coordinación como datos ordenados, usar Python estándar client-side y evitar red, concurrencia, reloj real, aleatoriedad, `hash()` e `input()`.

#### Scenario: Repetibilidad
- **WHEN** una solución se ejecuta dos veces con la misma entrada
- **THEN** produce el mismo `resultado` y la misma salida observable

#### Scenario: Garantías modeladas
- **WHEN** se auditan las soluciones de la ola
- **THEN** existen ejemplos verificables de progreso por offset, confirmación, snapshot, pausa, trazabilidad y replay idempotente

### Requirement: Contrato ejecutable por ejercicio
Cada micro-step SHALL incluir starter, solución, prueba de `resultado`, título, objetivo, prompt, pista y enlace contiguo.

#### Scenario: Ejecución canónica
- **WHEN** se ejecutan las 60 soluciones contra sus pruebas
- **THEN** todas pasan y una segunda aplicación no modifica archivos

### Requirement: Consistencia acumulativa
Catálogo, particiones conceptuales y tres journeys E2E SHALL compartir el techo 2980, preservando contratos de Olas 26–32.

#### Scenario: Verificación acumulativa
- **WHEN** se ejecutan validadores 26–33, pruebas Rust y conteos E2E
- **THEN** todos observan una cadena única y contigua hasta 2980
