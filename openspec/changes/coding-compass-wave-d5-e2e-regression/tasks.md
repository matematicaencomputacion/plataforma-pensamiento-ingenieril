## 1. Planificación

- [x] 1.1 Change `coding-compass-wave-d5-e2e-regression` (proposal /
      design / spec / tasks) anclado a `origin/main` @ `a6811ac`
- [x] 1.2 Fuera de alcance explícito: Docker, GCP, compose, load
      tests, re-shard, `trunk serve`, retags, mock de tags

## 2. Journey Playwright

- [x] 2.1 Añadir `web/e2e/tests/journey.concepts-hub.spec.ts`:
      register → `/concepts/1` → heatmap 100 celdas → drawer D.1 →
      AND D.2 → `#concept-prereq-alert` D.4 (sin mock de tags)
- [x] 2.2 Enganchar el spec a `run_web_journeys` en
      `scripts/harness/run.sh` (no tocar matrix de 6 shards)

## 3. Docs ADR 003

- [x] 3.1 `docs/testing/journeys.md`: página `/concepts/:id` + matriz
      + Mermaid del Journey D
- [x] 3.2 `TESTING.md` + mención en ADR 003 del journey del hub

## 4. Proof

- [x] 4.1 `make web-test` verde (local; no Playwright en la laptop)
- [ ] 4.2 PR TED; squash-merge cuando Backend + Frontend +
      Playwright aggregator estén verdes
