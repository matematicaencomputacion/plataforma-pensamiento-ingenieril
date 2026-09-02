## Purpose

Definir una progresión educativa determinista para razonar sobre consistencia, transacciones compensables, consenso y reparación de datos distribuidos sin infraestructura externa.

## Requirements

### Requirement: Catálogo exacto y original de Ola 34
El catálogo SHALL incorporar exactamente 60 ejercicios originales, únicos y contiguos entre 2981 y 3040, sin duplicar firmas completas de Olas 26–33.

#### Scenario: Rango y continuidad
- **WHEN** se inspecciona el catálogo integrado
- **THEN** existen una vez `py-2981` a `py-3040`, 2980 enlaza con 2981 y 3040 es el único terminal nuevo

### Requirement: Progresión de consistencia y recuperación 10x6
La ola MUST organizarse en diez familias de seis ejercicios sobre evolución de esquemas, orden total, outbox, sagas, leases, quorum, compactación, auditoría, reparación y recuperación integral.

#### Scenario: Familias completas
- **WHEN** se agrupan los ejercicios por familia
- **THEN** hay diez familias con seis ejercicios y cada sexta actividad integra las cinco anteriores

### Requirement: Semánticas deterministas y client-side
Cada ejercicio MUST representar estado y decisiones como datos ordenados, ejecutarse con Python estándar en el cliente y evitar red, concurrencia, reloj real, aleatoriedad, `hash()` e `input()`.

#### Scenario: Repetibilidad
- **WHEN** una solución se ejecuta dos veces con la misma entrada
- **THEN** produce el mismo `resultado` y la misma salida observable

### Requirement: Contrato ejecutable por ejercicio
Cada micro-step SHALL incluir starter, solución, prueba de `resultado`, título, objetivo, prompt, pista y enlace contiguo.

#### Scenario: Ejecución canónica
- **WHEN** se ejecutan las 60 soluciones contra sus pruebas
- **THEN** todas pasan y una segunda aplicación no modifica archivos

### Requirement: Consistencia acumulativa hasta 3040
Catálogo, particiones conceptuales y tres journeys E2E SHALL compartir el techo 3040 preservando los contratos de Olas 26–33.

#### Scenario: Verificación acumulativa
- **WHEN** se ejecutan validadores 26–34, pruebas Rust y conteos E2E
- **THEN** todos observan una cadena única y contigua hasta 3040
