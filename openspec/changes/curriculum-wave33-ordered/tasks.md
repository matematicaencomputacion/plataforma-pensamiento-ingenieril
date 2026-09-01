## 1. Reconciliación y generación

- [x] 1.1 Registrar el descarte técnico de PR #295 y verificar que el diff final no contenga sus cambios históricos ajenos
- [x] 1.2 Crear diez familias de seis ejercicios y verificar rango `2921..=2980`, slugs únicos y firmas originales contra 1–2920
- [x] 1.3 Generar contratos ejecutables y verificar 60/60 soluciones deterministas
- [x] 1.4 Crear aplicador protegido e idempotente y verificar segunda ejecución sin cambios

## 2. Integración

- [x] 2.1 Enlazar 2920 con 2921, insertar Ola 33 y dejar 2980 terminal verificando la cadena
- [x] 2.2 Registrar 60 constantes y verificar catálogo contiguo `1..=2980`
- [x] 2.3 Extender particiones conceptuales y freeze test hasta 2980
- [x] 2.4 Actualizar tres conteos E2E a 2980 y verificar ausencia del techo activo 2920

## 3. Validación local

- [x] 3.1 Ejecutar `python3 scripts/validate_wave33.py` y verificar 60 soluciones, 10x6 y restricciones
- [x] 3.2 Ejecutar validadores 26–33 acumulativamente y verificar todos verdes
- [x] 3.3 Ejecutar `cargo test` y verificar la suite Rust completa; documentar deuda preexistente de `cargo fmt --check`
- [x] 3.4 Ejecutar `make test` y `make harness-journeys` y verificar backend y 10 journeys
- [x] 3.5 Validar OpenSpec estricto y `git diff --check`

## 4. Entrega

- [x] 4.1 Auditar que el diff no contiene pasos mayores a 2980 ni archivos fuera de alcance
- [ ] 4.2 Crear commit, publicar rama y abrir PR único con evidencia y SHA
- [ ] 4.3 Verificar CI, Docker, E2E 6/6 y smoke del head exacto
- [ ] 4.4 Hacer squash merge autorizado y verificar CI, Docker, E2E y Deploy post-merge por SHA exacto
