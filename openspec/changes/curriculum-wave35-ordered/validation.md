## Evidencia local

- Aplicador ejecutado dos veces: primera aplicación PASS; segunda ejecución sin cambios.
- Validadores 26–35: PASS; Ola 35 ejecuta 60/60 soluciones, diez familias de seis y catálogo exacto `1..=3100`.
- `cd web && cargo test`: PASS, 140/140.
- `make test`: PASS.
- `make harness-journeys`: PASS; stack API, stack Trunk y Playwright 10/10.
- OpenSpec strict y `git diff --check`: PASS.
- Auditoría: 3100 entradas únicas y ordenadas, mínimo 1, máximo 3100, sin pasos superiores.

## Deuda de formato preexistente

`cd web && cargo fmt --all -- --check` continúa reportando deuda global preexistente: 17.041 líneas de diff, comenzando en `web/src/api.rs` y extendiéndose por archivos ajenos a Ola 35. No se aplica formateo global porque contaminaría el diff; las pruebas funcionales, contractuales y de integración son verdes.
