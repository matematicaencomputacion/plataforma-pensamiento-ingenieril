## Context

Wave A (`62cb120`, #235) dejó en `main`:

- `web/src/concepts.rs`: struct `ConceptPartition { id: u8, … }` y
  `PARTITIONS` (ids numéricos `1..=5`, slugs `data-model` …). **No existe**
  `PartitionId`, `ConceptLens` ni `GlossaryEntry`.
- `STEP_PARTITIONS` sparse, `mastery_percent` + `%` en `PartitionNav`,
  floors P1≥40 / P2≥15 / P3≥35, `4..=100` sin huecos, applied puntual
  `101..=160` (no 100% P3).
- Hubs `/concepts/:id` y badges bajo el enunciado de `/learn/:step`.
- Layout de learn: grid teoría | `#learn-editor` (textarea) | consola.
  No hay FAB, ni glosario, ni atajo de búsqueda.

El índice ya tiene filas sueltas más allá de 160 (legado de
`expand-concept-partition-tags`): ~101/200 pasos en `101..=300` taggeados y
~132 filas en `301..=1000`. Wave B **audita y extiende solo `101..=300`** y
**congela `≥ 301`**.

Stack canónico de Coding: Leptos CSR + Pyodide (ADR 002 / 003). El texto Qwik
de ADR 001 sigue siendo deuda documental; no se reescribe aquí. El crate
`web` usa `styles.css`, no Tailwind runtime — `color_badge()` son clases CSS
con nombres “badge-ish”, no una dependencia Tailwind.

## Goals / Non-Goals

**Goals**

- Widget progresivo de 4 estados en `/learn/:step` autenticado; no un
  Spotlight a pantalla completa que robe el editor.
- `Ctrl+K` / `Cmd+K` abre el Estado 1 (búsqueda + chips). Esc desde el peek
  vuelve al ejercicio.
- Dock ~40% panel / 60% editor; `#learn-editor` no se desmonta.
- Glosario estático bloqueado en `glossary.rs`; búsqueda WASM in-memory
  &lt;5ms; hasta 5 pills de color por término.
- Seed ~12–20 entradas que cubren 4 intenciones de búsqueda (no enciclopedia).
- Tags applied en `101..=300` en el índice de particiones (no en el glosario).
- Cero endpoints Go nuevos. Cero ejecución de Python en servidor.

**Non-Goals**

- DUA / Libres para Aprender como producto (sí: anatomía de micro-card).
- Search box en el header del shell.
- Sandbox, boss fights, mapas 4–5 ricos, analytics Go.
- Atajos `Ctrl/Cmd+1..5`.
- Densificar `101..=300` como `4..=100`.
- Retaguear `301..=1000`.
- 1000 entradas de glosario.
- Campo `partition` dentro de cada `CodingStep`.
- `make harness` completo como gate de este change.

## Decisions

1. **Módulo directorio; no forkar ids de partición**
   - `web/src/concepts.rs` → `web/src/concepts/mod.rs` (contenido Wave A) +
     `web/src/concepts/glossary.rs`.
   - `ConceptPartition` / `PARTITIONS` / `partitions_for_micro_step` /
     `mastery_percent` se reexportan **sin cambiar firma `u8`**.
   - Wave A **no** define `PartitionId`. Se introduce **un solo** enum en
     `glossary.rs`. No hay segundo tipo de id.
   - Mapa canónico (no duplicar `PARTITIONS`):

     | `PartitionId` | `u8` | slug Wave A | hex | `label()` | `color_badge()` |
     |---|---|---|---|---|---|
     | `P1MemoryData` | 1 | `data-model` | `#3B82F6` | Azul Memoria | `badge-lens-p1` |
     | `P2ScopeControl` | 2 | `scope-legb` | `#8B5CF6` | Violeta Ámbitos | `badge-lens-p2` |
     | `P3Paradigms` | 3 | `paradigms` | `#F59E0B` | Ámbar Paradigmas | `badge-lens-p3` |
     | `P4Ecosystem` | 4 | `ecosystem` | `#10B981` | Verde Ecosistema | `badge-lens-p4` |
     | `P5Domains` | 5 | `application-domains` | `#EC4899` | Magenta Dominios | `badge-lens-p5` |

   - `as_u8()` / `from_u8()` para hubs `/concepts/{id}` y `STEP_PARTITIONS`.
   - **Descartado:** newtype en `PartitionNav`; renombrar `ConceptPartition`;
     Tailwind CDN / clases JIT que el crate no usa.

2. **Modelo de datos bloqueado (glosario ≠ índice de drills)**

   ```rust
   pub enum PartitionId {
       P1MemoryData,
       P2ScopeControl,
       P3Paradigms,
       P4Ecosystem,
       P5Domains,
   }
   impl PartitionId {
       pub fn label(self) -> &'static str;
       pub fn color_badge(self) -> &'static str; // CSS class → --lens-pN
       pub fn as_u8(self) -> u8;                 // 1..=5
       pub fn from_u8(id: u8) -> Option<Self>;
   }

   pub struct ConceptLens {
       pub partition: PartitionId,
       pub headline: &'static str,
       pub tldr: &'static str,
       pub code_example: &'static str,
       pub related_step_id: Option<&'static str>, // CodingStep.id, p.ej. "py-20-lists"
   }

   pub struct GlossaryEntry {
       pub id: &'static str,
       pub title: &'static str,
       pub keywords: &'static [&'static str],
       pub lenses: &'static [ConceptLens], // 1..=5, una por partición distinta
       pub common_pitfall: Option<&'static str>,
   }

   pub static GLOSSARY_ENTRIES: &[GlossaryEntry];

   pub fn search_glossary(
       query: &str,
       lens: Option<PartitionId>,
   ) -> Vec<&'static GlossaryEntry>;
   ```

   - `STEP_PARTITIONS` (tags 101..=300) **sigue** en `concepts/mod.rs`. El
     glosario es el corpus de búsqueda, no el índice de drills.
   - `related_step_id` es el `id` de `CodingStep` (`py-…`), no el
     `micro_step: i32`. Debe existir en el catálogo si es `Some`.
   - `lenses.len() ≤ 5` y sin `partition` duplicado en la misma entrada.
   - **Descartado:** fetch HTTP, DB, `localStorage`, Fuse.js, un `ConceptLens`
     enum de 5 variantes (el tipo bloqueado es **struct** por faceta).

3. **Término multifacetado**
   - El mismo término (p. ej. `python-lists`, o `def` / Funciones) puede
     tener **hasta 5 cards de color**, una por lente.
   - Clic en una pill/card abre **la explicación de esa partición**
     (`ConceptLens` de ese `partition`), no un dump genérico.
   - CTA: “Ver modelo mental en Partición N” → `/concepts/{as_u8}` y, si hay
     `related_step_id`, enlace opcional a `/learn/{id}`.
   - UI: renderizar hasta 5 pills desde `entry.lenses` usando
     `color_badge()` + `label()`. Color nunca es el único canal (texto).

4. **Seed, no enciclopedia — 4 intenciones de búsqueda**
   - Wave B entrega **~12–20** entradas estáticas, no 1000.
   - Deben cubrir las 4 intenciones (agrupar resultados en Estado 1). La
     intención **no** es un campo extra en el struct bloqueado: se deriva del
     prefijo de `id`:

     | Prefijo `id` | Intención | Ejemplos |
     |---|---|---|
     | `model-` | Modelos mentales / particiones | `model-legb`, `model-mutability`, `model-recursion` |
     | `syntax-` | Sintaxis y stdlib | `syntax-extend`, `syntax-yield`, `syntax-zip` |
     | `pattern-` | Patrones y algoritmos | `pattern-two-pointers`, `pattern-sliding-window`, `pattern-bfs` |
     | `trap-` | Trampas y errores | `trap-unboundlocal`, `trap-tuple-typeerror`, `trap-aliasing` |

   - Entradas canónicas (id libre, p. ej. `python-lists`) cuentan como
     `model-` si no llevan prefijo; `common_pitfall` cubre la trampa en la
     misma card.
   - **Seed obligatorio `python-lists`:**

     | Lente | Enfoque |
     |---|---|
     | P1 | mutabilidad / alias (`xs.append` vs rebind) |
     | P2 | mutar la lista dentro de una función (LEGB + objeto) |
     | P3 | comprensión vs bucle |
     | P4 | `collections.deque` (stdlib; mapa, no PyPI pesado) |
     | P5 | listas ↔ JSON |
     | keywords | `append`, `extend`, `slice`, `list`, `deque`, … |

   - `search_glossary("extend")` debe devolver `python-lists` (keyword).
   - **Descartado:** generar entradas desde títulos del rail.

5. **Búsqueda in-memory &lt;5ms**
   - Normalizar: trim + lowercase. Match: `title` + `keywords` (+ `id`).
   - Ranking: keyword/título exacto → prefix → contains.
   - Filtro `lens`: conservar entradas que tengan esa `ConceptLens`.
   - Query vacía + lente = primeras N de esa partición.
   - Cap UI Estado 1: lista agrupada por intención (no más de ~8 títulos).
     Estado 2: **una** micro-card (peek), no un grid de 8.
   - Presupuesto: n ≤ 32 en Wave B; 1000 queries sintéticas en `cargo test`
     nativo. Techo holgado 50ms en CI ruidoso + assert `GLOSSARY_ENTRIES.len() ≤ 32`.
     El producto promete &lt;5ms en hardware de alumno porque n es diminuto.
   - **Descartado:** debounce de red; no hay red.

6. **Estados del FAB (progresivos, no rutas nuevas)**

   El FAB **crece** a clics sucesivos. No es un modal Spotlight.

   | Estado | UI | Entra | Sale |
   |---|---|---|---|
   | 0 `Collapsed` | burbuja ~40px `#concept-fab` — `🔮 Lentes [1]…[5]` | default `/learn/:step` | clic → 1; `Ctrl/Cmd+K` → 1 |
   | 1 `Search` | input `#concept-glossary-search` + 5 chips `#concept-lens-1..5` + hits agrupados por intención | FAB o atajo | Esc → 0; clic en un hit → 2 |
   | 2 `MicroCard` | peek de **una** entrada: 3 bullets (TL;DR, diferencia+snippet ≤3 líneas, pitfall o costo), acento de la lente activa, CTA a Partición N | hit o chip+query unívoca | Esc → **ejercicio (0)**; “Anclar” → 3 |
   | 3 `Docked` | `#concept-drawer` ~40% / editor ~60% | Anclar | Desanclar o Esc → 0 |

   - Solo en `LearnPage` con `:step`. No en `/learn` sin id, `/concepts`, workspace.
   - Búsqueda en el **header**: fuera de Wave B (opcional).
   - Atajo solo en esa vista; `preventDefault`. Si ya está en 1–3, `Ctrl/Cmd+K`
     enfoca el input (no cierra).
   - No persistir modo en `localStorage`. Al cambiar de `:step` → Estado 0.

7. **Anatomía de la micro-card (capas DUA, no producto DUA)**

   Ejemplo pedagógico (`list.extend` vs `append`), mapeado al modelo:

   | Capa | Campo |
   |---|---|
   | TL;DR una línea | `lens.tldr` |
   | Diferencia clave + snippet ≤3 líneas | `lens.headline` + `lens.code_example` |
   | Trampa / costo cognitivo | `entry.common_pitfall` (si `Some`) |
   | CTA | “Ver modelo mental en Partición N” → hub `lens.partition.as_u8()` |
   | Esc / volver | Estado 0; el editor no se toca |

   Wave B **no** implementa dual-coding, audio, ni “Libres para Aprender”.
   “Costo en tiempo” del borrador no es un campo: si hace falta, va en `tldr`
   o `common_pitfall`.

8. **Split-view sin perder código**
   - `code: RwSignal<String>` en `LearnPage`. Docking no recrea la página ni
     `#learn-editor`.
   - Clase `.learn__grid--with-drawer`: ~40% drawer / ~60% editor en
     `min-width: 1100px`. El FAB deja de flotar.
   - Viewport estrecho: overlay sobre la **teoría**, nunca `display:none` del
     editor.
   - **Descartado:** iframe, ruta `/learn/:step/glossary`.

9. **Cromática**
   - Tokens `--lens-p1..p5` = hex de la tabla (decisión 1).
   - Chips en Estado 1; pills en micro-card/dock desde `lenses`.
   - `PartitionNav` del header **no** se recolorea en esta wave.

10. **Política de tags `101..=300` (applied, no denso)**

    Continuidad de Wave A: two-pointers / sliding-window / bits **no** son
    P3 “porque itera”. Untagged es válido. El glosario **sí** indexa esos
    patrones (`pattern-two-pointers`) aunque el drill quede sin tag.

    | Rango (aprox.) | Familia | Primary | Applied / no tag |
    |---|---|---|---|
    | 101–102, 161–163, 245–256 | linked lists / nodos | 1 | 3 si invert/merge es diseño |
    | 103–112, 167–169, 203–208, 239–244 | árboles / grafos | 3 si DFS/BFS/recursión es la lección | 1 si el costo es el nodo |
    | 113–114 | heapq / PQ | 4 | 1 si muta heap |
    | 115–117 | union-find / MST | 2 si parent/rank | 3 si greedy explícito |
    | 120–130, 143–153, 197–202 | DP / greedy | 3 si memo/tabulation es el objetivo | 1 si la tabla es el load |
    | 133–135, 172, 227–232 | backtracking / trie | 1+3 | 2 si generador/alcance |
    | 182, 184, 209, 212–214, 266–267, 272 | ADTs | 1 | 3 si invariante de diseño |
    | 131–132, 138, 175–180, 215–218, 293–297 | two pointers / window | — | **no tag** |
    | 136, 148, 224–226, 299–300 | bits | — | **no tag** salvo máscaras como modelo de datos (1) |
    | **301..=1000** | congelado | — | **no añadir, quitar ni editar filas** |

    Pisos: `101..=300` ≥ 120 taggeados; no 100% son `3`; freeze `≥ 301` vs
    `62cb120`; floors Wave A intactos.

11. **Gates de prueba**
    - `make web-test`: glosario (seed `python-lists`, 5 lentes, keywords
      append/extend/slice) + freeze 301+ + política 101–300 + compile.
    - Playwright `concepts.drawer.spec.ts`: login → `/learn/:step` →
      marcador en editor → Estado 0→1 (`Control+K`) → query → peek → Anclar
      40/60 → el marcador sigue. Mock Pyodide.
    - Conservar `journey.concepts.spec.ts` y `concepts.partitions.spec.ts`.
    - Sin `make harness` completo.

## Risks / Trade-offs

| Riesgo | Mitigación |
|---|---|
| Dos tipos de id de partición | Un `PartitionId` nuevo; `ConceptPartition.id` sigue `u8`; mapa 1:1 |
| Docking borra el editor | Un `RwSignal`; CSS 40/60; E2E con marcador |
| `Cmd+K` vs Chrome | `preventDefault` solo en `/learn/:step` |
| Sobre-etiquetar DSA 161–300 como P3 | Tabla + test; patterns viven en el glosario |
| Freeze 301+ | Comparar subconjunto `≥ 301`, no el archivo entero |
| Enciclopedia infinita | Cap 32 entradas; seed 12–20 |
| `color_badge` vs stack real | Clases CSS en `styles.css`, no Tailwind |

## Migration Plan

1. Rama `feat/coding-compass-wave-b` desde `origin/main` @ `62cb120`.
2. Split de módulo + `glossary.rs` (tipos bloqueados + seed) → tags
   `101..=300` + freeze → FAB 4 estados → smoke E2E.
3. Pre-CI: `make web-test`; Playwright del widget + journey Wave A.
4. PR propio (TED slides). No mezclar DUA producto, Go analytics, ni mapas 4–5.
5. Rollback: revert del PR.

## Open Questions

- ¿Persistir “anclado” entre micro-pasos? → **No** (Estado 0 al cambiar `:step`).
- ¿FAB en `/learn` sin `:step`? → **No**.
- ¿Search en el header? → **No** en Wave B.
- ¿Ctrl/Cmd+1..5? → **No**.
- ¿Campo `SearchIntent` en el struct? → **No**; prefijo de `id`.
- ¿Costo en tiempo como campo? → **No**; va en `tldr` / `common_pitfall`.
