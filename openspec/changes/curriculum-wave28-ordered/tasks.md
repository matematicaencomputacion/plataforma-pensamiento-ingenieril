## 1. Contenido reproducible

- [x] 1.1 Crear diez familias originales de seis ejercicios y verificar rango `2621..=2680`, slugs únicos y separación clara entre chunks, parciales y combinación
- [x] 1.2 Implementar el emisor Rust y verificar que produce exactamente 60 constantes y 60 referencias
- [x] 1.3 Implementar el aplicador con anclas exactas y verificar que una segunda ejecución falla sin modificar archivos

## 2. Integración del catálogo

- [x] 2.1 Enlazar 2620 con 2621, insertar Ola 28 y dejar 2680 terminal, verificando la cadena completa
- [x] 2.2 Registrar las 60 constantes en `CODING_STEPS` y verificar catálogo exacto y contiguo `1..=2680`
- [x] 2.3 Extender particiones conceptuales hasta 2680 y verificar orden, tags válidos y ausencia de referencias posteriores
- [x] 2.4 Actualizar las tres expectativas E2E a 2680 y verificar que no quedan conteos 2620 en los journeys canónicos

## 3. Validación

- [x] 3.1 Crear el validador de Ola 28 y verificar las 60 soluciones contra sus pruebas en ejecución real
- [x] 3.2 Ajustar el validador de Ola 27 como contrato acumulativo y verificar el enlace de frontera hacia Ola 28
- [x] 3.3 Agregar pruebas Rust de Ola 28 y verificar tamaño, unicidad, navegación y terminal con `cargo test`
- [x] 3.4 Verificar no duplicación pedagógica contra Olas 26 y 27 comparando slug, prompt y solución
- [x] 3.5 Verificar que no se usan threads, procesos, red ni placeholders, y que las reducciones parciales declaradas equivalen a la reducción directa
- [x] 3.6 Ejecutar `make test` y `make harness-journeys`, documentando cualquier gate dependiente del entorno

## 4. Integración secuencial

- [x] 4.1 Auditar el diff y verificar que no contiene micro-steps mayores a 2680 ni cambios fuera del alcance
- [x] 4.2 Crear commit convencional, publicar la rama y abrir un PR con la evidencia local
- [x] 4.3 Monitorear CI, Docker y E2E del PR hasta verde o bloqueo externo identificado
- [x] 4.4 Tras autorización, integrar y verificar CI, Docker, E2E y Deploy post-merge antes de Ola 29
