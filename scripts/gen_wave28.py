"""Generate Wave 28: deterministic parallelizable pipelines and reductions."""


def exercise(num, slug, title, objective, solution, expected, equivalence=False):
    prompt = (
        f"**{title}**\n\n{objective}\n\n"
        "**Micro-reto:** construí el pipeline indicado, guardá el valor final "
        "en `resultado` y mostralo."
    )
    starter = "\n".join(f"# {line}" for line in solution.splitlines()) + "\n"
    name = slug.replace("-", "_")
    checks = [f"    assert ns['resultado'] == {expected!r}"]
    if equivalence:
        checks.append("    assert ns['resultado'] == ns['directo']")
    pytest = (
        f"def test_{name}(capsys):\n"
        "    ns = {}\n"
        "    exec(open('solution.py', encoding='utf-8').read(), ns)\n"
        + "\n".join(checks) + "\n"
        "    assert capsys.readouterr().out.strip() == str(ns['resultado'])\n"
    )
    return {
        "num": num, "slug": slug, "title": title, "objective": objective,
        "prompt": prompt, "starter": starter, "pytest": pytest,
        "hint": f"El resultado esperado es {expected!r}.", "solution": solution,
        "equivalence": equivalence,
    }


# Ten families of six. "Workers" are pure sequential simulations: no concurrency APIs.
CASES = [
    # 1. Partitioning and chunks
    ("chunk-tamano", "chunks · tamaño fijo", "Particionar datos en bloques independientes.", "datos = [1, 2, 3, 4, 5]\ntamano = 2\nresultado = [datos[i:i + tamano] for i in range(0, len(datos), tamano)]\nprint(resultado)", [[1, 2], [3, 4], [5]], False),
    ("chunk-indices", "chunks · índices", "Conservar el índice inicial de cada bloque.", "datos = ['a', 'b', 'c', 'd']\nresultado = [(i, datos[i:i + 2]) for i in range(0, len(datos), 2)]\nprint(resultado)", [(0, ["a", "b"]), (2, ["c", "d"])], False),
    ("chunk-equilibrado", "chunks · reparto equilibrado", "Repartir una secuencia en dos mitades ordenadas.", "datos = [1, 2, 3, 4, 5]\ncorte = (len(datos) + 1) // 2\nresultado = [datos[:corte], datos[corte:]]\nprint(resultado)", [[1, 2, 3], [4, 5]], False),
    ("chunk-etiquetas", "chunks · etiquetas", "Etiquetar unidades de trabajo reproducibles.", "chunks = [[10, 20], [30]]\nresultado = [{'worker': i, 'datos': parte} for i, parte in enumerate(chunks, start=1)]\nprint(resultado)", [{"worker": 1, "datos": [10, 20]}, {"worker": 2, "datos": [30]}], False),
    ("chunk-vacios", "chunks · entrada vacía", "Tratar una entrada vacía sin crear trabajo ficticio.", "datos = []\nresultado = [datos[i:i + 3] for i in range(0, len(datos), 3)]\nprint(resultado)", [], False),
    ("chunk-suite", "chunks · suite", "Particionar y comprobar la reconstrucción ordenada.", "datos = list(range(7))\nchunks = [datos[i:i + 3] for i in range(0, len(datos), 3)]\nresultado = [x for parte in chunks for x in parte]\nprint(resultado)", [0, 1, 2, 3, 4, 5, 6], False),
    # 2. Batching
    ("batch-pares", "batch · pares", "Procesar lotes pequeños con una función pura.", "lotes = [[1, 2], [3, 4]]\nresultado = [[x * 2 for x in lote] for lote in lotes]\nprint(resultado)", [[2, 4], [6, 8]], False),
    ("batch-sumas", "batch · sumas", "Producir un parcial por lote.", "lotes = [[1, 2, 3], [4, 5]]\nresultado = [sum(lote) for lote in lotes]\nprint(resultado)", [6, 9], False),
    ("batch-filtrado", "batch · filtro", "Filtrar dentro de cada lote sin compartir estado.", "lotes = [[1, 2, 3], [4, 5, 6]]\nresultado = [[x for x in lote if x % 2 == 0] for lote in lotes]\nprint(resultado)", [[2], [4, 6]], False),
    ("batch-normalizar", "batch · normalizar", "Aplicar la misma normalización a cada lote.", "lotes = [[' Sol ', 'MAR'], [' río']]\nresultado = [[s.strip().lower() for s in lote] for lote in lotes]\nprint(resultado)", [["sol", "mar"], ["río"]], False),
    ("batch-metadata", "batch · metadata", "Resumir tamaño y total de cada lote.", "lotes = [[2, 3], [5], []]\nresultado = [{'n': len(lote), 'total': sum(lote)} for lote in lotes]\nprint(resultado)", [{"n": 2, "total": 5}, {"n": 1, "total": 5}, {"n": 0, "total": 0}], False),
    ("batch-suite", "batch · suite", "Transformar lotes y reunirlos en orden estable.", "lotes = [[1, 2], [3], [4, 5]]\nprocesados = [[x * x for x in lote] for lote in lotes]\nresultado = [x for lote in procesados for x in lote]\nprint(resultado)", [1, 4, 9, 16, 25], False),
    # 3. Alignment with zip
    ("zip-pares", "alineación · zip", "Alinear dos fuentes por posición.", "nombres = ['ana', 'leo']\npuntos = [8, 9]\nresultado = list(zip(nombres, puntos))\nprint(resultado)", [("ana", 8), ("leo", 9)], False),
    ("zip-sumas", "alineación · suma", "Combinar valores alineados con una función pura.", "a = [1, 2, 3]\nb = [10, 20, 30]\nresultado = [x + y for x, y in zip(a, b)]\nprint(resultado)", [11, 22, 33], False),
    ("zip-dict", "alineación · diccionario", "Construir registros desde columnas alineadas.", "claves = ['cpu', 'ram']\nvalores = [70, 55]\nresultado = dict(zip(claves, valores))\nprint(resultado)", {"cpu": 70, "ram": 55}, False),
    ("zip-cortar", "alineación · fuente corta", "Observar que zip usa la fuente más corta.", "a = [1, 2, 3]\nb = ['x', 'y']\nresultado = list(zip(a, b))\nprint(resultado)", [(1, "x"), (2, "y")], False),
    ("zip-indice", "alineación · índice", "Asociar posiciones estables a pares alineados.", "a = ['A', 'B']\nb = [100, 200]\nresultado = [(i, x, y) for i, (x, y) in enumerate(zip(a, b))]\nprint(resultado)", [(0, "A", 100), (1, "B", 200)], False),
    ("zip-suite", "alineación · suite", "Validar y transformar dos columnas relacionadas.", "precios = [10, 20, 5]\ncantidades = [2, 1, 4]\nresultado = [p * q for p, q in zip(precios, cantidades)]\nprint(resultado)", [20, 20, 20], False),
    # 4. Fan-out pure transforms
    ("fanout-doble-triple", "fan-out · dos ramas", "Aplicar dos transformaciones independientes a la misma entrada.", "datos = [1, 2, 3]\nresultado = {'dobles': [x * 2 for x in datos], 'triples': [x * 3 for x in datos]}\nprint(resultado)", {"dobles": [2, 4, 6], "triples": [3, 6, 9]}, False),
    ("fanout-texto", "fan-out · texto", "Derivar vistas independientes de palabras.", "datos = ['Sol', 'mar']\nresultado = {'lower': [s.lower() for s in datos], 'largos': [len(s) for s in datos]}\nprint(resultado)", {"lower": ["sol", "mar"], "largos": [3, 3]}, False),
    ("fanout-predicados", "fan-out · predicados", "Evaluar condiciones independientes sin mutar la entrada.", "datos = [1, 2, 3, 4]\nresultado = {'pares': [x for x in datos if x % 2 == 0], 'mayores': [x for x in datos if x > 2]}\nprint(resultado)", {"pares": [2, 4], "mayores": [3, 4]}, False),
    ("fanout-estadisticas", "fan-out · estadísticas", "Calcular parciales independientes sobre los mismos datos.", "datos = [4, 1, 7]\nresultado = {'min': min(datos), 'max': max(datos), 'total': sum(datos)}\nprint(resultado)", {"min": 1, "max": 7, "total": 12}, False),
    ("fanout-chunks", "fan-out · por chunk", "Aplicar workers puros a chunks separados.", "chunks = [[1, 2], [3, 4]]\ndef worker(xs):\n    return [x + 10 for x in xs]\nresultado = [worker(parte) for parte in chunks]\nprint(resultado)", [[11, 12], [13, 14]], False),
    ("fanout-suite", "fan-out · suite", "Producir tres ramas y conservar su identidad.", "datos = [1, 2, 3]\nfunciones = [('id', lambda x: x), ('cuad', lambda x: x * x), ('neg', lambda x: -x)]\nresultado = {nombre: [f(x) for x in datos] for nombre, f in funciones}\nprint(resultado)", {"id": [1, 2, 3], "cuad": [1, 4, 9], "neg": [-1, -2, -3]}, False),
    # 5. Fan-in stable merge
    ("fanin-concatenar", "fan-in · concatenar", "Reunir resultados respetando el orden de workers.", "parciales = [[1, 2], [3], [4, 5]]\nresultado = [x for parcial in parciales for x in parcial]\nprint(resultado)", [1, 2, 3, 4, 5], False),
    ("fanin-etiquetado", "fan-in · etiquetado", "Ordenar parciales por una etiqueta explícita.", "parciales = [(2, ['c']), (0, ['a']), (1, ['b'])]\nresultado = [x for _, datos in sorted(parciales) for x in datos]\nprint(resultado)", ["a", "b", "c"], False),
    ("fanin-dicts", "fan-in · diccionarios", "Combinar mapas de claves disjuntas.", "parciales = [{'a': 1}, {'b': 2}, {'c': 3}]\nresultado = {k: v for parcial in parciales for k, v in parcial.items()}\nprint(resultado)", {"a": 1, "b": 2, "c": 3}, False),
    ("fanin-conflicto", "fan-in · conflicto", "Resolver claves repetidas con una regla ordenada.", "parciales = [{'x': 1}, {'x': 4}, {'y': 2}]\nresultado = {}\nfor parcial in parciales:\n    resultado.update(parcial)\nprint(resultado)", {"x": 4, "y": 2}, False),
    ("fanin-intercalar", "fan-in · intercalar", "Intercalar salidas alineadas de dos workers.", "izq = [1, 3, 5]\nder = [2, 4, 6]\nresultado = [x for par in zip(izq, der) for x in par]\nprint(resultado)", [1, 2, 3, 4, 5, 6], False),
    ("fanin-suite", "fan-in · suite", "Reunir, ordenar y deduplicar resultados parciales.", "parciales = [[3, 1], [2, 3], [1, 4]]\nresultado = sorted({x for parcial in parciales for x in parcial})\nprint(resultado)", [1, 2, 3, 4], False),
    # 6. Partial reductions
    ("parcial-suma", "reducción parcial · suma", "Comparar suma directa con suma de parciales.", "datos = [1, 2, 3, 4]\nchunks = [datos[:2], datos[2:]]\nparciales = [sum(c) for c in chunks]\ndirecto = sum(datos)\nresultado = sum(parciales)\nprint(resultado)", 10, True),
    ("parcial-producto", "reducción parcial · producto", "Combinar productos parciales con identidad uno.", "from math import prod\ndatos = [2, 3, 4]\nchunks = [datos[:1], datos[1:]]\nparciales = [prod(c) for c in chunks]\ndirecto = prod(datos)\nresultado = prod(parciales)\nprint(resultado)", 24, True),
    ("parcial-minimo", "reducción parcial · mínimo", "Combinar mínimos de chunks no vacíos.", "datos = [8, 3, 9, 1]\nchunks = [datos[:2], datos[2:]]\nparciales = [min(c) for c in chunks]\ndirecto = min(datos)\nresultado = min(parciales)\nprint(resultado)", 1, True),
    ("parcial-maximo", "reducción parcial · máximo", "Combinar máximos locales en un máximo global.", "datos = [8, 3, 9, 1]\nchunks = [datos[:2], datos[2:]]\nparciales = [max(c) for c in chunks]\ndirecto = max(datos)\nresultado = max(parciales)\nprint(resultado)", 9, True),
    ("parcial-conteo", "reducción parcial · conteo", "Sumar conteos locales de un predicado.", "datos = [1, 2, 4, 5, 6]\nchunks = [datos[:3], datos[3:]]\nparciales = [sum(1 for x in c if x % 2 == 0) for c in chunks]\ndirecto = sum(1 for x in datos if x % 2 == 0)\nresultado = sum(parciales)\nprint(resultado)", 3, True),
    ("parcial-suite", "reducción parcial · suite", "Combinar suma y conteo para calcular un promedio.", "datos = [2, 4, 6, 8]\nchunks = [datos[:2], datos[2:]]\nparciales = [(sum(c), len(c)) for c in chunks]\ntotal = sum(s for s, _ in parciales)\nn = sum(c for _, c in parciales)\ndirecto = sum(datos) / len(datos)\nresultado = total / n\nprint(resultado)", 5.0, True),
    # 7. Associative combination
    ("asociar-sumas", "asociatividad · suma", "Verificar dos agrupaciones equivalentes de la suma.", "a, b, c = 2, 3, 4\nizquierda = (a + b) + c\ndirecto = a + (b + c)\nresultado = izquierda\nprint(resultado)", 9, True),
    ("asociar-tuplas", "asociatividad · concatenación", "Conservar orden al reagrupar secuencias.", "a, b, c = (1,), (2,), (3,)\nresultado = (a + b) + c\ndirecto = a + (b + c)\nprint(resultado)", (1, 2, 3), True),
    ("asociar-max", "asociatividad · máximo", "Combinar máximos por niveles.", "a, b, c = 4, 9, 2\nresultado = max(max(a, b), c)\ndirecto = max(a, max(b, c))\nprint(resultado)", 9, True),
    ("asociar-sets", "asociatividad · unión", "Reagrupar uniones y presentar salida ordenada.", "a, b, c = {1, 2}, {2, 3}, {4}\nresultado = sorted((a | b) | c)\ndirecto = sorted(a | (b | c))\nprint(resultado)", [1, 2, 3, 4], True),
    ("asociar-dicts", "asociatividad · conteos", "Combinar mapas de conteos con una operación asociativa.", "def combinar(a, b):\n    claves = a.keys() | b.keys()\n    return {k: a.get(k, 0) + b.get(k, 0) for k in claves}\na, b, c = {'x': 1}, {'x': 2, 'y': 1}, {'y': 3}\nresultado = combinar(combinar(a, b), c)\ndirecto = combinar(a, combinar(b, c))\nprint(resultado)", {"x": 3, "y": 4}, True),
    ("asociar-suite", "asociatividad · suite", "Plegar parciales de suma en forma jerárquica.", "parciales = [3, 7, 5, 1]\nnivel = [parciales[0] + parciales[1], parciales[2] + parciales[3]]\nresultado = nivel[0] + nivel[1]\ndirecto = sum(parciales)\nprint(resultado)", 16, True),
    # 8. Grouped aggregation
    ("grupo-sumas", "agrupación · sumas", "Acumular valores por clave.", "filas = [('a', 2), ('b', 3), ('a', 4)]\nresultado = {}\nfor clave, valor in filas:\n    resultado[clave] = resultado.get(clave, 0) + valor\nprint(resultado)", {"a": 6, "b": 3}, False),
    ("grupo-conteos", "agrupación · conteos", "Contar registros por categoría.", "categorias = ['web', 'api', 'web', 'db', 'api']\nresultado = {}\nfor clave in categorias:\n    resultado[clave] = resultado.get(clave, 0) + 1\nprint(resultado)", {"web": 2, "api": 2, "db": 1}, False),
    ("grupo-listas", "agrupación · colecciones", "Reunir valores preservando orden por grupo.", "filas = [('x', 1), ('y', 2), ('x', 3)]\nresultado = {}\nfor clave, valor in filas:\n    resultado.setdefault(clave, []).append(valor)\nprint(resultado)", {"x": [1, 3], "y": [2]}, False),
    ("grupo-maximos", "agrupación · máximos", "Mantener un máximo parcial por clave.", "filas = [('a', 2), ('a', 5), ('b', 3)]\nresultado = {}\nfor clave, valor in filas:\n    resultado[clave] = max(resultado.get(clave, valor), valor)\nprint(resultado)", {"a": 5, "b": 3}, False),
    ("grupo-parciales", "agrupación · merge parciales", "Combinar agregados producidos por chunks.", "parciales = [{'a': 2, 'b': 1}, {'a': 3, 'c': 4}]\nresultado = {}\nfor parcial in parciales:\n    for clave, valor in parcial.items():\n        resultado[clave] = resultado.get(clave, 0) + valor\nprint(resultado)", {"a": 5, "b": 1, "c": 4}, False),
    ("grupo-suite", "agrupación · suite", "Calcular promedio por grupo desde suma y conteo.", "filas = [('a', 2), ('b', 4), ('a', 6)]\nacum = {}\nfor clave, valor in filas:\n    total, n = acum.get(clave, (0, 0))\n    acum[clave] = (total + valor, n + 1)\nresultado = {k: total / n for k, (total, n) in acum.items()}\nprint(resultado)", {"a": 4.0, "b": 4.0}, False),
    # 9. Windows and deterministic scheduling
    ("ventana-fija", "ventanas · fija", "Crear ventanas contiguas de tamaño constante.", "datos = [1, 2, 3, 4, 5]\nresultado = [datos[i:i + 3] for i in range(len(datos) - 2)]\nprint(resultado)", [[1, 2, 3], [2, 3, 4], [3, 4, 5]], False),
    ("ventana-sumas", "ventanas · sumas", "Reducir cada ventana de forma independiente.", "datos = [1, 2, 3, 4]\nresultado = [sum(datos[i:i + 2]) for i in range(len(datos) - 1)]\nprint(resultado)", [3, 5, 7], False),
    ("agenda-roundrobin", "planificación · round robin", "Asignar tareas a workers de manera reproducible.", "tareas = ['a', 'b', 'c', 'd', 'e']\nworkers = 2\nresultado = {w: [t for i, t in enumerate(tareas) if i % workers == w] for w in range(workers)}\nprint(resultado)", {0: ["a", "c", "e"], 1: ["b", "d"]}, False),
    ("agenda-costos", "planificación · costos", "Resumir carga asignada por worker.", "asignaciones = {0: [3, 2], 1: [4], 2: [1, 1]}\nresultado = {w: sum(costos) for w, costos in asignaciones.items()}\nprint(resultado)", {0: 5, 1: 4, 2: 2}, False),
    ("agenda-orden", "planificación · orden", "Reconstruir resultados según el índice original.", "resultados = [(2, 'C'), (0, 'A'), (1, 'B')]\nresultado = [valor for _, valor in sorted(resultados)]\nprint(resultado)", ["A", "B", "C"], False),
    ("ventana-suite", "ventanas · suite", "Evaluar ventanas y elegir el mejor parcial de forma estable.", "datos = [2, 5, 1, 4, 3]\nventanas = [(i, sum(datos[i:i + 2])) for i in range(len(datos) - 1)]\nresultado = max(ventanas, key=lambda par: (par[1], -par[0]))\nprint(resultado)", (0, 7), False),
    # 10. Local map-reduce capstone
    ("mr-tokenizar", "map-reduce · map", "Emitir pares clave-valor desde registros independientes.", "lineas = ['sol mar', 'mar río']\nresultado = [(palabra, 1) for linea in lineas for palabra in linea.split()]\nprint(resultado)", [("sol", 1), ("mar", 1), ("mar", 1), ("río", 1)], False),
    ("mr-shuffle", "map-reduce · shuffle", "Agrupar valores emitidos por clave.", "pares = [('a', 1), ('b', 1), ('a', 1)]\nresultado = {}\nfor clave, valor in pares:\n    resultado.setdefault(clave, []).append(valor)\nprint(resultado)", {"a": [1, 1], "b": [1]}, False),
    ("mr-reduce", "map-reduce · reduce", "Reducir valores agrupados por clave.", "grupos = {'a': [1, 1, 1], 'b': [1, 1]}\nresultado = {clave: sum(valores) for clave, valores in grupos.items()}\nprint(resultado)", {"a": 3, "b": 2}, False),
    ("mr-chunks", "map-reduce · chunks", "Producir conteos parciales por chunk.", "chunks = [['a', 'b', 'a'], ['b', 'c']]\ndef contar(xs):\n    return {x: xs.count(x) for x in dict.fromkeys(xs)}\nresultado = [contar(chunk) for chunk in chunks]\nprint(resultado)", [{"a": 2, "b": 1}, {"b": 1, "c": 1}], False),
    ("mr-combinar", "map-reduce · combinar", "Fusionar conteos parciales en el resultado global.", "parciales = [{'a': 2, 'b': 1}, {'b': 1, 'c': 1}]\nresultado = {}\nfor parcial in parciales:\n    for clave, valor in parcial.items():\n        resultado[clave] = resultado.get(clave, 0) + valor\ndirecto = {'a': 2, 'b': 2, 'c': 1}\nprint(resultado)", {"a": 2, "b": 2, "c": 1}, True),
    ("ola28-suite", "ola 28 · suite", "Cerrar la ola con map, chunks, parciales y reduce equivalentes.", "datos = [1, 2, 3, 4, 5, 6]\nchunks = [datos[:3], datos[3:]]\nparciales = [sum(x * x for x in chunk if x % 2 == 0) for chunk in chunks]\nresultado = sum(parciales)\ndirecto = sum(x * x for x in datos if x % 2 == 0)\nprint(resultado)", 56, True),
]


def build_steps():
    assert len(CASES) == 60
    return [exercise(2621 + index, *case) for index, case in enumerate(CASES)]


def _rust_escape(value):
    return value.replace("\\", "\\\\").replace("\n", "\\n").replace('"', '\\"')


def emit_rust(steps):
    blocks = []
    for index, item in enumerate(steps):
        const = f"PY{item['num']}_{item['slug'].upper().replace('-', '_')}"
        if index + 1 < len(steps):
            following = steps[index + 1]
            next_value = f'Some("py-{following["num"]}-{following["slug"]}")'
        else:
            next_value = "None"
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
    print(emit_rust(build_steps()))
