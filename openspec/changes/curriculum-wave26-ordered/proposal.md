## Why

El catálogo estable termina en el micro-step 2500 y las ramas históricas 26–33 mezclaron contenido futuro, contadores E2E y estados de CI incompatibles. Ola 26 debe retomarse como un incremento único y verificable para restablecer el avance secuencial sin acumulación de ramas.

## What Changes

- Agregar exactamente 60 ejercicios de agregación y reducción avanzada para cubrir `2501..=2560`.
- Encadenar el micro-step 2500 con 2501 y dejar 2560 como único final del rail.
- Extender el catálogo, las particiones conceptuales y los tests de contigüidad al techo 2560.
- Actualizar los tres conteos E2E canónicos del rail a 2560.
- Incorporar un validador determinista de Ola 26 y verificar Rust, harness journeys y CI antes de integrar.

### Alcance incluido

- Ola 26 y únicamente el rango `2501..=2560`.
- Frontend Leptos/Rust y ejecución Python en cliente, sin endpoints nuevos.
- Artefactos OpenSpec, generador/validador reproducibles, pruebas unitarias y E2E afectadas.

### Fuera de alcance

- Ola 27 o cualquier micro-step mayor a 2560.
- Cambios de backend, persistencia, autenticación, búsqueda global o infraestructura.
- Reutilizar o mergear en bloque las ramas históricas de Olas 26–33.

### Plan de rollback

- Revertir el único squash commit de Ola 26 para restaurar el catálogo estable `1..=2500`.
- El rollback debe devolver `PY2500_SCORE_CHECK.next` a `None`, retirar `2501..=2560`, restaurar particiones/conteos a 2500 y mantener intactos los cambios operativos ya integrados.

## Capabilities

### New Capabilities

- `curriculum-wave26-catalog`: Define la extensión ordenada, contigua y comprobable del catálogo desde 2500 hasta 2560.

### Modified Capabilities

Ninguna.

## Impact

- Código: `web/src/curriculum.rs`, `web/src/concepts/mod.rs` y scripts de Ola 26.
- Pruebas: unit tests Rust y conteos en `web/e2e/tests/`.
- Catálogo visible: el rail pasa de 2500 a 2560 elementos sin cambiar APIs ni dependencias.
- Integración: un solo PR sobre `main`; Ola 27 permanece bloqueada hasta verificar el merge y los workflows post-merge.
