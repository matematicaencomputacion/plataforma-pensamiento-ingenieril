## Why

El rail de Coding creció hasta 1000 micro-pasos y se percibe como un túnel lineal.
El alumno no ve la arquitectura del conocimiento (mutabilidad, LEGB, paradigmas,
ecosistema, dominios). Sin un mapa conceptual, el repaso y el dominio por tema
quedan a ciegas.

## What Changes

- Taxonomía estable de **5 particiones conceptuales** (lentes de razonamiento).
- Índice de tags multi-etiqueta (no excluyente) sobre micro-pasos existentes,
  sin duplicar el catálogo ni reordenar el rail.
- **Compás cognitivo** permanente `[1]…[5]` en la shell autenticada.
- Hub por partición: modelo mental + ejes + drills con estado de completed_levels.
- Badge conceptual en `/learn/:step` que enlaza al hub de la partición activa.
- Particiones 4–5 como mapa/lectura (ADR 002: sin PyPI pesado ni ejecución
  server-side).

### Alcance incluido

- OpenSpec + módulo `concepts` + UI PartitionNav / Concepts hub + chips en learn.
- Mapping inicial fuerte de particiones 1–3 (foundations); 4–5 con drills livianos.
- Unit tests del índice + E2E smoke del hub.

### Fuera de alcance

- Reordenar o partir el rail en tracks de progreso separados.
- Cambiar `completed_levels` / cursor.
- Ejecutar FastAPI/Django/NumPy/PyTorch en el browser o en Go.
- Taggear a mano los 1000 pasos en la primera entrega.
- Monaco / JupyterLite.

## Capabilities

### New Capabilities

- `coding-conceptual-partitions`: navegación y metadata conceptual sobre el rail.

### Modified Capabilities

- (ninguna main-spec previa de particiones)

## Impact

- `web/src/concepts.rs`, componentes UI, rutas `/concepts/:id`, estilos, E2E.
- OpenSpec change `coding-conceptual-partitions`.
- Sin endpoints Go de ejecución; progreso intacto (ADR 002 / #114).

## Plan de rollback

Quitar PartitionNav, ruta `/concepts` y chips; el índice conceptual es aditivo.
El rail y `/learn/:step` siguen funcionando sin tags.
