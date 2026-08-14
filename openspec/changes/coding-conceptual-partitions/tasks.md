## 1. OpenSpec + gobernanza

- [x] 1.1 Change `coding-conceptual-partitions` (proposal/design/spec/tasks)
- [x] 1.2 Nota ADR: Leptos CSR es el shell canónico de Coding (ADR 003); ADR 001 texto Qwik queda como deuda documental (registrada en `design.md`; sin reescribir ADR 001 en este slice)

## 2. Modelo conceptual

- [x] 2.1 Módulo `web/src/concepts.rs`: 5 particiones + ejes + modelo mental
- [x] 2.2 Índice `micro_step → partitions` (foundations 1–3; 4–5 liviano)
- [x] 2.3 Helpers: `partition_by_id`, `partitions_for_micro_step`, `drills_for_partition`, `%` dominio
- [x] 2.4 Unit tests: ids 1..=5, tags válidos, drills no vacíos en 1–3

## 3. UI — Compás + Hub

- [x] 3.1 Componente `PartitionNav` en header autenticado
- [x] 3.2 Ruta `/concepts/:id` (hub tres bloques + drills)
- [x] 3.3 Estilos compactos + focus-visible
- [x] 3.4 Resaltar lente activa cuando la URL es `/learn/:step` etiquetado

## 4. UI — Learn badge

- [x] 4.1 Badge(s) conceptuales bajo enunciado → `/concepts/:id`
- [x] 4.2 Omitir badge si el step no tiene tags

## 5. Proof

- [x] 5.1 `make web-test` (concepts + compile) — 66 ok
- [x] 5.2 E2E: login → `/concepts/1` → drill → `/learn/...` (`concepts.partitions.spec.ts`)
- [ ] 5.3 PR TED slides; gate Backend+Frontend (Playwright no bloqueante)
