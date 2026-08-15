## 1. Bloqueo y base

- [x] 1.1 **BLOCKED** hasta merge de `coding-compass-wave-c1` a `main`.
      No codear C2 sobre `a81a026` / freeze ≥ 301
- [x] 1.2 Rama `feat/coding-compass-wave-c2` desde `origin/main` **post-C1**.
      Snapshot de filas `301..=450` = contrato C1

## 2. Índice applied 451..=600

- [x] 2.1 Auditar legado `451..=600` contra la tabla de `design.md`
- [x] 2.2 Completar tags applied hasta ≥ 90 en `451..=600` (sin densificar;
      sin 100% P3; easy-arrays 451–462 no son P1 por defecto)
- [x] 2.3 Dejar untagged recetas: al menos 518, 543, 547

## 3. Freeze y tests

- [x] 3.1 Reemplazar freeze ≥ 451 por freeze ≥ 601 vs SHA C1
- [x] 3.2 Unit tests: piso ≥ 90; no 100% P3; snapshot C1 `301..=450`
      intacto; floors A/B intactos; índice ordenado
- [x] 3.3 No editar `glossary.rs`, FAB, ni filas `≥ 601` / `301..=450`

## 4. Proof

- [x] 4.1 `make web-test` 100% verde
- [x] 4.2 PR `feat/coding-compass-wave-c2` con TED slides; no mezclar C3
