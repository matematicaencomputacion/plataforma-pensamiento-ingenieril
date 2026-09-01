## 1. Reconciliación y generación

- [x] 1.1 Auditar PR #294 contra `main` y registrar en proposal/design por qué no se reutiliza, verificando que el diff final no contenga cambios históricos ajenos
- [x] 1.2 Crear diez familias de seis ejercicios y verificar rango `2861..=2920`, slugs únicos, progresión distribuida y ausencia de firmas duplicadas contra 1–2860
- [x] 1.3 Generar starter, prompt, pytest y Rust desde definiciones estructuradas, verificando que las 60 soluciones canónicas pasen
- [x] 1.4 Crear un aplicador protegido e idempotente y verificar que la segunda ejecución no produzca cambios

## 2. Integración del catálogo

- [x] 2.1 Enlazar 2860 con 2861, insertar Ola 32 y dejar 2920 terminal, verificando la cadena completa
- [x] 2.2 Registrar las 60 constantes en `CODING_STEPS` y verificar catálogo exacto y contiguo `1..=2920`
- [x] 2.3 Extender particiones conceptuales hasta 2920 y verificar orden, tags válidos y ausencia de referencias posteriores
- [x] 2.4 Actualizar las tres expectativas E2E a 2920 y verificar que no queden conteos activos en 2860

## 3. Validación local

- [x] 3.1 Ejecutar `python3 scripts/validate_wave32.py` y verificar 60/60 soluciones, 10x6 familias y restricciones deterministas
- [x] 3.2 Ejecutar acumulativamente los validadores de Olas 26–32 y verificar que todos pasen sobre el catálogo integrado
- [x] 3.3 Ejecutar `cargo fmt --all -- --check` y `cargo test` en `web`, verificando 137/137 tests y documentando por separado la deuda de formato preexistente fuera del diff de la ola
- [x] 3.4 Ejecutar `make test` y `make harness-journeys`, verificando backend, web y journeys canónicos
- [x] 3.5 Ejecutar `npx --yes @fission-ai/openspec@latest validate curriculum-wave32-ordered --strict` y verificar contrato válido

## 4. Entrega secuencial

- [x] 4.1 Auditar el diff y verificar que no contiene micro-steps mayores a 2920 ni cambios fuera del alcance
- [x] 4.2 Crear commit convencional, publicar la rama y abrir un único PR con evidencia local y SHA explícito
- [ ] 4.3 Verificar CI, Docker, E2E 6/6 y smoke del head exacto; corregir cualquier fallo en la misma rama
- [ ] 4.4 Hacer squash merge autorizado solo con todos los checks efectivos verdes y verificar CI, Docker, E2E y Deploy post-merge por el merge SHA exacto
