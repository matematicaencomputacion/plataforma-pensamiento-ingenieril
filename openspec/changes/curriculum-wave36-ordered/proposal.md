## Why

Ola 35 cerró resiliencia operacional hasta 3100; el siguiente incremento debe enseñar a cambiar sistemas en operación con compatibilidad, exposición gradual y reversión verificable. Ola 36 convierte decisiones de entrega segura en transformaciones deterministas de datos, sin desplegar infraestructura ni ejecutar código fuera del navegador.

## What Changes

- Agrega exactamente 60 micro-pasos originales y contiguos `3101..=3160`, organizados en diez familias de seis.
- Modela contratos de release, versionado semántico, compatibilidad, feature flags, canary, migraciones expand/contract, verificación, rollback, promoción y un capstone de entrega segura.
- Integra catálogo, particiones conceptuales, freeze guard, validadores acumulativos y tres journeys E2E al techo 3160.
- Mantiene soluciones Python estándar, deterministas, client-side y sin red, threads, reloj real, aleatoriedad ni `hash()`.

### Alcance incluido

- Generador declarativo, aplicador idempotente y validador ejecutable para Ola 36.
- Cadena única 3100→3101→…→3160, con 3160 terminal.
- Pruebas Rust, validación acumulativa 26–36 y conteos E2E exactos.

### Fuera de alcance

- Backend, APIs, despliegues reales, proveedores cloud, cambios de esquema persistentes o automatización externa de releases.
- Cambios a olas previas salvo enlaces, techos y validadores acumulativos necesarios.
- Ola 37 o pasos superiores a 3160.

### Plan de rollback

Revertir el commit de Ola 36 restaura el terminal 3100, el techo E2E 3100 y el freeze guard de Ola 35 sin migración de datos.

## Capabilities

### New Capabilities

- `curriculum-wave36-safe-delivery`: progresión determinista 10x6 de contratos, compatibilidad, exposición gradual, migración, verificación y reversión entre los micro-pasos 3101–3160.

### Modified Capabilities

Ninguna.

## Impact

- `web/src/curriculum.rs` y `web/src/concepts/mod.rs`.
- Tres specs Playwright con conteo del rail.
- Scripts de generación, aplicación y validación acumulativa.
- Nuevo change OpenSpec; sin cambios de API, dependencias o infraestructura.
