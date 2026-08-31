## Context

El working tree contiene un rollback intencional desde 2980 hacia 2500, pero quedaron contratos cruzados: la ola 24 termina en 2440 sin enlazar a 2441, un test de ola 25 valida el puente equivocado, existe una expectativa conceptual contradictoria y los E2E aún cuentan 2980 elementos. Ver `proposal.md` y la spec `curriculum-catalog-stability`.

La solución debe respetar ADR 002 (ejecución Python exclusivamente client-side) y ADR 003 (unitarias más journeys E2E como gate).

## Goals / Non-Goals

**Goals:**

- Establecer una única frontera `2500` comprobada por tests cercanos al dominio.
- Mantener una cadena continua `1..=2500`, con `2440 → 2441` y `2500 → None`.
- Alinear particiones y E2E sin relajar invariantes existentes.
- Dejar solo scripts activos que puedan ejecutarse sobre el catálogo estabilizado.

**Non-Goals:**

- Rediseñar el modelo `CodingStep` o el mapa conceptual.
- Recuperar selectivamente contenido 2501–2980.
- Cambiar APIs, persistencia o runtime del estudiante.

## Decisions

### 1. Corregir la cadena, no adaptar el test al enlace roto

`2380 → 2381` pertenece a la ola 24 existente. La ola 25 debe conectarse desde `2440 → 2441`; el test recorrerá únicamente `2441..=2500`. Esto preserva continuidad y evita saltarse 60 pasos. La alternativa de apuntar 2380 directamente a 2441 fue descartada porque vuelve inaccesible la ola 24.

### 2. Mantener las particiones 2381–2440

Las etiquetas de ola 24 ya existen y son coherentes con pasos activos. Se elimina la aserción duplicada que exige que 2381 esté vacío; la frontera conceptual válida pasa a ser posterior a 2500. La alternativa de borrar etiquetas fue descartada porque degradaría funcionalidad activa para satisfacer un test obsoleto.

### 3. Ajustar E2E al contrato, conservando asserts exactos

Los tres journeys seguirán usando `toHaveCount`, ahora con 2500. No se cambia a una condición flexible (`>=`) porque ocultaría futuras divergencias entre catálogo y UI.

### 4. Retirar olas posteriores como un cambio explícito

Los scripts de olas 26–33 se eliminan junto con su contenido, mientras las herramientas 24–25 se corrigen para que nombre, imports y rangos coincidan. `gen_wave34.py` y archivos temporales no forman parte del catálogo aceptado y se retiran. Se prefiere eliminación versionada frente a conservar generadores ejecutables capaces de reintroducir accidentalmente contenido fuera de contrato.

## Risks / Trade-offs

- [La eliminación de scripts reduce trazabilidad operativa] → el historial Git conserva las versiones y el plan de rollback las recupera por revert.
- [Los E2E exactos deberán cambiar al reabrir el catálogo] → cada nueva ola tendrá su propio change OpenSpec y actualizará el contrato deliberadamente.
- [El working tree mezcla archivos del usuario] → solo se eliminan temporales inequívocos y archivos de olas explícitamente fuera de alcance; `.agents/` y `AGENTS.md` se preservan.

## Migration Plan

1. Corregir cadena, tests conceptuales y conteos E2E.
2. Normalizar herramientas 24–25 y retirar 26–34 del árbol activo.
3. Eliminar temporales inequívocos.
4. Ejecutar validación OpenSpec, unitarias y journeys ADR 003.
5. Commit convencional en `fix/curriculum-stabilize-2500` y PR hacia `main`.

Rollback: revertir el commit del change restaura catálogo, scripts y expectativas anteriores en una sola operación auditable.
