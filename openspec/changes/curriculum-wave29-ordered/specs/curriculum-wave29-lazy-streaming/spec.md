## Purpose

Extender el rail curricular con pipelines lazy y agregaciones streaming deterministas, ejecutables íntegramente en el navegador y verificables paso a paso.

## ADDED Requirements

### Requirement: Catálogo exacto y contiguo hasta 2740
El catálogo MUST contener exactamente los identificadores únicos `1..=2740`, incluyendo sesenta pasos nuevos `2681..=2740` y ninguno posterior.

#### Scenario: Validación del rango
- **WHEN** se valida Ola 29
- **THEN** el catálogo contiene 2740 micro-steps únicos, ordenados y sin huecos

### Requirement: Progresión de pipelines lazy y streaming
Los sesenta ejercicios MUST enseñar funciones como valores, callbacks, generadores, materialización controlada, folding, logs, terminación temprana, agregación streaming y ranking; MUST NOT repetir integralmente las firmas pedagógicas de Olas 26, 27 y 28.

#### Scenario: Auditoría pedagógica
- **WHEN** se comparan slug, enunciado y solución de Olas 26 a 29
- **THEN** ningún ejercicio de Ola 29 coincide integralmente con un ejercicio anterior

### Requirement: Pereza observable y materialización explícita
Los ejercicios lazy MUST distinguir la construcción de un iterable de su consumo y MUST materializarlo únicamente cuando el resultado observable lo requiera.

#### Scenario: Consumo incremental
- **WHEN** un ejercicio usa un generador o iterador
- **THEN** su prueba demuestra consumo acotado, cortocircuito o materialización explícita y determinista

### Requirement: Folding y agregación reproducibles
Cada reducción MUST declarar una identidad o precondición válida, preservar el orden cuando la operación no sea conmutativa y producir un resultado comprobable con entradas pequeñas.

#### Scenario: Resultado reproducible
- **WHEN** se ejecuta una solución de folding o agregación streaming
- **THEN** el resultado y la salida estándar coinciden con el contrato del ejercicio

### Requirement: Runtime cliente seguro
Las soluciones MUST usar únicamente Python determinista compatible con el navegador y MUST NOT crear threads, procesos, solicitudes de red, acceso a archivos externos ni tareas asíncronas.

#### Scenario: Ejecución aislada
- **WHEN** se auditan y ejecutan las sesenta soluciones
- **THEN** todas terminan localmente sin APIs prohibidas ni efectos externos

### Requirement: Navegación de frontera y terminal
La navegación MUST enlazar 2680 con 2681, continuar en orden hasta 2740 y dejar 2740 como terminal.

#### Scenario: Recorrido completo
- **WHEN** el alumno recorre desde el último paso de Ola 28
- **THEN** alcanza todos los pasos `2681..=2740` sin saltos, ciclos ni referencias inexistentes

### Requirement: Particiones y conteos sincronizados
Las particiones conceptuales y los tres journeys E2E canónicos MUST reconocer el techo 2740 sin referencias a Ola 30.

#### Scenario: Render e indexación
- **WHEN** se compila y renderiza el catálogo completo
- **THEN** existen 2740 tarjetas y los límites conceptuales de Ola 29 pertenecen al catálogo vigente

### Requirement: Ejecución exclusivamente cliente
El código Python del alumno MUST ejecutarse en el navegador y MUST NOT crear endpoints backend de ejecución.

#### Scenario: Revisión arquitectónica
- **WHEN** se inspecciona el diff de Ola 29
- **THEN** los cambios se limitan al catálogo frontend, conceptos, scripts y pruebas relacionadas
