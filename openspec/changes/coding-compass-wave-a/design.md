## Context

`coding-conceptual-partitions` (PR #232) dejó:

- 5 particiones estables en `web/src/concepts.rs` (`map_only` en 4–5).
- Índice externo `STEP_PARTITIONS: &[(i32, &[u8])]` (hoy ~4..=99 con huecos;
  LEGB tiene pocos drills).
- `partition_mastery` / `mastery_percent` usados en `aria-label` del
  `PartitionNav`, no en UI.
- Smoke E2E: login → `#partition-nav-1` → `/concepts/1` → drill 20 → learn.
  No valida `[2]`, ni Validar, ni que el dominio cambie.

El rail y `completed_levels` no se tocan. Wave A solo hace verdadero el
compás.

Stack canónico de Coding: Leptos CSR + Pyodide (ADR 002 / 003). El texto
Qwik de ADR 001 sigue siendo deuda documental; no se reescribe aquí.

## Goals / Non-Goals

**Goals**

- Todo micro-paso de catálogo en `4..=100` tiene ≥1 tag válido.
- Pisos de drills: P1 ≥ 40, P2 ≥ 15, P3 ≥ 35 (P2 es naturalmente más flaco).
- Tags *applied* puntuales en `101..=160` solo si el costo cognitivo es
  mutabilidad, LEGB o paradigma.
- Cada botón `[n]` muestra dominio (aro o `%`) y `data-mastery="{0-100}"`.
- Journey E2E cierra: sesión → learn → `[2]` → hub 2 → drill → Validar
  (mock) → el drill/compás refleja completado.
- Cero endpoints Go nuevos. Cero ejecución de Python en servidor.

**Non-Goals**

- Drawer, atajos, DUA, sandbox, boss fights, mapas 4–5 ricos.
- Taggear `161..=1000`.
- Telemetría de reintentos/tiempo.
- Campo `partition` dentro de cada `CodingStep`.

## Decisions

1. **Índice externo, no 1000 edits**
   - Seguir `STEP_PARTITIONS` ordenado por `micro_step` (binary search).
   - Multi-label: primary = id numérico menor (ya implementado);
     applied = tags extra.
   - **Descartado:** `partition: u8` en `CodingStep` (ruido, 1000 diffs).

2. **Reglas de etiquetado (primary vs applied)**

   | Rango | Familia | Primary | Applied permitido |
   |---|---|---|---|
   | 7–15 | tipos / strings | 1 | — |
   | 16–45 | listas / tuplas / sets / dicts | 1 | 3 si el reto es bucle/comprensión |
   | 46–51 | control | 3 | 1 si muta colección |
   | 52–57 | funciones | 2 | 3 (paradigma) |
   | 58–63 | OOP | 3 | 2 si el eje es `self`/namespace |
   | 64–69 | stdlib | 4 (ya mapa) | 1 si el drill es mutabilidad |
   | 70–81 | files / howto-OOP | 3 o 5 | 2 si hay scope |
   | 82–100 | DSA / algorithms / finale | 3 *solo* si el enunciado enseña paradigma; si no, 1 applied o sin forzar 3 |
   | 101–160 | DSA+ | applied 1/2/3 **solo** si el mental load es la lente, no el algoritmo |

   Prohibido: etiquetar un sliding-window o un grafo como `3` “porque itera”.

3. **Mastery visible = proyección, no progreso nuevo**
   - `pct = mastery_percent(id, completed_levels)`.
   - UI: número `N%` compacto bajo el dígito **o** aro SVG de 1 trazo
     (`stroke-dashoffset`). Preferir **dígito `%`** si el aro satura el
     header; el contrato es `data-mastery` + texto accesible.
   - `0%` se muestra (deuda visible). No inventar “dominado” con umbrales
     mágicos en v1.
   - **Descartado:** endpoint Go `/api/concepts/mastery` — es derivable.

4. **Journey conceptual (tercera jornada nombrada)**
   - Archivo: `web/e2e/tests/journey.concepts.spec.ts`.
   - Pasos:
     1. Register + login → `/workspace`.
     2. Ir a `/learn/<step-tagged-2>` (o workspace → link del current).
     3. Clic `#partition-nav-2` → `/concepts/2`.
     4. Clic un drill de P2 → `/learn/:id`.
     5. Pegar `solution_example`, Validar (mock Pyodide; el mock actual
        solo aprueba `nombre`/`edad` — **este change MUST ajustar el mock
        para que `check` pase si el código no está vacío**, o usar un
        step cuyo solution dispare el heurístico. Decisión: **mock
        `check` pasa cuando hay `def` + `print`**, suficiente para
        journeys de navegación/estado; no sustituye Pyodide real).
     6. Assert: `#learn-progress-check` visible; al volver a `/concepts/2`
        el drill tiene estado done; `#partition-nav-2` `data-mastery` > 0.
   - ADR 003: documentar la jornada en `docs/testing/` (un mermaid corto)
     y engancharla a `make harness-journeys` si el target ya lista
     `journey.*.spec.ts`.
   - El smoke `#232` (`concepts.partitions.spec.ts`) se conserva.

5. **Gates de prueba**
   - `make web-test`: cobertura 4..=100, pisos P1/P2/P3, índice ordenado.
   - Journey conceptual en Playwright (mock, no `PPI_E2E_REAL_PYODIDE`).
   - Sin `make harness` completo como gate de este change (timeout de
     1000 micro-steps). Sí: `make web-test` + el spec de journey.

## Risks / Trade-offs

| Riesgo | Mitigación |
|---|---|
| Sobre-etiquetar DSA como “imperativo” | Tabla primary/applied; review del índice; test que 101..=160 no esté 100% taggeado como 3 |
| Mock `check` laxo aprueba basura | Solo en E2E mock; producción usa Pyodide real |
| Header saturado con 5 porcentajes | Tipografía chica; `%` no label larga; `title` ya tiene el nombre |
| #232 aún no mergeado | Base = `origin/main` post-#232 o cherry-pick del módulo `concepts` |
| Journey frágil por unlock 1..N | Elegir un drill P2 de foundations (p. ej. 52–55 / 62) y `unlockThroughMicroStep` hasta N-1 |

## Migration Plan

1. Rama `feat/coding-compass-wave-a` desde la base que ya tenga #232.
2. Índice + tests → UI mastery → mock check + journey.
3. Pre-CI: `make web-test`; Playwright del journey (y smoke de partitions).
4. PR propio (TED slides). No mezclar drawer ni DUA.
5. Rollback: revert del PR.

## Open Questions

- ¿Aro SVG o `%` texto? → **Default: `%` texto** (`data-mastery`). Aro
  solo si cabe en CSS sin romper el header en 320px.
- ¿El journey vive en `harness-journeys`? → **Sí**, si el glob ya incluye
  `journey.*.spec.ts`; si no, añadirlo. No crear un tercer Makefile target.
