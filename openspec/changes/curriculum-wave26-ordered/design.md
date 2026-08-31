## Context

`main` expone un catálogo continuo `1..=2500`. Existe una rama histórica de Ola 26, pero fue creada dentro de una cadena acumulativa y no actualiza todas las superficies del diseño actual. Se utilizará solamente como referencia de contenido; la integración se reconstruye sobre el `main` vigente y se valida como incremento independiente.

## Goals / Non-Goals

### Goals

- Publicar exactamente `PY2501_STEP..=PY2560_STEP` sobre el catálogo estable.
- Mantener navegación, registro Rust, particiones conceptuales y conteos E2E derivados del mismo techo 2560.
- Dejar generación y validación deterministas para poder reproducir y auditar la ola.
- Obtener evidencia local y de CI antes de considerar habilitada Ola 27.

### Non-Goals

- No incorporar contenidos ni constantes de Ola 27 en adelante.
- No mezclar arreglos de infraestructura, deploy, backend u otras capacidades.
- No mergear ramas históricas completas ni preservar su topología acumulativa.

## Decisions

### Reconstruir sobre `main` y no fusionar la rama histórica

Se audita el contenido histórico para evitar reinventar ejercicios ya definidos, pero se aplica sobre la arquitectura vigente. Esto evita arrastrar commits de olas posteriores, contadores obsoletos o supuestos de CI antiguos.

Alternativa descartada: mergear la rama histórica. Su ancestry acumulativo impide demostrar que el PR contiene solamente Ola 26.

### Mantener scripts separados de generación, aplicación y validación

El generador expresa los 60 ejercicios; el aplicador usa anclas estructurales exactas; el validador comprueba rango, enlaces, registro, particiones y conteos. Separarlos permite auditar contenido sin modificar archivos y detectar drift después de la aplicación.

Alternativa descartada: editar manualmente el bloque Rust. El volumen aumenta el riesgo de omisiones y referencias inconsistentes.

### Transferir la autoridad del techo al validador más reciente

El validador de Ola 25 seguirá comprobando su rango y frontera, pero no tratará 2500 como techo global una vez integrada Ola 26. El validador de Ola 26 será la autoridad para el techo 2560. Así los validadores históricos siguen siendo composables.

### Extender explícitamente las particiones conceptuales

La rama histórica no modifica `web/src/concepts/mod.rs`; esta reconstrucción sí incorporará los límites de Ola 26 de acuerdo con las familias generadas. El validador comprobará orden, existencia y límite final 2560.

## Risks / Trade-offs

- **Bloque Rust generado extenso:** se mitiga con un generador determinista y comprobación de las 60 constantes.
- **Anclas desactualizadas:** el aplicador falla de forma explícita si la frontera 2500 no coincide exactamente.
- **Drift entre Rust y E2E:** un mismo validador verifica tamaño, techo y los tres conteos canónicos.
- **Falso verde por generación aislada:** además del validador se ejecutan formato, compilación/tests Rust y harness journeys.
- **Particiones semánticas incompletas:** los límites se derivan de las familias de ejercicios y se verifican contra identificadores existentes.

## Migration Plan

1. Crear los artefactos OpenSpec y la rama exclusiva de Ola 26.
2. Incorporar scripts reproducibles y aplicar `2501..=2560` sobre la frontera 2500.
3. Actualizar tests Rust, particiones y tres journeys E2E.
4. Ejecutar validadores de Ola 25/26, formato, suite local y harness.
5. Abrir un único PR, monitorear todos los checks y corregir en la misma rama.
6. Tras autorización de merge, verificar SHA, workflows post-merge y catálogo en `main`; recién entonces habilitar Ola 27.

El rollback consiste en revertir el squash de Ola 26, restaurando la frontera y los conteos a 2500.
