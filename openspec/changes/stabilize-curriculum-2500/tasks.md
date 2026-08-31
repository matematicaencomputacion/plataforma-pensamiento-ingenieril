## 1. Contrato curricular

- [x] 1.1 Corregir la cadena para `2380 → 2381`, `2440 → 2441` y `2500 → None`; verificar con tests unitarios que `1..=2500` es continuo y único
- [x] 1.2 Eliminar expectativas conceptuales contradictorias y verificar que todas las particiones activas resuelven a pasos del catálogo
- [x] 1.3 Alinear los tres conteos E2E del workspace a 2500 y verificar que no quedan expectativas activas de 2980/3040

## 2. Herramientas y limpieza

- [x] 2.1 Corregir nombres, imports y rangos de scripts Wave 24–25; verificar que cada validador comprueba directamente su rango correspondiente
- [x] 2.2 Retirar scripts Wave 26–34 fuera del contrato y verificar que ningún archivo activo los referencia
- [x] 2.3 Eliminar `.tmp_e2e_edit.py`, `.tmp_wave12_edit.py`, `web/_fmt_probe.rs` y `scripts/__pycache__`; verificar un `git status` sin esos temporales

## 3. Gates

- [x] 3.1 Ejecutar `openspec validate stabilize-curriculum-2500 --strict` y dejarlo verde
- [x] 3.2 Ejecutar `make web-test` y `make test` y dejar ambas suites verdes
- [x] 3.3 Ejecutar `make harness-journeys` según ADR 003 y documentar cualquier bloqueo externo reproducible
- [x] 3.4 Revisar el diff final, confirmar que no incluye cambios ajenos y preparar commit convencional en `fix/curriculum-stabilize-2500`
