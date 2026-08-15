## Why

Wave A (#235) hizo verdadero el compás: índice denso `4..=100`, `%` visible y
journey `[2]` → hub → drill → ✔. En `/learn/:step` el alumno sigue sin un
glosario a mano: o se va al hub y pierde el editor, o ignora la lente. El
índice applied se detuvo en `101..=160`; el tramo `161..=300` queda a medias
(~101/200 taggeados) y el drawer quedó explícitamente fuera de Wave A.

## What Changes

Cerrar la **lente en el workspace de coding** (Wave B), sin analytics Go ni
producto DUA completo:

1. **Widget progresivo (4 estados)** en `/learn/:step` — no un Spotlight de
   página completa. El FAB crece a clics sucesivos:
   - Estado 0: burbuja ~40px (`🔮 Lentes [1]…[5]`)
   - Estado 1: búsqueda rápida + 5 chips cromáticos (`Ctrl+K` / `Cmd+K`)
   - Estado 2: micro-card peek (anatomía DUA: TL;DR, snippet, CTA a la
     partición; Esc vuelve al ejercicio)
   - Estado 3: split-view ~40% panel / 60% editor **sin perder código**
2. **Glosario cliente** (`web/src/concepts/glossary.rs`), modelo bloqueado:
   `PartitionId`, `ConceptLens`, `GlossaryEntry`, `GLOSSARY_ENTRIES`. Un
   término puede tener hasta 5 lentes de color. Búsqueda in-memory &lt;5ms
   agrupada en 4 intenciones (modelos, sintaxis, patrones, trampas). Seed,
   no enciclopedia.
3. **Tags applied `101..=300`** en `STEP_PARTITIONS` (continuidad de Wave A:
   no densificar como `4..=100`; no 100% P3). **No retaguear `301..=1000`.**

## Capabilities

### New Capabilities

- `coding-concept-drawer`: FAB progresivo de 4 estados, atajo de búsqueda,
  lentes cromáticas, micro-card multifacetado y split-view sobre
  `/learn/:step`, alimentado por el glosario cliente.

### Modified Capabilities

- `coding-conceptual-partitions`: cobertura applied del índice hasta el
  micro-paso 300; el módulo `concepts` se parte en directorio para hospedar
  `glossary.rs` sin tocar el contrato `u8` de `ConceptPartition` /
  `PartitionNav` / hubs.

## Alcance incluido

- Convertir `web/src/concepts.rs` → `web/src/concepts/mod.rs` +
  `glossary.rs` (reexportar APIs existentes; **un solo** `PartitionId` nuevo
  en el glosario, mapeado a `ConceptPartition.id` 1..=5).
- Seed `GLOSSARY_ENTRIES` (~12–20 entradas, 4 intenciones; incluye
  `python-lists` con 5 lentes).
- Componente Leptos del FAB montado solo en `LearnPage` con `:step`.
- Tokens CSS canónicos (`#3B82F6` … `#EC4899`); `color_badge()` son clases
  CSS del shell, no dependencia Tailwind.
- Extender `STEP_PARTITIONS` en `101..=300`; freeze de filas `≥ 301`.
- Unit tests del glosario (ranking, lentes múltiples, presupuesto).
- E2E smoke del widget (abrir, buscar, peek, anclar, el editor conserva
  texto). Conservar `journey.concepts.spec.ts` de Wave A.

## Fuera de alcance

- Tratamiento DUA / Libres para Aprender **completo** (ola posterior). Wave B
  solo usa la anatomía de micro-card (TL;DR + snippet + CTA).
- Búsqueda en el header del shell (opcional / más tarde). Superficie primaria
  = FAB en `/learn/:step`.
- Endpoints Go de analytics, glosario o tiempo.
- Sandbox / boss fights.
- Mapas ricos de particiones 4–5 (siguen `map_only`, ADR 002).
- Atajos `Ctrl/Cmd+1..5`.
- Densificar `4..=100` de nuevo, o retaguear `301..=1000`.
- Enciclopedia de 1000 entradas de glosario.
- Discriminador `Language::Go`.
- Cambiar `current_level` / `completed_levels`.
- `make harness` completo / timeouts de 12h en CI.
- Implementar código de producto en este change de planificación.

## Impact

- `web/src/concepts/` (split + glosario), `LearnPage`, CSS del workspace,
  `web/e2e/tests/`.
- Base = `origin/main` @ `62cb120` (Wave A squash-merge #235).
- Sin cambios de API Go ni de ejecución de alumnos (ADR 002).

## Plan de rollback

Revertir el PR de Wave B. El compás Wave A, los hubs `/concepts/:id` y el
editor de `/learn/:step` siguen usables. El glosario y el FAB no tienen
persistencia: no hay migración de datos.
