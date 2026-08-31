## Why

Deploy Cloud Run se dispara al terminar Docker y ocupa un runner consultando E2E cada diez segundos; como E2E admite hasta 120 minutos, el guard actual expira a los 30 minutos aunque todos los gates terminen verdes. El deploy debe reaccionar al gate más lento y verificar el mismo SHA sin espera activa prolongada.

## What Changes

- Disparar Deploy Cloud Run cuando finaliza `E2E Web (Playwright)` en un push a `main`.
- Exigir que el evento E2E sea exitoso y comprobar que CI y Docker también terminaron verdes para el mismo SHA.
- Eliminar el bucle de polling de hasta 180 consultas y reducir el timeout del job de deploy a su trabajo real.
- Alinear la documentación operativa con el contrato CI + Docker + Playwright del PR #296.

### Alcance incluido

- `.github/workflows/deploy.yml` y documentación operativa del slice D.
- Validación estática del YAML y del contrato de SHA compartido.

### Fuera de alcance

- Ejecutar un deploy manual, modificar WIF, secretos, Artifact Registry o configuración de Cloud Run.
- Cambiar la suite Playwright, su sharding o sus timeouts.
- Mezclar cambios curriculares del PR #297.

### Plan de rollback

Revertir el commit restaura el trigger basado en Docker y su polling. Si GitHub no emite el evento E2E esperado, el workflow puede ejecutarse manualmente tras un change posterior; este change no altera recursos GCP.

## Capabilities

### New Capabilities

- `cloud-run-deploy-gate`: promoción de un SHA de `main` únicamente después de que E2E, CI y Docker estén verdes para ese mismo commit.

### Modified Capabilities

Ninguna.

## Impact

- GitHub Actions: `Deploy Cloud Run` cambia su workflow disparador y deja de consumir minutos de runner esperando E2E.
- Documentación: `docs/ops/gcp-iteration-plan.md` refleja el gate efectivo.
- Sin cambios en código de producto, APIs o recursos de nube.
