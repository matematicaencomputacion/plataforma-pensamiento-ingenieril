## Why

`Playwright Chromium smoke` protege `main`, pero el filtro `pull_request.paths`
impide que el check exista en PRs documentales. GitHub las deja bloqueadas por
un check requerido que nunca puede comenzar. Ejecutar seis shards para cada ADR
resolvería el bloqueo a costa de minutos de runner sin valor de producto.

## What Changes

- Ejecutar el workflow E2E en toda PR a `main`.
- Clasificar el diff antes de reservar runners Playwright.
- Ejecutar los seis shards sólo cuando cambia la superficie de producto.
- Mantener el check agregado en PRs documentales y conservar el filtro de paths
  en pushes a `main` para no desplegar documentación.
- Documentar los checks requeridos y el contrato de protección de rama.

### Alcance incluido

- `.github/workflows/e2e.yml`, ADR 003 y documentación operativa de GitHub/GCP.
- Escenarios verificables para cambios de producto, documentación y fallos del
  clasificador o de la matriz.

### Fuera de alcance

- Cambiar tests Playwright, cantidad de shards, Cloud Run o credenciales.
- Actualizar versiones de GitHub Actions o resolver warnings de Docker/Node.
- Modificar currículo o la PR #313.

### Plan de rollback

Revertir el commit restaura el filtro de paths en PRs. Si fuera necesario
mantener branch protection durante el rollback, se debe retirar temporalmente
el contexto Playwright requerido para evitar checks pendientes imposibles.

## Capabilities

### New Capabilities

- `required-e2e-check-routing`: check E2E estable para protección de rama con
  consumo condicional de la matriz Playwright.

### Modified Capabilities

- `cloud-run-deploy-gate`: conserva E2E completo como disparador exclusivo de
  deploy en pushes de producto a `main`.

## Impact

- GitHub Actions: agrega un job clasificador pequeño en PRs.
- GitHub branch protection: el contexto requerido deja de bloquear PRs sólo-docs.
- Costos: no se crean seis runners Playwright cuando el diff no toca producto.
