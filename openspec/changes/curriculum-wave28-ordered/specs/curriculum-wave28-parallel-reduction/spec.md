## Purpose

Extender el rail curricular con una ola original y verificable de patrones paralelizables y reducción avanzada, ejecutable de forma segura y determinista en el navegador.

## ADDED Requirements

### Requirement: Catálogo exacto y contiguo hasta 2680

El catálogo MUST contener exactamente los identificadores únicos `1..=2680`, incluyendo sesenta pasos nuevos `2621..=2680` y ninguno posterior.

#### Scenario: Validación del rango
- **WHEN** se valida Ola 28
- **THEN** el catálogo contiene 2680 micro-steps únicos, ordenados y sin huecos

### Requirement: Contenido original de pipelines paralelizables

Los sesenta ejercicios MUST enseñar particionado, batching, alineación, fan-out/fan-in, resultados parciales, combinación asociativa, agrupación y map-reduce local; MUST NOT repetir integralmente los enunciados y soluciones de Olas 26 y 27.

#### Scenario: Auditoría pedagógica
- **WHEN** se comparan las firmas pedagógicas de Olas 26, 27 y 28
- **THEN** ningún ejercicio de Ola 28 coincide integralmente en slug, enunciado y solución con ejercicios anteriores

### Requirement: Paralelismo conceptual sin concurrencia insegura

Los ejercicios MUST modelar trabajo separable mediante datos y funciones puras, y MUST NOT crear threads, procesos, solicitudes de red ni carreras de estado compartido.

#### Scenario: Ejecución en runtime cliente
- **WHEN** el alumno ejecuta una solución de Ola 28
- **THEN** la solución termina determinísticamente usando solo Python compatible con el navegador

### Requirement: Reducciones parciales reproducibles

Cada ejercicio de reducción distribuible MUST preservar el resultado al combinar parciales en el orden contractual, con entradas pequeñas y resultados comprobables.

#### Scenario: Equivalencia de reducción
- **WHEN** una entrada se divide en chunks y se combinan sus resultados parciales
- **THEN** el valor final coincide con la reducción directa especificada por el ejercicio

### Requirement: Navegación de frontera y terminal

La navegación MUST enlazar 2620 con 2621, continuar en orden hasta 2680 y dejar 2680 como terminal.

#### Scenario: Recorrido completo
- **WHEN** el alumno recorre desde el último paso de Ola 27
- **THEN** alcanza todos los pasos `2621..=2680` sin saltos, ciclos ni referencias inexistentes

### Requirement: Particiones y conteos sincronizados

Las particiones conceptuales y los tres journeys E2E canónicos MUST reconocer el techo 2680 sin referencias a Ola 29.

#### Scenario: Render e indexación
- **WHEN** se compila y renderiza el catálogo completo
- **THEN** existen 2680 tarjetas y los límites conceptuales de Ola 28 pertenecen al catálogo vigente

### Requirement: Ejecución exclusivamente cliente

El código Python del alumno MUST ejecutarse en el navegador y MUST NOT crear endpoints backend de ejecución.

#### Scenario: Revisión arquitectónica
- **WHEN** se inspecciona el diff de Ola 28
- **THEN** los cambios se limitan al catálogo frontend, conceptos, scripts y pruebas relacionadas
