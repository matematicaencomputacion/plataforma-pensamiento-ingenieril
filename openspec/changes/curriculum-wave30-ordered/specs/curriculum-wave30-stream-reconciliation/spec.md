## Purpose

Extender el rail de Python con una progresión determinista y verificable sobre merge ordenado, agregación online, checkpoints puros e idempotencia.

## ADDED Requirements

### Requirement: Ola 30 contigua y original
El catálogo SHALL incorporar exactamente 60 ejercicios originales con identificadores únicos y contiguos entre 2741 y 2800, sin duplicar la firma completa de ejercicios de Olas 26–29.

#### Scenario: Catálogo exacto
- **WHEN** se valida el catálogo integrado
- **THEN** existen una vez todos los identificadores `py-2741` a `py-2800`, no existe un identificador posterior y 2800 es el único terminal nuevo

#### Scenario: Frontera con Ola 29
- **WHEN** el alumno completa el ejercicio 2740
- **THEN** la navegación continúa hacia 2741 y recorre Ola 30 en orden hasta 2800

#### Scenario: Contenido no duplicado
- **WHEN** se comparan slug, prompt y solución de cada ejercicio de Ola 30 contra Olas 26–29
- **THEN** ninguna firma completa coincide

### Requirement: Progresión de reconciliación y agregación online
La ola SHALL cubrir diez familias de seis pasos sobre merge ordenado, heaps, top-k, estadísticas online, estado incremental, ventanas, checkpoints, idempotencia, reconciliación y un cierre integrador.

#### Scenario: Familias completas
- **WHEN** se inspeccionan los 60 ejercicios en grupos consecutivos de seis
- **THEN** cada familia progresa desde una operación aislada hasta una aplicación compuesta y verificable

#### Scenario: Resultados deterministas
- **WHEN** una solución recibe la misma entrada más de una vez
- **THEN** produce el mismo valor y orden sin depender de reloj, azar, red, archivos ni concurrencia

### Requirement: Ejecución real y segura en navegador
Cada solución SHALL usar únicamente Python compatible con Pyodide y SHALL ser ejecutada contra sus pruebas durante la validación de la ola.

#### Scenario: Sesenta soluciones ejecutadas
- **WHEN** se ejecuta el validador de Ola 30
- **THEN** las 60 soluciones se cargan y cada prueba generada se invoca realmente sobre su archivo de solución

#### Scenario: APIs excluidas
- **WHEN** se auditan prompts, soluciones y pruebas
- **THEN** no aparecen threads, procesos, async, red, archivos externos, reloj de pared, aleatoriedad ni placeholders

### Requirement: Integración acumulativa coherente
El catálogo, las particiones conceptuales y los tres journeys E2E canónicos SHALL compartir el techo 2800, preservando los contratos de olas previas.

#### Scenario: Techo único
- **WHEN** se ejecutan validadores acumulativos, pruebas Rust y journeys E2E
- **THEN** todos observan 2800 ejercicios únicos, una cadena contigua y ninguna referencia posterior

#### Scenario: Aplicación atómica
- **WHEN** falta un ancla esperada o se intenta aplicar Ola 30 por segunda vez
- **THEN** el aplicador falla sin modificar parcialmente ningún archivo
