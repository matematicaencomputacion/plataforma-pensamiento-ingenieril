"""Generate Wave 30: deterministic online aggregation and reconciliation."""


def exercise(num, slug, title, objective, solution, expected, family):
    prompt = (
        f"**{title}**\n\n{objective}\n\n"
        "**Micro-reto:** implementá el algoritmo indicado, guardá el valor final "
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
    # 1. Ordered merge
    ("merge-dos-listas", "merge ordenado · dos fuentes", "Fusionar dos secuencias ya ordenadas.", "from heapq import merge\nresultado = list(merge([1, 4, 9], [2, 3, 10]))\nprint(resultado)", [1, 2, 3, 4, 9, 10], "merge"),
    ("merge-tres-listas", "merge ordenado · tres fuentes", "Fusionar tres secuencias sin reordenar cada entrada.", "from heapq import merge\nresultado = list(merge([1, 7], [2, 8], [3, 9]))\nprint(resultado)", [1, 2, 3, 7, 8, 9], "merge"),
    ("merge-clave", "merge ordenado · clave", "Fusionar registros ordenados por una clave explícita.", "from heapq import merge\na = [{'seq': 1, 'v': 'a'}, {'seq': 4, 'v': 'd'}]\nb = [{'seq': 2, 'v': 'b'}, {'seq': 3, 'v': 'c'}]\nresultado = [x['v'] for x in merge(a, b, key=lambda x: x['seq'])]\nprint(resultado)", ["a", "b", "c", "d"], "merge"),
    ("merge-desempate", "merge ordenado · desempate", "Aplicar una clave total para resolver empates.", "from heapq import merge\na = [(1, 'api'), (3, 'web')]\nb = [(1, 'db'), (2, 'api')]\nresultado = list(merge(a, b, key=lambda x: (x[0], x[1])))\nprint(resultado)", [(1, "api"), (1, "db"), (2, "api"), (3, "web")], "merge"),
    ("merge-deduplicar", "merge ordenado · únicos", "Eliminar duplicados consecutivos luego del merge.", "from heapq import merge\nmezcla = merge([1, 2, 5], [2, 3, 5])\nresultado = []\nfor valor in mezcla:\n    if not resultado or resultado[-1] != valor:\n        resultado.append(valor)\nprint(resultado)", [1, 2, 3, 5], "merge"),
    ("merge-suite", "merge ordenado · suite", "Fusionar eventos y conservar el último valor por secuencia.", "from heapq import merge\na = [(1, 'x', 2), (4, 'x', 8)]\nb = [(2, 'y', 3), (3, 'x', 5)]\nestado = {}\nfor seq, clave, valor in merge(a, b):\n    estado[clave] = (seq, valor)\nresultado = {k: v for k, (_, v) in sorted(estado.items())}\nprint(resultado)", {"x": 8, "y": 3}, "merge"),

    # 2. Heap priorities
    ("heap-minimo", "heap · mínimo", "Extraer la menor prioridad de un heap.", "import heapq\ncola = [7, 2, 5]\nheapq.heapify(cola)\nresultado = heapq.heappop(cola)\nprint(resultado)", 2, "heap"),
    ("heap-insertar", "heap · insertar", "Insertar una prioridad manteniendo el invariante.", "import heapq\ncola = [3, 8]\nheapq.heapify(cola)\nheapq.heappush(cola, 1)\nresultado = [heapq.heappop(cola) for _ in range(len(cola))]\nprint(resultado)", [1, 3, 8], "heap"),
    ("heap-tuplas", "heap · prioridades estables", "Priorizar tareas con un desempate textual explícito.", "import heapq\ncola = [(2, 'web'), (1, 'db'), (1, 'api')]\nheapq.heapify(cola)\nresultado = [heapq.heappop(cola) for _ in range(len(cola))]\nprint(resultado)", [(1, "api"), (1, "db"), (2, "web")], "heap"),
    ("heap-reemplazar", "heap · reemplazo", "Reemplazar la raíz y observar el elemento expulsado.", "import heapq\ncola = [2, 4, 9]\nheapq.heapify(cola)\nexpulsado = heapq.heapreplace(cola, 6)\nresultado = (expulsado, sorted(cola))\nprint(resultado)", (2, [4, 6, 9]), "heap"),
    ("heap-pushpop", "heap · pushpop", "Insertar y extraer en una única operación acotada.", "import heapq\ncola = [4, 7, 9]\nheapq.heapify(cola)\nexpulsado = heapq.heappushpop(cola, 6)\nresultado = (expulsado, sorted(cola))\nprint(resultado)", (4, [6, 7, 9]), "heap"),
    ("heap-suite", "heap · suite", "Procesar tareas por prioridad y secuencia estable.", "import heapq\neventos = [(2, 3, 'c'), (1, 4, 'b'), (1, 2, 'a')]\ncola = []\nfor prioridad, seq, nombre in eventos:\n    heapq.heappush(cola, (prioridad, seq, nombre))\nresultado = [heapq.heappop(cola)[2] for _ in range(len(cola))]\nprint(resultado)", ["a", "b", "c"], "heap"),

    # 3. Bounded top-k
    ("topk-mayores", "top-k · mayores", "Obtener los tres valores mayores.", "import heapq\nresultado = heapq.nlargest(3, [4, 9, 1, 7, 6])\nprint(resultado)", [9, 7, 6], "topk"),
    ("topk-menores", "top-k · menores", "Obtener los dos valores menores.", "import heapq\nresultado = heapq.nsmallest(2, [8, 3, 5, 1])\nprint(resultado)", [1, 3], "topk"),
    ("topk-clave", "top-k · clave", "Seleccionar registros por una métrica explícita.", "import heapq\nfilas = [('a', 4), ('b', 9), ('c', 6)]\nresultado = heapq.nlargest(2, filas, key=lambda x: x[1])\nprint(resultado)", [("b", 9), ("c", 6)], "topk"),
    ("topk-acotado", "top-k · heap acotado", "Mantener solo tres candidatos durante el recorrido.", "import heapq\ncola = []\nfor valor in [5, 1, 9, 4, 8, 2]:\n    if len(cola) < 3:\n        heapq.heappush(cola, valor)\n    elif valor > cola[0]:\n        heapq.heapreplace(cola, valor)\nresultado = sorted(cola, reverse=True)\nprint(resultado)", [9, 8, 5], "topk"),
    ("topk-desempate", "top-k · desempate", "Rankear por puntaje descendente y nombre ascendente.", "filas = [('beta', 8), ('alfa', 8), ('gamma', 7)]\nresultado = sorted(filas, key=lambda x: (-x[1], x[0]))[:2]\nprint(resultado)", [("alfa", 8), ("beta", 8)], "topk"),
    ("topk-suite", "top-k · suite", "Agregar puntajes por clave y conservar un top estable.", "totales = {}\nfor nombre, puntos in [('api', 3), ('web', 8), ('api', 7), ('db', 9)]:\n    totales[nombre] = totales.get(nombre, 0) + puntos\nresultado = sorted(totales.items(), key=lambda x: (-x[1], x[0]))[:2]\nprint(resultado)", [("api", 10), ("db", 9)], "topk"),

    # 4. Online mean/count
    ("online-conteo", "online · conteo", "Actualizar un contador con cada observación.", "estado = 0\nfor _ in [4, 7, 2]:\n    estado += 1\nresultado = estado\nprint(resultado)", 3, "mean"),
    ("online-suma", "online · suma", "Actualizar suma y cantidad en una pasada.", "total = cantidad = 0\nfor valor in [4, 7, 2]:\n    total += valor\n    cantidad += 1\nresultado = (total, cantidad)\nprint(resultado)", (13, 3), "mean"),
    ("online-media", "online · media", "Calcular una media incremental sin guardar la fuente.", "media = 0.0\nfor n, valor in enumerate([10, 20, 15], 1):\n    media += (valor - media) / n\nresultado = media\nprint(resultado)", 15.0, "mean"),
    ("online-media-vacia", "online · entrada vacía", "Representar explícitamente una media sin observaciones.", "total = cantidad = 0\nfor valor in []:\n    total += valor\n    cantidad += 1\nresultado = None if cantidad == 0 else total / cantidad\nprint(resultado)", None, "mean"),
    ("online-por-clave", "online · por clave", "Mantener suma y cantidad por categoría.", "estado = {}\nfor clave, valor in [('a', 2), ('b', 7), ('a', 4)]:\n    total, n = estado.get(clave, (0, 0))\n    estado[clave] = (total + valor, n + 1)\nresultado = {k: total / n for k, (total, n) in sorted(estado.items())}\nprint(resultado)", {"a": 3.0, "b": 7.0}, "mean"),
    ("online-media-suite", "online · suite", "Combinar medias parciales mediante suma y cantidad.", "parciales = [(12, 2), (9, 3), (4, 1)]\ntotal = sum(s for s, _ in parciales)\nn = sum(c for _, c in parciales)\nresultado = total / n\nprint(resultado)", 25 / 6, "mean"),

    # 5. Online variance and extrema
    ("estado-minimo", "estado incremental · mínimo", "Actualizar el mínimo observado.", "minimo = None\nfor valor in [8, 3, 6, 2]:\n    minimo = valor if minimo is None else min(minimo, valor)\nresultado = minimo\nprint(resultado)", 2, "stats"),
    ("estado-extremos", "estado incremental · extremos", "Actualizar mínimo y máximo en una pasada.", "minimo = maximo = None\nfor valor in [8, 3, 6, 2]:\n    minimo = valor if minimo is None else min(minimo, valor)\n    maximo = valor if maximo is None else max(maximo, valor)\nresultado = (minimo, maximo)\nprint(resultado)", (2, 8), "stats"),
    ("estado-cambios", "estado incremental · cambios", "Contar cambios respecto del valor anterior.", "anterior = object()\ncambios = 0\nfor valor in ['ok', 'ok', 'err', 'err', 'ok']:\n    if anterior != valor:\n        cambios += 1\n    anterior = valor\nresultado = cambios\nprint(resultado)", 3, "stats"),
    ("welford-media", "Welford · media", "Actualizar cantidad y media con la recurrencia de Welford.", "n = 0\nmedia = 0.0\nfor x in [2.0, 4.0, 6.0]:\n    n += 1\n    media += (x - media) / n\nresultado = (n, media)\nprint(resultado)", (3, 4.0), "stats"),
    ("welford-varianza", "Welford · varianza", "Calcular varianza poblacional en una pasada estable.", "n = 0\nmedia = m2 = 0.0\nfor x in [2.0, 4.0, 6.0]:\n    n += 1\n    delta = x - media\n    media += delta / n\n    m2 += delta * (x - media)\nresultado = m2 / n\nprint(resultado)", 8 / 3, "stats"),
    ("welford-suite", "Welford · suite", "Reportar conteo, media, varianza y extremos juntos.", "datos = [1.0, 2.0, 5.0]\nn = 0\nmedia = m2 = 0.0\nfor x in datos:\n    n += 1\n    delta = x - media\n    media += delta / n\n    m2 += delta * (x - media)\nresultado = {'n': n, 'media': media, 'var': m2 / n, 'min': min(datos), 'max': max(datos)}\nprint(resultado)", {"n": 3, "media": 2.666666666666667, "var": 26 / 9, "min": 1.0, "max": 5.0}, "stats"),

    # 6. Incremental windows
    ("ventana-deque", "ventanas · deque", "Conservar solo las últimas tres observaciones.", "from collections import deque\nventana = deque(maxlen=3)\nfor valor in [1, 2, 3, 4]:\n    ventana.append(valor)\nresultado = list(ventana)\nprint(resultado)", [2, 3, 4], "window"),
    ("ventana-sumas", "ventanas · sumas", "Emitir la suma de cada ventana completa.", "from collections import deque\nventana = deque(maxlen=3)\nresultado = []\nfor valor in [1, 2, 3, 4, 5]:\n    ventana.append(valor)\n    if len(ventana) == 3:\n        resultado.append(sum(ventana))\nprint(resultado)", [6, 9, 12], "window"),
    ("ventana-media", "ventanas · media", "Mantener una media móvil con suma incremental.", "from collections import deque\nventana = deque()\ntotal = 0\nresultado = []\nfor valor in [2, 4, 8, 10]:\n    ventana.append(valor); total += valor\n    if len(ventana) > 2:\n        total -= ventana.popleft()\n    if len(ventana) == 2:\n        resultado.append(total / 2)\nprint(resultado)", [3.0, 6.0, 9.0], "window"),
    ("ventana-deltas", "ventanas · deltas", "Calcular diferencias entre observaciones consecutivas.", "from collections import deque\nventana = deque(maxlen=2)\nresultado = []\nfor valor in [5, 8, 6, 11]:\n    ventana.append(valor)\n    if len(ventana) == 2:\n        resultado.append(ventana[1] - ventana[0])\nprint(resultado)", [3, -2, 5], "window"),
    ("ventana-por-clave", "ventanas · por clave", "Mantener una ventana independiente por categoría.", "from collections import defaultdict, deque\nventanas = defaultdict(lambda: deque(maxlen=2))\nfor clave, valor in [('a', 1), ('b', 5), ('a', 3), ('a', 7)]:\n    ventanas[clave].append(valor)\nresultado = {k: list(v) for k, v in sorted(ventanas.items())}\nprint(resultado)", {"a": [3, 7], "b": [5]}, "window"),
    ("ventana-suite", "ventanas · suite", "Detectar picos respecto de una media móvil previa.", "from collections import deque\nventana = deque(maxlen=3)\nresultado = []\nfor valor in [2, 3, 4, 10, 5]:\n    if len(ventana) == 3 and valor > 2 * (sum(ventana) / 3):\n        resultado.append(valor)\n    ventana.append(valor)\nprint(resultado)", [10], "window"),

    # 7. Pure checkpoints
    ("checkpoint-tupla", "checkpoint · tupla", "Representar un estado reanudable con una tupla.", "estado = (0, 0)\nfor valor in [2, 5, 1]:\n    total, n = estado\n    estado = (total + valor, n + 1)\nresultado = estado\nprint(resultado)", (8, 3), "checkpoint"),
    ("checkpoint-copia", "checkpoint · copia", "Crear un snapshot que no cambia con el estado posterior.", "estado = {'total': 3, 'n': 1}\nsnapshot = estado.copy()\nestado['total'] += 4\nestado['n'] += 1\nresultado = (snapshot, estado)\nprint(resultado)", ({"total": 3, "n": 1}, {"total": 7, "n": 2}), "checkpoint"),
    ("checkpoint-avanzar", "checkpoint · transición pura", "Aplicar una transición sin mutar el estado recibido.", "def avanzar(estado, valor):\n    return {'total': estado['total'] + valor, 'n': estado['n'] + 1}\ninicial = {'total': 2, 'n': 1}\nfinal = avanzar(inicial, 5)\nresultado = (inicial, final)\nprint(resultado)", ({"total": 2, "n": 1}, {"total": 7, "n": 2}), "checkpoint"),
    ("checkpoint-reanudar", "checkpoint · reanudar", "Reanudar desde un snapshot y continuar el cálculo.", "def aplicar(estado, datos):\n    total, n = estado\n    for valor in datos:\n        total, n = total + valor, n + 1\n    return total, n\nsnapshot = aplicar((0, 0), [2, 3])\nresultado = aplicar(snapshot, [5, 7])\nprint(resultado)", (17, 4), "checkpoint"),
    ("checkpoint-equivalencia", "checkpoint · equivalencia", "Comprobar que ejecución continua y reanudada coinciden.", "def aplicar(estado, datos):\n    return (estado[0] + sum(datos), estado[1] + len(datos))\ncontinua = aplicar((0, 0), [1, 2, 3, 4])\nreanuda = aplicar(aplicar((0, 0), [1, 2]), [3, 4])\nresultado = continua == reanuda\nprint(resultado)", True, "checkpoint"),
    ("checkpoint-suite", "checkpoint · suite", "Reanudar agregados por clave preservando snapshots previos.", "def aplicar(estado, eventos):\n    nuevo = estado.copy()\n    for clave, valor in eventos:\n        nuevo[clave] = nuevo.get(clave, 0) + valor\n    return nuevo\nprimero = aplicar({}, [('a', 2), ('b', 3)])\nsegundo = aplicar(primero, [('a', 5)])\nresultado = (primero, segundo)\nprint(resultado)", ({"a": 2, "b": 3}, {"a": 7, "b": 3}), "checkpoint"),

    # 8. Stable idempotency
    ("idem-claves", "idempotencia · claves", "Conservar una sola aparición de cada clave.", "vistos = set()\nresultado = []\nfor clave in ['a', 'b', 'a', 'c']:\n    if clave not in vistos:\n        vistos.add(clave); resultado.append(clave)\nprint(resultado)", ["a", "b", "c"], "idempotency"),
    ("idem-eventos", "idempotencia · eventos", "Aplicar cada evento identificado una sola vez.", "vistos = set(); total = 0\nfor eid, valor in [('e1', 3), ('e2', 5), ('e1', 3)]:\n    if eid not in vistos:\n        vistos.add(eid); total += valor\nresultado = total\nprint(resultado)", 8, "idempotency"),
    ("idem-primer-valor", "idempotencia · primer valor", "Preservar el primer registro para una clave repetida.", "estado = {}\nfor clave, valor in [('a', 2), ('a', 9), ('b', 4)]:\n    estado.setdefault(clave, valor)\nresultado = estado\nprint(resultado)", {"a": 2, "b": 4}, "idempotency"),
    ("idem-ultimo-valor", "idempotencia · último valor", "Conservar el registro con mayor secuencia por clave.", "estado = {}\nfor clave, seq, valor in [('a', 2, 'nuevo'), ('a', 1, 'viejo'), ('b', 1, 'ok')]:\n    if clave not in estado or seq > estado[clave][0]:\n        estado[clave] = (seq, valor)\nresultado = {k: v for k, (_, v) in sorted(estado.items())}\nprint(resultado)", {"a": "nuevo", "b": "ok"}, "idempotency"),
    ("idem-reintento", "idempotencia · reintento", "Demostrar que reaplicar el mismo lote no cambia el resultado.", "def aplicar(estado, eventos):\n    nuevo = dict(estado)\n    for eid, valor in eventos:\n        nuevo.setdefault(eid, valor)\n    return nuevo\nlote = [('x1', 2), ('x2', 7)]\nuna = aplicar({}, lote)\ndos = aplicar(una, lote)\nresultado = una == dos\nprint(resultado)", True, "idempotency"),
    ("idem-suite", "idempotencia · suite", "Agregar montos una sola vez y conservar claves procesadas.", "def aplicar(estado, vistos, eventos):\n    nuevo = estado.copy(); ids = set(vistos)\n    for eid, clave, monto in eventos:\n        if eid not in ids:\n            ids.add(eid); nuevo[clave] = nuevo.get(clave, 0) + monto\n    return nuevo, ids\neventos = [('1', 'a', 3), ('2', 'a', 4), ('1', 'a', 3)]\nestado, vistos = aplicar({}, set(), eventos)\nresultado = (estado, sorted(vistos))\nprint(resultado)", ({"a": 7}, ["1", "2"]), "idempotency"),

    # 9. Deterministic reconciliation
    ("recon-secuencia", "reconciliación · secuencia", "Elegir el registro con mayor secuencia.", "versiones = [(2, 'medio'), (1, 'viejo'), (3, 'nuevo')]\nresultado = max(versiones, key=lambda x: x[0])[1]\nprint(resultado)", "nuevo", "reconciliation"),
    ("recon-desempate", "reconciliación · desempate", "Resolver igual secuencia con un origen estable.", "versiones = [(4, 'web', 8), (4, 'api', 7)]\nresultado = max(versiones, key=lambda x: (x[0], x[1]))\nprint(resultado)", (4, "web", 8), "reconciliation"),
    ("recon-por-clave", "reconciliación · por clave", "Seleccionar la versión vigente de cada clave.", "estado = {}\nfor clave, seq, valor in [('a', 1, 3), ('b', 2, 8), ('a', 3, 5)]:\n    if clave not in estado or seq > estado[clave][0]:\n        estado[clave] = (seq, valor)\nresultado = {k: v for k, (_, v) in sorted(estado.items())}\nprint(resultado)", {"a": 5, "b": 8}, "reconciliation"),
    ("recon-tombstone", "reconciliación · tombstone", "Eliminar una clave cuando gana una versión tombstone.", "versiones = [('a', 1, 5), ('a', 3, None), ('b', 2, 7)]\nactual = {}\nfor clave, seq, valor in versiones:\n    if clave not in actual or seq > actual[clave][0]:\n        actual[clave] = (seq, valor)\nresultado = {k: v for k, (_, v) in sorted(actual.items()) if v is not None}\nprint(resultado)", {"b": 7}, "reconciliation"),
    ("recon-conmutativa", "reconciliación · orden de llegada", "Comprobar independencia del orden de llegada con clave total.", "def reconciliar(eventos):\n    estado = {}\n    for clave, seq, origen, valor in eventos:\n        if clave not in estado or (seq, origen) > estado[clave][:2]:\n            estado[clave] = (seq, origen, valor)\n    return {k: v for k, (_, _, v) in estado.items()}\ne = [('a', 2, 'api', 5), ('a', 2, 'web', 7), ('b', 1, 'api', 3)]\nresultado = reconciliar(e) == reconciliar(reversed(e))\nprint(resultado)", True, "reconciliation"),
    ("recon-suite", "reconciliación · suite", "Fusionar dos réplicas con tombstones y desempate estable.", "def fusionar(*replicas):\n    estado = {}\n    for replica in replicas:\n        for clave, seq, origen, valor in replica:\n            candidato = (seq, origen, valor)\n            if clave not in estado or candidato[:2] > estado[clave][:2]:\n                estado[clave] = candidato\n    return {k: v for k, (_, _, v) in sorted(estado.items()) if v is not None}\na = [('x', 1, 'api', 3), ('y', 4, 'api', 8)]\nb = [('x', 2, 'web', 5), ('y', 5, 'web', None)]\nresultado = fusionar(a, b)\nprint(resultado)", {"x": 5}, "reconciliation"),

    # 10. Capstone
    ("cap-normalizar", "capstone · normalizar", "Normalizar eventos con una clave total.", "eventos = [('b', 2, 4), ('a', 1, 3)]\nresultado = sorted(eventos, key=lambda x: (x[1], x[0]))\nprint(resultado)", [("a", 1, 3), ("b", 2, 4)], "capstone"),
    ("cap-reconciliar", "capstone · reconciliar", "Conservar la última versión explícita antes de agregar.", "eventos = [('a', 1, 3), ('a', 2, 5), ('b', 1, 4)]\nestado = {}\nfor clave, seq, valor in eventos:\n    if clave not in estado or seq > estado[clave][0]: estado[clave] = (seq, valor)\nresultado = estado\nprint(resultado)", {"a": (2, 5), "b": (1, 4)}, "capstone"),
    ("cap-agregar", "capstone · agregar", "Agregar valores vigentes por grupo.", "vigentes = [('api', 'a', 5), ('web', 'b', 4), ('api', 'c', 3)]\ntotales = {}\nfor grupo, _, valor in vigentes:\n    totales[grupo] = totales.get(grupo, 0) + valor\nresultado = totales\nprint(resultado)", {"api": 8, "web": 4}, "capstone"),
    ("cap-checkpoint", "capstone · checkpoint", "Continuar un agregado desde un snapshot independiente.", "def aplicar(estado, filas):\n    nuevo = estado.copy()\n    for clave, valor in filas: nuevo[clave] = nuevo.get(clave, 0) + valor\n    return nuevo\ns1 = aplicar({}, [('api', 3)])\ns2 = aplicar(s1, [('api', 4), ('web', 5)])\nresultado = (s1, s2)\nprint(resultado)", ({"api": 3}, {"api": 7, "web": 5}), "capstone"),
    ("cap-topk", "capstone · top-k", "Rankear agregados con desempate estable.", "totales = {'web': 7, 'api': 9, 'db': 9}\nresultado = sorted(totales.items(), key=lambda x: (-x[1], x[0]))[:2]\nprint(resultado)", [("api", 9), ("db", 9)], "capstone"),
    ("ola30-suite", "ola 30 · suite", "Cerrar la ola con deduplicación, reconciliación, checkpoint y ranking.", "eventos = [('e1', 'api', 1, 4), ('e2', 'web', 1, 8), ('e1', 'api', 1, 4), ('e3', 'api', 2, 7), ('e4', 'db', 1, 7)]\nvistos = set(); vigentes = {}\nfor eid, clave, seq, valor in eventos:\n    if eid in vistos: continue\n    vistos.add(eid)\n    if clave not in vigentes or seq > vigentes[clave][0]: vigentes[clave] = (seq, valor)\nsnapshot = vigentes.copy()\nresultado = sorted(((k, v) for k, (_, v) in snapshot.items()), key=lambda x: (-x[1], x[0]))[:2]\nprint(resultado)", [("web", 8), ("api", 7)], "capstone"),
]

RAW = CASES


def build_raw(entries):
    assert len(entries) == 60
    return [exercise(2741 + index, *case) for index, case in enumerate(entries)]


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
