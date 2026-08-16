## 1. Planificación

- [x] 1.1 Change `coding-compass-wave-d1-heatmap-drilldown` (proposal /
      design / spec / tasks) anclado a `origin/main` @ `f7734a8`
- [x] 1.2 Fuera de alcance explícito: faceted search, Go analytics, DAG,
      visual-regression suite, retags `1..=1000`

## 2. Modelo

- [x] 2.1 Publicar `heatmap_decade_drills(partition_id, band)` en
      `web/src/concepts/mod.rs` **sin** editar `STEP_PARTITIONS`
- [x] 2.2 Unit: P1 década `1..=10` solo micros de partición 1; década
      `empty` → vec vacío; freeze Wave D `1..=1000` sigue verde

## 3. Hub drawer

- [x] 3.1 Celdas no-`empty` abren `#concept-decade-drawer` (dialog) con
      `#concept-decade-list` / `#concept-decade-drill-{n}`; `empty` no
      abre ni navega
- [x] 3.2 Esc y botón cerrar quitan el dialog; lista Wave A intacta
- [x] 3.3 Clic en un ítem → `/learn/:id`; no desmontar editor (hub only)

## 4. Tests y proof

- [x] 4.1 Extender `concepts.partitions.spec.ts`: heatmap visible → clic
      no-`empty` → lista de esos micros → clic → `/learn/:id`; celda
      `empty` sin lista/nav; Esc cierra
- [ ] 4.2 `make web-test` verde; PR TED; squash-merge cuando Backend +
      Frontend + Playwright aggregator estén verdes
