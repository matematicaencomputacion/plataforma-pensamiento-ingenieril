## Context

Wave B (#236) implementó `ConceptMicroCard`: pills, TL;DR, headline,
`<pre>` snippet, pitfall, CTA. Explicitamente **no** dual-coding ni
audio. El pedido de producto Wave C (“diagramas visuales DUA”) es esta
capa, no un retag 601..=1000.

C3 asume C1 mergeado (secuencia: índice → DUA). No edita
`STEP_PARTITIONS`. Preferible no solapar la rama de implementación con
C2 (ambos tocan `web/src/concepts/` si C2 corre; C3 toca `glossary.rs` +
FAB).

Stack: Leptos CSR, CSS del crate (`styles.css`), Pyodide en cliente
(ADR 002). Sin Tailwind JIT.

## Goals / Non-Goals

**Goals**

- Dual-coding mínimo: diagrama estático + texto en peek y dock.
- Lectura estratificada (diagrama primero).
- Seed de modelos mentales (`python-lists`, `model-legb`,
  `model-mutability`, `model-recursion`); el resto MAY ser `None`.
- Editor intacto (mismo contrato Wave B).
- Cero red / Go / Mermaid runtime.

**Non-Goals**

- Retag 601..=1000. Tags C1/C2.
- Audio, TTS, UDL completo, sandbox, boss fights.
- 1000 SVGs. Imágenes HTTP. Diagramas animados.
- Search en header. Recolorear `PartitionNav`.

## Decisions

1. **Campo additive en `ConceptLens`, no tipo nuevo de id**
   - `pub diagram_svg: Option<&'static str>` (SVG markup compacto).
   - `None` = anatomía Wave B. No segundo enum de partición.
   - **Descartado:** archivos en `public/`; `<img src>`; Mermaid.js;
     campo en `GlossaryEntry` (el diagrama es por lente: alias P1 ≠
     LEGB P2).

2. **SVG inline compilado, no innerHTML de usuario**
   - Literales `'static` en `glossary.rs`. Render: nodo SVG Leptos o
     `view!` con markup fijo por id de entrada (sin parsear input del
     alumno).
   - Presupuesto: cada SVG ≤ ~2 KB; sin scripts ni `<foreignObject>`
     con HTML.
   - Tokens `--lens-pN` para acentos; etiquetas de texto en el SVG
     (color no es el único canal). `figure` + `aria-label` /
     `#concept-diagram`.

3. **Lectura por capas (DUA / dual-coding)**
   - DOM order: `#concept-diagram` → TL;DR → headline/snippet → pitfall
     → CTA.
   - El diagrama ilustra el **modelo** (cajas/flechas de alias, LEGB
     en cascada, stack de recursión), no el output del drill.
   - **Descartado:** reemplazar el snippet por el diagrama; canvas JS.

4. **Seed, no enciclopedia**
   - Obligatorio diagrama en: `python-lists` (lente P1), `model-legb`,
     `model-mutability`, `model-recursion`.
   - Cap `GLOSSARY_ENTRIES.len() ≤ 32` intacto. No añadir 20 términos
     nuevos en C3.

5. **Gates**
   - Unit: las 4 entradas tienen `diagram_svg = Some` en la lente
     canónica; SVG no contiene `<script`.
   - Playwright: extender `concepts.drawer.spec.ts` — query `extend`
     o `legb` → peek `#concept-diagram` → Anclar → marcador en editor.
   - Test de no-regresión: filas `≥ 601` unchanged (defensa del
     “no retag”).
   - Sin `make harness` completo.

6. **Secuencia**
   - Implementar en `feat/coding-compass-wave-c3` **después** de C1
     merge. Si C2 está en vuelo, rebase; no mezclar tags en el PR DUA.

## Risks / Trade-offs

| Riesgo | Mitigación |
|---|---|
| SVG ilegible a 320px | diagramas de 3–6 nodos; CSS max-height; texto al lado |
| XSS si se usa innerHTML | solo literales `'static`; prohibido input de alumno |
| Scope creep a UDL completo | seed 4 modelos; audio fuera |
| Conflicto con C2 en `glossary.rs` | C3 no toca `STEP_PARTITIONS`; C2 no toca glossary |

## Migration Plan

1. Esperar merge C1. Rama `feat/coding-compass-wave-c3`.
2. Campo + seed SVG → render peek/dock → tests.
3. `make web-test` + smoke drawer.
4. PR TED propio. Rollback: revert; FAB texto permanece.

## Open Questions

- ¿Diagrama también en Estado 1 (lista de hits)? → **No**; solo peek/dock.
- ¿Una lente sin diagrama en una entrada multi-lente? → **Sí, `None`**.
