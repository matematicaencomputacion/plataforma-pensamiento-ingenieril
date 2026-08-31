## Purpose

Definir la extensión ordenada y verificable del rail curricular con los sesenta micro-steps de Ola 26, sin incorporar contenido de olas posteriores ni alterar la ejecución local en el navegador.

## ADDED Requirements

### Requirement: Catálogo exacto y contiguo hasta 2560

El catálogo curricular MUST contener exactamente los identificadores únicos `PY0001_STEP` a `PY2560_STEP`, con los sesenta identificadores nuevos `PY2501_STEP..=PY2560_STEP` y sin identificadores superiores a 2560.

#### Scenario: Verificación del rango completo

- **WHEN** se ejecuta el validador determinista de Ola 26
- **THEN** confirma 2560 identificadores únicos, el rango exacto `1..=2560` y ausencia de huecos o duplicados

### Requirement: Navegación encadenada de Ola 26

La navegación MUST enlazar `PY2500_SCORE_CHECK` con `PY2501_STEP`, continuar en orden por todos los micro-steps de Ola 26 y dejar `PY2560_STEP` como único terminal nuevo.

#### Scenario: Recorrido desde la frontera anterior

- **WHEN** el alumno completa el micro-step 2500 y recorre Ola 26
- **THEN** la secuencia avanza por `2501..=2560` sin saltos, ciclos ni referencias inexistentes

### Requirement: Registro integral en el catálogo Rust

Cada constante de Ola 26 MUST estar incluida exactamente una vez en `CODING_STEPS`, y las pruebas Rust SHALL comprobar el tamaño total, la contigüidad y los límites de la ola.

#### Scenario: Compilación del catálogo

- **WHEN** se compila y ejecuta la suite Rust del frontend
- **THEN** todas las referencias resuelven y las aserciones del catálogo aceptan exactamente 2560 entradas

### Requirement: Particiones conceptuales coherentes

Las particiones conceptuales MUST permanecer ordenadas, referenciar límites existentes y cubrir la extensión de Ola 26 hasta el micro-step 2560 sin introducir límites de Ola 27.

#### Scenario: Inspección de particiones

- **WHEN** se validan los límites conceptuales después de generar Ola 26
- **THEN** el último límite es 2560 y todos los límites pertenecen al catálogo vigente

### Requirement: Conteos visibles sincronizados

Los tres journeys E2E canónicos del rail SHALL esperar 2560 tarjetas visibles y MUST conservar sus verificaciones de identificadores únicos y navegación.

#### Scenario: Render del rail completo

- **WHEN** Playwright carga el catálogo curricular completo
- **THEN** encuentra 2560 tarjetas y no detecta duplicados ni un techo distinto del catálogo Rust

### Requirement: Ejecución Python exclusivamente cliente

Los ejercicios Python de Ola 26 MUST conservar la ejecución en Pyodide/JupyterLite dentro del navegador y MUST NOT agregar endpoints de backend para ejecutar código del alumno.

#### Scenario: Revisión de impacto arquitectónico

- **WHEN** se inspecciona el diff de Ola 26
- **THEN** los cambios quedan limitados al catálogo frontend, conceptos, scripts y pruebas asociadas
