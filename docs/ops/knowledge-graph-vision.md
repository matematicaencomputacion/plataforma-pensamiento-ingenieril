# Visión diferida: grafo de conocimiento (Neo4j / 4 capas)

**Estado:** diferida. Este archivo **no** es el siguiente slice de implementación.
**Ancla:** `origin/main` @ `8f77810` (PR #253, plan de iteración GCP).
**Producto activo:** coding compass de Pensamiento Ingenieril (Go + Leptos CSR + Pyodide).
**Ops activa:** [`docs/ops/gcp-iteration-plan.md`](gcp-iteration-plan.md) — cola A–E.

Un borrador externo (“4 gigantes / 3 capas / fricción / recomendador / Aura Free”)
propone Neo4j Aura, Cypher, ETL Python de ESCO y un motor de recomendación tipo
Dijkstra/PageRank. **Se archiva aquí para no perder la idea y para no
desviar el loop GHA/CI recién anclado.** No se implementa Neo4j, no se
añade Aura, no se escribe Cypher, no se ingiere ESCO.

---

## 1. Mapeo: capas del borrador → lo que PPI ya tiene

El borrador habla de cuatro “gigantes” de dominio. En PPI esas capas
**ya existen** con el stack canónico (ADR 001 Go, ADR 002 Wasm en el
cliente, SQLite hoy / Postgres en el PRD). No hace falta un grafo
gestionada para que el producto funcione.

| Capa del borrador | Equivalente PPI **ya en `main`** | Dónde |
|---|---|---|
| **Capa 2 — pedagogía** (particiones, glosario, prerrequisitos) | `ConceptPartition` (ids `1..=5`) + glosario + heatmap + DAG estático en WASM con aristas `requires` / `reinforces` | `web/src/concepts.rs`, `web/src/concepts/glossary.rs`, `web/src/concepts/dag.rs`, hub `/concepts/:id`. Wave D.4. **Cero Neo4j.** |
| **Capa 3 — estudiante** | JWT + progreso en SQLite (`completed_levels`, `current_level`) + eventos conceptuales D.3 | `backend/` auth + `ppi.db`; Leptos CSR |
| **Capa 4 — telemetría** | Tabla `concept_events` + `domain.Aggregate` (rollups por partición/década + bottleneck). **Agregados estilo arista en SQL**, no un nodo por clic | `POST /api/concept-events`, `GET /api/concept-analytics`, widget `#concept-analytics` |
| **Capa 1 — mercado** (MIP, ESCO, CIIU, VSM, Leontief) | **Fuera de alcance** del producto actual (coding compass de Pensamiento Ingenieril). El “mercado de skills” de PPI es curriculum versionado + niveles | `curriculum/`, `backend/data/` |

### Por qué la Capa 4 se queda en SQL

El propio borrador advierte: **no explotar Aura** con un nodo por clic.
D.3 ya aplicó esa lección sin grafo: los eventos se persisten en
`concept_events` y el read model es un agregado (`PartitionCount`,
`DecadeCount`, `Bottleneck`). Un clickstream como grafo de propiedades
en Aura Free es exactamente la trampa de los 200k nodos. SQL (SQLite
hoy, Postgres cuando exista el ADR del slice B) es el motor correcto
para conteos de fricción.

---

## 2. Captura de fricción (ya shipped)

No hace falta una base nueva para “sentir dónde se traba el alumno”:

- `learn_validate_fail` / `learn_validate_pass` / `learn_step_enter`
- `concept_dwell`, `heatmap_decade_open`, `dua_fab_open`
- Hint de cuello de botella en `#concept-analytics` / `#concept-analytics-hint`

**Autopoiesis / upgrades de producto** (el grafo que “se reescribe a sí
mismo”, curricula que mutan solas, un segundo cerebro de mercado) son
visión de producto **posterior**. No justifican un motor de grafo ahora
ni un sidecar Python.

---

## 3. Recomendador: v1 = caminar `Requires`

Wave D.4 ya recorre las aristas `Requires` del DAG en WASM y muestra
**“Base faltante”** (`#concept-prereq-alert`) cuando el `to` no está
empezado. Esa es la primera versión del recomendador: alerta pedagógica
local, no un ranking global.

| Idea del borrador | Decisión PPI |
|---|---|
| Caminar prerrequisitos | **Ya existe** (`web/src/concepts/dag.rs` + alerta en el hub) |
| Dijkstra / PageRank / Neo4j GDS | Solo si **alguna vez** introducimos un motor de grafo. **No ahora.** |
| Desbloquear el rail con el DAG | Explicitamente *non-goal* de D.4 (alerta, no gate) |

---

## 4. Costo: no añadir Neo4j Aura

Aura Free (o cualquier instancia Aura “por las dudas”) no entra al
stack:

- Trampa de **~200k nodos** + pausa por inactividad: incompatible con
  clickstream y con un catálogo ESCO de 13k skills.
- El PRD ya elige **PostgreSQL en producción / SQLite en local**.
- El loop de iteración en la nube **ya es GitHub Actions** (PR #253).
- Neo4j Community en Docker, si alguna vez se evalúa, es un *maybe*
  del **paso 4 Fedora** (slice E del plan GCP: compose **después** de
  que la nube sea fuente de verdad). **No es cloud-first.** No va en
  el `Dockerfile` distroless ni en `docker.yml`.

---

## 5. Rechazo explícito (no portar)

Ingerir ESCO (~13k skills), un script de init Cypher, o un ETL Python
hacia Aura **violaría**:

- **ADR 001** — API core en Go; queda prohibido un entorno Python como
  API. Un sidecar FastAPI/Neo4j “solo para el grafo de aprendizaje”
  es exactamente esa migración.
- **ADR 002** — el código de alumnos no corre en Go ni en ningún
  servidor; Pyodide en el cliente. Un pipeline Python “de conocimiento”
  al lado del learn path se confunde con ejecución pedagógica y abre
  la puerta a RCE / segundo runtime.

Tampoco se portan:

- `NEO4J_*` / driver Bolt / Secret Manager de Aura
- routers FastAPI `mercado` / `conocimiento` / `estudiante` / `recomendador`
- numpy / Leontief / VSM como runtime de producto
- cualquier cambio a `Dockerfile` o compose que instale Neo4j **ahora**

---

## 6. Relación con el plan GCP (esto no salta la cola)

La secuencia operativa vigente es **A → B → C → D → E** en
[`gcp-iteration-plan.md`](gcp-iteration-plan.md):

1. **A** — `JWT_SECRET` fail-closed en production
2. **B** — ADR Postgres vs SQLite
3. **C** — driver Postgres opcional
4. **D** — deploy GHA (WIF → Artifact Registry → Cloud Run), bloqueado
   por project id + billing
5. **E** — compose Fedora clonando el contrato de prod (sin Neo4j de
   arranque)

Este documento es **visión aparcada**. No inserta un slice “grafo” entre
A y B. No bloquea el loop GHA. Quien retome Capa 1 (ESCO/MIP) o GDS
debe abrir un change OpenSpec **nuevo**, después de B/C, y justificar
por qué SQL + el DAG WASM ya no alcanzan.
