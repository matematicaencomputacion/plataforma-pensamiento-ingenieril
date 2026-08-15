## 1. Base y gobernanza

- [x] 1.1 Confirmar base = `origin/main` con #232 mergeado (módulo
      `web/src/concepts.rs` + `PartitionNav`). Si no, rebase/cherry-pick
      antes de codear
- [x] 1.2 Change `coding-compass-wave-a` (este proposal/design/spec/tasks)

## 2. Índice denso 1–3

- [x] 2.1 Completar `STEP_PARTITIONS` para todo micro-paso de catálogo en
      `4..=100` según la tabla primary/applied de `design.md`
- [x] 2.2 Añadir tags *applied* puntuales en `101..=160` (sin bulk-`3`)
- [x] 2.3 Unit tests: sin huecos `4..=100`; pisos P1≥40, P2≥15, P3≥35;
      `101..=160` no es 100% tag `3`; índice ordenado
- [x] 2.4 `make web-test` verde en este módulo

## 3. Mastery visible en el header

- [x] 3.1 Renderizar `%` (o aro) en cada `#partition-nav-N` usando
      `mastery_percent`
- [x] 3.2 Setear `data-mastery="{0-100}"` y mantener `aria-label` con el %
- [x] 3.3 CSS compacto: no romper header a 320px; `0%` visible
- [x] 3.4 Test de componente o unit: `data-mastery` refleja
      `completed_levels` sintéticos

## 4. Mock E2E + journey conceptual

- [x] 4.1 Ajustar `installPyodideMock` `check`: pasar si el código tiene
      `def` + `print` (el heurístico `nombre`/`edad` no sirve para P2)
- [x] 4.2 Añadir `web/e2e/tests/journey.concepts.spec.ts` (escenario del
      spec: `[2]` → hub → drill → Validar → done + `data-mastery > 0`)
- [x] 4.3 Conservar `concepts.partitions.spec.ts` (smoke #232)
- [x] 4.4 Enganchar el journey a `make harness-journeys` si el glob no
      incluye `journey.*.spec.ts`
- [x] 4.5 Mermaid corto en `docs/testing/` (jornada conceptual)

## 5. Proof

- [x] 5.1 `make web-test` 100% verde
- [ ] 5.2 Playwright: `journey.concepts.spec.ts` + smoke partitions
- [x] 5.3 PR `feat/coding-compass-wave-a` con TED slides; no mezclar
      drawer/DUA/4–5 maps
