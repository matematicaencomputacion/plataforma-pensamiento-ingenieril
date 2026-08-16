## Context

C5 asume C4 mergeado: freeze ≥ 751, DUA en peek/dock. El catálogo
`751..=900` es DSA replay en bandas de 6 (canonicalize / prefix-state /
bounded-window / lower-boundary / dependency-order / minimum-transition):
stacks 751–756, queues 757–762, listas 763–768, BST 769–774, árboles
775–780, DFS 781–786, BFS 787–792, toposort 793–798, UF 799–804,
shortest path 805–810, MST 811–816, trie 817–822, heaps 823–828,
intervalos 829–834, greedy 835–840, DP arrays 841–846, DP strings
847–852, backtracking 853–858, bitmask 859–864, number theory 865–870,
combinatorics 871–876, geometry 877–882, prefix 883–888, diff 889–894,
monotonic stack 895–900.

## Goals / Non-Goals

**Goals**

- Applied `751..=900`; piso ≥ 90 / 150; no 100% P3.
- Freeze mueve a `≥ 901` (ancla = SHA C4).
- Contrato C4 (`601..=750`) y C2/C1 intactos.
- Window / bits / math-recipe del tramo pueden quedar untagged.

**Non-Goals**

- Tags `901..=1000`. Nuevos diagramas DUA. Densificar.
- Tocar `glossary.rs` / FAB.

## Decisions

1. **Corte en 900** — misma magnitud que C1/C2/C4 (150 pasos).
2. **Política** — stacks/queues/listas/UF/trie/DP/grafos/ADT se taggean;
   bounded-window de cada banda, bitmask 859–864, number-theory /
   combinatorics / geometry: **no tag** (salvo 1–2 de representación).
3. **Representantes untagged de prueba:** 753, 853, 859.
   853 ya es contrato del test `applied_families_use_specific_lenses`.
4. **Gates:** `make web-test`. Sin harness completo.

## Risks / Trade-offs

| Riesgo | Mitigación |
|---|---|
| Pisar C4 | Test de snapshot `601..=750` y freeze ≥ 901 |
| Bitmask/window taggeados como P3 | Tabla + test 753/853/859 vacíos |
