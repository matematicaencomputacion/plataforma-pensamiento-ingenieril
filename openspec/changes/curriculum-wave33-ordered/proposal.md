## Why

Ola 32 cerró en 2920 con particionado y agregación distribuida determinista; falta enseñar cómo coordinar, observar y recuperar ese procesamiento sin introducir infraestructura real ni no determinismo. La rama histórica #295 solo agrega un validador roto sobre archivos inexistentes y arrastra una base contaminada, por lo que debe reemplazarse por una ola completa sobre `main` verificado.

## What Changes

- Agregar exactamente 60 ejercicios originales para `2921..=2980` en diez familias de coordinación y garantías operativas de pipelines.
- Cubrir offsets, acknowledgements, deduplicación, checkpoints, barreras, backpressure, lineage, replay, reconciliación de estado y un capstone coordinado.
- Enlazar 2920 con 2921 y dejar 2980 como único terminal nuevo.
- Extender catálogo, particiones conceptuales, pruebas Rust y tres conteos E2E al techo 2980.
- Incorporar generador, aplicador idempotente y validador acumulativo de Ola 33.

### Alcance incluido

- Ola 33 y únicamente `2921..=2980`.
- Simulaciones deterministas mediante listas, tuplas y diccionarios ordenados.
- Compatibilidad acumulativa de validadores 26–33.

### Fuera de alcance

- Identificadores mayores a 2980 o una Ola 34.
- Brokers, bases externas, red, threads, multiprocessing, reloj real o nuevas dependencias.
- Cambios de backend, despliegue, autenticación o UI fuera de conteos del catálogo.
- Reutilización o reapertura del PR histórico #295.

### Plan de rollback

- Restaurar `PY2920_OLA32_SUITE.next` a `None`, retirar `2921..=2980` y devolver particiones, tests y E2E a 2920.
- Revertir el squash de Ola 33 sin alterar Olas 31–32 verificadas.

## Capabilities

### New Capabilities

- `curriculum-wave33-coordinated-pipelines`: extensión contigua del catálogo con coordinación, observabilidad y recuperación deterministas para pipelines entre 2921 y 2980.

### Modified Capabilities

- Ninguna.

## Impact

- `web/src/curriculum.rs`, `web/src/concepts/mod.rs` y tres specs E2E de conteo.
- `scripts/gen_wave33.py`, `scripts/apply_wave33.py`, `scripts/validate_wave33.py` y validadores acumulativos previos.
- Artefactos SDD en `openspec/changes/curriculum-wave33-ordered/`.
- Sin cambios de API, dependencias ni infraestructura.
