## Why

Tras cerrar la coordinación operativa de Ola 33, el catálogo necesita progresar hacia consistencia de datos y recuperación transaccional sin introducir servicios reales ni concurrencia no determinista. Ola 34 extiende el rail con modelos ejecutables de evolución, consenso y reparación que puedan razonarse íntegramente en Pyodide.

## What Changes

- Agrega exactamente 60 micro-pasos originales y contiguos `2981..=3040`, organizados en diez familias de seis.
- Modela evolución de esquemas, orden total, outbox transaccional, sagas compensables, leases lógicos, quorum, compactación de logs, auditoría, reparación y un capstone integral.
- Integra catálogo, particiones conceptuales, freeze guard, validadores acumulativos y tres journeys E2E al nuevo techo 3040.
- Mantiene soluciones Python estándar, deterministas, client-side y sin red, threads, reloj real, aleatoriedad ni `hash()`.

### Alcance incluido

- Generador declarativo, aplicador idempotente y validador ejecutable para Ola 34.
- Cadena única 2980→2981→…→3040, con 3040 terminal.
- Pruebas Rust, validación acumulativa 26–34 y conteos E2E exactos.

### Fuera de alcance

- Backend, APIs, persistencia real, brokers, bases distribuidas o ejecución remota de código.
- Cambios a olas previas salvo enlaces, techos y validadores acumulativos necesarios.
- Ola 35 o pasos superiores a 3040.

### Plan de rollback

Revertir el commit de Ola 34 restaura el terminal 2980, el techo E2E 2980 y el freeze guard de Ola 33 sin migración de datos.

## Capabilities

### New Capabilities

- `curriculum-wave34-consistent-recovery`: progresión determinista 10x6 de consistencia, transacciones compensables, consenso y reparación entre los micro-pasos 2981–3040.

### Modified Capabilities

<!-- Ninguna capacidad principal existente cambia sus requisitos. -->

## Impact

- `web/src/curriculum.rs` y `web/src/concepts/mod.rs`.
- Tres specs Playwright con conteo del rail.
- Scripts de generación, aplicación y validación acumulativa.
- Nuevo change OpenSpec; sin cambios de API, dependencias o infraestructura.
