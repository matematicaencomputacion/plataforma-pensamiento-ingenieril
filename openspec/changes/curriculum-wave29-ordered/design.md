## Context

`main` está verificado hasta 2680. La rama histórica de Ola 29 parte de una historia acumulativa con Olas 26–30; aunque contiene material pedagógico aprovechable, su aplicador admite placeholders y su validador no ejecuta las funciones pytest. Véanse `proposal.md` y la especificación para el contrato corregido.

## Goals / Non-Goals

**Goals:**
- Generar sesenta ejercicios originales, reproducibles y ejecutables en Pyodide.
- Enseñar composición funcional y consumo incremental sin depender de I/O ni concurrencia.
- Mantener una sola fuente declarativa para constantes Rust, referencias y validación.
- Probar pereza, cortocircuito, folding y agregación mediante resultados observables.

**Non-Goals:**
- Importar la rama histórica o su aplicador tolerante.
- Introducir Rx, async, threads, multiprocessing, red o archivos externos.
- Modificar servicios Go, infraestructura o persistencia.

## Decisions

### Organizar diez familias pedagógicas de seis pasos

Las familias serán: map/filter y lambdas; callbacks y orden superior; generadores básicos; composición lazy; folding; pipelines de logs; materialización y consumo único; terminación temprana; agregación streaming; y cierre de scoring/ranking. Cada familia progresa desde una operación aislada hasta una suite integradora.

Alternativa descartada: copiar sin cambios el generador histórico. Su contenido es útil, pero su identidad de ola es incorrecta y no tiene garantías suficientes de originalidad ni ejecución.

### Hacer observable la pereza sin instrumentación externa

Los ejercicios usarán contadores locales, `next`, `islice`, `takewhile`, `any` y `all` para demostrar cuánto se consume. No medirán tiempos ni memoria, porque esos resultados dependen del runtime.

Alternativa descartada: benchmarks. Son inestables en Wasm y no prueban el contrato semántico.

### Generador declarativo y aplicador transaccional por anclas exactas

El generador emitirá las 60 constantes y referencias. El aplicador transformará todos los archivos en memoria, exigirá una frontera terminal 2680 y escribirá solo después de validar todas las anclas. Una segunda aplicación debe fallar sin cambios.

Alternativa descartada: detectar cualquier aparición de `py-2681` y completar con `Step N`; puede ocultar catálogos parciales inválidos.

### Validador acumulativo con ejecución real

El validador cargará las funciones pytest generadas, ejecutará cada solución en un directorio temporal y comprobará rango, navegación, referencias, particiones, E2E, seguridad y no duplicación contra Olas 26–28. El validador de Ola 28 pasará a aceptar el nuevo techo, conservando autoridad sobre su propio rango y frontera.

### Orden y agotamiento como parte del contrato

Las reducciones de texto conservarán el orden explícito; los iteradores de un solo uso se probarán antes y después del consumo. Las operaciones sobre entradas vacías usarán identidad o una precondición declarada.

## Risks / Trade-offs

- [Confundir lazy con más rápido] → probar consumo y semántica, no rendimiento.
- [Reutilizar un iterador agotado] → ejercicios y tests explícitos de consumo único.
- [Folding sin identidad válida] → exigir inicializador o entrada no vacía.
- [Duplicación con contenido previo] → comparar firmas completas contra Olas 26–28.
- [Drift de catálogo] → validar techo único 2740 en Rust, conceptos y E2E.
- [Rama histórica contaminada] → reconstrucción sobre `origin/main`, sin merge ni cherry-pick.

## Migration Plan

1. Generar y aplicar únicamente `2681..=2740` desde el merge verificado de Ola 28.
2. Ejecutar validadores acumulativos, Rust, backend y harness journeys.
3. Abrir un PR único, esperar CI completo y solicitar autorización de merge.
4. Verificar CI, Docker, E2E y Deploy post-merge antes de Ola 30.

Rollback: revertir el squash de Ola 29 para devolver frontera, particiones y conteos a 2680.
