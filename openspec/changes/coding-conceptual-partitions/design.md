## Context

Coding vive en Leptos CSR (`/learn`, Pyodide) con rail 1..=1000 y progreso por
`completed_levels` (#114). El alumno entra directo al editor (baja fricción) pero
carece de anclaje conceptual. Las particiones de Python (mutabilidad, LEGB,
paradigmas, ecosistema, dominios) son lentes pedagógicas, no otra arquitectura
de runtime.

ADR 002: Python del alumno solo en el browser. Particiones 4–5 no habilitan labs
con PyPI pesado.

## Goals / Non-Goals

**Goals**

- Compás `[1]…[5]` siempre visible con sesión viva (workspace + learn).
- Hub por partición: modelo mental, ejes, drills con ✔/pendiente.
- Tags multi-partición; énfasis primary/applied.
- Badge en learn → vuelve al hub de esa partición.
- Progreso por concepto derivado de `completed_levels ∩ drills`.

**Non-Goals**

- Campo `partition: u8` obligatorio en cada `CodingStep` (1000 edits).
- Tracks de progreso separados por partición.
- Runtime de terceros en Go o Pyodide micropip masivo.

## Decisions

1. **Índice externo** (`web/src/concepts.rs`): particiones + `micro_step → &[u8]`
   sin mutar las 1000 constantes `CodingStep`. Validado en unit tests.
2. **Tags multi-etiqueta**: un step puede pertenecer a varias particiones.
3. **IDs estables**: `1..=5` con slugs `data-model`, `scope-legb`, `paradigms`,
   `ecosystem`, `application-domains`.
4. **Ruta `/concepts/:id`**: hub de tres bloques; el filtro no crea progreso propio.
5. **PartitionNav en header**: junto a SessionBar cuando hay user; resalta si el
   step actual (query/ruta learn) cae en esa partición.
6. **ADR 002**: 4–5 = mapa + drills stdlib/scripting livianos ya en Pyodide.
7. **Rollout**: v1 mapea foundations (≈1–100) con primary; DSA posteriores
   pueden recibir tags applied en slices siguientes.

## Risks / Trade-offs

| Riesgo | Mitigación |
|--------|------------|
| Drift índice vs MICRO_STEP_COUNT | Tests: todo micro_step etiquetado existe |
| Sobre-etiquetar DSA como “imperativo” | Solo foundations + énfasis primary |
| UI congestionada | Compás compacto; hub dedicado |
| Confusión Qwik vs Leptos en docs | Spec asume Leptos CSR (ADR 003) |

## Migration Plan

1. Ship índice + nav + hub + badge.
2. Ampliar tags por familias DSA en PRs pequeños.
3. Métricas de dominio en workspace (barra %).

## Open Questions

- ¿Portada anónima muestra el compás? → **No** (solo sesión autenticada).
- ¿Un step sin tags? → Compás sin highlight; badge omitido.
