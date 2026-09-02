"""Generate Wave 35: deterministic operational-resilience exercises."""

from gen_wave34 import exercise, emit_refs, emit_rust


CASES = [
    # 1. Structured telemetry
    ("telemetria-normalizar", "telemetría · normalizar", "Normalizar una señal a campos canónicos.", "senal = {'servicio': 'api', 'valor': 7}\nresultado = {'servicio': senal['servicio'], 'tipo': 'contador', 'valor': senal['valor']}\nprint(resultado)", {"servicio": "api", "tipo": "contador", "valor": 7}, "telemetry"),
    ("telemetria-etiquetas", "telemetría · etiquetas", "Ordenar etiquetas para una señal reproducible.", "etiquetas = {'region': 'sur', 'servicio': 'web'}\nresultado = tuple(sorted(etiquetas.items()))\nprint(resultado)", (("region", "sur"), ("servicio", "web")), "telemetry"),
    ("telemetria-agrupar", "telemetría · agrupar", "Agrupar señales por servicio.", "senales = [('api', 3), ('web', 2), ('api', 4)]\nresultado = {s: [v for ss, v in senales if ss == s] for s in sorted({s for s, _ in senales})}\nprint(resultado)", {"api": [3, 4], "web": [2]}, "telemetry"),
    ("telemetria-filtrar", "telemetría · filtrar", "Conservar señales por encima de un umbral.", "senales = [('cpu', 40), ('cola', 8), ('cpu', 75)]\nresultado = [senal for senal in senales if senal[1] >= 50]\nprint(resultado)", [("cpu", 75)], "telemetry"),
    ("telemetria-resumir", "telemetría · resumen", "Resumir cantidad y máximo por tipo.", "senales = {'latencia': [8, 5, 11], 'errores': [0, 2]}\nresultado = {k: {'cantidad': len(v), 'maximo': max(v)} for k, v in sorted(senales.items())}\nprint(resultado)", {"errores": {"cantidad": 2, "maximo": 2}, "latencia": {"cantidad": 3, "maximo": 11}}, "telemetry"),
    ("telemetria-suite", "telemetría · suite", "Normalizar, agrupar y resumir señales.", "entrada = [('web', 'latencia', 8), ('api', 'errores', 2), ('web', 'latencia', 5)]\ngrupos = {}\nfor servicio, tipo, valor in entrada: grupos.setdefault((servicio, tipo), []).append(valor)\nresultado = {f'{s}:{t}': {'total': sum(v), 'maximo': max(v)} for (s, t), v in sorted(grupos.items())}\nprint(resultado)", {"api:errores": {"total": 2, "maximo": 2}, "web:latencia": {"total": 13, "maximo": 8}}, "telemetry"),

    # 2. Derived metrics
    ("metrica-tasa", "métricas · tasa", "Calcular eventos por tick lógico.", "eventos, ticks = 24, 6\nresultado = eventos // ticks\nprint(resultado)", 4, "metrics"),
    ("metrica-error", "métricas · error", "Calcular tasa de error en puntos base.", "errores, total = 3, 200\nresultado = errores * 10000 // total\nprint(resultado)", 150, "metrics"),
    ("metrica-percentil", "métricas · percentil", "Elegir un percentil discreto ordenado.", "valores = [9, 2, 7, 4, 12]\nordenados = sorted(valores); indice = (90 * len(ordenados) + 99) // 100 - 1\nresultado = ordenados[indice]\nprint(resultado)", 12, "metrics"),
    ("metrica-ventana", "métricas · ventana", "Calcular promedios en ventanas fijas.", "valores = [2, 4, 6, 8, 10, 12]\ntamano = 3\nresultado = [sum(valores[i:i + tamano]) // tamano for i in range(0, len(valores), tamano)]\nprint(resultado)", [4, 10], "metrics"),
    ("metrica-delta", "métricas · delta", "Derivar incrementos de un contador acumulado.", "contador = [3, 7, 8, 14]\nresultado = [b - a for a, b in zip(contador, contador[1:])]\nprint(resultado)", [4, 1, 6], "metrics"),
    ("metrica-suite", "métricas · suite", "Derivar tasa, errores y latencia máxima.", "muestras = [{'ok': True, 'latencia': 8}, {'ok': False, 'latencia': 15}, {'ok': True, 'latencia': 6}]\nresultado = {'total': len(muestras), 'errores_bp': sum(not m['ok'] for m in muestras) * 10000 // len(muestras), 'latencia_max': max(m['latencia'] for m in muestras)}\nprint(resultado)", {"total": 3, "errores_bp": 3333, "latencia_max": 15}, "metrics"),

    # 3. SLI and SLO
    ("sli-disponibilidad", "SLI · disponibilidad", "Medir respuestas satisfactorias en puntos base.", "correctas, total = 997, 1000\nresultado = correctas * 10000 // total\nprint(resultado)", 9970, "slo"),
    ("sli-latencia", "SLI · latencia", "Medir proporción dentro del objetivo de latencia.", "latencias = [80, 120, 95, 150]; limite = 120\nresultado = sum(v <= limite for v in latencias) * 10000 // len(latencias)\nprint(resultado)", 7500, "slo"),
    ("slo-cumplido", "SLO · cumplimiento", "Comparar un SLI con su objetivo.", "sli, objetivo = 9985, 9980\nresultado = sli >= objetivo\nprint(resultado)", True, "slo"),
    ("slo-por-servicio", "SLO · por servicio", "Evaluar objetivos independientes.", "slis = {'api': 9990, 'web': 9950}; objetivos = {'api': 9980, 'web': 9970}\nresultado = {s: slis[s] >= objetivos[s] for s in sorted(slis)}\nprint(resultado)", {"api": True, "web": False}, "slo"),
    ("slo-brecha", "SLO · brecha", "Calcular distancia hasta el objetivo.", "sli, objetivo = 9960, 9990\nresultado = max(0, objetivo - sli)\nprint(resultado)", 30, "slo"),
    ("slo-suite", "SLO · suite", "Calcular SLIs y clasificar objetivos.", "servicios = {'api': (998, 1000, 9950), 'web': (985, 1000, 9900)}\nresultado = {}\nfor nombre, (ok, total, objetivo) in sorted(servicios.items()):\n    sli = ok * 10000 // total; resultado[nombre] = {'sli': sli, 'cumple': sli >= objetivo}\nprint(resultado)", {"api": {"sli": 9980, "cumple": True}, "web": {"sli": 9850, "cumple": False}}, "slo"),

    # 4. Error budgets
    ("presupuesto-total", "presupuesto · total", "Calcular fallos permitidos por objetivo.", "total, objetivo_bp = 10000, 9990\nresultado = total * (10000 - objetivo_bp) // 10000\nprint(resultado)", 10, "budgets"),
    ("presupuesto-consumido", "presupuesto · consumido", "Medir presupuesto consumido.", "permitidos, fallos = 20, 7\nresultado = {'usado': fallos, 'restante': permitidos - fallos}\nprint(resultado)", {"usado": 7, "restante": 13}, "budgets"),
    ("presupuesto-agotado", "presupuesto · agotado", "Detectar presupuesto agotado.", "permitidos, fallos = 5, 6\nresultado = fallos > permitidos\nprint(resultado)", True, "budgets"),
    ("presupuesto-velocidad", "presupuesto · velocidad", "Calcular velocidad de consumo por ventana.", "consumos = [2, 3, 1, 4]\nresultado = sum(consumos) // len(consumos)\nprint(resultado)", 2, "budgets"),
    ("presupuesto-politica", "presupuesto · política", "Elegir política según restante.", "restante, total = 2, 20\nporcentaje = restante * 100 // total\nresultado = 'congelar' if porcentaje < 20 else 'continuar'\nprint(resultado)", "congelar", "budgets"),
    ("presupuesto-suite", "presupuesto · suite", "Distribuir y evaluar presupuestos por servicio.", "trafico = {'api': 8000, 'web': 2000}; objetivo = 9990; fallos = {'api': 6, 'web': 3}\nresultado = {}\nfor s in sorted(trafico):\n    permitido = trafico[s] * (10000 - objetivo) // 10000; resultado[s] = {'permitido': permitido, 'restante': permitido - fallos[s]}\nprint(resultado)", {"api": {"permitido": 8, "restante": 2}, "web": {"permitido": 2, "restante": -1}}, "budgets"),

    # 5. Deterministic anomaly detection
    ("anomalia-umbral", "anomalías · umbral", "Detectar valores sobre un límite fijo.", "valores = [4, 7, 13, 6]; limite = 10\nresultado = [v for v in valores if v > limite]\nprint(resultado)", [13], "anomalies"),
    ("anomalia-desvio", "anomalías · desvío", "Detectar desvíos respecto de una base entera.", "base, tolerancia = 20, 5; valores = [18, 26, 15, 22]\nresultado = [v for v in valores if abs(v - base) > tolerancia]\nprint(resultado)", [26], "anomalies"),
    ("anomalia-rachas", "anomalías · rachas", "Medir la mayor racha sobre umbral.", "valores = [2, 8, 9, 3, 7, 10, 11]; limite = 6\nracha = mejor = 0\nfor v in valores:\n    racha = racha + 1 if v > limite else 0; mejor = max(mejor, racha)\nresultado = mejor\nprint(resultado)", 3, "anomalies"),
    ("anomalia-cambio", "anomalías · cambio", "Detectar saltos entre muestras consecutivas.", "valores = [10, 12, 25, 27]; max_delta = 8\nresultado = [(a, b) for a, b in zip(valores, valores[1:]) if abs(b - a) > max_delta]\nprint(resultado)", [(12, 25)], "anomalies"),
    ("anomalia-por-clave", "anomalías · por clave", "Aplicar umbrales distintos por señal.", "muestras = {'cpu': 81, 'cola': 12}; limites = {'cpu': 80, 'cola': 20}\nresultado = sorted(k for k in muestras if muestras[k] > limites[k])\nprint(resultado)", ["cpu"], "anomalies"),
    ("anomalia-suite", "anomalías · suite", "Combinar umbral, racha y severidad.", "valores = [5, 12, 14, 4, 18]; limite = 10\nindices = [i for i, v in enumerate(valores) if v > limite]\nresultado = {'indices': indices, 'cantidad': len(indices), 'severidad': max(valores) - limite}\nprint(resultado)", {"indices": [1, 2, 4], "cantidad": 3, "severidad": 8}, "anomalies"),

    # 6. Queue backpressure
    ("presion-ocupacion", "backpressure · ocupación", "Calcular ocupación de cola en porcentaje.", "usado, capacidad = 7, 10\nresultado = usado * 100 // capacidad\nprint(resultado)", 70, "backpressure"),
    ("presion-estado", "backpressure · estado", "Clasificar presión por umbrales.", "ocupacion = 85\nresultado = 'critica' if ocupacion >= 90 else 'alta' if ocupacion >= 70 else 'normal'\nprint(resultado)", "alta", "backpressure"),
    ("presion-admision", "backpressure · admisión", "Limitar admisiones a la capacidad libre.", "pendientes, libres = 8, 3\nresultado = min(pendientes, libres)\nprint(resultado)", 3, "backpressure"),
    ("presion-drenaje", "backpressure · drenaje", "Simular drenaje por ticks lógicos.", "cola, drenaje = 11, 4\nresultado = [max(0, cola - drenaje * tick) for tick in range(1, 4)]\nprint(resultado)", [7, 3, 0], "backpressure"),
    ("presion-fuente", "backpressure · fuente", "Elegir fuentes que deben pausarse.", "colas = {'a': 9, 'b': 3, 'c': 7}; limite = 7\nresultado = sorted(k for k, v in colas.items() if v >= limite)\nprint(resultado)", ["a", "c"], "backpressure"),
    ("presion-suite", "backpressure · suite", "Admitir, drenar y clasificar una cola.", "capacidad, inicial, llegadas, drenaje = 10, 6, 7, 4\nadmitidos = min(llegadas, capacidad - inicial); final = max(0, inicial + admitidos - drenaje)\nresultado = {'admitidos': admitidos, 'rechazados': llegadas - admitidos, 'final': final, 'presion': final * 100 // capacidad}\nprint(resultado)", {"admitidos": 4, "rechazados": 3, "final": 6, "presion": 60}, "backpressure"),

    # 7. Load shedding
    ("descarte-prioridad", "descarte · prioridad", "Conservar solicitudes de mayor prioridad.", "solicitudes = [('a', 2), ('b', 5), ('c', 3)]; cupo = 2\nresultado = [x for x, _ in sorted(solicitudes, key=lambda item: (-item[1], item[0]))[:cupo]]\nprint(resultado)", ["b", "c"], "shedding"),
    ("descarte-costo", "descarte · costo", "Aceptar solicitudes dentro de un presupuesto.", "solicitudes = [('a', 3), ('b', 5), ('c', 2)]; presupuesto = 6\nusado, aceptadas = 0, []\nfor nombre, costo in solicitudes:\n    if usado + costo <= presupuesto: usado += costo; aceptadas.append(nombre)\nresultado = aceptadas\nprint(resultado)", ["a", "c"], "shedding"),
    ("descarte-clase", "descarte · clase", "Proteger una clase esencial.", "solicitudes = [('salud', True), ('reporte', False), ('login', True)]\nresultado = [nombre for nombre, esencial in solicitudes if esencial]\nprint(resultado)", ["salud", "login"], "shedding"),
    ("descarte-cuota", "descarte · cuota", "Aplicar cuotas independientes por cliente.", "solicitudes = ['a', 'a', 'b', 'a', 'b']; cuota = 2\nconteos, aceptadas = {}, []\nfor cliente in solicitudes:\n    if conteos.get(cliente, 0) < cuota: conteos[cliente] = conteos.get(cliente, 0) + 1; aceptadas.append(cliente)\nresultado = aceptadas\nprint(resultado)", ["a", "a", "b", "b"], "shedding"),
    ("descarte-resumen", "descarte · resumen", "Contar aceptadas y descartadas.", "decisiones = [True, False, True, False, False]\nresultado = {'aceptadas': sum(decisiones), 'descartadas': len(decisiones) - sum(decisiones)}\nprint(resultado)", {"aceptadas": 2, "descartadas": 3}, "shedding"),
    ("descarte-suite", "descarte · suite", "Aplicar prioridad, capacidad y protección esencial.", "solicitudes = [('salud', 9, True), ('lote', 3, False), ('login', 8, True), ('reporte', 5, False)]; capacidad = 3\nordenadas = sorted(solicitudes, key=lambda x: (not x[2], -x[1], x[0])); aceptadas = ordenadas[:capacidad]\nresultado = {'aceptadas': [x[0] for x in aceptadas], 'descartadas': [x[0] for x in ordenadas[capacidad:]]}\nprint(resultado)", {"aceptadas": ["salud", "login", "reporte"], "descartadas": ["lote"]}, "shedding"),

    # 8. Capacity planning
    ("capacidad-utilizacion", "capacidad · utilización", "Calcular utilización en porcentaje.", "demanda, capacidad = 72, 90\nresultado = demanda * 100 // capacidad\nprint(resultado)", 80, "capacity"),
    ("capacidad-holgura", "capacidad · holgura", "Calcular capacidad disponible.", "demanda, capacidad = 72, 90\nresultado = capacidad - demanda\nprint(resultado)", 18, "capacity"),
    ("capacidad-instancias", "capacidad · instancias", "Calcular instancias necesarias con división techo.", "demanda, por_instancia = 101, 25\nresultado = (demanda + por_instancia - 1) // por_instancia\nprint(resultado)", 5, "capacity"),
    ("capacidad-pico", "capacidad · pico", "Dimensionar según el pico observado.", "demanda = [40, 55, 70, 62]; margen = 20\nresultado = max(demanda) * (100 + margen) // 100\nprint(resultado)", 84, "capacity"),
    ("capacidad-plan", "capacidad · plan", "Proyectar capacidad para varios servicios.", "demanda = {'api': 90, 'web': 45}; unidad = {'api': 30, 'web': 20}\nresultado = {s: (demanda[s] + unidad[s] - 1) // unidad[s] for s in sorted(demanda)}\nprint(resultado)", {"api": 3, "web": 3}, "capacity"),
    ("capacidad-suite", "capacidad · suite", "Calcular pico, margen e instancias.", "muestras = [60, 85, 70]; margen = 25; unidad = 30\npico = max(muestras); objetivo = (pico * (100 + margen) + 99) // 100\nresultado = {'pico': pico, 'objetivo': objetivo, 'instancias': (objetivo + unidad - 1) // unidad}\nprint(resultado)", {"pico": 85, "objetivo": 107, "instancias": 4}, "capacity"),

    # 9. Incident response
    ("incidente-severidad", "incidentes · severidad", "Clasificar severidad por impacto.", "impacto = 82\nresultado = 'sev1' if impacto >= 80 else 'sev2' if impacto >= 50 else 'sev3'\nprint(resultado)", "sev1", "incidents"),
    ("incidente-linea", "incidentes · línea", "Ordenar eventos de una línea temporal lógica.", "eventos = [(3, 'mitigado'), (1, 'detectado'), (2, 'asignado')]\nresultado = [evento for _, evento in sorted(eventos)]\nprint(resultado)", ["detectado", "asignado", "mitigado"], "incidents"),
    ("incidente-propietario", "incidentes · propietario", "Asignar propietario por componente.", "guardias = {'api': 'ana', 'db': 'leo'}; componente = 'db'\nresultado = guardias[componente]\nprint(resultado)", "leo", "incidents"),
    ("incidente-acciones", "incidentes · acciones", "Priorizar acciones por dependencia.", "dependencias = {'mitigar': {'diagnosticar'}, 'comunicar': {'diagnosticar'}, 'cerrar': {'mitigar', 'comunicar'}}\nhechas, orden = set(), []\nwhile len(orden) < len(dependencias) + 1:\n    lista = sorted(a for a, req in dependencias.items() if a not in hechas and req <= hechas)\n    if not lista: hechas.add('diagnosticar'); orden.append('diagnosticar')\n    else: hechas.add(lista[0]); orden.append(lista[0])\nresultado = orden\nprint(resultado)", ["diagnosticar", "comunicar", "mitigar", "cerrar"], "incidents"),
    ("incidente-duracion", "incidentes · duración", "Calcular duración por ticks de inicio y cierre.", "inicio, mitigacion, cierre = 4, 9, 12\nresultado = {'hasta_mitigar': mitigacion - inicio, 'total': cierre - inicio}\nprint(resultado)", {"hasta_mitigar": 5, "total": 8}, "incidents"),
    ("incidente-suite", "incidentes · suite", "Resumir severidad, línea y tiempos.", "incidente = {'inicio': 2, 'mitigacion': 7, 'cierre': 10, 'impacto': 65}; eventos = [(7, 'mitigado'), (2, 'detectado'), (10, 'cerrado')]\nresultado = {'severidad': 'sev2' if incidente['impacto'] >= 50 else 'sev3', 'linea': [e for _, e in sorted(eventos)], 'mttr': incidente['mitigacion'] - incidente['inicio']}\nprint(resultado)", {"severidad": "sev2", "linea": ["detectado", "mitigado", "cerrado"], "mttr": 5}, "incidents"),

    # 10. Operational-resilience capstone
    ("operacion-senales", "capstone · señales", "Resumir señales para una decisión operativa.", "senales = {'errores': 4, 'total': 200, 'latencia': 130}\nresultado = {'error_bp': senales['errores'] * 10000 // senales['total'], 'latencia': senales['latencia']}\nprint(resultado)", {"error_bp": 200, "latencia": 130}, "capstone"),
    ("operacion-objetivo", "capstone · objetivo", "Evaluar señales contra objetivos.", "metricas = {'error_bp': 200, 'latencia': 130}; limites = {'error_bp': 100, 'latencia': 120}\nresultado = sorted(k for k in metricas if metricas[k] > limites[k])\nprint(resultado)", ["error_bp", "latencia"], "capstone"),
    ("operacion-presupuesto", "capstone · presupuesto", "Calcular presupuesto restante.", "permitidos, observados = 10, 7\nresultado = permitidos - observados\nprint(resultado)", 3, "capstone"),
    ("operacion-carga", "capstone · carga", "Elegir admisión según presión y prioridad.", "solicitudes = [('login', 9), ('reporte', 3), ('salud', 10)]; libres = 2\nresultado = [n for n, _ in sorted(solicitudes, key=lambda x: (-x[1], x[0]))[:libres]]\nprint(resultado)", ["salud", "login"], "capstone"),
    ("operacion-incidente", "capstone · incidente", "Abrir incidente cuando se incumplen objetivos.", "incumplidos = ['error_bp', 'latencia']; presupuesto = 3\nresultado = {'abrir': bool(incumplidos), 'severidad': 'sev1' if presupuesto <= 0 else 'sev2'}\nprint(resultado)", {"abrir": True, "severidad": "sev2"}, "capstone"),
    ("ola35-suite", "ola 35 · suite", "Cerrar la ola con señales, SLO, presupuesto, carga e incidente.", "muestras = [{'ok': True, 'latencia': 80}, {'ok': False, 'latencia': 150}, {'ok': True, 'latencia': 110}, {'ok': False, 'latencia': 140}]\ntotal = len(muestras); errores = sum(not m['ok'] for m in muestras); error_bp = errores * 10000 // total; lentas = sum(m['latencia'] > 120 for m in muestras)\npresupuesto = 1; restante = presupuesto - errores; incumplidos = [n for n, fallo in [('errores', error_bp > 1000), ('latencia', lentas > 1)] if fallo]\nresultado = {'metricas': {'error_bp': error_bp, 'lentas': lentas}, 'presupuesto': restante, 'incumplidos': incumplidos, 'accion': 'mitigar' if incumplidos else 'observar'}\nprint(resultado)", {"metricas": {"error_bp": 5000, "lentas": 2}, "presupuesto": -1, "incumplidos": ["errores", "latencia"], "accion": "mitigar"}, "capstone"),
]

RAW = CASES


def build_raw(entries=CASES):
    assert len(entries) == 60
    return [exercise(3041 + index, *case) for index, case in enumerate(entries)]


if __name__ == "__main__":
    print(emit_rust(build_raw()))
