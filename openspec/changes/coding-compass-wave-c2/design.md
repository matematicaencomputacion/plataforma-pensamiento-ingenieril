## Context

C2 asume `coding-compass-wave-c1` **mergeado**: freeze ≥ 451, piso ≥ 90
en `301..=450`, tests C1 verdes. Hasta entonces este change es solo
planificación.

El catálogo `451..=600` es DSA replay / easy: arrays 451–462, árboles/grafos
463–474, DP 475–480, heaps 481–486, listas 487–492, sort/interval 493–498,
matrices 499–504, strings/hash 505–510, greedy 511–516, two-pointers
517–522, stack 523–528, BFS 529–534, binsearch 535–540, window 541–546,
bits 547–552, backtracking 553–558, math 559–564, strings 565–570, easy
arrays 571–576, ADTs 577–582, árboles 583–588, grafos 589–594, capstone
595–600.

Stack: Leptos CSR + Pyodide (ADR 002). Cero Go de ejecución.

## Goals / Non-Goals

**Goals**

- Applied `451..=600`; piso ≥ 90 / 150; no 100% P3.
- Freeze mueve a `≥ 601` (ancla = SHA de C1).
- Contrato C1 (`301..=450`) bit-a-bit intacto.
- Cero Python en servidor.

**Non-Goals**

- Implementar antes del merge C1.
- Tags `601..=1000`. Diagramas DUA (C3). Densificar.
- Tocar glosario/FAB.

## Decisions

1. **Misma política que C1/Wave B; corte en 600**
   - Cierra el pedido de producto “tags 301..=600” en **dos** PRs de
     índice. 600 es el capstone two-sum; 601+ queda para una wave
     posterior, no C3.

2. **Política primary/applied `451..=600`**

   | Rango (aprox.) | Familia | Primary | Applied / no tag |
   |---|---|---|---|
   | 451–462, 571–576 | arrays easy / receta | — | 1 solo si mutabilidad/alias es el load |
   | 463–474, 583–594 | árboles / grafos | 3 si DFS/BFS/recursión es la lección | 1 si el nodo es el costo |
   | 475–480 | DP | 3 si memo/tabulation | 1 si la tabla es el load |
   | 481–486 | heap | 4 | 1 si muta heap |
   | 487–492 | linked lists | 1 | 3 si invert/merge es diseño |
   | 493–498 | sort / intervals | 1 | 3 si greedy/invariante |
   | 499–504 | matrices | 1 | — receta de índices |
   | 505–510, 565–570 | strings / hash | 1 | — |
   | 511–516 | greedy | 3 si la prueba de greedy es la lección | — receta |
   | 517–522 | two pointers | — | **no tag** |
   | 523–528 | stack | 1 | 3 si invariante |
   | 529–534 | BFS | 3 | 1 si el nodo/cola es el load |
   | 535–540 | binary search | — | 3 solo si el predicado es el eje |
   | 541–546 | sliding window | — | **no tag** |
   | 547–552 | bits | — | **no tag** |
   | 553–558 | backtracking | 1+3 | 2 si generador |
   | 559–564 | math | — | 1 si representación |
   | 577–582 | ADTs (LRU, stacks) | 1+3 | 2 si estado encapsulado |
   | 595–600 | capstone / majority | 1 | 3 si el diseño es el eje |

   Representantes untagged de prueba: **518** (3Sum), **543** (min window),
   **547** (single number / bits).

3. **Freeze ≥ 601 vs SHA C1**
   - No anclar a `a81a026` (C1 ya cambió 301–450).
   - Test extra: filas `301..=450` == snapshot C1.

4. **Gates:** `make web-test`. Sin harness completo. Sin E2E nuevo.

## Risks / Trade-offs

| Riesgo | Mitigación |
|---|---|
| Implementar sobre freeze ≥ 301 | Blocker en tasks 1.1; rebase a main post-C1 |
| Easy arrays 451–462 taggeados en masa como P1 | Tabla: receta → untagged |
| Pisar C1 | Test de snapshot `301..=450` |

## Migration Plan

1. Esperar merge C1. Rama `feat/coding-compass-wave-c2` desde ese `main`.
2. Índice 451..=600 + freeze ≥ 601 + tests.
3. `make web-test`. PR TED propio. No mezclar C3.
4. Rollback: revert del PR de implementación.

## Open Questions

- Ninguna que bloquee el spec: el freeze SHA se rellena al implementar.
