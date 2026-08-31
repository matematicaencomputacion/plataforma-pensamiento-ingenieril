## Context

El PR #296 endureció el deploy para exigir CI y Playwright verdes en el mismo SHA. El workflow continúa disparándose al finalizar Docker, que tarda aproximadamente cuatro minutos, mientras E2E ejecuta ~1145 tests y admite 120 minutos. El runner de deploy sondea E2E por 30 minutos y produce un falso rojo si la suite termina después.

## Goals / Non-Goals

**Goals:**

- Mantener el gate estricto CI + Docker + E2E sobre un SHA único.
- Eliminar polling prolongado y minutos de runner ocioso.
- Hacer que un E2E fallido impida iniciar cualquier paso con GCP.

**Non-Goals:**

- Optimizar la duración interna de Playwright.
- Cambiar credenciales, secretos o recursos de GCP.
- Desplegar cambios que no disparan E2E, como documentación aislada.

## Decisions

### 1. Disparar desde el workflow más lento

`workflow_run` observará `E2E Web (Playwright)` en lugar de Docker. El evento ya aporta el SHA exacto y su conclusión; el `if` del job exige `success` y `push`. Alternativa descartada: ampliar el polling a 120 minutos, porque factura un runner sin trabajo y sigue acoplando dos timeouts.

### 2. Verificar CI y Docker una sola vez

Cuando la matriz E2E termina, CI y Docker normalmente finalizaron muchos minutos antes. El gate consulta cada workflow por SHA y exige estado `completed` con conclusión `success`. No se reintenta: un resultado ausente o pendiente indica una incoherencia del pipeline y falla cerrado.

### 3. Mantener separación entre validación y GCP

La comprobación permanece como primer step, antes de checkout, OIDC/WIF y Artifact Registry. Así un gate inconsistente no solicita tokens ni modifica recursos externos.

## Risks / Trade-offs

- [Un push que no dispara E2E tampoco dispara deploy] → los paths E2E ya cubren código de producto, backend, web, Docker y workflows relevantes; cambios solo documentales no necesitan revisión de Cloud Run.
- [CI o Docker podrían aparecer pendientes por latencia de eventos] → ambos son mucho más cortos que E2E; si ocurre una anomalía, fallar cerrado es preferible a promocionar sin evidencia.
- [El nombre del workflow es contrato] → conservar exactamente `E2E Web (Playwright)`, `CI` y `Docker`, con validación estática en el change.

## Migration Plan

1. Cambiar el workflow observado a E2E y el comentario de cabecera.
2. Sustituir el bucle de polling por verificaciones únicas de CI y Docker.
3. Alinear `docs/ops/gcp-iteration-plan.md`.
4. Validar YAML/OpenSpec y abrir PR operativo independiente.

Rollback: revertir el commit recupera el trigger Docker y el polling anterior sin modificar infraestructura.
