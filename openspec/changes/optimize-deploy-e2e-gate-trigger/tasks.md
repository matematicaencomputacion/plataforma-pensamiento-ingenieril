## 1. Gate de promoción

- [x] 1.1 Cambiar el trigger `workflow_run` de Docker a E2E y verificar que el job solo corre para `push`, `main` y conclusión exitosa
- [x] 1.2 Reemplazar el polling por comprobaciones únicas de CI y Docker para el mismo SHA; verificar que una conclusión distinta de success falla antes de GCP
- [x] 1.3 Ajustar el timeout y permisos del workflow al trabajo efectivo; verificar que no quedan bucles `sleep` ni permisos de Actions innecesarios

## 2. Coherencia documental

- [x] 2.1 Actualizar `docs/ops/gcp-iteration-plan.md` para describir E2E como disparador y CI + Docker como gates del mismo SHA

## 3. Validación y entrega

- [x] 3.1 Ejecutar `openspec validate optimize-deploy-e2e-gate-trigger --strict` y dejarlo verde
- [x] 3.2 Validar sintaxis YAML y contrato estático de nombres/trigger con herramientas locales disponibles
- [x] 3.3 Ejecutar `make test`, revisar `git diff --check` y preparar commit convencional
