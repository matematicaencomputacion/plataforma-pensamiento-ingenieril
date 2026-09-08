## 1. Contrato y routing

- [x] 1.1 Quitar el filtro de paths sólo del trigger pull_request
- [x] 1.2 Agregar clasificador fail-closed por diff base/head
- [x] 1.3 Ejecutar la matriz sólo cuando cambia producto
- [x] 1.4 Mantener `Playwright Chromium smoke` como agregador estable

## 2. Documentación

- [x] 2.1 Documentar routing y check requerido en ADR 003
- [x] 2.2 Documentar branch protection y comportamiento sólo-docs en operaciones

## 3. Validación y entrega

- [x] 3.1 Validar OpenSpec, YAML y expresiones GitHub Actions
- [x] 3.2 Probar escenarios del clasificador con paths representativos
- [x] 3.3 Ejecutar pre-CI local y `git diff --check`
- [ ] 3.4 Abrir PR y verificar matriz completa más check agregado
