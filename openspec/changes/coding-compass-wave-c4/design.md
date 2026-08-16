## Context

C4 asume C2+C3 mergeados: freeze ≥ 601, DUA en peek/dock. El catálogo
`601..=750` es DSA replay: prefix 601–612, window/deque 613–618, UF
619–624, tries 625–630, intervalos 631–636, greedy 637–642, DP
643–654, grafos 655–666, árboles 667–678, listas 679–684, heaps
685–690, strings 691–696, Fenwick/segtree 697–708, binary lift
709–714, number theory 715–720, bands VI 721–750.

## Goals / Non-Goals

**Goals**

- Applied `601..=750`; piso ≥ 90 / 150; no 100% P3.
- Freeze mueve a `≥ 751` (ancla = SHA C3).
- Contrato C2 (`451..=600`) y C1 (`301..=450`) intactos.
- Window / two-pointers / bits-style del tramo pueden quedar untagged.

**Non-Goals**

- Tags `751..=1000`. Nuevos diagramas DUA. Densificar.
- Tocar `glossary.rs` / FAB.

## Decisions

1. **Corte en 750** — misma magnitud que C1/C2 (150 pasos).
2. **Política** — prefix/UF/trie/DP/grafos/ADT se taggean; window
   613–618 y 733–738, two-pointers 739–744, xor 630: **no tag**.
3. **Representantes untagged de prueba:** 613, 630, 739.
4. **Gates:** `make web-test`. Sin harness completo.

## Risks / Trade-offs

| Riesgo | Mitigación |
|---|---|
| Pisar C2/C3 | Tests de snapshot `451..=600` y freeze ≥ 751 |
| Window/VI taggeados como P3 | Tabla + test 613/739 vacíos |
