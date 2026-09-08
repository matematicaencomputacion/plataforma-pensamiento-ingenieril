## Purpose

Garantiza que la protección de `main` reciba un contexto E2E determinista sin
consumir la matriz Playwright cuando el cambio no afecta al producto.

## ADDED Requirements

### Requirement: Toda PR produce el check E2E requerido

El workflow E2E SHALL ejecutarse para toda pull request dirigida a `main` y MUST
producir el contexto `Playwright Chromium smoke` para el SHA evaluado.

#### Scenario: PR documental

- **GIVEN** una PR cuyo diff no modifica paths de producto
- **WHEN** se ejecuta el workflow E2E
- **THEN** la matriz Playwright queda omitida y el check agregado termina verde

#### Scenario: Fallo del clasificador

- **GIVEN** una PR cuyo alcance no puede determinarse
- **WHEN** el clasificador falla o no produce un output válido
- **THEN** el check agregado termina rojo

### Requirement: Cambios de producto ejecutan la suite completa

Una PR que modifique `web/**`, `backend/**`, `Dockerfile`, `.dockerignore` o los
workflows E2E/Docker MUST ejecutar los seis shards y MUST NOT aprobar el check
agregado salvo que toda la matriz termine exitosamente.

#### Scenario: Matriz verde

- **GIVEN** una PR con cambios de producto
- **WHEN** los seis shards terminan exitosamente
- **THEN** `Playwright Chromium smoke` termina verde

#### Scenario: Un shard falla

- **GIVEN** una PR con cambios de producto
- **WHEN** cualquier shard falla o es cancelado
- **THEN** `Playwright Chromium smoke` termina rojo

### Requirement: Push documental no despliega

El workflow E2E SHALL conservar filtros de producto para eventos push a `main`,
de modo que un merge sólo documental no genere una señal de deploy.

#### Scenario: Merge sólo documental

- **GIVEN** un push a `main` que sólo modifica documentación
- **WHEN** GitHub evalúa los triggers
- **THEN** E2E no se inicia y Deploy Cloud Run no recibe un evento nuevo
