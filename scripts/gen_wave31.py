"""Generate Wave 31: deterministic resilient pipeline exercises."""


def exercise(num, slug, title, objective, solution, expected, family):
    prompt = (
        f"**{title}**\n\n{objective}\n\n"
        "**Micro-reto:** modelá el comportamiento indicado, guardá el valor final "
        "en `resultado` y mostralo."
    )
    starter = "\n".join(f"# {line}" for line in solution.splitlines()) + "\n"
    name = slug.replace("-", "_")
    pytest = (
        f"def test_{name}(capsys):\n"
        "    ns = {}\n"
        "    exec(open('solution.py', encoding='utf-8').read(), ns)\n"
        f"    assert ns['resultado'] == {expected!r}\n"
        "    assert capsys.readouterr().out.strip() == str(ns['resultado'])\n"
    )
    return {
        "num": num, "slug": slug, "title": title, "objective": objective,
        "prompt": prompt, "starter": starter, "pytest": pytest,
        "hint": f"El resultado esperado es {expected!r}.", "solution": solution,
        "family": family,
    }


CASES = [
    # 1. Structured validation
    ("validar-requeridos", "validación · campos requeridos", "Detectar campos obligatorios ausentes.", "fila = {'id': 7}\nrequeridos = ['id', 'monto']\nresultado = [campo for campo in requeridos if campo not in fila]\nprint(resultado)", ["monto"], "validation"),
    ("validar-tipos", "validación · tipos", "Validar tipos con reglas declarativas.", "fila = {'id': 7, 'activo': 'sí'}\nreglas = {'id': int, 'activo': bool}\nresultado = [campo for campo, tipo in reglas.items() if not isinstance(fila.get(campo), tipo)]\nprint(resultado)", ["activo"], "validation"),
    ("validar-rango", "validación · rango", "Devolver un código estable para un valor fuera de rango.", "def validar_puntaje(valor):\n    return ('ok', valor) if 0 <= valor <= 100 else ('error', 'fuera_de_rango')\nresultado = validar_puntaje(120)\nprint(resultado)", ("error", "fuera_de_rango"), "validation"),
    ("validar-normalizar", "validación · normalizar", "Normalizar una entrada antes de comprobarla.", "def email_valido(texto):\n    limpio = texto.strip().lower()\n    return ('ok', limpio) if '@' in limpio else ('error', 'email_invalido')\nresultado = email_valido('  A@B.COM ')\nprint(resultado)", ("ok", "a@b.com"), "validation"),
    ("validar-reglas", "validación · múltiples reglas", "Evaluar todas las reglas sin cortar en el primer error.", "fila = {'nombre': '', 'edad': -2}\nerrores = []\nif not fila['nombre']: errores.append('nombre_vacio')\nif fila['edad'] < 0: errores.append('edad_negativa')\nresultado = errores\nprint(resultado)", ["nombre_vacio", "edad_negativa"], "validation"),
    ("validar-suite", "validación · suite", "Separar registros válidos y errores estructurados.", "filas = [{'id': 1, 'monto': 5}, {'id': 2}, {'id': 'x', 'monto': 3}]\nvalidos, errores = [], []\nfor indice, fila in enumerate(filas):\n    fallos = []\n    if not isinstance(fila.get('id'), int): fallos.append('id')\n    if not isinstance(fila.get('monto'), int): fallos.append('monto')\n    (errores if fallos else validos).append((indice, fallos) if fallos else fila)\nresultado = (validos, errores)\nprint(resultado)", ([{"id": 1, "monto": 5}], [(1, ["monto"]), (2, ["id"])]), "validation"),

    # 2. Errors as values and accumulation
    ("resultado-parsear", "resultados · parsear", "Representar parseo exitoso o fallido como dato.", "def parsear(texto):\n    try:\n        return ('ok', int(texto))\n    except ValueError:\n        return ('error', 'no_entero')\nresultado = parsear('abc')\nprint(resultado)", ("error", "no_entero"), "errors"),
    ("resultado-mapear", "resultados · map", "Transformar solo un resultado exitoso.", "def map_ok(res, f):\n    return ('ok', f(res[1])) if res[0] == 'ok' else res\nresultado = map_ok(('ok', 4), lambda n: n * 3)\nprint(resultado)", ("ok", 12), "errors"),
    ("resultado-encadenar", "resultados · bind", "Encadenar validaciones que también pueden fallar.", "def bind(res, f):\n    return f(res[1]) if res[0] == 'ok' else res\ndef positivo(n):\n    return ('ok', n) if n > 0 else ('error', 'no_positivo')\nresultado = bind(('ok', -3), positivo)\nprint(resultado)", ("error", "no_positivo"), "errors"),
    ("errores-acumular", "resultados · acumular", "Acumular todos los errores de un lote.", "resultados = [('ok', 3), ('error', 'tipo'), ('error', 'rango')]\nresultado = [detalle for estado, detalle in resultados if estado == 'error']\nprint(resultado)", ["tipo", "rango"], "errors"),
    ("errores-indexar", "resultados · contexto", "Adjuntar el índice de entrada a cada error.", "valores = ['4', 'x', '7', '?']\nresultado = []\nfor indice, texto in enumerate(valores):\n    try: int(texto)\n    except ValueError: resultado.append({'indice': indice, 'codigo': 'no_entero'})\nprint(resultado)", [{"indice": 1, "codigo": "no_entero"}, {"indice": 3, "codigo": "no_entero"}], "errors"),
    ("errores-suite", "resultados · suite", "Producir valores, errores y un resumen reproducible.", "entradas = ['2', 'mal', '-1', '5']\nvalores, errores = [], []\nfor indice, texto in enumerate(entradas):\n    try:\n        n = int(texto)\n        if n < 0: raise ValueError('negativo')\n        valores.append(n)\n    except ValueError as exc:\n        errores.append((indice, str(exc)))\nresultado = {'valores': valores, 'errores': errores, 'ok': not errores}\nprint(resultado)", {"valores": [2, 5], "errores": [(1, "invalid literal for int() with base 10: 'mal'"), (2, "negativo")], "ok": False}, "errors"),

    # 3. Quarantine / dead-letter modeling
    ("cuarentena-separar", "cuarentena · separar", "Separar registros aceptados y rechazados.", "filas = [3, -1, 5, -2]\nresultado = ([n for n in filas if n >= 0], [n for n in filas if n < 0])\nprint(resultado)", ([3, 5], [-1, -2]), "quarantine"),
    ("cuarentena-razon", "cuarentena · razón", "Guardar el registro rechazado junto con su razón.", "filas = [{'id': 1}, {'valor': 3}]\nresultado = [(fila, 'sin_id') for fila in filas if 'id' not in fila]\nprint(resultado)", [({"valor": 3}, "sin_id")], "quarantine"),
    ("cuarentena-orden", "cuarentena · orden", "Preservar el orden de llegada en la salida lateral.", "filas = [('a', True), ('b', False), ('c', False)]\nresultado = [clave for clave, valido in filas if not valido]\nprint(resultado)", ["b", "c"], "quarantine"),
    ("cuarentena-reprocesar", "cuarentena · reprocesar", "Corregir y reevaluar un registro aislado.", "cuarentena = [{'id': '7', 'razon': 'tipo_id'}]\nreprocesados = []\nfor item in cuarentena:\n    corregido = {'id': int(item['id'])}\n    reprocesados.append(corregido)\nresultado = reprocesados\nprint(resultado)", [{"id": 7}], "quarantine"),
    ("cuarentena-metricas", "cuarentena · métricas", "Contar rechazos por código estable.", "rechazos = [('a', 'tipo'), ('b', 'rango'), ('c', 'tipo')]\nconteos = {}\nfor _, razon in rechazos: conteos[razon] = conteos.get(razon, 0) + 1\nresultado = dict(sorted(conteos.items()))\nprint(resultado)", {"rango": 1, "tipo": 2}, "quarantine"),
    ("cuarentena-suite", "cuarentena · suite", "Validar un lote sin perder entradas sanas ni diagnóstico.", "filas = [{'id': 1, 'monto': 3}, {'id': 2, 'monto': -1}, {'monto': 4}]\naceptados, rechazo = [], []\nfor fila in filas:\n    razon = 'sin_id' if 'id' not in fila else ('monto_negativo' if fila['monto'] < 0 else None)\n    (rechazo if razon else aceptados).append((fila, razon) if razon else fila)\nresultado = {'aceptados': aceptados, 'cuarentena': rechazo}\nprint(resultado)", {"aceptados": [{"id": 1, "monto": 3}], "cuarentena": [({"id": 2, "monto": -1}, "monto_negativo"), ({"monto": 4}, "sin_id")]}, "quarantine"),

    # 4. Pure retry planning
    ("retry-intentos", "reintentos · intentos", "Construir números de intento acotados.", "max_intentos = 4\nresultado = list(range(1, max_intentos + 1))\nprint(resultado)", [1, 2, 3, 4], "retry"),
    ("retry-backoff", "reintentos · backoff", "Calcular ticks de espera exponenciales sin dormir.", "base = 2\nresultado = [base * (2 ** intento) for intento in range(4)]\nprint(resultado)", [2, 4, 8, 16], "retry"),
    ("retry-exito", "reintentos · corte por éxito", "Detener el plan en el primer resultado exitoso.", "respuestas = ['error', 'error', 'ok', 'ok']\nusados = []\nfor intento, estado in enumerate(respuestas, 1):\n    usados.append(intento)\n    if estado == 'ok': break\nresultado = usados\nprint(resultado)", [1, 2, 3], "retry"),
    ("retry-agotado", "reintentos · agotado", "Representar el agotamiento como estado terminal.", "respuestas = ['error'] * 3\nresultado = ('agotado', len(respuestas)) if 'ok' not in respuestas else ('ok', respuestas.index('ok') + 1)\nprint(resultado)", ("agotado", 3), "retry"),
    ("retry-codigos", "reintentos · códigos", "Reintentar solo códigos declarados recuperables.", "recuperables = {'timeout', 'ocupado'}\ncodigos = ['timeout', 'ocupado', 'invalido']\nresultado = [codigo in recuperables for codigo in codigos]\nprint(resultado)", [True, True, False], "retry"),
    ("retry-suite", "reintentos · suite", "Simular un plan acotado con ticks y salida terminal.", "respuestas = [('timeout', None), ('ocupado', None), ('ok', 42)]\nplan = [0, 2, 6]\nhistorial = []\nresultado = None\nfor intento, ((estado, valor), tick) in enumerate(zip(respuestas, plan), 1):\n    historial.append((intento, tick, estado))\n    if estado == 'ok':\n        resultado = {'estado': 'ok', 'valor': valor, 'historial': historial}; break\nif resultado is None: resultado = {'estado': 'agotado', 'historial': historial}\nprint(resultado)", {"estado": "ok", "valor": 42, "historial": [(1, 0, "timeout"), (2, 2, "ocupado"), (3, 6, "ok")]}, "retry"),

    # 5. Circuit breaker as a pure state machine
    ("circuito-fallos", "circuit breaker · umbral", "Abrir el circuito al alcanzar un umbral de fallos.", "estado = {'modo': 'cerrado', 'fallos': 0}\nfor ok in [False, False, False]:\n    estado['fallos'] = 0 if ok else estado['fallos'] + 1\n    if estado['fallos'] >= 3: estado['modo'] = 'abierto'\nresultado = estado\nprint(resultado)", {"modo": "abierto", "fallos": 3}, "circuit"),
    ("circuito-rechazar", "circuit breaker · rechazo", "Rechazar una operación mientras el circuito está abierto.", "estado = {'modo': 'abierto'}\nresultado = ('rechazado', 'circuito_abierto') if estado['modo'] == 'abierto' else ('aceptado', None)\nprint(resultado)", ("rechazado", "circuito_abierto"), "circuit"),
    ("circuito-halfopen", "circuit breaker · prueba", "Pasar a prueba cuando llega el tick de reapertura.", "estado = {'modo': 'abierto', 'reabrir_en': 8}\ntick = 8\nresultado = {**estado, 'modo': 'prueba'} if tick >= estado['reabrir_en'] else estado\nprint(resultado)", {"modo": "prueba", "reabrir_en": 8}, "circuit"),
    ("circuito-recuperar", "circuit breaker · recuperación", "Cerrar el circuito tras una prueba exitosa.", "estado = {'modo': 'prueba', 'fallos': 3}\nprueba_ok = True\nresultado = {'modo': 'cerrado', 'fallos': 0} if prueba_ok else estado\nprint(resultado)", {"modo": "cerrado", "fallos": 0}, "circuit"),
    ("circuito-reabrir", "circuit breaker · recaída", "Reabrir y programar otro tick tras fallar la prueba.", "tick = 10\nestado = {'modo': 'prueba', 'fallos': 3}\nresultado = {'modo': 'abierto', 'fallos': 4, 'reabrir_en': tick + 5}\nprint(resultado)", {"modo": "abierto", "fallos": 4, "reabrir_en": 15}, "circuit"),
    ("circuito-suite", "circuit breaker · suite", "Recorrer transiciones y conservar un historial observable.", "eventos = [(1, False), (2, False), (3, False), (8, True)]\nmodo, fallos, reabrir = 'cerrado', 0, None\nhistorial = []\nfor tick, ok in eventos:\n    if modo == 'abierto' and tick >= reabrir: modo = 'prueba'\n    if modo in ('cerrado', 'prueba'):\n        if ok: modo, fallos, reabrir = 'cerrado', 0, None\n        else:\n            fallos += 1\n            if fallos >= 3: modo, reabrir = 'abierto', tick + 5\n    historial.append((tick, modo))\nresultado = (modo, historial)\nprint(resultado)", ("cerrado", [(1, "cerrado"), (2, "cerrado"), (3, "abierto"), (8, "cerrado")]), "circuit"),

    # 6. Logical rate limiting
    ("limite-ventana", "límites · ventana fija", "Contar operaciones dentro de una ventana lógica.", "ticks = [1, 2, 4, 11]\ninicio, ancho = 0, 10\nresultado = sum(inicio <= tick < inicio + ancho for tick in ticks)\nprint(resultado)", 3, "rate"),
    ("limite-permitir", "límites · decisión", "Permitir hasta una capacidad declarada.", "capacidad = 3\nusados = 2\nresultado = usados < capacidad\nprint(resultado)", True, "rate"),
    ("bucket-consumir", "token bucket · consumir", "Consumir un token sin producir saldo negativo.", "tokens = 2\nif tokens > 0:\n    tokens -= 1; decision = 'permitido'\nelse: decision = 'limitado'\nresultado = (decision, tokens)\nprint(resultado)", ("permitido", 1), "rate"),
    ("bucket-rellenar", "token bucket · rellenar", "Rellenar por ticks respetando la capacidad.", "tokens, capacidad, ultimo_tick = 1, 5, 3\ntick, tasa = 7, 1\ntokens = min(capacidad, tokens + (tick - ultimo_tick) * tasa)\nresultado = tokens\nprint(resultado)", 5, "rate"),
    ("limite-por-clave", "límites · por clave", "Mantener presupuestos independientes por consumidor.", "capacidad = 2\nusos = {}\ndecisiones = []\nfor clave in ['a', 'b', 'a', 'a']:\n    usados = usos.get(clave, 0)\n    decisiones.append(usados < capacidad)\n    if usados < capacidad: usos[clave] = usados + 1\nresultado = decisiones\nprint(resultado)", [True, True, True, False], "rate"),
    ("limite-suite", "límites · suite", "Aplicar token bucket con ticks explícitos a una secuencia.", "tokens, capacidad, tasa, ultimo = 2, 3, 1, 0\ndecisiones = []\nfor tick in [0, 0, 0, 2, 2]:\n    tokens = min(capacidad, tokens + (tick - ultimo) * tasa); ultimo = tick\n    permitido = tokens >= 1\n    if permitido: tokens -= 1\n    decisiones.append((tick, permitido, tokens))\nresultado = decisiones\nprint(resultado)", [(0, True, 1), (0, True, 0), (0, False, 0), (2, True, 1), (2, True, 0)], "rate"),

    # 7. Event-time watermarks
    ("watermark-maximo", "watermarks · máximo visto", "Actualizar el event-time máximo observado.", "max_visto = None\nfor marca in [4, 9, 6]: max_visto = marca if max_visto is None else max(max_visto, marca)\nresultado = max_visto\nprint(resultado)", 9, "watermark"),
    ("watermark-calcular", "watermarks · cálculo", "Restar una tolerancia al máximo observado.", "max_visto, tolerancia = 12, 3\nresultado = max_visto - tolerancia\nprint(resultado)", 9, "watermark"),
    ("watermark-monotono", "watermarks · monotonicidad", "Evitar que el watermark retroceda con eventos desordenados.", "watermark = 5\nresultado = []\nfor max_visto in [8, 7, 12]:\n    watermark = max(watermark, max_visto - 2)\n    resultado.append(watermark)\nprint(resultado)", [6, 6, 10], "watermark"),
    ("watermark-clasificar", "watermarks · clasificar", "Clasificar un evento respecto del watermark actual.", "watermark = 10\nevent_time = 8\nresultado = 'tardio' if event_time < watermark else 'a_tiempo'\nprint(resultado)", "tardio", "watermark"),
    ("watermark-cerrar", "watermarks · cerrar ventana", "Cerrar ventanas cuyo final no supera el watermark.", "ventanas = [(0, 5), (5, 10), (10, 15)]\nwatermark = 10\nresultado = [ventana for ventana in ventanas if ventana[1] <= watermark]\nprint(resultado)", [(0, 5), (5, 10)], "watermark"),
    ("watermark-suite", "watermarks · suite", "Avanzar watermark y emitir cierres deterministas.", "eventos = [3, 9, 6, 14]\ntolerancia, max_visto, cerradas = 2, None, []\nfor marca in eventos:\n    max_visto = marca if max_visto is None else max(max_visto, marca)\n    watermark = max_visto - tolerancia\n    cerradas = [fin for fin in [5, 10, 15] if fin <= watermark]\nresultado = {'watermark': watermark, 'cerradas': cerradas}\nprint(resultado)", {"watermark": 12, "cerradas": [5, 10]}, "watermark"),

    # 8. Late-event handling
    ("tardios-separar", "eventos tardíos · separar", "Separar eventos según el watermark.", "eventos = [(4, 'a'), (11, 'b'), (8, 'c')]\nwatermark = 9\nresultado = ([e for e in eventos if e[0] >= watermark], [e for e in eventos if e[0] < watermark])\nprint(resultado)", ([(11, "b")], [(4, "a"), (8, "c")]), "late"),
    ("tardios-gracia", "eventos tardíos · gracia", "Aceptar eventos dentro de una gracia explícita.", "watermark, gracia = 10, 3\neventos = [6, 7, 9, 12]\nresultado = [(m, 'aceptado' if m >= watermark - gracia else 'descartado') for m in eventos]\nprint(resultado)", [(6, "descartado"), (7, "aceptado"), (9, "aceptado"), (12, "aceptado")], "late"),
    ("tardios-side-output", "eventos tardíos · salida lateral", "Enviar descartes a una salida con razón.", "eventos = [(3, 'x'), (9, 'y')]\nwatermark, gracia = 8, 2\nresultado = [(evento, 'demasiado_tardio') for evento in eventos if evento[0] < watermark - gracia]\nprint(resultado)", [((3, "x"), "demasiado_tardio")], "late"),
    ("tardios-corregir", "eventos tardíos · corrección", "Aplicar una corrección explícita a un agregado emitido.", "emitido = {'a': 5}\nevento_tardio = ('a', 3)\ncorregido = emitido.copy(); corregido[evento_tardio[0]] += evento_tardio[1]\nresultado = (emitido, corregido)\nprint(resultado)", ({"a": 5}, {"a": 8}), "late"),
    ("tardios-metricas", "eventos tardíos · métricas", "Contar aceptados, tardíos y descartados.", "clases = ['aceptado', 'tardio', 'aceptado', 'descartado']\nresultado = {clase: clases.count(clase) for clase in sorted(set(clases))}\nprint(resultado)", {"aceptado": 2, "descartado": 1, "tardio": 1}, "late"),
    ("tardios-suite", "eventos tardíos · suite", "Clasificar con watermark y gracia preservando el orden.", "eventos = [(5, 'a'), (12, 'b'), (8, 'c'), (3, 'd')]\nwatermark, gracia = 10, 3\nsalidas = {'a_tiempo': [], 'tardio_aceptado': [], 'descartado': []}\nfor evento in eventos:\n    marca = evento[0]\n    clave = 'a_tiempo' if marca >= watermark else ('tardio_aceptado' if marca >= watermark - gracia else 'descartado')\n    salidas[clave].append(evento)\nresultado = salidas\nprint(resultado)", {"a_tiempo": [(12, "b")], "tardio_aceptado": [(8, "c")], "descartado": [(5, "a"), (3, "d")]}, "late"),

    # 9. Compensation / saga modeling
    ("saga-plan", "compensación · plan", "Asociar cada acción con su compensación.", "pasos = [('reservar', 'liberar'), ('cobrar', 'reembolsar')]\nresultado = pasos\nprint(resultado)", [("reservar", "liberar"), ("cobrar", "reembolsar")], "saga"),
    ("saga-reversa", "compensación · orden inverso", "Ejecutar compensaciones en orden inverso.", "completados = [('reservar', 'liberar'), ('cobrar', 'reembolsar')]\nresultado = [deshacer for _, deshacer in reversed(completados)]\nprint(resultado)", ["reembolsar", "liberar"], "saga"),
    ("saga-detener", "compensación · detener", "Detener el plan ante el primer fallo.", "resultados = [('reservar', True), ('cobrar', False), ('notificar', True)]\ncompletados = []\nfor nombre, ok in resultados:\n    if not ok: break\n    completados.append(nombre)\nresultado = completados\nprint(resultado)", ["reservar"], "saga"),
    ("saga-compensar", "compensación · recuperar", "Construir compensaciones solo para pasos completados.", "plan = [('reservar', 'liberar', True), ('cobrar', 'reembolsar', False)]\ncompletados = []\nfor hacer, deshacer, ok in plan:\n    if not ok: break\n    completados.append((hacer, deshacer))\nresultado = [deshacer for _, deshacer in reversed(completados)]\nprint(resultado)", ["liberar"], "saga"),
    ("saga-idempotente", "compensación · idempotencia", "Evitar compensar dos veces el mismo paso.", "solicitadas = ['reembolsar', 'liberar', 'reembolsar']\nvistas, aplicadas = set(), []\nfor accion in solicitadas:\n    if accion not in vistas: vistas.add(accion); aplicadas.append(accion)\nresultado = aplicadas\nprint(resultado)", ["reembolsar", "liberar"], "saga"),
    ("saga-suite", "compensación · suite", "Modelar ejecución, fallo y recuperación completa como datos.", "plan = [('reservar', 'liberar', True), ('cobrar', 'reembolsar', True), ('enviar', 'cancelar_envio', False)]\ncompletados, fallo = [], None\nfor hacer, deshacer, ok in plan:\n    if not ok: fallo = hacer; break\n    completados.append((hacer, deshacer))\nresultado = {'fallo': fallo, 'completados': [h for h, _ in completados], 'compensaciones': [d for _, d in reversed(completados)]}\nprint(resultado)", {"fallo": "enviar", "completados": ["reservar", "cobrar"], "compensaciones": ["reembolsar", "liberar"]}, "saga"),

    # 10. Resilient ingestion capstone
    ("resiliente-validar", "capstone · validar", "Validar eventos antes del pipeline resiliente.", "eventos = [{'id': 'e1', 'tick': 3}, {'tick': 4}]\nresultado = [evento for evento in eventos if 'id' in evento and isinstance(evento.get('tick'), int)]\nprint(resultado)", [{"id": "e1", "tick": 3}], "capstone"),
    ("resiliente-aislar", "capstone · aislar", "Aislar entradas inválidas sin detener el lote.", "eventos = [('e1', 4), ('e2', -1), ('e3', 7)]\nresultado = {'ok': [e for e in eventos if e[1] >= 0], 'cuarentena': [e for e in eventos if e[1] < 0]}\nprint(resultado)", {"ok": [("e1", 4), ("e3", 7)], "cuarentena": [("e2", -1)]}, "capstone"),
    ("resiliente-reintentar", "capstone · reintentar", "Aplicar un plan de respuestas acotado.", "respuestas = ['timeout', 'ok']\nresultado = next((('ok', i) for i, estado in enumerate(respuestas, 1) if estado == 'ok'), ('agotado', len(respuestas)))\nprint(resultado)", ("ok", 2), "capstone"),
    ("resiliente-watermark", "capstone · watermark", "Clasificar eventos con tiempo lógico explícito.", "eventos = [(4, 'a'), (9, 'b'), (6, 'c')]\nwatermark = max(m for m, _ in eventos) - 2\nresultado = {'watermark': watermark, 'tardios': [e for e in eventos if e[0] < watermark]}\nprint(resultado)", {"watermark": 7, "tardios": [(4, "a"), (6, "c")]}, "capstone"),
    ("resiliente-recuperar", "capstone · recuperar", "Crear compensaciones ante un paso terminal fallido.", "pasos = [('validar', None, True), ('guardar', 'retirar', True), ('publicar', 'despublicar', False)]\nhechos = []\nfor hacer, deshacer, ok in pasos:\n    if not ok: break\n    hechos.append((hacer, deshacer))\nresultado = [d for _, d in reversed(hechos) if d]\nprint(resultado)", ["retirar"], "capstone"),
    ("ola31-suite", "ola 31 · suite", "Cerrar la ola con validación, cuarentena, retry y recuperación observables.", "entradas = [{'id': 'e1', 'valor': 4}, {'id': 'e2', 'valor': -1}, {'valor': 8}]\naceptados, cuarentena = [], []\nfor fila in entradas:\n    razon = 'sin_id' if 'id' not in fila else ('negativo' if fila['valor'] < 0 else None)\n    (cuarentena if razon else aceptados).append((fila, razon) if razon else fila)\nrespuestas = ['timeout', 'ok']\nintentos = next(i for i, estado in enumerate(respuestas, 1) if estado == 'ok')\nresultado = {'aceptados': aceptados, 'cuarentena': cuarentena, 'retry': ('ok', intentos), 'estado': 'recuperado'}\nprint(resultado)", {"aceptados": [{"id": "e1", "valor": 4}], "cuarentena": [({"id": "e2", "valor": -1}, "negativo"), ({"valor": 8}, "sin_id")], "retry": ("ok", 2), "estado": "recuperado"}, "capstone"),
]

