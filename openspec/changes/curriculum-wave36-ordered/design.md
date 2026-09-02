## Context

Ola 35 deja un catálogo exacto `1..=3100`, terminal 3100, freeze guard posterior a 3040 y validadores acumulativos 26–35. La implementación de Ola 36 debe preservar esos invariantes y modelar releases como transformaciones puras sobre datos, conforme a ADR 0002.

## Goals / Non-Goals

**Goals:**

- Incorporar 60 contratos pedagógicos originales en una cadena determinista y auditable.
- Enseñar compatibilidad, exposición gradual y reversión con estados explícitos, sin ejecutar despliegues reales.
- Mantener generación, integración y validación idempotentes y acumulativas.

**Non-Goals:**

- Cambiar workflows, desplegar servicios, migrar bases reales o incorporar SDKs de proveedores.
- Simular concurrencia, reloj de pared o tráfico aleatorio.
- Formatear globalmente la deuda Rust preexistente.

## Decisions

### Generador declarativo 10x6

`gen_wave36.py` contendrá 60 casos explícitos agrupados en diez familias de entrega segura. Reutilizará únicamente los helpers de emisión estables de Ola 35, no su contenido pedagógico. Se descarta copiar ramas históricas o producir variaciones cosméticas porque el validador probará originalidad y semántica ejecutable.

### Releases como datos explícitos

Versiones, cohortes, porcentajes, fases, migraciones y señales de verificación serán colecciones y enteros suministrados como entrada. Esto enseña canary, promoción y rollback sin red, reloj real, infraestructura ni mutaciones externas, y mantiene repetibilidad en Pyodide.

### Aplicador exacto e idempotente

`apply_wave36.py` exigirá anchors únicos del baseline 3100, enlazará 3100→3101, insertará constantes y referencias, actualizará techos a 3160 y retornará sin cambios al detectar el terminal 3160. La segunda ejecución será parte del contrato.

### Validador ejecutable y acumulativo

`validate_wave36.py` ejecutará cada solución contra su pytest canónico y verificará 10x6, slugs únicos, firmas originales, tokens prohibidos, catálogo exacto, conceptos y E2E. Los validadores 29–35 aceptarán el techo acumulativo 3160 sin debilitar sus prefijos.

### Freeze guard de la ola activa

El freeze guard conceptual se moverá de `>3040` a `>3100` y enumerará únicamente 3101–3160. Así detectará inserciones, remociones o cambios de tags posteriores al baseline de Ola 35.

## Risks / Trade-offs

- [Riesgo] Confundir modelado pedagógico con automatización real de deploy → mantener toda decisión como transformación pura y no tocar workflows ni backend.
- [Riesgo] Repetir sagas, checkpoints o incidentes de olas previas → limitar Ola 36 a contratos de cambio, compatibilidad, exposición y reversión; comparar firmas completas contra Olas 26–35.
- [Riesgo] Aplicación parcial sobre un baseline incorrecto → anchors exactos con cardinalidad uno y worktree basado en el merge archivado de Ola 35.
- [Riesgo] `rustfmt` global amplía miles de líneas fuera del alcance → documentar deuda y priorizar pruebas funcionales, contractuales y diff limpio.

## Migration Plan

1. Aplicar sobre el merge de archivado de Ola 35 `e0c1074de6707bdb97ad11a39601d03f6b987df0`.
2. Ejecutar validadores 26–36 y pruebas locales completas.
3. Integrar mediante PR único y verificar por SHA exacto.
4. Revertir el squash merge para restaurar terminal y techo 3100 si falla un gate post-merge.
