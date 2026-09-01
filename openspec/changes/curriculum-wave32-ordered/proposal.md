## Why

Ola 31 cerró el catálogo en 2860 con pipelines resilientes; el siguiente incremento debe enseñar distribución y agregación sin romper la continuidad ni reutilizar material elemental de olas previas. La rama histórica de Ola 32 mezcla cambios ajenos y ejercicios de `map`/`filter` ya cubiertos, por lo que se reemplaza por una ola original, determinista y auditable sobre `main` verificado.

## What Changes

- Agregar exactamente 60 ejercicios originales para `2861..=2920`, organizados en diez familias de particionado, fan-out/fan-in y agregación distribuida determinista.
- Enlazar 2860 con 2861 y dejar 2920 como único terminal nuevo del rail.
- Extender catálogo, particiones conceptuales, pruebas Rust y los tres conteos E2E canónicos al techo 2920.
- Incorporar generador, aplicador idempotente y validador acumulativo para Ola 32.
- Mantener ejecución Python stdlib-only, local al navegador, sin red, threads, reloj real ni aleatoriedad.

### Alcance incluido

- Ola 32 y únicamente el rango `2861..=2920`.
- Sharding estable, particionado por clave, fan-out/fan-in, map-reduce, agregados parciales, ventanas lógicas, tratamiento de skew, reequilibrio como datos, merge idempotente y cierre integrador.
- Reconciliación explícita contra los contratos congelados de Olas 26–31.

### Fuera de alcance

- Ola 33 o identificadores mayores a 2920.
- Procesamiento distribuido real, red, multiprocessing, threads, servicios externos o nuevas dependencias.
- Cambios de backend, autenticación, despliegue o UI fuera de los conteos del catálogo.
- Reutilización directa del PR histórico #294.

### Plan de rollback

- Restaurar `PY2860_OLA31_SUITE.next` a `None`, retirar `2861..=2920` y devolver particiones, pruebas y conteos a 2860.
- Revertir el único commit/merge de Ola 32 sin afectar el cierre verificado de Ola 31.

## Capabilities

### New Capabilities

- `curriculum-wave32-distributed-aggregation`: extensión contigua y verificable del catálogo con simulaciones deterministas de particionado, combinación y agregación distribuida entre 2861 y 2920.

### Modified Capabilities

- Ninguna.

## Impact

- `web/src/curriculum.rs`: 60 constantes nuevas, cadena y registro del catálogo.
- `web/src/concepts/mod.rs`: particiones conceptuales hasta 2920.
- `web/e2e/tests/`: tres expectativas canónicas pasan de 2860 a 2920.
- `scripts/`: generación, aplicación y validación de Ola 32.
- `openspec/changes/curriculum-wave32-ordered/`: contrato SDD completo.
- Sin cambios de API, dependencias o infraestructura.
