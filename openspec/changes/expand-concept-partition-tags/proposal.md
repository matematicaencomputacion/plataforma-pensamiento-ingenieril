## Why

El compás conceptual ya existe, pero su índice cubre principalmente los primeros
100 micro-pasos. Los hubs 1–3 todavía no representan el currículo de 1000 pasos,
por lo que dominio y repaso conceptual quedan sesgados hacia Foundations.

## What Changes

- Ampliar el índice multi-tag de `concepts.rs` por familias curriculares reales.
- Clasificar cada familia solo cuando ejercita de forma material Modelo de Datos,
  LEGB o Paradigmas; evitar marcar todo DSA como “Paradigmas”.
- Mantener el índice compacto mediante rangos declarativos y excepciones.
- Agregar pruebas de cobertura, orden, existencia y estabilidad del mapeo.

### Alcance incluido

- Auditoría de Foundations, colecciones, funciones, clases y familias DSA.
- Tags de particiones 1–3 para micro-pasos 1–1000.
- Helpers internos para expandir rangos sin duplicar 1000 filas manuales.
- Tests unitarios y actualización del OpenSpec.

### Fuera de alcance

- Cambios visuales al compás o al hub.
- Analytics en Go, almacenamiento nuevo o cambios de progreso.
- Drawer, diagnóstico de errores, atajos o soporte multilenguaje.
- Nuevos ejercicios o “boss fights”.

## Capabilities

### Modified Capabilities

- `coding-conceptual-partitions`: el índice conceptual cubre familias del
  currículo completo con reglas verificables.

## Rollback

Revertir el commit restaura el índice explícito anterior. No hay migración de
datos porque los tags se derivan en el cliente y no alteran progreso.
