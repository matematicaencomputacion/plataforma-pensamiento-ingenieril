"""Generate Wave 32: deterministic distributed aggregation exercises."""


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
    # 1. Stable partitioning
    ("particionar-paridad", "particionado · paridad", "Separar enteros en dos particiones estables.", "datos = [5, 2, 7, 4]\npartes = {0: [], 1: []}\nfor valor in datos: partes[valor % 2].append(valor)\nresultado = partes\nprint(resultado)", {0: [2, 4], 1: [5, 7]}, "partition"),
    ("particionar-rango", "particionado · rangos", "Asignar valores a rangos declarados.", "datos = [2, 8, 13, 4]\nresultado = {'bajo': [n for n in datos if n < 5], 'medio': [n for n in datos if 5 <= n < 10], 'alto': [n for n in datos if n >= 10]}\nprint(resultado)", {"bajo": [2, 4], "medio": [8], "alto": [13]}, "partition"),
    ("particionar-clave", "particionado · clave", "Agrupar registros preservando el orden por clave.", "filas = [('sur', 3), ('norte', 2), ('sur', 5)]\npartes = {}\nfor clave, valor in filas: partes.setdefault(clave, []).append(valor)\nresultado = partes\nprint(resultado)", {"sur": [3, 5], "norte": [2]}, "partition"),
    ("particionar-indice", "particionado · round robin", "Distribuir por índice sin concurrencia.", "datos = list('abcde')\npartes = [[], []]\nfor indice, valor in enumerate(datos): partes[indice % 2].append(valor)\nresultado = partes\nprint(resultado)", [["a", "c", "e"], ["b", "d"]], "partition"),
    ("particionar-vacios", "particionado · vacíos", "Conservar particiones vacías en el contrato.", "datos = [2, 4]\npartes = {i: [] for i in range(3)}\nfor valor in datos: partes[valor % 3].append(valor)\nresultado = partes\nprint(resultado)", {0: [], 1: [4], 2: [2]}, "partition"),
    ("particionar-suite", "particionado · suite", "Particionar eventos y resumir tamaños verificables.", "eventos = [('a', 4), ('b', 7), ('c', 10), ('d', 5)]\npartes = {0: [], 1: []}\nfor evento in eventos: partes[evento[1] % 2].append(evento)\nresultado = {'partes': partes, 'tamanos': [len(partes[i]) for i in range(2)]}\nprint(resultado)", {"partes": {0: [("a", 4), ("c", 10)], 1: [("b", 7), ("d", 5)]}, "tamanos": [2, 2]}, "partition"),

    # 2. Stable sharding by key
    ("shard-entero", "sharding · entero", "Calcular shard con módulo explícito.", "clave, cantidad = 17, 4\nresultado = clave % cantidad\nprint(resultado)", 1, "sharding"),
    ("shard-texto", "sharding · texto", "Derivar un shard textual estable sin hash implícito.", "clave, cantidad = 'ana', 3\nresultado = sum(ord(c) for c in clave) % cantidad\nprint(resultado)", 1, "sharding"),
    ("shard-registros", "sharding · registros", "Enrutar registros por su clave estable.", "filas = [{'id': 2}, {'id': 5}, {'id': 8}]\nresultado = [(fila['id'], fila['id'] % 3) for fila in filas]\nprint(resultado)", [(2, 2), (5, 2), (8, 2)], "sharding"),
    ("shard-consistente", "sharding · consistencia", "Comprobar que una clave repite destino.", "claves = ['sur', 'norte', 'sur']\nresultado = [sum(map(ord, clave)) % 4 for clave in claves]\nprint(resultado)", [2, 0, 2], "sharding"),
    ("shard-distribucion", "sharding · distribución", "Contar carga por shard.", "claves = [1, 2, 3, 4, 5]\nconteos = {i: 0 for i in range(3)}\nfor clave in claves: conteos[clave % 3] += 1\nresultado = conteos\nprint(resultado)", {0: 1, 1: 2, 2: 2}, "sharding"),
    ("shard-suite", "sharding · suite", "Enrutar claves y conservar un manifiesto ordenado.", "claves = ['aa', 'b', 'cc', 'd']\nshards = {i: [] for i in range(3)}\nfor clave in claves: shards[sum(map(ord, clave)) % 3].append(clave)\nresultado = [(i, shards[i]) for i in sorted(shards)]\nprint(resultado)", [(0, ["cc"]), (1, ["d"]), (2, ["aa", "b"])], "sharding"),

    # 3. Fan-out
    ("fanout-replicar", "fan-out · replicar", "Replicar una tarea a destinos declarados.", "tarea = ('t1', 7)\ndestinos = ['a', 'b', 'c']\nresultado = [(destino, tarea) for destino in destinos]\nprint(resultado)", [("a", ("t1", 7)), ("b", ("t1", 7)), ("c", ("t1", 7))], "fanout"),
    ("fanout-segmentar", "fan-out · segmentar", "Crear tareas por segmentos contiguos.", "datos = [1, 2, 3, 4, 5]\ntamano = 2\nresultado = [datos[i:i + tamano] for i in range(0, len(datos), tamano)]\nprint(resultado)", [[1, 2], [3, 4], [5]], "fanout"),
    ("fanout-etiquetar", "fan-out · etiquetar", "Etiquetar cada tarea con un identificador estable.", "lotes = [['a'], ['b', 'c']]\nresultado = [{'tarea': i, 'datos': lote} for i, lote in enumerate(lotes)]\nprint(resultado)", [{"tarea": 0, "datos": ["a"]}, {"tarea": 1, "datos": ["b", "c"]}], "fanout"),
    ("fanout-filtrar", "fan-out · seleccionar", "Crear tareas solo para destinos habilitados.", "destinos = [('a', True), ('b', False), ('c', True)]\nresultado = [nombre for nombre, activo in destinos if activo]\nprint(resultado)", ["a", "c"], "fanout"),
    ("fanout-costo", "fan-out · costo", "Calcular costo lógico por tarea antes de distribuir.", "lotes = [[1, 2], [3], [4, 5, 6]]\nresultado = [(i, len(lote)) for i, lote in enumerate(lotes)]\nprint(resultado)", [(0, 2), (1, 1), (2, 3)], "fanout"),
    ("fanout-suite", "fan-out · suite", "Segmentar, etiquetar y asignar tareas a workers lógicos.", "datos = list(range(7))\nlotes = [datos[i:i + 3] for i in range(0, len(datos), 3)]\nresultado = [{'tarea': i, 'worker': i % 2, 'datos': lote} for i, lote in enumerate(lotes)]\nprint(resultado)", [{"tarea": 0, "worker": 0, "datos": [0, 1, 2]}, {"tarea": 1, "worker": 1, "datos": [3, 4, 5]}, {"tarea": 2, "worker": 0, "datos": [6]}], "fanout"),

    # 4. Ordered fan-in
    ("fanin-ordenar", "fan-in · ordenar", "Reunir respuestas por identificador de tarea.", "respuestas = [(2, 'c'), (0, 'a'), (1, 'b')]\nresultado = [valor for _, valor in sorted(respuestas)]\nprint(resultado)", ["a", "b", "c"], "fanin"),
    ("fanin-faltantes", "fan-in · faltantes", "Detectar tareas que aún no respondieron.", "esperadas = {0, 1, 2, 3}\nrecibidas = {0, 2}\nresultado = sorted(esperadas - recibidas)\nprint(resultado)", [1, 3], "fanin"),
    ("fanin-duplicados", "fan-in · duplicados", "Aceptar una sola respuesta por tarea.", "respuestas = [(0, 'a'), (1, 'b'), (0, 'otra')]\npor_tarea = {}\nfor tarea, valor in respuestas: por_tarea.setdefault(tarea, valor)\nresultado = por_tarea\nprint(resultado)", {0: "a", 1: "b"}, "fanin"),
    ("fanin-errores", "fan-in · errores", "Separar respuestas exitosas y fallidas.", "respuestas = [(0, 'ok', 4), (1, 'error', 'timeout')]\nresultado = {'ok': [(i, v) for i, e, v in respuestas if e == 'ok'], 'error': [(i, v) for i, e, v in respuestas if e == 'error']}\nprint(resultado)", {"ok": [(0, 4)], "error": [(1, "timeout")]}, "fanin"),
    ("fanin-completo", "fan-in · completo", "Decidir si llegó el conjunto completo.", "esperadas = 3\nrecibidas = {0: 'a', 2: 'c'}\nresultado = len(recibidas) == esperadas\nprint(resultado)", False, "fanin"),
    ("fanin-suite", "fan-in · suite", "Deduplicar, ordenar y resumir respuestas.", "respuestas = [(2, 5), (0, 3), (2, 9), (1, 4)]\nprimeras = {}\nfor tarea, valor in respuestas: primeras.setdefault(tarea, valor)\nordenadas = [primeras[i] for i in sorted(primeras)]\nresultado = {'valores': ordenadas, 'total': sum(ordenadas), 'completo': set(primeras) == {0, 1, 2}}\nprint(resultado)", {"valores": [3, 4, 5], "total": 12, "completo": True}, "fanin"),

    # 5. Map-reduce
    ("mapreduce-map", "map-reduce · map", "Emitir pares clave-valor desde palabras.", "palabras = ['sol', 'mar', 'sol']\nresultado = [(palabra, 1) for palabra in palabras]\nprint(resultado)", [("sol", 1), ("mar", 1), ("sol", 1)], "mapreduce"),
    ("mapreduce-shuffle", "map-reduce · shuffle", "Agrupar valores emitidos por clave.", "pares = [('a', 1), ('b', 2), ('a', 3)]\ngrupo = {}\nfor clave, valor in pares: grupo.setdefault(clave, []).append(valor)\nresultado = grupo\nprint(resultado)", {"a": [1, 3], "b": [2]}, "mapreduce"),
    ("mapreduce-reduce", "map-reduce · reduce", "Reducir grupos mediante suma.", "grupos = {'a': [1, 3], 'b': [2]}\nresultado = {clave: sum(grupos[clave]) for clave in sorted(grupos)}\nprint(resultado)", {"a": 4, "b": 2}, "mapreduce"),
    ("mapreduce-conteo", "map-reduce · conteo", "Contar palabras con map y reduce explícitos.", "palabras = 'rojo azul rojo'.split()\nconteos = {}\nfor palabra in palabras: conteos[palabra] = conteos.get(palabra, 0) + 1\nresultado = dict(sorted(conteos.items()))\nprint(resultado)", {"azul": 1, "rojo": 2}, "mapreduce"),
    ("mapreduce-maximo", "map-reduce · máximo", "Reducir mediciones al máximo por clave.", "medidas = [('x', 3), ('y', 8), ('x', 5)]\nmaximos = {}\nfor clave, valor in medidas: maximos[clave] = max(valor, maximos.get(clave, valor))\nresultado = maximos\nprint(resultado)", {"x": 5, "y": 8}, "mapreduce"),
    ("mapreduce-suite", "map-reduce · suite", "Ejecutar map, shuffle y reduce sobre ventas.", "ventas = [('sur', 4), ('norte', 3), ('sur', 6)]\nemitidos = [(region, monto) for region, monto in ventas]\ngrupo = {}\nfor clave, valor in emitidos: grupo.setdefault(clave, []).append(valor)\nresultado = [(clave, sum(grupo[clave])) for clave in sorted(grupo)]\nprint(resultado)", [("norte", 3), ("sur", 10)], "mapreduce"),

    # 6. Partial aggregates
    ("parcial-suma", "agregados parciales · suma", "Combinar sumas parciales.", "parciales = [7, 4, 9]\nresultado = sum(parciales)\nprint(resultado)", 20, "partials"),
    ("parcial-conteo", "agregados parciales · conteo", "Combinar conteos por clave.", "parciales = [{'a': 2, 'b': 1}, {'a': 3}]\ntotal = {}\nfor parcial in parciales:\n    for clave, valor in parcial.items(): total[clave] = total.get(clave, 0) + valor\nresultado = total\nprint(resultado)", {"a": 5, "b": 1}, "partials"),
    ("parcial-promedio", "agregados parciales · promedio", "Combinar suma y cantidad sin promediar promedios.", "parciales = [(10, 2), (9, 3)]\nsuma = sum(p[0] for p in parciales); cantidad = sum(p[1] for p in parciales)\nresultado = suma / cantidad\nprint(resultado)", 3.8, "partials"),
    ("parcial-minmax", "agregados parciales · extremos", "Combinar mínimos y máximos parciales.", "parciales = [(2, 8), (1, 7), (3, 10)]\nresultado = (min(p[0] for p in parciales), max(p[1] for p in parciales))\nprint(resultado)", (1, 10), "partials"),
    ("parcial-vacio", "agregados parciales · vacío", "Usar identidad al combinar un conjunto vacío.", "parciales = []\nresultado = sum(parciales, 0)\nprint(resultado)", 0, "partials"),
    ("parcial-suite", "agregados parciales · suite", "Combinar estados parciales por región.", "parciales = [{'sur': (8, 2), 'norte': (3, 1)}, {'sur': (7, 1)}]\ntotal = {}\nfor parcial in parciales:\n    for clave, (suma, cantidad) in parcial.items():\n        anterior = total.get(clave, (0, 0)); total[clave] = (anterior[0] + suma, anterior[1] + cantidad)\nresultado = {clave: (suma, cantidad, suma / cantidad) for clave, (suma, cantidad) in sorted(total.items())}\nprint(resultado)", {"norte": (3, 1, 3.0), "sur": (15, 3, 5.0)}, "partials"),

    # 7. Logical windows
    ("ventana-asignar", "ventanas · asignar", "Asignar ticks a ventanas fijas.", "ticks = [1, 4, 5, 9]\nancho = 5\nresultado = [(tick, (tick // ancho) * ancho) for tick in ticks]\nprint(resultado)", [(1, 0), (4, 0), (5, 5), (9, 5)], "windows"),
    ("ventana-agrupar", "ventanas · agrupar", "Agrupar valores por inicio de ventana.", "eventos = [(1, 2), (4, 3), (7, 5)]\nventanas = {}\nfor tick, valor in eventos: ventanas.setdefault((tick // 5) * 5, []).append(valor)\nresultado = ventanas\nprint(resultado)", {0: [2, 3], 5: [5]}, "windows"),
    ("ventana-sumar", "ventanas · sumar", "Agregar valores dentro de cada ventana.", "ventanas = {0: [2, 3], 5: [5]}\nresultado = {inicio: sum(valores) for inicio, valores in ventanas.items()}\nprint(resultado)", {0: 5, 5: 5}, "windows"),
    ("ventana-cerrar", "ventanas · cerrar", "Cerrar ventanas según un watermark lógico.", "inicios = [0, 5, 10]\nancho, watermark = 5, 10\nresultado = [inicio for inicio in inicios if inicio + ancho <= watermark]\nprint(resultado)", [0, 5], "windows"),
    ("ventana-particiones", "ventanas · particiones", "Combinar ventanas parciales de dos shards.", "parciales = [{0: 4, 5: 2}, {0: 3, 5: 8}]\ntotal = {}\nfor parcial in parciales:\n    for ventana, valor in parcial.items(): total[ventana] = total.get(ventana, 0) + valor\nresultado = total\nprint(resultado)", {0: 7, 5: 10}, "windows"),
    ("ventana-suite", "ventanas · suite", "Particionar, agregar y emitir ventanas cerradas.", "eventos = [(1, 'a', 2), (6, 'a', 4), (3, 'b', 5), (8, 'a', 1)]\ntotal = {}\nfor tick, clave, valor in eventos:\n    grupo = ((tick // 5) * 5, clave); total[grupo] = total.get(grupo, 0) + valor\nwatermark = 10\nresultado = [(grupo, total[grupo]) for grupo in sorted(total) if grupo[0] + 5 <= watermark]\nprint(resultado)", [((0, "a"), 2), ((0, "b"), 5), ((5, "a"), 5)], "windows"),

    # 8. Skew detection and mitigation
    ("skew-cargas", "skew · cargas", "Medir carga por shard.", "asignaciones = [0, 0, 0, 1, 2]\nresultado = {i: asignaciones.count(i) for i in range(3)}\nprint(resultado)", {0: 3, 1: 1, 2: 1}, "skew"),
    ("skew-detectar", "skew · detectar", "Detectar shards sobre un umbral.", "cargas = {0: 7, 1: 2, 2: 3}\numbral = 5\nresultado = [shard for shard, carga in sorted(cargas.items()) if carga > umbral]\nprint(resultado)", [0], "skew"),
    ("skew-ratio", "skew · razón", "Calcular la razón entre máxima carga y promedio.", "cargas = [6, 2, 4]\nresultado = max(cargas) / (sum(cargas) / len(cargas))\nprint(resultado)", 1.5, "skew"),
    ("skew-salting", "skew · salting", "Distribuir una clave caliente con sufijos estables.", "eventos = [('hot', i) for i in range(5)]\nresultado = [(clave + ':' + str(indice % 2), valor) for indice, (clave, valor) in enumerate(eventos)]\nprint(resultado)", [("hot:0", 0), ("hot:1", 1), ("hot:0", 2), ("hot:1", 3), ("hot:0", 4)], "skew"),
    ("skew-desalar", "skew · combinar sales", "Recombinar parciales de una clave salteada.", "parciales = {'hot:0': 8, 'hot:1': 5, 'cold:0': 2}\ntotal = {}\nfor clave, valor in parciales.items():\n    base = clave.split(':')[0]; total[base] = total.get(base, 0) + valor\nresultado = total\nprint(resultado)", {"hot": 13, "cold": 2}, "skew"),
    ("skew-suite", "skew · suite", "Detectar clave caliente, salar y recombinar su agregado.", "eventos = [('hot', 2), ('hot', 3), ('cold', 4), ('hot', 5)]\nconteos = {clave: sum(1 for k, _ in eventos if k == clave) for clave in sorted({k for k, _ in eventos})}\ncaliente = max(conteos, key=conteos.get)\nparciales = {}\nfor indice, (clave, valor) in enumerate(eventos):\n    efectiva = clave + ':' + str(indice % 2) if clave == caliente else clave + ':0'; parciales[efectiva] = parciales.get(efectiva, 0) + valor\ntotal = {}\nfor clave, valor in parciales.items(): total[clave.split(':')[0]] = total.get(clave.split(':')[0], 0) + valor\nresultado = {'caliente': caliente, 'total': total}\nprint(resultado)", {"caliente": "hot", "total": {"hot": 10, "cold": 4}}, "skew"),

    # 9. Rebalancing as data
    ("rebalance-diferencia", "rebalanceo · diferencia", "Calcular diferencia de carga entre shards.", "cargas = {0: 6, 1: 2}\nresultado = max(cargas.values()) - min(cargas.values())\nprint(resultado)", 4, "rebalance"),
    ("rebalance-origen-destino", "rebalanceo · extremos", "Elegir origen y destino deterministas.", "cargas = {0: 6, 1: 2, 2: 4}\nresultado = (max(cargas, key=lambda k: (cargas[k], -k)), min(cargas, key=lambda k: (cargas[k], k)))\nprint(resultado)", (0, 1), "rebalance"),
    ("rebalance-mover", "rebalanceo · mover", "Representar un movimiento sin ejecutarlo.", "partes = {0: ['a', 'b', 'c'], 1: ['d']}\nresultado = {'clave': partes[0][-1], 'desde': 0, 'hacia': 1}\nprint(resultado)", {"clave": "c", "desde": 0, "hacia": 1}, "rebalance"),
    ("rebalance-aplicar", "rebalanceo · aplicar", "Aplicar un plan sobre una copia de particiones.", "partes = {0: ['a', 'b', 'c'], 1: ['d']}\nplan = {'clave': 'c', 'desde': 0, 'hacia': 1}\nnuevas = {k: list(v) for k, v in partes.items()}\nnuevas[plan['desde']].remove(plan['clave']); nuevas[plan['hacia']].append(plan['clave'])\nresultado = nuevas\nprint(resultado)", {0: ["a", "b"], 1: ["d", "c"]}, "rebalance"),
    ("rebalance-idempotente", "rebalanceo · idempotencia", "No repetir un movimiento ya aplicado.", "aplicados = {'m1'}\nplanes = [('m1', 'a'), ('m2', 'b'), ('m2', 'b')]\nresultado = []\nfor identificador, clave in planes:\n    if identificador not in aplicados: aplicados.add(identificador); resultado.append(clave)\nprint(resultado)", ["b"], "rebalance"),
    ("rebalance-suite", "rebalanceo · suite", "Planificar y aplicar un movimiento hasta equilibrar.", "partes = {0: ['a', 'b', 'c', 'd'], 1: ['e', 'f']}\nplan = {'id': 'm1', 'clave': partes[0][-1], 'desde': 0, 'hacia': 1}\nnuevas = {k: list(v) for k, v in partes.items()}\nnuevas[0].remove(plan['clave']); nuevas[1].append(plan['clave'])\nresultado = {'plan': plan, 'cargas': {k: len(v) for k, v in nuevas.items()}, 'partes': nuevas}\nprint(resultado)", {"plan": {"id": "m1", "clave": "d", "desde": 0, "hacia": 1}, "cargas": {0: 3, 1: 3}, "partes": {0: ["a", "b", "c"], 1: ["e", "f", "d"]}}, "rebalance"),

    # 10. Distributed aggregation capstone
    ("distribuido-enrutar", "capstone · enrutar", "Enrutar eventos a shards estables.", "eventos = [('a', 2), ('b', 3), ('a', 4)]\nresultado = [(sum(map(ord, clave)) % 2, clave, valor) for clave, valor in eventos]\nprint(resultado)", [(1, "a", 2), (0, "b", 3), (1, "a", 4)], "capstone"),
    ("distribuido-parciales", "capstone · parciales", "Calcular agregados parciales por shard y clave.", "shards = {0: [('b', 3)], 1: [('a', 2), ('a', 4)]}\nresultado = {}\nfor shard, filas in shards.items():\n    parcial = {}\n    for clave, valor in filas: parcial[clave] = parcial.get(clave, 0) + valor\n    resultado[shard] = parcial\nprint(resultado)", {0: {"b": 3}, 1: {"a": 6}}, "capstone"),
    ("distribuido-merge", "capstone · merge", "Combinar parciales en orden estable.", "parciales = {0: {'b': 3}, 1: {'a': 6, 'b': 2}}\ntotal = {}\nfor shard in sorted(parciales):\n    for clave, valor in parciales[shard].items(): total[clave] = total.get(clave, 0) + valor\nresultado = dict(sorted(total.items()))\nprint(resultado)", {"a": 6, "b": 5}, "capstone"),
    ("distribuido-deduplicar", "capstone · deduplicar", "Ignorar un parcial repetido por identificador.", "mensajes = [('p1', {'a': 2}), ('p2', {'a': 3}), ('p1', {'a': 2})]\nvistos, total = set(), {}\nfor identificador, parcial in mensajes:\n    if identificador in vistos: continue\n    vistos.add(identificador)\n    for clave, valor in parcial.items(): total[clave] = total.get(clave, 0) + valor\nresultado = total\nprint(resultado)", {"a": 5}, "capstone"),
    ("distribuido-manifiesto", "capstone · manifiesto", "Emitir un manifiesto de cobertura y carga.", "shards = {0: [('b', 3)], 1: [('a', 2), ('a', 4)]}\nresultado = [{'shard': shard, 'eventos': len(shards[shard]), 'claves': sorted({k for k, _ in shards[shard]})} for shard in sorted(shards)]\nprint(resultado)", [{"shard": 0, "eventos": 1, "claves": ["b"]}, {"shard": 1, "eventos": 2, "claves": ["a"]}], "capstone"),
    ("ola32-suite", "ola 32 · suite", "Cerrar la ola con sharding, parciales, deduplicación y merge observables.", "eventos = [('e1', 'a', 2), ('e2', 'b', 3), ('e3', 'a', 4), ('e1', 'a', 2)]\nvistos, shards = set(), {0: [], 1: []}\nfor identificador, clave, valor in eventos:\n    if identificador in vistos: continue\n    vistos.add(identificador); shards[sum(map(ord, clave)) % 2].append((clave, valor))\nparciales = {}\nfor shard, filas in shards.items():\n    parcial = {}\n    for clave, valor in filas: parcial[clave] = parcial.get(clave, 0) + valor\n    parciales[shard] = parcial\ntotal = {}\nfor shard in sorted(parciales):\n    for clave, valor in parciales[shard].items(): total[clave] = total.get(clave, 0) + valor\nresultado = {'shards': shards, 'parciales': parciales, 'total': dict(sorted(total.items())), 'eventos_unicos': len(vistos)}\nprint(resultado)", {"shards": {0: [("b", 3)], 1: [("a", 2), ("a", 4)]}, "parciales": {0: {"b": 3}, 1: {"a": 6}}, "total": {"a": 6, "b": 3}, "eventos_unicos": 3}, "capstone"),
]

RAW = CASES


def build_raw(entries):
    assert len(entries) == 60
    return [exercise(2861 + index, *case) for index, case in enumerate(entries)]


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
