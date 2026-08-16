## Context

C6 asume C5 mergeado: freeze ≥ 901. El catálogo `901..=1000` es DSA
replay en bandas de 6 (canonicalize / prefix-state / bounded-window /
lower-boundary / dependency-order / minimum-transition), con review
997–1000 recortado a 4 pasos: monotonic queue 901–906, Fenwick 907–912,
segtree 913–918, sparse table 919–924, binary lift 925–930, string
matching 931–936, rolling hash 937–942, suffix 943–948, flow 949–954,
matching 955–960, game 961–966, randomized 967–972, offline 973–978,
meet-in-middle 979–984, matrix exp 985–990, geometry 991–996, review
997–1000.

## Goals / Non-Goals

**Goals**

- Applied `901..=1000`; piso ≥ 60 / 100; no 100% P3.
- Freeze: ninguna fila `> 1000`.
- Contrato C5 (`751..=900`) intacto.
- 913, 955 y 1000 permanecen untagged (tests previos + recetas).

**Non-Goals**

- Filas `> 1000`. Nuevos diagramas DUA. Densificar.
- Tocar `glossary.rs` / FAB.

## Decisions

1. **Corte en 1000** — banda final de 100 pasos; piso escalado ≥ 60.
2. **Política** — queue/Fenwick/segtree/lift/strings/flow/offline se
   taggean; bounded-window, geometry 991–996, randomized 967–972:
   **no tag**.
3. **Representantes untagged de prueba:** 913, 955, 1000
   (contratos ya existentes en `applied_families` / `expanded_drills`).
4. **Gates:** `make web-test`. Sin harness completo.

## Risks / Trade-offs

| Riesgo | Mitigación |
|---|---|
| Pisar C5 | Test de snapshot `751..=900` |
| Taggear 913/955/1000 | Tests previos + recetas C6 |
