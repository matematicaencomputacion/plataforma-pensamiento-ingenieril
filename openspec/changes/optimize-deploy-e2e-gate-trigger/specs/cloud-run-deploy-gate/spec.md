## Purpose

Garantiza que cada promoción a Cloud Run use un único commit de `main` validado por todos los gates de producto, sin consumir un runner en esperas activas prolongadas.

## ADDED Requirements

### Requirement: E2E exitoso inicia la promoción
El sistema de entrega SHALL iniciar la evaluación de deploy únicamente cuando `E2E Web (Playwright)` complete exitosamente para un evento push de `main`.

#### Scenario: E2E verde en main
- **GIVEN** un push a `main` cuyo workflow E2E termina exitosamente
- **WHEN** GitHub publica el evento de finalización
- **THEN** el workflow de deploy evalúa ese mismo SHA para promoción

#### Scenario: E2E fallido
- **GIVEN** un push a `main` cuyo workflow E2E falla o es cancelado
- **WHEN** GitHub publica el evento de finalización
- **THEN** el job de deploy no autentica contra GCP ni publica una imagen

### Requirement: Todos los gates corresponden al mismo SHA
Antes de autenticar contra GCP, el deploy MUST comprobar que CI y Docker terminaron exitosamente para el mismo SHA entregado por el evento E2E.

#### Scenario: Gates coherentes
- **GIVEN** E2E exitoso para un SHA de `main`
- **WHEN** CI y Docker del mismo SHA también están completos y verdes
- **THEN** la promoción puede continuar a checkout, autenticación y deploy

#### Scenario: Gate ausente o fallido
- **GIVEN** E2E exitoso para un SHA de `main`
- **WHEN** CI o Docker del mismo SHA está ausente, pendiente o no exitoso
- **THEN** el workflow falla antes de cualquier interacción con GCP

### Requirement: Sin espera activa prolongada
El gate de deploy MUST resolver mediante consultas acotadas después del evento E2E y SHALL NOT mantener un runner sondeando E2E durante decenas de minutos.

#### Scenario: Evaluación del gate
- **GIVEN** que E2E ya terminó y disparó el workflow
- **WHEN** se verifican CI y Docker
- **THEN** cada workflow se consulta una vez y el gate termina de forma determinista
