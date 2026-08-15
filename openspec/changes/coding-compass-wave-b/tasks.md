## 1. Base y gobernanza

- [x] 1.1 Confirmar base = `origin/main` @ `62cb120` (Wave A #235:
      `ConceptPartition { id: u8 }`, `PartitionNav` con `%`, journey
      conceptual). No existe `PartitionId` en Wave A — no hay colisión de tipo
- [x] 1.2 Change `coding-compass-wave-b` (este proposal/design/specs/tasks)

## 2. Módulo glosario (modelo bloqueado, seed)

- [x] 2.1 Convertir `web/src/concepts.rs` → `web/src/concepts/mod.rs`
      reexportando `PARTITIONS`, `ConceptPartition`,
      `partitions_for_micro_step`, `mastery_percent` (firmas `u8` intactas)
- [x] 2.2 Añadir `web/src/concepts/glossary.rs` con el enum `PartitionId`
      (`P1MemoryData` … `P5Domains`), `label()`, `color_badge()` (clases CSS
      `badge-lens-pN`, no Tailwind), `as_u8()` / `from_u8()` mapeando 1..=5
- [x] 2.3 Tipos `ConceptLens` y `GlossaryEntry` + `GLOSSARY_ENTRIES` exactamente
      como en `design.md` (sin segundo tipo de id; sin campo `SearchIntent`)
- [x] 2.4 Seed **~12–20 entradas** (cap 32), cubriendo 4 intenciones por
      prefijo `model-` / `syntax-` / `pattern-` / `trap-`. Obligatorio:
      `python-lists` con **5 lentes** (P1 alias/mutabilidad, P2 mutate-in-fn,
      P3 comprensión, P4 deque, P5 JSON) y keywords `append`/`extend`/`slice`.
      No enciclopedia de 1000 términos
- [x] 2.5 `search_glossary(query, lens)` sobre `title` + `keywords`; ranking
      exact/prefix/contains; filtro por `ConceptLens.partition`
- [x] 2.6 Unit tests: `from_u8(0/6)` falla y `1↔P1MemoryData`; `python-lists`
      tiene 5 lentes distintas; `search_glossary("extend")` lo encuentra;
      cada intención tiene ≥1 entrada; `related_step_id` válido; `len() ≤ 32`

## 3. Índice applied 101..=300 (no el glosario)

- [x] 3.1 Auditar `STEP_PARTITIONS` en `101..=300` contra la tabla
      primary/applied de `design.md` (tags siguen en `concepts/mod.rs`)
- [x] 3.2 Completar tags applied hasta ≥ 120 pasos taggeados en
      `101..=300` (sin densificar; sin 100% P3; two-pointers/window/bits
      sin tag — el patrón vive en el glosario)
- [x] 3.3 Unit tests: piso ≥ 120; no 100% tag `3`; 131/132/175 vacíos;
      freeze de filas `micro_step ≥ 301` vs `62cb120`; `4..=100` denso;
      índice ordenado
- [x] 3.4 `make web-test` verde en el módulo `concepts`

## 4. FAB progresivo (estados 0–2) + cromática + atajo

- [x] 4.1 Componente Leptos montado solo en `LearnPage` con `:step`; Estado 0
      burbuja ~40px `#concept-fab` (`🔮 Lentes [1]…[5]`)
- [x] 4.2 Estado 1: input `#concept-glossary-search` + chips
      `#concept-lens-1..5` (`data-lens`, texto, `aria-pressed`, hex
      `--lens-p1..p5`) + hits agrupados por las 4 intenciones
- [x] 4.3 `Ctrl+K` / `Cmd+K` abre Estado 1 (`preventDefault` solo en
      `/learn/:step`); re-foco si ya está abierto; no search en el header
- [x] 4.4 Estado 2 peek: hasta 5 pills desde `lenses`; TL;DR + snippet ≤3
      líneas + `common_pitfall`; CTA “Ver modelo mental en Partición N”;
      Esc → Estado 0 (ejercicio)
- [x] 4.5 No recolorear `PartitionNav`

## 5. Split-view Estado 3 (dock)

- [x] 5.1 Anclar → Estado 3 `#concept-drawer` (~40% panel / ~60% editor en
      ≥ 1100px; estrecho = overlay sobre teoría)
- [x] 5.2 No desmontar `#learn-editor`; el signal `code` sobrevive dock y
      undock; nunca `display: none` del editor
- [x] 5.3 Desanclar / Esc desde Estado 3 → Estado 0; no persistir modo
      entre micro-pasos (default 0 al cambiar `:step`)

## 6. E2E + proof

- [x] 6.1 Playwright `web/e2e/tests/concepts.drawer.spec.ts`: login →
      `/learn/:step` → marcador en editor → Estado 0 → `Control+K` (Estado 1)
      → query `extend` → peek (Estado 2) → Anclar (Estado 3) → el marcador
      sigue; undock no lo borra (mock Pyodide)
- [x] 6.2 Conservar `journey.concepts.spec.ts` y
      `concepts.partitions.spec.ts`
- [x] 6.3 `make web-test` 100% verde
- [x] 6.4 Playwright del smoke drawer (+ journey Wave A si el entorno E2E
      está disponible). No `make harness` completo
- [x] 6.5 PR `feat/coding-compass-wave-b` con TED slides; no mezclar DUA
      producto, analytics Go, mapas 4–5, ni retags `301+`
