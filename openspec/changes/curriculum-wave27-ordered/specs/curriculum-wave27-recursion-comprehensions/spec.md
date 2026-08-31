## Purpose

Extender el rail curricular con una ola original y verificable de recursión y comprensiones, evitando duplicar el contenido funcional de la ola anterior.

## ADDED Requirements

### Requirement: Catálogo exacto y contiguo hasta 2620

El catálogo MUST contener exactamente los identificadores únicos `1..=2620`, incluyendo sesenta pasos nuevos `2561..=2620` y ninguno posterior.

#### Scenario: Validación del rango
- **WHEN** se valida Ola 27
- **THEN** el catálogo contiene 2620 micro-steps únicos, ordenados y sin huecos

### Requirement: Contenido original de recursión y comprensiones

Los sesenta ejercicios MUST enseñar recursión y comprensiones mediante casos base, reducción de problema, recorridos estructurales y comprensiones de listas, diccionarios y conjuntos; MUST NOT repetir los enunciados y soluciones de Ola 26.

#### Scenario: Auditoría pedagógica
- **WHEN** se comparan las firmas pedagógicas de Olas 26 y 27
- **THEN** ningún ejercicio de Ola 27 coincide integralmente en slug, enunciado y solución con su contraparte de Ola 26

### Requirement: Navegación de frontera y terminal

La navegación MUST enlazar 2560 con 2561, continuar en orden hasta 2620 y dejar 2620 como terminal.

#### Scenario: Recorrido completo
- **WHEN** el alumno recorre desde el último paso de Ola 26
- **THEN** alcanza todos los pasos `2561..=2620` sin saltos, ciclos ni referencias inexistentes

### Requirement: Ejercicios deterministas y seguros en navegador

Cada solución y prueba MUST usar solamente Python compatible con el runtime cliente, sin red, threads, `input()` ni recursión no acotada.

#### Scenario: Ejecución de soluciones esperadas
- **WHEN** el validador ejecuta las sesenta soluciones contra sus pruebas
- **THEN** todas finalizan determinísticamente y pasan sin servicios externos

### Requirement: Particiones y conteos sincronizados

Las particiones conceptuales y los tres journeys E2E canónicos MUST reconocer el techo 2620 sin referencias a Ola 28.

#### Scenario: Render e indexación
- **WHEN** se compila y renderiza el catálogo completo
- **THEN** existen 2620 tarjetas y los límites conceptuales de Ola 27 pertenecen al catálogo vigente

### Requirement: Ejecución exclusivamente cliente

El código Python del alumno MUST ejecutarse en el navegador y MUST NOT crear endpoints backend de ejecución.

#### Scenario: Revisión arquitectónica
- **WHEN** se inspecciona el diff de Ola 27
- **THEN** los cambios se limitan al catálogo frontend, conceptos, scripts y pruebas relacionadas
