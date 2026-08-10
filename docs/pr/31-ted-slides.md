## Slide 1 — Hook
Dejamos de perder horas en el bucle 409/401: hay un camino documentado y ejecutable para entrar al workspace y probarlo página a página.

## Slide 2 — Insight
El fallo no era bcrypt “cambiando entre runs”: era SQLite dual (`ppi.db` vs harness), contraseña distinta a la registrada, y un forgot-password DX que no navegaba al token real. Sin un sistema de journeys, cada intento era manual y opaco.

## Slide 3 — Move
- Tooling local: `ppi-authctl` + `make dev-set-password`; exposición ampliada de `resetToken` en dev; forgot → reset auto-nav; purge de sesión en 401/409.
- ADR 003 + Mermaid en `docs/testing/journeys.md`.
- Playwright journeys P1→P3 (`make harness-journeys`).

## Slide 4 — Proof
- Local: `go test ./...`, `make web-test`, `make harness-journeys` (6/6 en la corrida de referencia).
- CI: Backend / Frontend / Playwright deben quedar verdes en este PR.
- Unlock humano (dev): `make dev-set-password EMAIL=… PASSWORD=…`

## Slide 5 — Ask
Merge a `main` tras CI verde. Esto es el follow-up de #30 (hub `753aa77` ya mergeado); revisar como upgrade de **desbloqueo auth + sistema de pruebas**, no como re-litigio del hub.

---

### Summary (reviewer checklist)
- [ ] No secretos en el diff
- [ ] `make harness-journeys` o CI E2E verde
- [ ] Docs ADR 003 / journeys legibles
