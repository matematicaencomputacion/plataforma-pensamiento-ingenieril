## 1. Base

- [x] 1.1 Rama `feat/coding-compass-wave-c1` desde `origin/main` @ `a81a026`
      (Wave B #236). No usar el checkout sucio. No implementar en el PR
      de planificación
- [x] 1.2 Confirmar `wave_b_freeze_rows_301_and_up` rojo si se edita
      `301..=450` — ese test se reemplaza en 3.x

## 2. Índice applied 301..=450

- [x] 2.1 Auditar filas legado `301..=450` contra la tabla primary/applied
      de `design.md` (corregir recetas mal taggeadas)
- [x] 2.2 Completar tags applied hasta ≥ 90 pasos taggeados en
      `301..=450` (sin densificar; sin 100% P3)
- [x] 2.3 Dejar untagged bits/two-pointers/window: al menos 301, 329, 371

## 3. Freeze y tests

- [x] 3.1 Reemplazar freeze ≥ 301 por freeze ≥ 451 vs `a81a026`
      (`WAVE_B_FROZEN_451`)
- [x] 3.2 Unit tests: piso ≥ 90; no 100% tag `3`; 301/329/371 vacíos;
      `4..=100` denso; piso 101..=300 ≥ 120; índice ordenado
- [x] 3.3 No editar `glossary.rs`, FAB, ni filas `≥ 451`

## 4. Proof

- [x] 4.1 `make web-test` 100% verde
- [ ] 4.2 PR `feat/coding-compass-wave-c1` con TED slides; no mezclar
      C2/C3 ni DUA
