## Purpose

Definir una progresión educativa determinista para razonar sobre entregas compatibles, exposición gradual, migraciones y reversión verificable sin infraestructura externa.

## ADDED Requirements

### Requirement: Catálogo exacto y original de Ola 36
El catálogo SHALL incorporar exactamente 60 ejercicios originales, únicos y contiguos entre 3101 y 3160, sin duplicar firmas completas de Olas 26–35.

#### Scenario: Rango y continuidad
- **WHEN** se inspecciona el catálogo integrado
- **THEN** existen una vez `py-3101` a `py-3160`, 3100 enlaza con 3101 y 3160 es el único terminal nuevo

### Requirement: Progresión de entrega segura 10x6
La ola MUST organizarse en diez familias de seis ejercicios sobre contratos de release, versionado, compatibilidad, feature flags, canary, migraciones, verificación, rollback, promoción y entrega integral.

#### Scenario: Familias completas
- **WHEN** se agrupan los ejercicios por familia
- **THEN** hay diez familias con seis ejercicios y cada sexta actividad integra las cinco anteriores

### Requirement: Decisiones de entrega deterministas y client-side
Cada ejercicio MUST representar versiones, cohortes, porcentajes, estados y decisiones como datos ordenados, ejecutarse con Python estándar en el cliente y evitar red, concurrencia, reloj real, aleatoriedad, `hash()` e `input()`.

#### Scenario: Repetibilidad
- **WHEN** una solución se ejecuta dos veces con la misma entrada
- **THEN** produce el mismo `resultado` y la misma salida observable

### Requirement: Contrato ejecutable por ejercicio
Cada micro-step SHALL incluir starter, solución, prueba de `resultado`, título, objetivo, prompt, pista y enlace contiguo.

#### Scenario: Ejecución canónica
- **WHEN** se ejecutan las 60 soluciones contra sus pruebas
- **THEN** todas pasan y una segunda aplicación no modifica archivos

### Requirement: Consistencia acumulativa hasta 3160
Catálogo, particiones conceptuales y tres journeys E2E SHALL compartir el techo 3160 preservando los contratos de Olas 26–35.

#### Scenario: Verificación acumulativa
- **WHEN** se ejecutan validadores 26–36, pruebas Rust y conteos E2E
- **THEN** todos observan una cadena única y contigua hasta 3160