RAW = CASES


def build_raw(entries):
    assert len(entries) == 60
    return [exercise(2801 + index, *case) for index, case in enumerate(entries)]


def _rust_escape(value):
    return value.replace("\\", "\\\\").replace("\n", "\\n").replace('"', '\\"')


def emit_rust(steps):
    blocks = []
    for index, item in enumerate(steps):
        const = f"PY{item['num']}_{item['slug'].upper().replace('-', '_')}"
        next_value = (
            f'Some("py-{steps[index + 1]["num"]}-{steps[index + 1]["slug"]}")'
            if index + 1 < len(steps) else "None"
        )
        blocks.append(
            f"pub const {const}: CodingStep = CodingStep {{\n"
            f"    id: \"py-{item['num']}-{item['slug']}\", title: \"{_rust_escape(item['title'])}\", objective: \"{_rust_escape(item['objective'])}\",\n"
            f"    prompt_md: \"{_rust_escape(item['prompt'])}\",\n"
            f"    starter_code: \"{_rust_escape(item['starter'])}\",\n"
            f"    pytest: \"{_rust_escape(item['pytest'])}\",\n"
            f"    hint: \"{_rust_escape(item['hint'])}\",\n"
            f"    solution_example: \"{_rust_escape(item['solution'])}\",\n"
            f"    next: {next_value}, show_type_chips: false, micro_step: {item['num']},\n"
            "};"
        )
    return "\n".join(blocks)


def emit_refs(steps):
    return "\n".join(
        f"    &PY{item['num']}_{item['slug'].upper().replace('-', '_')},"
        for item in steps
    )


if __name__ == "__main__":
    print(emit_rust(build_raw(CASES)))
