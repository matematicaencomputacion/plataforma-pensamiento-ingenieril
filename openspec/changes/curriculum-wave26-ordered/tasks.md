## 1. Generación reproducible

- [x] 1.1 Incorporar el generador determinista de los 60 ejercicios `2501..=2560` y verificar que emite exactamente 60 identificadores únicos sin valores fuera del rango
- [x] 1.2 Incorporar el aplicador con anclas exactas para la frontera 2500 y verificar que falla sin modificar archivos cuando la frontera no coincide
- [x] 1.3 Incorporar el validador de Ola 26 y ajustar el validador de Ola 25 para composición, verificando que ambos aceptan el catálogo extendido

## 2. Catálogo y navegación

- [x] 2.1 Aplicar las constantes de Ola 26, enlazar 2500 con 2501 y dejar 2560 terminal, verificando la cadena completa con el validador
- [x] 2.2 Registrar `PY2501_STEP..=PY2560_STEP` exactamente una vez en `CODING_STEPS` y verificar tamaño 2560 y contigüidad Rust
- [x] 2.3 Extender las particiones conceptuales hasta 2560 y verificar orden, existencia de límites y ausencia de referencias posteriores

## 3. Pruebas y journeys

- [x] 3.1 Agregar pruebas Rust específicas de límites y unicidad de Ola 26 y verificar que la suite del módulo pasa
- [x] 3.2 Actualizar los tres conteos E2E canónicos a 2560 y verificar que el validador detecta cualquier drift
- [x] 3.3 Ejecutar formato, validadores, `make test` y `make harness-journeys`, documentando por separado cualquier gate que dependa del entorno

## 4. Integración secuencial

- [x] 4.1 Revisar que el diff contenga solamente Ola 26 y artefactos asociados, verificando ausencia de identificadores mayores a 2560
- [x] 4.2 Crear commit convencional, publicar la rama y abrir un PR estructurado con la evidencia local
- [x] 4.3 Monitorear todos los checks del PR y corregir fallos propios en la misma rama hasta quedar verde o identificar un bloqueo externo
- [x] 4.4 Tras autorización de merge, verificar SHA y workflows post-merge en `main` antes de habilitar Ola 27
