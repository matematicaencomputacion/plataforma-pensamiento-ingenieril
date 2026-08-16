## 1. Base

- [x] 1.1 Rama `feat/coding-compass-wave-c5` desde `origin/main` **post-C4**.
      Snapshot de filas `601..=750` = contrato C4

## 2. Índice applied 751..=900

- [x] 2.1 Auditar legado `751..=900` contra la tabla de `design.md`
- [x] 2.2 Completar tags applied hasta ≥ 90 en `751..=900` (sin densificar;
      sin 100% P3; bitmask/window/math no son P3 por defecto)
- [x] 2.3 Dejar untagged recetas: al menos 753, 853, 859

## 3. Freeze y tests

- [x] 3.1 Reemplazar freeze ≥ 751 por freeze ≥ 901 vs SHA C4
- [x] 3.2 Unit tests: piso ≥ 90; no 100% P3; snapshot C4 `601..=750`
      intacto; floors A/B/C1/C2 intactos; índice ordenado
- [x] 3.3 No editar `glossary.rs`, FAB, ni filas `≥ 901` / `601..=750`

## 4. Proof

- [x] 4.1 `make web-test` 100% verde
- [ ] 4.2 PR `feat/coding-compass-wave-c5` con TED slides
