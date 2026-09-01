## Context

`main` está verificado hasta 2800. La rama histórica de Ola 31 contiene sesenta copias exactas de Ola 26; su aplicador inserta placeholders, agrega solo cinco filas conceptuales y no enlaza una cadena válida, mientras su validador compila funciones pytest sin invocarlas. Véanse `proposal.md` y la especificación para el contrato corregido.

## Goals / Non-Goals

**Goals:**
- Generar sesenta ejercicios originales, reproducibles y ejecutables en Pyodide.
- Enseñar fiabilidad de pipelines mediante estados y resultados observables.
- Mantener una fuente declarativa para constantes Rust, referencias y validación.
- Probar aislamiento de fallos, recuperación y tiempo lógico sin efectos externos.

**Non-Goals:**
- Importar, fusionar o cherry-pickear la rama histórica.
- Introducir async, threads, multiprocessing, red, archivos, sleeps, reloj o azar.
- Modificar servicios Go, infraestructura o persistencia.

## Decisions

### Organizar diez familias pedagógicas de seis pasos

Las familias serán: validación de esquemas; acumulación de errores; cuarentena de registros; reintentos con plan puro; circuit breaker como máquina de estados; rate limiting con ticks explícitos; watermarks sobre event-time entero; clasificación de eventos tardíos; compensaciones tipo saga como datos; y cierre de ingesta resiliente. Cada familia progresa desde una operación aislada hasta una suite integradora.

Alternativa descartada: renumerar el generador histórico. Repite completamente Ola 26 y no agrega aprendizaje.

### Modelar errores y recuperación como valores

Las soluciones devolverán tuplas, diccionarios o listas con estados `ok/error`, códigos y datos de cuarentena. Las excepciones se capturarán solo en fronteras pedagógicas controladas, evitando que un registro inválido aborte el lote.

Alternativa descartada: depender de excepciones no capturadas. Impide observar resultados parciales y enseñar aislamiento.

### Tiempo lógico provisto por los datos

Backoff, token buckets, watermarks y lateness operarán sobre ticks enteros explícitos. No se usará `time`, `datetime`, `sleep` ni mediciones de rendimiento.

Alternativa descartada: reloj real. Produce pruebas lentas y no reproducibles en Wasm.

### Generador declarativo y aplicador transaccional por anclas exactas

El generador emitirá las 60 constantes y referencias. El aplicador transformará todos los archivos en memoria, exigirá una frontera terminal 2800 y escribirá solo después de validar todas las anclas. Una segunda aplicación debe fallar sin cambios.

Alternativa descartada: completar faltantes con contenido genérico. Puede ocultar integraciones parciales inválidas.

### Validador acumulativo con ejecución real

El validador cargará cada función pytest generada, ejecutará su solución en un directorio temporal y comprobará rango, navegación, referencias, particiones, E2E, seguridad y no duplicación contra Olas 26–30. El validador de Ola 30 aceptará el nuevo techo, conservando autoridad sobre su propio rango y frontera.

## Risks / Trade-offs

- [Confundir simulación con infraestructura real] → declarar que los estados modelan decisiones puras, no realizan I/O.
- [Reintentos infinitos] → planes acotados con máximo de intentos y resultados terminales.
- [Circuit breaker ambiguo] → transiciones explícitas y ticks de reapertura provistos.
- [Watermarks no monotónicos] → validar avance con `max` y probar entradas fuera de orden.
- [Duplicación pedagógica] → comparar firmas completas contra Olas 26–30.
- [Drift de catálogo] → validar techo único 2860 en Rust, conceptos y E2E.

## Migration Plan

1. Generar y aplicar únicamente `2801..=2860` desde el merge verificado de Ola 30.
2. Ejecutar validadores acumulativos, Rust, backend y harness journeys.
3. Abrir un PR único, esperar CI completo y solicitar autorización de merge.
4. Verificar CI, Docker, E2E y Deploy post-merge antes de Ola 32.

Rollback: revertir el squash de Ola 31 para devolver frontera, particiones y conteos a 2800.
