## ADDED Requirements

### Requirement: Renderizado de Enunciados por Track

El sistema SHALL mostrar el objetivo y contexto del reto actual antes de que el usuario escriba código, adaptándose al tipo de track (Micro-paso o Reto Ingenieril).

#### Scenario: Visualización de un Micro-paso

- GIVEN un estudiante en el Track 1 (Gimnasio de Sintaxis)
- WHEN carga un nivel de tipo "micro_paso"
- THEN el frontend muestra una tarjeta con instrucciones paso a paso
- AND el motor de evaluación se configura con un prompt de "Tutor Básico"

#### Scenario: Visualización de un Reto Ingenieril

- GIVEN un estudiante en el Track 2 (Laboratorio)
- WHEN carga un nivel de tipo "reto_ingenieril"
- THEN el frontend muestra un problema abierto del mundo real
- AND el motor de evaluación se configura con un prompt de "Arquitecto de Software"
