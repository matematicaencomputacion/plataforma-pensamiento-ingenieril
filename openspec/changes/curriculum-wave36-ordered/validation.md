## Evidencia local

- `python3 scripts/validate_wave36.py`: PASS; 60/60 soluciones, diez familias de seis, 41 ejercicios con marcadores de entrega segura y catálogo exacto `1..=3160`.
- Validadores acumulativos 26–36: PASS.
- Segunda aplicación de `scripts/apply_wave36.py`: sin cambios.
- `cd web && cargo test`: PASS, 141/141.
- `make test`: PASS.
- `make harness-journeys`: PASS, 10/10 journeys; stack API y Trunk verdes.
- `npx --yes @fission-ai/openspec@latest validate curriculum-wave36-ordered --strict`: PASS.
- `git diff --check`: PASS.

## Nota de formato

`cd web && cargo fmt --all -- --check` continúa reportando deuda global preexistente: 17.054 líneas de diff, comenzando en `web/src/api.rs` y extendiéndose por archivos ajenos a Ola 36. No se aplica formateo global porque contaminaría el diff; las pruebas funcionales, contractuales y de integración son verdes.
