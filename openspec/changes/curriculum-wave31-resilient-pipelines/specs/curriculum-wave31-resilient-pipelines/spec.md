## Purpose

Extender el rail de Python con una progresión determinista y verificable sobre validación, aislamiento de fallos y recuperación de pipelines.

## ADDED Requirements

### Requirement: Ola 31 contigua y original
El catálogo SHALL incorporar exactamente 60 ejercicios originales con identificadores únicos y contiguos entre 2801 y 2860, sin duplicar la firma completa de ejercicios de Olas 26–30.

#### Scenario: Catálogo exacto
- **WHEN** se valida el catálogo integrado
- **THEN** existen una vez todos los identificadores `py-2801` a `py-2860`, no existe un identificador posterior y 2860 es el único terminal nuevo

#### Scenario: Frontera con Ola 30
- **WHEN** el alumno completa el ejercicio 2800
- **THEN** la navegación continúa hacia 2801 y recorre Ola 31 en orden hasta 2860

#### Scenario: Contenido no duplicado
- **WHEN** se comparan slug, prompt y solución de cada ejercicio de Ola 31 contra Olas 26–30
- **THEN** ninguna firma completa coincide

### Requirement: Progresión de fiabilidad determinista
La ola SHALL cubrir diez familias de seis pasos sobre validación, errores acumulados, cuarentena, reintentos, circuit breaker, límites lógicos, watermarks, eventos tardíos, compensaciones y recuperación integrada.

#### Scenario: Familias completas
- **WHEN** se inspeccionan los 60 ejercicios en grupos consecutivos de seis
- **THEN** cada familia progresa desde una operación aislada hasta una aplicación compuesta y verificable

#### Scenario: Fallos como datos
- **WHEN** una entrada no satisface un contrato o una transición falla
- **THEN** la solución representa el resultado de manera explícita y determinista sin excepciones no controladas ni efectos externos

#### Scenario: Tiempo lógico reproducible
- **WHEN** una solución evalúa reintentos, límites o eventos tardíos
- **THEN** usa ticks o marcas enteras provistas en los datos y produce el mismo resultado para la misma entrada

### Requirement: Ejecución real y segura en navegador
Cada solución SHALL usar únicamente Python compatible con Pyodide y SHALL ser ejecutada contra sus pruebas durante la validación de la ola.

#### Scenario: Sesenta soluciones ejecutadas
- **WHEN** se ejecuta el validador de Ola 31
- **THEN** las 60 soluciones se cargan y cada prueba generada se invoca realmente sobre su archivo de solución

#### Scenario: APIs excluidas
- **WHEN** se auditan prompts, soluciones y pruebas
- **THEN** no aparecen threads, procesos, async, sleeps, red, archivos externos, reloj de pared, aleatoriedad ni placeholders

### Requirement: Integración acumulativa coherente
El catálogo, las particiones conceptuales y los tres journeys E2E canónicos SHALL compartir el techo 2860, preservando los contratos de olas previas.

#### Scenario: Techo único
- **WHEN** se ejecutan validadores acumulativos, pruebas Rust y journeys E2E
- **THEN** todos observan 2860 ejercicios únicos, una cadena contigua y ninguna referencia posterior

#### Scenario: Aplicación atómica
- **WHEN** falta un ancla esperada o se intenta aplicar Ola 31 por segunda vez
- **THEN** el aplicador falla sin modificar parcialmente ningún archivo
