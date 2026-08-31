"""Generate Wave 29: deterministic lazy pipelines and streaming folds."""


def exercise(num, slug, title, objective, solution, expected, lazy=False):
    prompt = (
        f"**{title}**\n\n{objective}\n\n"
        "**Micro-reto:** construí el pipeline indicado, guardá el valor final "
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
        "lazy": lazy,
    }


CASES = [
    # 1. Higher-order transformations
    ("hof-normalizar", "orden superior · normalizar", "Aplicar una función nombrada a cada registro.", "registros = ['  API ', ' Web']\ndef normalizar(s):\n    return s.strip().lower()\nresultado = list(map(normalizar, registros))\nprint(resultado)", ["api", "web"], True),
    ("hof-proyectar", "orden superior · proyectar", "Proyectar un campo con una función recibida.", "filas = [{'id': 3}, {'id': 7}]\ndef proyectar(f, datos):\n    return [f(x) for x in datos]\nresultado = proyectar(lambda fila: fila['id'], filas)\nprint(resultado)", [3, 7], False),
    ("hof-componer", "orden superior · componer", "Componer dos transformaciones puras.", "def componer(f, g):\n    return lambda x: f(g(x))\nlimpiar_largo = componer(len, str.strip)\nresultado = [limpiar_largo(s) for s in [' sol ', 'mar  ']]\nprint(resultado)", [3, 3], False),
    ("hof-despachar", "orden superior · despachar", "Elegir una transformación desde una tabla de funciones.", "operaciones = {'doble': lambda x: x * 2, 'cubo': lambda x: x ** 3}\nresultado = operaciones['cubo'](3)\nprint(resultado)", 27, False),
    ("hof-clave-compuesta", "orden superior · clave compuesta", "Ordenar con una clave que resuelve empates.", "filas = [('b', 2), ('a', 2), ('c', 1)]\nresultado = sorted(filas, key=lambda fila: (fila[1], fila[0]))\nprint(resultado)", [("c", 1), ("a", 2), ("b", 2)], False),
    ("hof-suite", "orden superior · suite", "Filtrar y transformar con callbacks nombrados.", "datos = [-3, 2, 5, -1]\ndef positivo(x):\n    return x > 0\ndef etiqueta(x):\n    return f'v={x * 10}'\nresultado = list(map(etiqueta, filter(positivo, datos)))\nprint(resultado)", ["v=20", "v=50"], True),
    # 2. Generator state
    ("gen-estados", "generador · estados", "Emitir estados acumulados sin materializar la fuente.", "def acumulados(datos):\n    total = 0\n    for valor in datos:\n        total += valor\n        yield total\nresultado = list(acumulados([2, 5, -1]))\nprint(resultado)", [2, 7, 6], True),
    ("gen-pares-indice", "generador · índice", "Emitir índice y valor transformado de manera incremental.", "def enumerar_cuadrados(datos):\n    for indice, valor in enumerate(datos):\n        yield indice, valor * valor\nresultado = list(enumerar_cuadrados([3, 1, 4]))\nprint(resultado)", [(0, 9), (1, 1), (2, 16)], True),
    ("gen-sentinela", "generador · sentinela", "Detener la emisión ante un marcador explícito.", "def hasta_fin(datos):\n    for valor in datos:\n        if valor == 'FIN':\n            return\n        yield valor.lower()\nresultado = list(hasta_fin(['A', 'B', 'FIN', 'C']))\nprint(resultado)", ["a", "b"], True),
    ("gen-aplanar", "generador · aplanar", "Aplanar lotes conservando consumo incremental.", "def aplanar(lotes):\n    for lote in lotes:\n        yield from lote\nresultado = list(aplanar([[1, 2], [], [3]]))\nprint(resultado)", [1, 2, 3], True),
    ("gen-retorno", "generador · transformación", "Transformar solo los valores aceptados por el pipeline.", "def codigos(datos):\n    for texto in datos:\n        limpio = texto.strip()\n        if limpio:\n            yield limpio.upper()\nresultado = list(codigos([' api ', ' ', 'db']))\nprint(resultado)", ["API", "DB"], True),
    ("gen-suite", "generador · suite", "Encadenar dos generadores sin listas intermedias.", "def enteros(textos):\n    for texto in textos:\n        yield int(texto)\ndef pares(datos):\n    for valor in datos:\n        if valor % 2 == 0:\n            yield valor\nresultado = list(pares(enteros(['3', '4', '8'])))\nprint(resultado)", [4, 8], True),
    # 3. Bounded consumption
    ("lazy-islice", "lazy · islice", "Consumir solo una ventana inicial de una fuente extensa.", "from itertools import islice\nfuente = (n * n for n in range(1000))\nresultado = list(islice(fuente, 4))\nprint(resultado)", [0, 1, 4, 9], True),
    ("lazy-next-default", "lazy · next default", "Buscar el primer valor con un default explícito.", "fuente = (n for n in [1, 3, 5] if n % 2 == 0)\nresultado = next(fuente, 'sin-par')\nprint(resultado)", "sin-par", True),
    ("lazy-contador", "lazy · consumo observable", "Contar cuántos elementos exige el consumidor.", "estado = {'vistos': 0}\ndef fuente():\n    for n in range(10):\n        estado['vistos'] += 1\n        yield n\ng = fuente()\nprimeros = [next(g), next(g), next(g)]\nresultado = (primeros, estado['vistos'])\nprint(resultado)", ([0, 1, 2], 3), True),
    ("lazy-skip-take", "lazy · saltar y tomar", "Saltar un prefijo y consumir una ventana acotada.", "from itertools import islice\nfuente = (chr(65 + n) for n in range(10))\nresultado = list(islice(fuente, 2, 5))\nprint(resultado)", ["C", "D", "E"], True),
    ("lazy-cadena", "lazy · cadena", "Componer filtro y mapeo antes de materializar.", "fuente = (n for n in range(12))\nfiltrados = filter(lambda n: n % 3 == 0, fuente)\ntransformados = map(lambda n: f'id-{n}', filtrados)\nresultado = list(transformados)\nprint(resultado)", ["id-0", "id-3", "id-6", "id-9"], True),
    ("lazy-suite", "lazy · suite", "Combinar islice, filter y map con una única materialización.", "from itertools import islice\nfuente = (n for n in range(50))\npipeline = map(lambda n: n + 100, filter(lambda n: n % 5 == 0, fuente))\nresultado = list(islice(pipeline, 3))\nprint(resultado)", [100, 105, 110], True),
    # 4. Iterator exhaustion and tee
    ("iter-agotamiento", "iterador · agotamiento", "Observar que un iterador consumido no se reinicia.", "fuente = iter([4, 5, 6])\nprimera = list(fuente)\nsegunda = list(fuente)\nresultado = (primera, segunda)\nprint(resultado)", ([4, 5, 6], []), True),
    ("iter-parcial", "iterador · consumo parcial", "Continuar desde la posición posterior a next.", "fuente = iter(['a', 'b', 'c'])\nprimero = next(fuente)\nresto = list(fuente)\nresultado = (primero, resto)\nprint(resultado)", ("a", ["b", "c"]), True),
    ("iter-tee", "iterador · tee", "Duplicar una fuente cuando se necesitan dos consumidores.", "from itertools import tee\nbase = (n * 2 for n in range(4))\na, b = tee(base)\nresultado = (list(a), sum(b))\nprint(resultado)", ([0, 2, 4, 6], 12), True),
    ("iter-materializar", "iterador · materialización", "Materializar una vez para reutilizar resultados.", "fuente = (palabra.upper() for palabra in ['sol', 'mar'])\ncache = tuple(fuente)\nresultado = (cache, '-'.join(cache))\nprint(resultado)", (("SOL", "MAR"), "SOL-MAR"), True),
    ("iter-reversed", "iterador · reversa acotada", "Materializar antes de recorrer en orden inverso.", "fuente = (n + 1 for n in range(4))\ncache = list(fuente)\nresultado = list(reversed(cache))\nprint(resultado)", [4, 3, 2, 1], True),
    ("iter-suite", "iterador · suite", "Compartir una captura estable entre varios agregadores.", "fuente = (n for n in [7, 2, 9, 4])\ncache = tuple(fuente)\nresultado = {'total': sum(cache), 'max': max(cache), 'n': len(cache)}\nprint(resultado)", {"total": 22, "max": 9, "n": 4}, True),
    # 5. Folding with explicit identities
    ("fold-suma-inicial", "fold · identidad suma", "Plegar con identidad cero explícita.", "from functools import reduce\nresultado = reduce(lambda acc, n: acc + n, [4, 1, 3], 0)\nprint(resultado)", 8, False),
    ("fold-producto-inicial", "fold · identidad producto", "Plegar con identidad uno, incluso con entrada vacía.", "from functools import reduce\nresultado = reduce(lambda acc, n: acc * n, [], 1)\nprint(resultado)", 1, False),
    ("fold-texto-orden", "fold · orden textual", "Concatenar tokens preservando el orden de llegada.", "from functools import reduce\ntokens = ['api', 'v2', 'users']\nresultado = reduce(lambda ruta, token: f'{ruta}/{token}', tokens, '')\nprint(resultado)", "/api/v2/users", False),
    ("fold-registro", "fold · registro", "Acumular suma y cantidad en un único estado.", "from functools import reduce\ndatos = [3, 8, 4]\nresultado = reduce(lambda acc, n: (acc[0] + n, acc[1] + 1), datos, (0, 0))\nprint(resultado)", (15, 3), False),
    ("fold-deduplicar", "fold · deduplicar estable", "Eliminar repetidos sin perder el primer orden.", "from functools import reduce\ndef agregar(acc, valor):\n    return acc if valor in acc else acc + [valor]\nresultado = reduce(agregar, ['b', 'a', 'b', 'c'], [])\nprint(resultado)", ["b", "a", "c"], False),
    ("fold-suite", "fold · suite", "Construir un histograma mediante un estado acumulado.", "from functools import reduce\ndef contar(acc, clave):\n    acc[clave] = acc.get(clave, 0) + 1\n    return acc\nresultado = reduce(contar, ['ok', 'err', 'ok', 'ok'], {})\nprint(resultado)", {"ok": 3, "err": 1}, False),
    # 6. Log pipelines
    ("log-parsear", "logs · parsear", "Convertir líneas válidas en registros tipados.", "lineas = ['INFO|200|/home', 'WARN|429|/api']\nresultado = [dict(nivel=n, codigo=int(c), ruta=r) for n, c, r in (linea.split('|') for linea in lineas)]\nprint(resultado)", [{"nivel": "INFO", "codigo": 200, "ruta": "/home"}, {"nivel": "WARN", "codigo": 429, "ruta": "/api"}], True),
    ("log-descartar", "logs · descartar ruido", "Excluir líneas de depuración antes de transformar.", "lineas = ['DEBUG cache', 'INFO inicio', 'ERROR timeout']\nresultado = [linea for linea in lineas if not linea.startswith('DEBUG') ]\nprint(resultado)", ["INFO inicio", "ERROR timeout"], False),
    ("log-codigos", "logs · códigos", "Extraer códigos de estado desde registros.", "lineas = ['GET / 200', 'POST /x 201', 'GET /z 404']\nresultado = [int(linea.rsplit(' ', 1)[1]) for linea in lineas]\nprint(resultado)", [200, 201, 404], False),
    ("log-errores-ruta", "logs · errores por ruta", "Contar errores del servidor agrupados por ruta.", "filas = [('/a', 500), ('/b', 200), ('/a', 503)]\nresultado = {}\nfor ruta, codigo in filas:\n    if codigo >= 500:\n        resultado[ruta] = resultado.get(ruta, 0) + 1\nprint(resultado)", {"/a": 2}, False),
    ("log-latencias", "logs · latencias", "Calcular promedio de latencias válidas por streaming.", "valores = (int(x) for x in ['12', '18', '30'])\ntotal = 0\nn = 0\nfor valor in valores:\n    total += valor\n    n += 1\nresultado = total / n\nprint(resultado)", 20.0, True),
    ("log-suite", "logs · suite", "Filtrar errores, proyectar rutas y producir un ranking.", "lineas = [('api', 503), ('web', 200), ('db', 500), ('api', 502)]\nconteos = {}\nfor servicio, codigo in filter(lambda fila: fila[1] >= 500, lineas):\n    conteos[servicio] = conteos.get(servicio, 0) + 1\nresultado = sorted(conteos.items(), key=lambda par: (-par[1], par[0]))\nprint(resultado)", [("api", 2), ("db", 1)], True),
    # 7. Predicate pipelines
    ("predicado-fabrica", "predicados · fábrica", "Crear un predicado parametrizado por umbral.", "def al_menos(umbral):\n    return lambda valor: valor >= umbral\nresultado = list(filter(al_menos(7), [4, 7, 9, 2]))\nprint(resultado)", [7, 9], True),
    ("predicado-compuesto", "predicados · composición", "Combinar condiciones nombradas con claridad.", "def valido(n):\n    return n > 0 and n % 3 == 0\nresultado = list(filter(valido, [-3, 3, 4, 6, 0]))\nprint(resultado)", [3, 6], True),
    ("predicado-campos", "predicados · campos", "Filtrar registros usando más de un campo.", "filas = [{'activo': True, 'puntos': 8}, {'activo': False, 'puntos': 10}, {'activo': True, 'puntos': 4}]\nresultado = list(filter(lambda fila: fila['activo'] and fila['puntos'] >= 5, filas))\nprint(resultado)", [{"activo": True, "puntos": 8}], True),
    ("predicado-none", "predicados · valores ausentes", "Separar valores presentes sin perder ceros.", "datos = [None, 0, 3, None, 5]\nresultado = list(filter(lambda valor: valor is not None, datos))\nprint(resultado)", [0, 3, 5], True),
    ("predicado-etiquetas", "predicados · etiquetas", "Transformar únicamente registros aceptados.", "filas = [('a', 2), ('b', 7), ('c', 5)]\naceptados = filter(lambda fila: fila[1] >= 5, filas)\nresultado = list(map(lambda fila: f'{fila[0]}:{fila[1]}', aceptados))\nprint(resultado)", ["b:7", "c:5"], True),
    ("predicado-suite", "predicados · suite", "Construir una tubería configurable de validación.", "reglas = [lambda n: n % 2 == 0, lambda n: n < 10]\ndatos = range(1, 15)\nresultado = [n for n in datos if all(regla(n) for regla in reglas)]\nprint(resultado)", [2, 4, 6, 8], True),
    # 8. Early termination
    ("early-any-contador", "cortocircuito · any", "Demostrar que any detiene el consumo al primer éxito.", "estado = {'vistos': 0}\ndef pruebas():\n    for n in [1, 3, 8, 10]:\n        estado['vistos'] += 1\n        yield n % 2 == 0\nencontrado = any(pruebas())\nresultado = (encontrado, estado['vistos'])\nprint(resultado)", (True, 3), True),
    ("early-all-contador", "cortocircuito · all", "Demostrar que all detiene el consumo al primer fallo.", "estado = {'vistos': 0}\ndef pruebas():\n    for n in [2, 4, 5, 8]:\n        estado['vistos'] += 1\n        yield n % 2 == 0\ncompleto = all(pruebas())\nresultado = (completo, estado['vistos'])\nprint(resultado)", (False, 3), True),
    ("early-takewhile", "cortocircuito · takewhile", "Consumir mientras se mantenga una condición ordenada.", "from itertools import takewhile\nresultado = list(takewhile(lambda n: n < 10, [2, 5, 9, 12, 3]))\nprint(resultado)", [2, 5, 9], True),
    ("early-dropwhile", "cortocircuito · dropwhile", "Saltar una cabecera y conservar el resto completo.", "from itertools import dropwhile\nlineas = ['# meta', '# fecha', 'dato-1', '# literal']\nresultado = list(dropwhile(lambda linea: linea.startswith('#'), lineas))\nprint(resultado)", ["dato-1", "# literal"], True),
    ("early-primer-error", "cortocircuito · primer error", "Encontrar el primer registro inválido con next.", "filas = [('a', 200), ('b', 404), ('c', 500)]\nresultado = next((fila for fila in filas if fila[1] >= 400), None)\nprint(resultado)", ("b", 404), True),
    ("early-suite", "cortocircuito · suite", "Buscar el primer múltiplo dentro de una ventana acotada.", "from itertools import islice\nfuente = (n for n in range(1, 100))\nventana = islice(fuente, 12)\nresultado = next((n for n in ventana if n % 7 == 0), None)\nprint(resultado)", 7, True),
    # 9. Streaming aggregation
    ("stream-suma-cuadrados", "streaming · suma", "Agregar cuadrados sin lista intermedia.", "resultado = sum(n * n for n in range(1, 6))\nprint(resultado)", 55, True),
    ("stream-conteo", "streaming · conteo", "Contar eventos aceptados con una expresión generadora.", "eventos = ['ok', 'skip', 'ok', 'error', 'ok']\nresultado = sum(1 for evento in eventos if evento == 'ok')\nprint(resultado)", 3, True),
    ("stream-min-clave", "streaming · mínimo", "Elegir el registro mínimo por una clave.", "filas = ((nombre, costo) for nombre, costo in [('x', 9), ('y', 4), ('z', 7)])\nresultado = min(filas, key=lambda fila: fila[1])\nprint(resultado)", ("y", 4), True),
    ("stream-promedio", "streaming · promedio", "Calcular suma y cantidad en una pasada.", "fuente = (n for n in [10, 20, 15, 5])\ntotal = 0\ncantidad = 0\nfor valor in fuente:\n    total += valor\n    cantidad += 1\nresultado = total / cantidad\nprint(resultado)", 12.5, True),
    ("stream-por-clave", "streaming · por clave", "Mantener agregados pequeños por categoría.", "filas = ((k, v) for k, v in [('a', 2), ('b', 5), ('a', 3)])\nresultado = {}\nfor clave, valor in filas:\n    resultado[clave] = resultado.get(clave, 0) + valor\nprint(resultado)", {"a": 5, "b": 5}, True),
    ("stream-suite", "streaming · suite", "Combinar filtro, proyección y suma en una expresión.", "pedidos = [('ok', 12), ('cancelado', 99), ('ok', 8)]\nresultado = sum(monto for estado, monto in pedidos if estado == 'ok')\nprint(resultado)", 20, True),
    # 10. Deterministic scoring capstone
    ("score-normalizar", "scoring · normalizar", "Normalizar métricas heterogéneas antes de puntuar.", "filas = [('a', 80, 20), ('b', 60, 10)]\nresultado = [(nombre, calidad / 100 - latencia / 100) for nombre, calidad, latencia in filas]\nprint(resultado)", [("a", 0.6000000000000001), ("b", 0.5)], False),
    ("score-ponderar", "scoring · ponderar", "Aplicar pesos explícitos a dos señales.", "filas = [('x', 8, 6), ('y', 5, 9)]\nresultado = [(nombre, a * 0.7 + b * 0.3) for nombre, a, b in filas]\nprint(resultado)", [("x", 7.3999999999999995), ("y", 6.199999999999999)], False),
    ("score-filtrar", "scoring · umbral", "Descartar candidatos que no alcanzan el umbral.", "puntajes = [('a', 7.2), ('b', 4.9), ('c', 8.1)]\nresultado = list(filter(lambda par: par[1] >= 7, puntajes))\nprint(resultado)", [("a", 7.2), ("c", 8.1)], True),
    ("score-desempatar", "scoring · desempate", "Ordenar por puntaje descendente y nombre ascendente.", "puntajes = [('beta', 8), ('alfa', 8), ('gamma', 7)]\nresultado = sorted(puntajes, key=lambda par: (-par[1], par[0]))\nprint(resultado)", [("alfa", 8), ("beta", 8), ("gamma", 7)], False),
    ("score-top-lazy", "scoring · top lazy", "Tomar un top acotado luego de ordenar candidatos válidos.", "from itertools import islice\npuntajes = [('a', 3), ('b', 9), ('c', 6), ('d', 8)]\nordenados = iter(sorted((p for p in puntajes if p[1] >= 5), key=lambda p: -p[1]))\nresultado = list(islice(ordenados, 2))\nprint(resultado)", [("b", 9), ("d", 8)], True),
    ("ola29-suite", "ola 29 · suite", "Cerrar la ola con pipeline lazy, agregación y ranking estable.", "from itertools import islice\neventos = [('api', 5), ('web', 2), ('api', 4), ('db', 7), ('web', 3)]\ntotales = {}\nfor servicio, puntos in filter(lambda fila: fila[1] >= 3, eventos):\n    totales[servicio] = totales.get(servicio, 0) + puntos\nranking = iter(sorted(totales.items(), key=lambda par: (-par[1], par[0])))\nresultado = list(islice(ranking, 2))\nprint(resultado)", [("api", 9), ("db", 7)], True),
]

# Public declarative source consumed by applicator and validators.
RAW = CASES


def build_raw(entries):
    assert len(entries) == 60
    return [exercise(2681 + index, *case) for index, case in enumerate(entries)]


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
    print(emit_rust(build_raw(CASES)))
