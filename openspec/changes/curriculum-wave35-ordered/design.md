## Context

Ola 34 deja un catálogo exacto `1..=3040`, terminal 3040, freeze guard posterior a 2980 y validadores acumulativos 26–34. La implementación de Ola 35 debe preservar esos invariantes y modelar operación como transformaciones puras sobre datos, conforme a ADR 0002.

## Goals / Non-Goals

**Goals:**

- Incorporar 60 contratos pedagógicos originales en una cadena determinista y auditable.
- Enseñar decisiones operativas con señales y presupuestos explícitos, sin depender de servicios reales.
- Mantener generación, integración y validación idempotentes y acumulativas.

**Non-Goals:**

- Instrumentar la aplicación, emitir alertas reales o cambiar infraestructura.
- Simular concurrencia, tiempo de pared o carga aleatoria.
- Formatear globalmente la deuda Rust preexistente.

## Decisions

### Generador declarativo 10x6

`gen_wave35.py` contendrá 60 casos explícitos agrupados en diez familias operativas. Reutilizará solo los helpers de emisión estables de Ola 34, no su contenido pedagógico. Se descarta copiar ramas históricas o generar variaciones cosméticas porque el validador debe probar originalidad y semántica ejecutable.

### Tiempo y carga como datos explícitos

Ventanas, latencias, tasas y capacidad se representarán mediante enteros y secuencias suministradas como entrada. Esto permite enseñar SLO, backpressure y respuesta a incidentes sin `time`, sleeps, threads ni infraestructura, preservando repetibilidad en Pyodide.

### Aplicador exacto e idempotente

`apply_wave35.py` exigirá anchors únicos del baseline 3040, enlazará 3040→3041, insertará constantes y referencias, actualizará techos a 3100 y retornará sin cambios al detectar el terminal 3100. La segunda ejecución será parte del contrato.

### Validador ejecutable y acumulativo

`validate_wave35.py` ejecutará cada solución contra su pytest canónico en un directorio temporal y verificará 10x6, slugs únicos, firmas originales, tokens prohibidos, catálogo exacto, conceptos y E2E. Los validadores 29–34 aceptarán el techo acumulativo 3100 sin debilitar sus prefijos.

### Freeze guard de la ola activa

El freeze guard conceptual se moverá de `>2980` a `>3040` y enumerará únicamente 3041–3100. Así detectará inserciones, remociones o cambios de tags posteriores al baseline de Ola 34.

## Risks / Trade-offs

- [Riesgo] Repetir circuit breakers o reintentos de Ola 31 bajo nuevos nombres → limitar Ola 35 a señales, objetivos, capacidad y políticas operativas; comparar firmas completas contra Olas 26–34.
- [Riesgo] Umbrales que dependan de floats ambiguos → preferir razones enteras, conteos y redondeo explícito en contratos y tests.
- [Riesgo] Aplicación parcial sobre un baseline incorrecto → anchors exactos con cardinalidad uno y worktree basado en el merge archivado de Ola 34.
- [Riesgo] `rustfmt` global amplía miles de líneas fuera del alcance → documentar deuda y priorizar pruebas funcionales, contractuales y diff limpio.

## Migration Plan

1. Aplicar sobre el merge de archivado de Ola 34 `74be51948c29399ebc7d0c173ebed3f2dbf89f19`.
2. Ejecutar validadores 26–35 y pruebas locales completas.
3. Integrar mediante PR único y verificar por SHA exacto.
4. Revertir el squash merge para restaurar terminal y techo 3040 si falla un gate post-merge.
