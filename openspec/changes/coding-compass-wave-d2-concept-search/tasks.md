## 1. Planificación

- [x] 1.1 Change `coding-compass-wave-d2-concept-search` (proposal /
      design / spec / tasks) anclado a `origin/main` @ `362ec98`
- [x] 1.2 Fuera de alcance explícito: analytics Go, DAG, visual-regression
      suite, retags `1..=1000`, rebind Ctrl/Cmd+K del FAB

## 2. Modelo

- [x] 2.1 Publicar `ConceptFacetFilter` + `filtered_drills_for_partition`
      en `web/src/concepts/mod.rs` **sin** editar `STEP_PARTITIONS` ni
      el seed del glosario
- [x] 2.2 Heatmap/década aceptan el set filtrado (`heatmap_cells_for_drills`)
- [x] 2.3 Unit: `append` en P1 incluye 20 y excluye 1; `recursion`+`dfs`
      en P3 incluye 109 y no 133; P1∧P3 ⊆ tags `[1,3]`; freeze `1..=1000`

## 3. Hub UI

- [x] 3.1 `#concept-facet-bar` con query, chips AND `#concept-facet-p{n}`,
      conteo; sin Ctrl/Cmd+K en `/concepts`
- [x] 3.2 Lista Wave A + heatmap + drawer D.1 renderizan el set filtrado;
      celdas con match exponen `data-facet="hit"`
- [x] 3.3 Clic en un drill filtrado → `/learn/:id`; no desmontar editor

## 4. Tests y proof

- [x] 4.1 Playwright: barra visible → `append` → lista/heatmap cambian →
      `#concepts-drill-20` → `/learn/py-20-list-change`; heatmap D.1 y
      FAB `/learn` siguen
- [ ] 4.2 `make web-test` verde; PR TED; squash-merge cuando Backend +
      Frontend + Playwright aggregator estén verdes
