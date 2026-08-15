## 1. Bloqueo y base

- [ ] 1.1 **BLOCKED** hasta merge de `coding-compass-wave-c1` a `main`.
      No codear DUA en paralelo al índice C1
- [ ] 1.2 Rama `feat/coding-compass-wave-c3` desde `origin/main` **post-C1**.
      No retaguear `STEP_PARTITIONS` (incluido `601..=1000`)

## 2. Modelo y seed

- [ ] 2.1 Añadir `diagram_svg: Option<&'static str>` a `ConceptLens`
      (additive; `PartitionId` / `search_glossary` intactos)
- [ ] 2.2 Seed SVG (≤ ~2 KB, sin `<script>`) en `python-lists` P1,
      `model-legb`, `model-mutability`, `model-recursion`
- [ ] 2.3 Cap `GLOSSARY_ENTRIES.len() ≤ 32`; no enciclopedia nueva

## 3. Peek / dock

- [ ] 3.1 Render `#concept-diagram` en `ConceptMicroCard` (Estados 2 y 3)
      **antes** de TL;DR; `figure` + nombre accesible; color no es el
      único canal
- [ ] 3.2 Entradas sin diagrama: anatomía Wave B (sin `#concept-diagram`)
- [ ] 3.3 Dock no desmonta `#learn-editor`; mismo signal `code`

## 4. Tests y proof

- [ ] 4.1 Unit: 4 seeds con `Some`; SVG sin script; filas `≥ 601` unchanged
- [ ] 4.2 Extender `concepts.drawer.spec.ts`: peek diagrama → Anclar →
      marcador en editor
- [ ] 4.3 `make web-test` verde; PR TED propio; no mezclar tags C2
