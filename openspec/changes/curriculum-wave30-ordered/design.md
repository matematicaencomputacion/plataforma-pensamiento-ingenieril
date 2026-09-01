## Context

`main` está verificado hasta 2740. La rama histórica de Ola 30 acumula Olas 26–30 y sus sesenta ejercicios son copias exactas de contenido previo; además, su aplicador genera placeholders y su validador no ejecuta las pruebas. Véanse `proposal.md` y la especificación para el contrato corregido.

## Goals / Non-Goals

**Goals:**
- Generar sesenta ejercicios originales, reproducibles y ejecutables en Pyodide.
- Enseñar algoritmos incrementales con estado explícito, orden estable y resultados observables.
- Mantener una fuente declarativa para constantes Rust, referencias y validación.
- Probar ejecución real, idempotencia, checkpoints y reconciliación.

**Non-Goals:**
- Importar, fusionar o cherry-pickear la rama histórica.
- Introducir async, threads, multiprocessing, red, archivos, reloj o azar.
- Modificar servicios Go, infraestructura o persistencia.

## Decisions

### Organizar diez familias pedagógicas de seis pasos

Las familias serán: merge de secuencias ordenadas; prioridades con heap; top-k acotado; media y conteo online; varianza y extremos incrementales; ventanas con deque; checkpoint y reanudación puros; deduplicación por claves estables; reconciliación por secuencia explícita; y cierre de agregación/ranking. Cada familia progresa desde una operación aislada hasta una suite integradora.

Alternativa descartada: renumerar el generador histórico. Repite por completo material anterior y no satisface la progresión pedagógica.

### Estado explícito y determinista

Las soluciones representarán el estado acumulado con tuplas, diccionarios copiados o dataclasses inmutables cuando aporte claridad. La precedencia se decidirá por secuencias presentes en los datos, nunca por tiempo de ejecución.

Alternativa descartada: objetos globales mutables. Ocultan dependencias, dificultan reanudar y vuelven frágiles las pruebas.

### Generador declarativo y aplicador transaccional por anclas exactas

El generador emitirá las 60 constantes y referencias. El aplicador transformará todos los archivos en memoria, exigirá una frontera terminal 2740 y escribirá solo después de validar todas las anclas. Una segunda aplicación debe fallar sin cambios.

Alternativa descartada: completar faltantes con contenido genérico. Puede ocultar integraciones parciales inválidas.

### Validador acumulativo con ejecución real

El validador cargará cada función pytest generada, ejecutará su solución en un directorio temporal y comprobará rango, navegación, referencias, particiones, E2E, seguridad y no duplicación contra Olas 26–29. El validador de Ola 29 aceptará el nuevo techo, conservando autoridad sobre su propio rango y frontera.

### Propiedades observables sobre métricas de rendimiento

Los tests comprobarán orden, tamaño de heaps, equivalencia de reanudación, estabilidad de claves y resultados incrementales. No medirán tiempo ni memoria porque esas métricas son inestables en Wasm.

## Risks / Trade-offs

- [Confundir orden de llegada con precedencia] → incluir la secuencia explícita en los datos y probar empates.
- [Mutar checkpoints previos] → copiar estado y comprobar que snapshots anteriores no cambian.
- [Heaps con empates no deterministas] → usar claves totales y desempates explícitos.
- [Duplicación pedagógica] → comparar firmas completas contra Olas 26–29.
- [Drift de catálogo] → validar techo único 2800 en Rust, conceptos y E2E.
- [Rama histórica contaminada] → reconstrucción sobre `origin/main`, sin merge ni cherry-pick.

## Migration Plan

1. Generar y aplicar únicamente `2741..=2800` desde el merge verificado de Ola 29.
2. Ejecutar validadores acumulativos, Rust, backend y harness journeys.
3. Abrir un PR único, esperar CI completo y solicitar autorización de merge.
4. Verificar CI, Docker, E2E y Deploy post-merge antes de Ola 31.

Rollback: revertir el squash de Ola 30 para devolver frontera, particiones y conteos a 2740.
