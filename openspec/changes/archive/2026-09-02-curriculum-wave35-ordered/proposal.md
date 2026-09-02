## Why

Ola 34 cerró consistencia y recuperación hasta 3040; el siguiente incremento debe enseñar cómo operar sistemas bajo objetivos explícitos sin introducir infraestructura real ni repetir los mecanismos transaccionales ya cubiertos. Ola 35 convierte señales, presupuestos y decisiones operativas en datos deterministas ejecutables íntegramente en el navegador.

## What Changes

- Agrega exactamente 60 micro-pasos originales y contiguos `3041..=3100`, organizados en diez familias de seis.
- Modela telemetría estructurada, métricas derivadas, SLI/SLO, presupuestos de error, detección de anomalías, backpressure, load shedding, capacidad, respuesta a incidentes y un capstone de resiliencia operacional.
- Integra catálogo, particiones conceptuales, freeze guard, validadores acumulativos y tres journeys E2E al techo 3100.
- Mantiene soluciones Python estándar, deterministas, client-side y sin red, threads, reloj real, aleatoriedad ni `hash()`.

### Alcance incluido

- Generador declarativo, aplicador idempotente y validador ejecutable para Ola 35.
- Cadena única 3040→3041→…→3100, con 3100 terminal.
- Pruebas Rust, validación acumulativa 26–35 y conteos E2E exactos.

### Fuera de alcance

- Backend, APIs, observabilidad real, alertas externas, autoscaling de infraestructura o ejecución remota de código.
- Cambios a olas previas salvo enlaces, techos y validadores acumulativos necesarios.
- Ola 36 o pasos superiores a 3100.

### Plan de rollback

Revertir el commit de Ola 35 restaura el terminal 3040, el techo E2E 3040 y el freeze guard de Ola 34 sin migración de datos.

## Capabilities

### New Capabilities

- `curriculum-wave35-operational-resilience`: progresión determinista 10x6 de señales, objetivos de confiabilidad, control de carga, capacidad y respuesta a incidentes entre los micro-pasos 3041–3100.

### Modified Capabilities

Ninguna.

## Impact

- `web/src/curriculum.rs` y `web/src/concepts/mod.rs`.
- Tres specs Playwright con conteo del rail.
- Scripts de generación, aplicación y validación acumulativa.
- Nuevo change OpenSpec; sin cambios de API, dependencias o infraestructura.
