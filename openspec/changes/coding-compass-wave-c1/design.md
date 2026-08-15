## Context

Wave B (`a81a026`, #236) dejó:

- `STEP_PARTITIONS` applied hasta 300 (piso ≥ 120; no 100% P3;
  two-pointers 131/132/175 vacíos).
- Freeze de filas `micro_step ≥ 301` vs Wave A `62cb120` (~132 filas
  legado, de las cuales ~47 caen en `301..=450`).
- Glosario + FAB 4 estados. C1 **no** los toca.

El catálogo `301..=450` es DSA replay: bits 301–304 / 377–382, árboles
305–310, intervalos 311–316, backtracking 317–322, two-pointers 329–334,
tries 335–340, prefix/stack/UF 341–358, binsearch/heaps/window 359–376,
grafos/DP/listas 383–400, sort/matrix/string/ADT/greedy 401–430, hash/math
431–450. Misma política Wave B: receta ≠ lente.

Stack: Leptos CSR + Pyodide (ADR 002 / 003). Cero Go de ejecución.

## Goals / Non-Goals

**Goals**

- Applied (no denso) `301..=450`; piso ≥ 90 tagged / 150.
- Freeze mueve a `≥ 451` (ancla `a81a026`).
- Untagged válido para bits / two-pointers / window del tramo.
- Floors Wave A y piso 101..=300 intactos.
- Cero endpoints Go. Cero Python en servidor.

**Non-Goals**

- Tags `451..=600` (C2) o `601..=1000`.
- Diagramas DUA (C3). Densificar como `4..=100`.
- Tocar `glossary.rs`, FAB, mastery UI, `completed_levels`.
- `make harness` completo como gate.

## Decisions

1. **Cortar en 450, no 600**
   - 150 pasos = misma magnitud que la mitad de Wave B (200). Un PR de
     índice cabe en review; C2 replica el patrón.
   - **Descartado:** mega Wave C 301..=600 + DUA en un change.

2. **Política primary/applied `301..=450` (continuidad Wave B)**

   | Rango (aprox.) | Familia | Primary | Applied / no tag |
   |---|---|---|---|
   | 301–304, 377–382 | bits | — | **no tag** salvo máscara como modelo de datos (1) |
   | 305–310, 383–388 | árboles / grafos | 3 si DFS/BFS/recursión es la lección | 1 si el costo es el nodo |
   | 311–316 | intervalos | 3 si greedy/invariante es el objetivo | — receta pura |
   | 317–322 | backtracking | 1+3 | 2 si generador/alcance |
   | 323–328, 437–442 | math / roman | — | 1 si el load es representación |
   | 329–334 | two pointers / in-place | — | **no tag** (patrón vive en glosario) |
   | 335–340 | trie | 1+3 | 2 si scope del nodo |
   | 341–346 | prefix / running sum | 1 | 3 si el diseño es el objetivo |
   | 347–352 | stack monotónico | 1 | 3 si invariante de diseño |
   | 353–358 | union-find | 2 (parent/rank) | 1 si el array parent es el load |
   | 359–364 | binary search | — | 3 solo si el eje es el predicado/diseño |
   | 365–370 | heap / PQ | 4 | 1 si muta heap |
   | 371–376 | sliding window | — | **no tag** |
   | 389–394 | DP | 3 si memo/tabulation es el objetivo | 1 si la tabla es el load |
   | 395–400 | linked lists | 1 | 3 si invert/merge es diseño |
   | 401–406 | sort | 1 | 3 si el comparador es paradigma |
   | 407–412 | matrices | 1 | — receta de índices |
   | 413–418 | strings | 1 | 3 si decode/stack es diseño |
   | 419–424 | ADTs (LRU, min-stack) | 1+3 | 2 si el estado encapsulado es LEGB |
   | 425–430 | greedy | 3 si la prueba de greedy es la lección | — receta |
   | 431–436 | hash maps | 1 | — |

   Prohibido: etiquetar window/bits/two-pointers como `3` “porque itera”.
   Filas legado en el freeze se **auditan**: se puede añadir, quitar o
   corregir tags **solo** si `301 <= n <= 450`.

3. **Piso 90, no denso**
   - 90/150 ≈ 60%, análogo al piso Wave B 120/200.
   - Untagged es válido. No exigir 150/150.

4. **Freeze ≥ 451 vs `a81a026`**
   - Copiar el subconjunto actual `≥ 451` de `WAVE_A_FROZEN_301` a
     `WAVE_B_FROZEN_451`.
   - Borrar `wave_b_freeze_rows_301_and_up`.
   - **Descartado:** freeze del archivo entero (C1 debe editar 301–450).

5. **Gates**
   - `make web-test` (unidad del índice). Sin `make harness` completo.
   - Conservar tests Wave A/B de floors y two-pointers 131/132/175.
   - Journey conceptual y drawer smoke **no** se reescriben.

6. **Secuencia**
   - Implementar C1 en `feat/coding-compass-wave-c1` **después** de
     mergear este PR de planificación.
   - C2/C3 blocked on C1 merge (el freeze y el módulo `concepts` no se
     pisan en paralelo).

## Risks / Trade-offs

| Riesgo | Mitigación |
|---|---|
| Sobre-etiquetar bits/window como P3 | Tabla + test 301/329/371 vacíos |
| Piso 90 inalcanzable sin forzar recetas | Auditar legado (~47) + UF/trie/ADT/grafos/DP |
| C2 arranca sobre freeze viejo | tasks C2: blocker explícito |
| Tocar glossary “de paso” | Fuera de alcance; review del diff |

## Migration Plan

1. Rama de implementación `feat/coding-compass-wave-c1` desde `main`
   post-merge de este PR docs.
2. Índice 301..=450 + freeze ≥ 451 + tests.
3. Pre-CI: `make web-test`.
4. PR propio TED. No mezclar C2/C3.
5. Rollback: revert del PR de implementación.

## Open Questions

- ¿Ajustar tags legado incorrectos (p. ej. two-pointer 333 taggeado 1)?
  → **Sí**, si el load es receta; el freeze ya no cubre `301..=450`.
