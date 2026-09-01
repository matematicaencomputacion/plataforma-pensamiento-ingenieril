## Purpose

Define una extensión educativa determinista que modela particionado y agregación distribuida sin infraestructura concurrente ni servicios externos.

## ADDED Requirements

### Requirement: Catálogo exacto y original de Ola 32
El catálogo SHALL incorporar exactamente 60 ejercicios originales con identificadores únicos y contiguos entre 2861 y 2920, sin duplicar la firma completa de ejercicios de Olas 26–31.

#### Scenario: Rango y unicidad
- **WHEN** se inspecciona el catálogo resultante
- **THEN** existen una vez todos los identificadores `py-2861` a `py-2920`, no existe un identificador posterior y 2920 es el único terminal nuevo

#### Scenario: Continuidad desde Ola 31
- **WHEN** un estudiante completa el micro-step 2860
- **THEN** la navegación continúa hacia 2861 y recorre Ola 32 en orden hasta 2920

### Requirement: Progresión pedagógica distribuida
La ola MUST organizarse en diez familias de seis ejercicios que progresen desde operaciones aisladas hasta una integración, cubriendo particionado estable, sharding por clave, fan-out/fan-in, map-reduce, agregados parciales, ventanas lógicas, skew, reequilibrio como datos, merge idempotente y capstone distribuido.

#### Scenario: Familias 10x6
- **WHEN** se agrupan los ejercicios por su familia declarada
- **THEN** existen exactamente diez familias con seis ejercicios cada una y la sexta actividad de cada familia integra las cinco anteriores

#### Scenario: Evidencia de distribución y agregación
- **WHEN** se auditan objetivos, soluciones y resultados de los 60 ejercicios
- **THEN** cada ejercicio pertenece a una familia declarada y el conjunto contiene evidencia verificable de particionado, combinación parcial y reducción determinista

### Requirement: Ejecución determinista en cliente
Cada solución MUST usar únicamente Python estándar compatible con ejecución client-side y MUST evitar red, threads, multiprocessing, reloj real, aleatoriedad e `input()`.

#### Scenario: Repetibilidad
- **WHEN** una solución se ejecuta repetidamente con la misma entrada
- **THEN** produce el mismo valor `resultado` y la misma salida observable

#### Scenario: Restricciones de runtime
- **WHEN** el validador inspecciona y ejecuta las soluciones
- **THEN** ninguna requiere infraestructura distribuida real, I/O externo ni paquetes de terceros

### Requirement: Contrato ejecutable por ejercicio
Cada micro-step SHALL incluir starter, solución y prueba ejecutable que valide `resultado`, además de título, objetivo, prompt, pista y enlace al siguiente paso.

#### Scenario: Solución canónica
- **WHEN** se ejecuta cada solución contra su prueba
- **THEN** las 60 pruebas pasan y la segunda aplicación del generador no produce cambios

#### Scenario: Cadena completa
- **WHEN** se recorren los enlaces desde 2860
- **THEN** se visitan exactamente 2861 a 2920 en orden y el paso 2920 termina con `next = None`

### Requirement: Consistencia acumulativa del catálogo
El catálogo, las particiones conceptuales y los tres journeys E2E canónicos SHALL compartir el techo 2920, preservando los contratos ejecutables de Olas 26–31.

#### Scenario: Techo único
- **WHEN** se ejecutan los validadores acumulativos, las pruebas Rust y las aserciones E2E
- **THEN** todos observan 2920 ejercicios únicos, una cadena contigua y ninguna referencia posterior

#### Scenario: No regresión de olas previas
- **WHEN** se validan secuencialmente las Olas 26–32
- **THEN** todos los validadores pasan sobre el catálogo integrado de Ola 32
