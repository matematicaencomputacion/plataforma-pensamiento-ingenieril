## Context

La protección de `main` exige `Playwright Chromium smoke`. GitHub sólo puede
satisfacer un contexto requerido si el workflow produce un check para el commit
de la PR. El filtro actual omite por completo el workflow en diffs documentales.

## Goals / Non-Goals

**Goals:**

- Producir el contexto E2E requerido en toda PR.
- Ejecutar la suite completa ante cualquier cambio de producto.
- Evitar runners Playwright y deploys para cambios sólo documentales.
- Fallar cerrado si el clasificador no puede determinar el alcance.

**Non-Goals:**

- Reducir la duración de los ~1145 tests.
- Sustituir Playwright o cambiar la estrategia de seis shards.
- Hacer que cambios documentales disparen Cloud Run.

## Decisions

### 1. Separar triggers de PR y push

`pull_request` se ejecuta sin filtro para producir siempre el check requerido.
`push` conserva los paths de producto; así un merge documental no ejecuta E2E
ni dispara el workflow de deploy que observa su finalización.

### 2. Clasificar antes de la matriz

Un job pequeño compara `base.sha` y `head.sha` con historial completo. La misma
lista de paths que gobernaba el workflow decide si se lanzan los shards. En un
push que ya superó el filtro, el output es siempre `true`.

### 3. El agregador valida tanto ejecución como omisión

El check estable usa `if: always()`. Exige clasificador exitoso; si el alcance es
de producto, exige matriz verde. Si no lo es, exige output `false` y matriz
`skipped`. Un output vacío, error de checkout o cancelación queda rojo.

## Risks / Trade-offs

- [La lista de paths puede divergir] → vive una sola vez en el clasificador para
  PR; el trigger push conserva la misma lista y el contrato se valida estáticamente.
- [Checkout completo consume tiempo] → sólo ocurre en PR y cuesta mucho menos
  que seis instalaciones de navegador.
- [Un cambio de workflow se clasifica a sí mismo] → `e2e.yml` y `docker.yml`
  están incluidos, por lo que este PR ejecutará la matriz completa.

## Migration Plan

1. Abrir PR desde un worktree limpio basado en `origin/main`.
2. Validar YAML, OpenSpec y escenarios de routing.
3. Comprobar que esta PR ejecuta seis shards y deja el agregador verde.
4. Después del merge, comprobar con una PR documental futura que los shards se
   omiten y el check requerido queda verde.

Rollback: revertir el commit y ajustar temporalmente branch protection como se
indica en la propuesta.
