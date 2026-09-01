## Context

`main` termina en 2920 con Ola 32 y sus cuatro gates post-merge verdes. La rama histórica #295 contiene solo un validador con `os.path.abspify`, depende de archivos inexistentes y proviene de una base que elimina miles de líneas; no es una fuente integrable.

## Goals / Non-Goals

**Goals:**

- Crear 60 ejercicios originales 10x6 sobre coordinación y garantías operativas.
- Representar estados distribuidos mediante datos deterministas y Wasm-safe.
- Mantener aplicación exacta, protegida e idempotente sobre el techo 2920.
- Preservar validadores acumulativos y un diff limitado.

**Non-Goals:**

- Ejecutar brokers, workers, red o concurrencia real.
- Introducir dependencias, APIs o cambios de infraestructura.
- Recuperar artefactos del PR histórico #295.

## Decisions

### D1 — Implementación nueva sobre el merge verificado

Se crean generador, aplicador y validador desde `main@ea4d6f7e`; no se hace cherry-pick histórico. Esto elimina tanto el error tipográfico como la contaminación de base.

### D2 — Coordinación como máquina de estados pura

Offsets, confirmaciones, snapshots, barreras, créditos, linaje y replay se expresan con enteros, listas, tuplas y diccionarios ordenados. Se descartan threads, colas reales y tiempo de pared porque romperían Wasm-safety y repetibilidad.

### D3 — Diez familias con progresión integradora

Las familias serán offsets; acknowledgements; deduplicación de entrega; checkpoints; barreras; backpressure por créditos; lineage; replay; reconciliación de estados; y capstone coordinado. Cada familia avanza desde una operación aislada hasta una suite.

### D4 — Generación compacta pero explícita

Cada familia declara seis casos estructurados. El generador reutiliza únicamente las funciones de emisión de Ola 32, mientras casos, slugs, soluciones y expectativas son exclusivos de Ola 33.

### D5 — Integración protegida e idempotente

El aplicador exige techo 2920, anclas únicas y ausencia de 2980 antes de escribir; una segunda ejecución devuelve cero cambios. También actualiza techo Rust, particiones, freeze test y tres E2E.

### D6 — Validación acumulativa

El validador ejecuta soluciones reales, verifica 10x6, originalidad, tokens prohibidos, cadena y techo. Se ejecutarán validadores 26–33, 138 tests Rust esperados, backend, journeys y OpenSpec estricto.

## Risks / Trade-offs

- [Conceptos operativos abstractos] → usar estados pequeños y resultados visibles.
- [Solapamiento con resiliencia] → enfocar Ola 33 en coordinación, progreso y trazabilidad, no manejo de fallos de Ola 31.
- [No determinismo] → orden explícito y prohibición de `hash()`, tiempo y concurrencia.
- [Drift de catálogo] → verificar 2980 en todas las capas.
- [Deuda de rustfmt global] → no reformatear archivos ajenos; documentar el gate preexistente y priorizar tests funcionales.

## Migration Plan

1. Aplicar `2921..=2980` desde el merge exacto de Ola 32.
2. Verificar soluciones, acumulación, Rust, backend, journeys y OpenSpec.
3. Publicar un único PR, esperar CI/Docker/E2E y hacer squash merge autorizado.
4. Verificar post-merge por SHA exacto; revertir el squash ante regresión.
