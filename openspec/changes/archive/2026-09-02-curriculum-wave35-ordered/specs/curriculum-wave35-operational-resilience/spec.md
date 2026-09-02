## Purpose

Definir una progresión educativa determinista para razonar sobre observabilidad, objetivos de confiabilidad, control de carga y respuesta a incidentes sin infraestructura externa.

## ADDED Requirements

### Requirement: Catálogo exacto y original de Ola 35
El catálogo SHALL incorporar exactamente 60 ejercicios originales, únicos y contiguos entre 3041 y 3100, sin duplicar firmas completas de Olas 26–34.

#### Scenario: Rango y continuidad
- **WHEN** se inspecciona el catálogo integrado
- **THEN** existen una vez `py-3041` a `py-3100`, 3040 enlaza con 3041 y 3100 es el único terminal nuevo

### Requirement: Progresión de resiliencia operacional 10x6
La ola MUST organizarse en diez familias de seis ejercicios sobre telemetría, métricas, SLI/SLO, presupuestos de error, anomalías, backpressure, descarte controlado, capacidad, incidentes y resiliencia integral.

#### Scenario: Familias completas
- **WHEN** se agrupan los ejercicios por familia
- **THEN** hay diez familias con seis ejercicios y cada sexta actividad integra las cinco anteriores

### Requirement: Decisiones operativas deterministas y client-side
Cada ejercicio MUST representar señales, umbrales y decisiones como datos ordenados, ejecutarse con Python estándar en el cliente y evitar red, concurrencia, reloj real, aleatoriedad, `hash()` e `input()`.

#### Scenario: Repetibilidad
- **WHEN** una solución se ejecuta dos veces con la misma entrada
- **THEN** produce el mismo `resultado` y la misma salida observable

### Requirement: Contrato ejecutable por ejercicio
Cada micro-step SHALL incluir starter, solución, prueba de `resultado`, título, objetivo, prompt, pista y enlace contiguo.

#### Scenario: Ejecución canónica
- **WHEN** se ejecutan las 60 soluciones contra sus pruebas
- **THEN** todas pasan y una segunda aplicación no modifica archivos

### Requirement: Consistencia acumulativa hasta 3100
Catálogo, particiones conceptuales y tres journeys E2E SHALL compartir el techo 3100 preservando los contratos de Olas 26–34.

#### Scenario: Verificación acumulativa
- **WHEN** se ejecutan validadores 26–35, pruebas Rust y conteos E2E
- **THEN** todos observan una cadena única y contigua hasta 3100
