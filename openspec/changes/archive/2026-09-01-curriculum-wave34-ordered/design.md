## Context

Ola 33 deja un catálogo exacto `1..=2980`, terminal 2980, freeze guard más allá de 2920 y validadores acumulativos 26–33. La implementación de Ola 34 debe preservar esos invariantes y la ejecución Python client-side definida por ADR 0002.

## Goals / Non-Goals

**Goals:**

- Incorporar 60 contratos pedagógicos originales en una cadena determinista y auditable.
- Hacer que generación, integración y validación sean idempotentes y acumulativas.
- Probar soluciones reales, no solo presencia textual del catálogo.

**Non-Goals:**

- Simular concurrencia, tiempo real o infraestructura distribuida.
- Cambiar backend, APIs o dependencias.
- Formatear globalmente la deuda Rust preexistente.

## Decisions

### Generador declarativo 10x6

`gen_wave34.py` contendrá 60 casos explícitos y reutilizará únicamente helpers de emisión de Ola 33. Cada familia modelará una garantía mediante estructuras ordenadas y cerrará con un ejercicio integrador. Se descarta copiar ramas históricas o hacer sustituciones numéricas masivas porque ambas estrategias contaminan el rail.

### Aplicador exacto e idempotente

`apply_wave34.py` exigirá anchors únicos del baseline 2980, enlazará 2980→2981, insertará constantes y referencias, actualizará techos a 3040 y retornará sin cambios si detecta el terminal 3040. La segunda ejecución será parte del contrato.

### Validador ejecutable y acumulativo

`validate_wave34.py` ejecutará cada solución contra su pytest canónico en un directorio temporal, verificará 10x6, slugs únicos, firmas originales, tokens prohibidos, catálogo exacto, conceptos y E2E. Los validadores 29–33 aceptarán el nuevo techo acumulativo 3040 sin debilitar sus prefijos.

### Freeze guard de la ola activa

El freeze guard conceptual se moverá de `>2920` a `>2980` y enumerará únicamente 2981–3040. Esto detecta inserciones, remociones o cambios de tags posteriores al baseline de Ola 33.

## Risks / Trade-offs

- [Riesgo] Soluciones sintácticamente válidas pero pedagógicamente repetidas → comparar firmas completas contra Olas 26–33 y exigir diez familias distintas.
- [Riesgo] Aplicación parcial sobre un baseline incorrecto → anchors exactos con cardinalidad uno y worktree basado en `origin/main` verificado.
- [Riesgo] `rustfmt` global amplía miles de líneas fuera del alcance → documentar deuda y priorizar `cargo test`, validadores y diff limpio.
- [Riesgo] Conteos E2E desalineados → verificar exactamente una expectativa 3040 por cada journey canónico.

## Migration Plan

1. Aplicar sobre el merge de archivado de Ola 33.
2. Ejecutar validadores 26–34 y pruebas locales completas.
3. Integrar mediante PR único y verificar por SHA exacto.
4. Revertir el squash merge para restaurar terminal y techo 2980 si falla un gate post-merge.
