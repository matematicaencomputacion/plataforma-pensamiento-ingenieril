## 1. Planificación (este PR)

- [x] 1.1 Change `coding-compass-wave-d-concept-map` (proposal / design /
      spec / tasks) anclado a `origin/main` @ `87a5334`
- [x] 1.2 Fuera de alcance explícito: retags `1..=1000`, Go analytics,
      más SVG DUA, producto en este PR

## 2. Modelo de celdas (PR de implementación)

- [ ] 2.1 Helpers de décadas (100 bandas) + estado
      `empty|pending|partial|done` en `web/src/concepts/mod.rs`
      **sin** editar `STEP_PARTITIONS`
- [ ] 2.2 Unit: 100 celdas; década de `52` en P2 no `empty`; completed
      sintético mueve estado; freeze `(micro_step, tags)` vs `87a5334`

## 3. Hub

- [ ] 3.1 Render `#concept-heatmap` en `ConceptsPage` **antes** de
      `#concepts-drill-list`; celdas `#concept-heat-{lo}` + `data-state`
      + nombre accesible (`done/total`)
- [ ] 3.2 Clic en no-`empty` → primer drill pendiente de la década
      (o el primero si todos done); `empty` no navega
- [ ] 3.3 Las 5 particiones (incluidas 4–5 `map_only`); lista Wave A intacta

## 4. Tests y proof

- [ ] 4.1 Extender `concepts.partitions.spec.ts`: heatmap visible en
      `/concepts/1` → clic celda no-`empty` → `/learn/:step`
- [ ] 4.2 `make web-test` verde; PR TED de implementación; no tocar
      `glossary.rs` / peek/dock C3
