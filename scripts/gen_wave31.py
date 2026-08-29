"""Genera los micro-steps 2801-2860 de la Ola 24 ("Pipelines lazy y map-filter").

Emite el bloque Rust de 60 `CodingStep` listo para insertar en `web/src/curriculum.rs`.

Reglas: stdlib-only, Wasm-safe, determinista (cero red/threads), 10x6,
explain->try->check, <=120 palabras, sin `input()`, pytest asserts con `resultado`.
"""


def step(num, slug, title, objective, prompt, starter, pytest, hint, solution):
    """Construye un dict de paso con el `next` calculado por el caller (cadena)."""
    return {
        "num": num,
        "slug": slug,
        "title": title,
        "objective": objective,
        "prompt": prompt,
        "starter": starter,
        "pytest": pytest,
        "hint": hint,
        "solution": solution,
    }


# (num, slug, title, objective, prompt, starter, pytest_tail, hint, solution)
RAW = [
    # ---- Grupo 1: 2801-2806 map y filter con lambdas (paradigms [3]) ----
    (2801, "map-lambda", "map · lambda", "Aplicar map(lambda, ...) a una lista.",
     "**map(lambda, iterable)**\n\n`map` aplica una función a cada elemento sin escribir un `for`.\n\n**Micro-reto:**\n1. `resultado = list(map(lambda x: x * 2, [1, 2, 3]))`\n2. Mostrá",
     "# resultado = list(map(lambda x: x * 2, [1, 2, 3]))\n# print(resultado)\n",
     "assert ns['resultado'] == [2, 4, 6]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[2, 4, 6]",
     "resultado = list(map(lambda x: x * 2, [1, 2, 3]))\nprint(resultado)"),
    (2802, "filter-lambda", "filter · lambda", "Filtrar con filter(lambda, ...).",
     "**filter(lambda, iterable)**\n\n`filter` conserva solo los elementos que cumplen el predicado.\n\n**Micro-reto:**\n1. `resultado = list(filter(lambda x: x > 2, [1, 2, 3]))`\n2. Mostrá",
     "# resultado = list(filter(lambda x: x > 2, [1, 2, 3]))\n# print(resultado)\n",
     "assert ns['resultado'] == [3]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[3]",
     "resultado = list(filter(lambda x: x > 2, [1, 2, 3]))\nprint(resultado)"),
    (2803, "mapfilter-combo", "map · filter combinado", "Encadenar filter y map.",
     "**Combo funcional**\n\nPodés filtrar y transformar en cadena: primer `filter`, luego `map`.\n\n**Micro-reto:**\n1. De [1, 2, 3, 4] filtrá pares\n2. Elevá al cuadrado con `map`\n3. `resultado = list(...)`",
     "# paso = filter(lambda x: x % 2 == 0, [1, 2, 3, 4])\n# resultado = list(map(lambda x: x * x, paso))\n# print(resultado)\n",
     "assert ns['resultado'] == [4, 16]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[4, 16]",
     "pares = filter(lambda x: x % 2 == 0, [1, 2, 3, 4])\nresultado = list(map(lambda x: x * x, pares))\nprint(resultado)"),
    (2804, "lambda-sorted", "lambda · sorted", "Usar lambda como clave de orden.",
     "**sorted(key=lambda)**\n\n`key=` define el criterio sin cambiar los valores.\n\n**Micro-reto:**\n1. `datos = [(1, 'b'), (3, 'a')]`\n2. `resultado = sorted(datos, key=lambda t: t[1])`\n3. Mostrá",
     "# datos = [(1, 'b'), (3, 'a')]\n# resultado = sorted(datos, key=lambda t: t[1])\n# print(resultado)\n",
     "assert ns['resultado'] == [(3, 'a'), (1, 'b')]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[(3, 'a'), (1, 'b')]",
     "datos = [(1, 'b'), (3, 'a')]\nresultado = sorted(datos, key=lambda t: t[1])\nprint(resultado)"),
    (2805, "map-multi", "map · varios iterables", "Aplicar map con dos listas.",
     "**map multi-iterable**\n\n`map` acepta varios iterables: combina por posición.\n\n**Micro-reto:**\n1. `a = [1, 2]; b = [10, 20]`\n2. `resultado = list(map(lambda x, y: x + y, a, b))`\n3. Mostrá",
     "# a = [1, 2]; b = [10, 20]\n# resultado = list(map(lambda x, y: x + y, a, b))\n# print(resultado)\n",
     "assert ns['resultado'] == [11, 22]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[11, 22]",
     "a = [1, 2]; b = [10, 20]\nresultado = list(map(lambda x, y: x + y, a, b))\nprint(resultado)"),
    (2806, "mapfilter-check", "map/filter · Suite", "Cerrar grupo: map + filter.",
     "**Suite funcional**\n\nFiltrá negativos y duplicá.\n\n**Micro-reto:**\n1. `datos = [1, -2, 3]`\n2. Filtrá los positivos\n3. `map` para duplicar; `resultado = list(...)`",
     "# datos = [1, -2, 3]\n# positivos = filter(lambda x: x > 0, datos)\n# resultado = list(map(lambda x: x * 2, positivos))\n# print(resultado)\n",
     "assert ns['resultado'] == [2, 6]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[2, 6]",
     "datos = [1, -2, 3]\npositivos = filter(lambda x: x > 0, datos)\nresultado = list(map(lambda x: x * 2, positivos))\nprint(resultado)"),

    # ---- Grupo 2: 2807-2812 funciones como datos / callbacks (paradigms + scope-legb [3,2]) ----
    (2807, "fun-dato", "funciones · como datos", "Guardar una función en una variable.",
     "**funciones como valores**\n\nEn Python las funciones son objetos: se asignan y llaman con `()`.\n\n**Micro-reto:**\n1. `def doble(x): return x * 2`\n2. `f = doble`\n3. `resultado = f(5)`",
     "# def doble(x): return x * 2\n# f = doble\n# resultado = f(5)\n# print(resultado)\n",
     "assert ns['resultado'] == 10\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "10",
     "def doble(x): return x * 2\nf = doble\nresultado = f(5)\nprint(resultado)"),
    (2808, "sort-callback", "callback · sorted", "Usar una función como callback.",
     "**callback**\n\n`key=` puede ser una función definida, no solo lambda.\n\n**Micro-reto:**\n1. `def largo(s): return len(s)`\n2. `resultado = sorted(['aa', 'b'], key=largo)`\n3. Mostrá",
     "# def largo(s): return len(s)\n# resultado = sorted(['aa', 'b'], key=largo)\n# print(resultado)\n",
     "assert ns['resultado'] == ['b', 'aa']\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "['b', 'aa']",
     "def largo(s): return len(s)\nresultado = sorted(['aa', 'b'], key=largo)\nprint(resultado)"),
    (2809, "callback-filter", "callback · filter", "Pasar una función definida a filter.",
     "**callback nombrado**\n\nUna función nombrada funciona igual que una lambda en `filter`.\n\n**Micro-reto:**\n1. `def es_par(x): return x % 2 == 0`\n2. `resultado = list(filter(es_par, [1, 2, 3, 4]))`\n3. Mostrá",
     "# def es_par(x): return x % 2 == 0\n# resultado = list(filter(es_par, [1, 2, 3, 4]))\n# print(resultado)\n",
     "assert ns['resultado'] == [2, 4]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[2, 4]",
     "def es_par(x): return x % 2 == 0\nresultado = list(filter(es_par, [1, 2, 3, 4]))\nprint(resultado)"),
    (2810, "higher-order", "orden · superior", "Escribir una función que reciba función.",
     "**función de orden superior**\n\nUna función que recibe otra se llama de orden superior.\n\n**Micro-reto:**\n1. `def aplicar(f, n): return f(n)`\n2. `resultado = aplicar(lambda x: x + 1, 4)`\n3. Mostrá",
     "# def aplicar(f, n): return f(n)\n# resultado = aplicar(lambda x: x + 1, 4)\n# print(resultado)\n",
     "assert ns['resultado'] == 5\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "5",
     "def aplicar(f, n): return f(n)\nresultado = aplicar(lambda x: x + 1, 4)\nprint(resultado)"),
    (2811, "callback-inline", "callback · inline", "Usar lambda como argumento.",
     "**lambda como argumento**\n\nPasar una lambda directo a `sorted`/`filter`.\n\n**Micro-reto:**\n1. `resultado = sorted([3, 1, 2], key=lambda x: -x)`\n2. Mostrá",
     "# resultado = sorted([3, 1, 2], key=lambda x: -x)\n# print(resultado)\n",
     "assert ns['resultado'] == [3, 2, 1]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[3, 2, 1]",
     "resultado = sorted([3, 1, 2], key=lambda x: -x)\nprint(resultado)"),
    (2812, "callback-check", "callback · Suite", "Cerrar grupo: callbacks.",
     "**Suite callbacks**\n\nCada palabra por longitud, descendente.\n\n**Micro-reto:**\n1. `palabras = ['hola', 'y', 'mundo']`\n2. `sorted(palabras, key=len, reverse=True)`\n3. `resultado = ...`",
     "# palabras = ['hola', 'y', 'mundo']\n# resultado = sorted(palabras, key=len, reverse=True)\n# print(resultado)\n",
     "assert ns['resultado'] == ['mundo', 'hola', 'y']\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "['mundo', 'hola', 'y']",
     "palabras = ['hola', 'y', 'mundo']\nresultado = sorted(palabras, key=len, reverse=True)\nprint(resultado)"),

    # ---- Grupo 3: 2813-2818 encadenar generadores lazy (paradigms + ecosystem [3,4]) ----
    (2813, "gen-yield", "generador · yield", "Crear una función generadora.",
     "**yield**\n\nUna función con `yield` es un generador: produce valores uno a uno.\n\n**Micro-reto:**\n1. `def pares(): yield 2; yield 4`\n2. `resultado = list(pares())`\n3. Mostrá",
     "# def pares():\n#     yield 2\n#     yield 4\n# resultado = list(pares())\n# print(resultado)\n",
     "assert ns['resultado'] == [2, 4]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[2, 4]",
     "def pares():\n    yield 2\n    yield 4\nresultado = list(pares())\nprint(resultado)"),
    (2814, "gen-lazy", "generador · lazy", "Entender la pereza de un generador.",
     "**pereza**\n\nUn generador consume de a uno; no materializa toda la lista\n\n**Micro-reto:**\n1. `g = (x for x in range(3))`\n2. `resultado = list(g)`\n3. Mostrá",
     "# g = (x for x in range(3))\n# resultado = list(g)\n# print(resultado)\n",
     "assert ns['resultado'] == [0, 1, 2]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[0, 1, 2]",
     "g = (x for x in range(3))\nresultado = list(g)\nprint(resultado)"),
    (2815, "gen-chain", "encadenar · generadores", "Encadenar dos generadores.",
     "**encadenar generadores**\n\nUsar el output de un generador como entrada de otro.\n\n**Micro-reto:**\n1. `a = (x for x in range(3))`\n2. `b = (x * 10 for x in a)`\n3. `resultado = list(b)`",
     "# a = (x for x in range(3))\n# b = (x * 10 for x in a)\n# resultado = list(b)\n# print(resultado)\n",
     "assert ns['resultado'] == [0, 10, 20]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[0, 10, 20]",
     "a = (x for x in range(3))\nb = (x * 10 for x in a)\nresultado = list(b)\nprint(resultado)"),
    (2816, "gen-expr", "generador · expresión", "Usar una generator expression.",
     "**generator expression**\n\nEntre paréntesis crea un iterable perezoso (no una lista).\n\n**Micro-reto:**\n1. `rango = range(4)`\n2. `resultado = list(x * x for x in rango if x % 2)`\n3. Mostrá",
     "# rango = range(4)\n# resultado = list(x * x for x in rango if x % 2)\n# print(resultado)\n",
     "assert ns['resultado'] == [1, 9]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[1, 9]",
     "rango = range(4)\nresultado = list(x * x for x in rango if x % 2)\nprint(resultado)"),
    (2817, "gen-infinite", "generador · infinito", "Iterar un generador infinito con tope.",
     "**generador infinito**\n\nProduce indefinidamente; cortá con `next()` o `islice`.\n\n**Micro-reto:**\n1. `g = iter(range(1, 100))`\n2. `resultado = [next(g) for _ in range(3)]`\n3. Mostrá",
     "# g = iter(range(1, 100))\n# resultado = [next(g) for _ in range(3)]\n# print(resultado)\n",
     "assert ns['resultado'] == [1, 2, 3]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[1, 2, 3]",
     "g = iter(range(1, 100))\nresultado = [next(g) for _ in range(3)]\nprint(resultado)"),
    (2818, "gen-check", "generador · Suite", "Cerrar grupo: lazy pipelines.",
     "**Suite lazy**\n\nSumá sobre un generador con `filter`.\n\n**Micro-reto:**\n1. `datos = range(5)`\n2. `resultado = sum(x for x in datos if x % 2 == 0)`\n3. Mostrá",
     "# datos = range(5)\n# resultado = sum(x for x in datos if x % 2 == 0)\n# print(resultado)\n",
     "assert ns['resultado'] == 6\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "6",
     "datos = range(5)\nresultado = sum(x for x in datos if x % 2 == 0)\nprint(resultado)"),

    # ---- Grupo 4: 2819-2824 reduce / folding (paradigms + data-model [3,1]) ----
    (2819, "accum-manual", "acumulador · manual", "Sumar con acumulador manual.",
     "**acumulador**\n\nRecorrer y plegar: guardás el resultado parcial en una variable.\n\n**Micro-reto:**\n1. `total = 0`\n2. `for x in [1, 2, 3]: total += x`\n3. `resultado = total`",
     "# total = 0\n# for x in [1, 2, 3]:\n#     total += x\n# resultado = total\n# print(resultado)\n",
     "assert ns['resultado'] == 6\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "6",
     "total = 0\nfor x in [1, 2, 3]:\n    total += x\nresultado = total\nprint(resultado)"),
    (2820, "fold-left", "pliegue · izquierda", "Plegar por la izquierda acumulando.",
     "**pliegue**\n\nReducís una secuencia a un valor aplicando una operación repetida.\n\n**Micro-reto:**\n1. `acc = 1`\n2. `for x in [2, 3]: acc *= x`\n3. `resultado = acc`",
     "# acc = 1\n# for x in [2, 3]:\n#     acc *= x\n# resultado = acc\n# print(resultado)\n",
     "assert ns['resultado'] == 6\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "6",
     "acc = 1\nfor x in [2, 3]:\n    acc *= x\nresultado = acc\nprint(resultado)"),
    (2821, "reduce-import", "reduce · import", "Usar functools.reduce.",
     "**functools.reduce**\n\n`reduce(func, iterable)` pliega de izquierda. Importalo de `functools`.\n\n**Micro-reto:**\n1. `from functools import reduce`\n2. `resultado = reduce(lambda a, b: a + b, [1, 2, 3])`\n3. Mostrá",
     "# from functools import reduce\n# resultado = reduce(lambda a, b: a + b, [1, 2, 3])\n# print(resultado)\n",
     "assert ns['resultado'] == 6\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "6",
     "from functools import reduce\nresultado = reduce(lambda a, b: a + b, [1, 2, 3])\nprint(resultado)"),
    (2822, "reduce-init", "reduce · initial", "Reduce con valor inicial.",
     "**initializer**\n\n`reduce(func, iterable, inicial)` arranca con un valor base.\n\n**Micro-reto:**\n1. `from functools import reduce`\n2. `resultado = reduce(lambda a, b: a * b, [2, 3], 10)`\n3. Mostrá",
     "# from functools import reduce\n# resultado = reduce(lambda a, b: a * b, [2, 3], 10)\n# print(resultado)\n",
     "assert ns['resultado'] == 60\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "60",
     "from functools import reduce\nresultado = reduce(lambda a, b: a * b, [2, 3], 10)\nprint(resultado)"),
    (2823, "reduce-op", "reduce · operación", "Reduce con operador combinado.",
     "**operaciones en reduce**\n\nPodés plegar strings, listas o máximos.\n\n**Micro-reto:**\n1. `from functools import reduce`\n2. `resultado = reduce(lambda a, b: a + '-' + b, ['a', 'b'])`\n3. Mostrá",
     "# from functools import reduce\n# resultado = reduce(lambda a, b: a + '-' + b, ['a', 'b'])\n# print(resultado)\n",
     "assert ns['resultado'] == 'a-b'\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "'a-b'",
     "from functools import reduce\nresultado = reduce(lambda a, b: a + '-' + b, ['a', 'b'])\nprint(resultado)"),
    (2824, "fold-check", "reduce · Suite", "Cerrar grupo: folding.",
     "**Suite folding**\n\nMáximo con `reduce`.\n\n**Micro-reto:**\n1. `from functools import reduce`\n2. `resultado = reduce(max, [3, 7, 1])`\n3. Mostrá",
     "# from functools import reduce\n# resultado = reduce(max, [3, 7, 1])\n# print(resultado)\n",
     "assert ns['resultado'] == 7\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "7",
     "from functools import reduce\nresultado = reduce(max, [3, 7, 1])\nprint(resultado)"),

    # ---- Grupo 5: 2825-2830 pipelines sobre logs (application-domains + data-model [5,1]) ----
    (2825, "log-split", "log · split", "Dividir líneas de un log.",
     "**pipelines de logs**\n\nLos logs son texto; `split` separa en líneas y campos.\n\n**Micro-reto:**\n1. `texto = 'INFO ok\\nERROR bad'`\n2. `resultado = texto.split('\\n')`\n3. Mostrá",
     "# texto = 'INFO ok\\nERROR bad'\n# resultado = texto.split('\\n')\n# print(resultado)\n",
     "assert ns['resultado'] == ['INFO ok', 'ERROR bad']\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "['INFO ok', 'ERROR bad']",
     "texto = 'INFO ok\\nERROR bad'\nresultado = texto.split('\\n')\nprint(resultado)"),
    (2826, "log-filter", "log · filter", "Filtrar log por nivel.",
     "**filtrar niveles**\n\n`filter` selecciona las líneas de un nivel dado.\n\n**Micro-reto:**\n1. `lineas = ['INFO ok', 'ERROR bad']`\n2. `resultado = list(filter(lambda l: l.startswith('ERROR'), lineas))`\n3. Mostrá",
     "# lineas = ['INFO ok', 'ERROR bad']\n# resultado = list(filter(lambda l: l.startswith('ERROR'), lineas))\n# print(resultado)\n",
     "assert ns['resultado'] == ['ERROR bad']\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "['ERROR bad']",
     "lineas = ['INFO ok', 'ERROR bad']\nresultado = list(filter(lambda l: l.startswith('ERROR'), lineas))\nprint(resultado)"),
    (2827, "log-map", "log · map", "Extraer un campo de cada línea.",
     "**extraer campo**\n\n`map` + `split(' ', 1)[0]` toma el nivel de cada registro.\n\n**Micro-reto:**\n1. `lineas = ['INFO ok', 'ERROR bad']`\n2. `resultado = list(map(lambda l: l.split(' ', 1)[0], lineas))`\n3. Mostrá",
     "# lineas = ['INFO ok', 'ERROR bad']\n# resultado = list(map(lambda l: l.split(' ', 1)[0], lineas))\n# print(resultado)\n",
     "assert ns['resultado'] == ['INFO', 'ERROR']\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "['INFO', 'ERROR']",
     "lineas = ['INFO ok', 'ERROR bad']\nresultado = list(map(lambda l: l.split(' ', 1)[0], lineas))\nprint(resultado)"),
    (2828, "log-pipeline", "log · pipeline", "Encadenar filter y map en logs.",
     "**pipeline de líneas**\n\nPrimero filtrá, luego transformá en cadena.\n\n**Micro-reto:**\n1. `lineas = ['INFO ok', 'ERROR bad']`\n2. Filtrá `ERROR`\n3. `resultado = list(map(lambda l: l.title(), ...))`",
     "# lineas = ['INFO ok', 'ERROR bad']\n# errores = filter(lambda l: l.startswith('ERROR'), lineas)\n# resultado = list(map(lambda l: l.title(), errores))\n# print(resultado)\n",
     "assert ns['resultado'] == ['Error Bad']\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "['Error Bad']",
     "lineas = ['INFO ok', 'ERROR bad']\nerrores = filter(lambda l: l.startswith('ERROR'), lineas)\nresultado = list(map(lambda l: l.title(), errores))\nprint(resultado)"),
    (2829, "log-count", "log · count", "Contar líneas por nivel.",
     "**conteo por nivel**\n\nSumá booleanos: `list.count` o `sum(gen)`.\n\n**Micro-reto:**\n1. `niveles = ['INFO', 'ERROR', 'INFO']`\n2. `resultado = niveles.count('INFO')`\n3. Mostrá",
     "# niveles = ['INFO', 'ERROR', 'INFO']\n# resultado = niveles.count('INFO')\n# print(resultado)\n",
     "assert ns['resultado'] == 2\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "2",
     "niveles = ['INFO', 'ERROR', 'INFO']\nresultado = niveles.count('INFO')\nprint(resultado)"),
    (2830, "log-check", "log · Suite", "Cerrar grupo: pipelines logs.",
     "**Suite logs**\n\nContá líneas que empiecen con `ERROR`.\n\n**Micro-reto:**\n1. `lineas = ['ERROR a', 'INFO b', 'ERROR c']`\n2. `resultado = sum(1 for l in lineas if l.startswith('ERROR'))`\n3. Mostrá",
     "# lineas = ['ERROR a', 'INFO b', 'ERROR c']\n# resultado = sum(1 for l in lineas if l.startswith('ERROR'))\n# print(resultado)\n",
     "assert ns['resultado'] == 2\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "2",
     "lineas = ['ERROR a', 'INFO b', 'ERROR c']\nresultado = sum(1 for l in lineas if l.startswith('ERROR'))\nprint(resultado)"),

    # ---- Grupo 6: 2831-2836 filter con predicados (paradigms + scope-legb [3,2]) ----
    (2831, "pred-range", "predicado · rango", "Filtrar por rango.",
     "**predicado de rango**\n\nLa condición puede combinar comparaciones.\n\n**Micro-reto:**\n1. `resultado = list(filter(lambda x: 1 <= x <= 3, [0, 2, 5]))`\n2. Mostrá",
     "# resultado = list(filter(lambda x: 1 <= x <= 3, [0, 2, 5]))\n# print(resultado)\n",
     "assert ns['resultado'] == [2]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[2]",
     "resultado = list(filter(lambda x: 1 <= x <= 3, [0, 2, 5]))\nprint(resultado)"),
    (2832, "pred-and", "predicado · and", "Combinar condiciones con and.",
     "**and**\n\nUn predicado con `and` exige que ambas condiciones se cumplan.\n\n**Micro-reto:**\n1. `datos = [2, 8, 5]`\n2. `resultado = list(filter(lambda x: x > 1 and x < 7, datos))`\n3. Mostrá",
     "# datos = [2, 8, 5]\n# resultado = list(filter(lambda x: x > 1 and x < 7, datos))\n# print(resultado)\n",
     "assert ns['resultado'] == [2, 5]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[2, 5]",
     "datos = [2, 8, 5]\nresultado = list(filter(lambda x: x > 1 and x < 7, datos))\nprint(resultado)"),
    (2833, "pred-or", "predicado · or", "Combinar condiciones con or.",
     "**or**\n\nUn predicado con `or` acepta si alguna condición se cumple.\n\n**Micro-reto:**\n1. `datos = [1, 3, 4]`\n2. `resultado = list(filter(lambda x: x == 1 or x == 3, datos))`\n3. Mostrá",
     "# datos = [1, 3, 4]\n# resultado = list(filter(lambda x: x == 1 or x == 3, datos))\n# print(resultado)\n",
     "assert ns['resultado'] == [1, 3]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[1, 3]",
     "datos = [1, 3, 4]\nresultado = list(filter(lambda x: x == 1 or x == 3, datos))\nprint(resultado)"),
    (2834, "pred-def", "predicado · def", "Predicado como función definida.",
     "**def como predicado**\n\nClaro para condiciones largas: escribí `def f(x): ...`.\n\n**Micro-reto:**\n1. `def es_vocal(c): return c in 'aeiou'`\n2. `resultado = list(filter(es_vocal, 'hola'))`\n3. Mostrá",
     "# def es_vocal(c): return c in 'aeiou'\n# resultado = list(filter(es_vocal, 'hola'))\n# print(resultado)\n",
     "assert ns['resultado'] == ['o', 'a']\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "['o', 'a']",
     "def es_vocal(c): return c in 'aeiou'\nresultado = list(filter(es_vocal, 'hola'))\nprint(resultado)"),
    (2835, "pred-callable", "predicado · callable", "Filtrar con función callable.",
     "**truthiness**\n\n`filter` conserva los elementos donde el callable devuelve verdadero.\n\n**Micro-reto:**\n1. `resultado = list(filter(lambda s: len(s) > 1, ['a', 'ab']))`\n2. Mostrá",
     "# resultado = list(filter(lambda s: len(s) > 1, ['a', 'ab']))\n# print(resultado)\n",
     "assert ns['resultado'] == ['ab']\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "['ab']",
     "resultado = list(filter(lambda s: len(s) > 1, ['a', 'ab']))\nprint(resultado)"),
    (2836, "pred-check", "predicado · Suite", "Cerrar grupo: predicados.",
     "**Suite predicados**\n\nFiltrá números terminados en 0.\n\n**Micro-reto:**\n1. `datos = [10, 3, 20]`\n2. `resultado = list(filter(lambda x: x % 10 == 0, datos))`\n3. Mostrá",
     "# datos = [10, 3, 20]\n# resultado = list(filter(lambda x: x % 10 == 0, datos))\n# print(resultado)\n",
     "assert ns['resultado'] == [10, 20]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[10, 20]",
     "datos = [10, 3, 20]\nresultado = list(filter(lambda x: x % 10 == 0, datos))\nprint(resultado)"),

    # ---- Grupo 7: 2837-2842 mapped/serial ETL (application-domains + paradigms [5,3]) ----
    (2837, "etl-mapper", "ETL · mapper", "Definir una función de mapeo.",
     "**ETL en memoria**\n\nExtraer, transformar, cargar: la transformación es mapear filas.\n\n**Micro-reto:**\n1. `def limpio(x): return int(x)`\n2. `resultado = list(map(limpio, ['1', '2']))`\n3. Mostrá",
     "# def limpio(x): return int(x)\n# resultado = list(map(limpio, ['1', '2']))\n# print(resultado)\n",
     "assert ns['resultado'] == [1, 2]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[1, 2]",
     "def limpio(x): return int(x)\nresultado = list(map(limpio, ['1', '2']))\nprint(resultado)"),
    (2838, "etl-map", "ETL · map", "Aplicar transformación a filas.",
     "**map en ETL**\n\nNormalizá cada fila con `map` y una lambda.\n\n**Micro-reto:**\n1. `filas = [1, 2, 3]`\n2. `resultado = list(map(lambda x: x + 1, filas))`\n3. Mostrá",
     "# filas = [1, 2, 3]\n# resultado = list(map(lambda x: x + 1, filas))\n# print(resultado)\n",
     "assert ns['resultado'] == [2, 3, 4]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[2, 3, 4]",
     "filas = [1, 2, 3]\nresultado = list(map(lambda x: x + 1, filas))\nprint(resultado)"),
    (2839, "etl-filter", "ETL · filter", "Limpiar datos inválidos.",
     "**filtrar inválidos**\n\n`filter` descarta filas que no cumplen el negocio.\n\n**Micro-reto:**\n1. `filas = [0, 5, -1]`\n2. `resultado = list(filter(lambda x: x > 0, filas))`\n3. Mostrá",
     "# filas = [0, 5, -1]\n# resultado = list(filter(lambda x: x > 0, filas))\n# print(resultado)\n",
     "assert ns['resultado'] == [5]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[5]",
     "filas = [0, 5, -1]\nresultado = list(filter(lambda x: x > 0, filas))\nprint(resultado)"),
    (2840, "etl-tuple", "ETL · tuplas", "Normalizar a tuplas.",
     "**tuplas normalizadas**\n\nConvertí entradas crudas en tuplas con `map`.\n\n**Micro-reto:**\n1. `filas = ['a:1', 'b:2']`\n2. `resultado = list(map(lambda s: tuple(s.split(':')), filas))`\n3. Mostrá",
     "# filas = ['a:1', 'b:2']\n# resultado = list(map(lambda s: tuple(s.split(':')), filas))\n# print(resultado)\n",
     "assert ns['resultado'] == [('a', '1'), ('b', '2')]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[('a', '1'), ('b', '2')]",
     "filas = ['a:1', 'b:2']\nresultado = list(map(lambda s: tuple(s.split(':')), filas))\nprint(resultado)"),
    (2841, "etl-serial", "ETL · serial", "ETL completo en memoria.",
     "**ETL serial**\n\nExtraer -> limpiar -> transformar en secuencia.\n\n**Micro-reto:**\n1. `crudo = [1, -2, 3, -4]`\n2. Filtrá positivos, luego elevá al cuadrado\n3. `resultado = list(...)`",
     "# crudo = [1, -2, 3, -4]\n# positivos = filter(lambda x: x > 0, crudo)\n# resultado = list(map(lambda x: x ** 2, positivos))\n# print(resultado)\n",
     "assert ns['resultado'] == [1, 9]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[1, 9]",
     "crudo = [1, -2, 3, -4]\npositivos = filter(lambda x: x > 0, crudo)\nresultado = list(map(lambda x: x ** 2, positivos))\nprint(resultado)"),
    (2842, "etl-check", "ETL · Suite", "Cerrar grupo: ETL serial.",
     "**Suite ETL**\n\nSumá los cuadrados de los pares.\n\n**Micro-reto:**\n1. `datos = [1, 2, 3]`\n2. `resultado = sum(x * x for x in datos if x % 2 == 0)`\n3. Mostrá",
     "# datos = [1, 2, 3]\n# resultado = sum(x * x for x in datos if x % 2 == 0)\n# print(resultado)\n",
     "assert ns['resultado'] == 4\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "4",
     "datos = [1, 2, 3]\nresultado = sum(x * x for x in datos if x % 2 == 0)\nprint(resultado)"),

    # ---- Grupo 8: 2843-2848 early termination (paradigms + ecosystem [3,4]) ----
    (2843, "any-find", "any() · early", "Usar any() para corte temprano.",
     "**any()**\n\nDevuelve `True` al primer elemento verdadero; corta la iteración.\n\n**Micro-reto:**\n1. `resultado = any(x > 5 for x in [1, 9, 2])`\n2. Mostrá",
     "# resultado = any(x > 5 for x in [1, 9, 2])\n# print(resultado)\n",
     "assert ns['resultado'] is True\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "True",
     "resultado = any(x > 5 for x in [1, 9, 2])\nprint(resultado)"),
    (2844, "all-check", "all() · check", "Usar all() en un stream.",
     "**all()**\n\nDevuelve `False` al primer falso: corte temprano.\n\n**Micro-reto:**\n1. `resultado = all(x > 0 for x in [1, 2, -1])`\n2. Mostrá",
     "# resultado = all(x > 0 for x in [1, 2, -1])\n# print(resultado)\n",
     "assert ns['resultado'] is False\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "False",
     "resultado = all(x > 0 for x in [1, 2, -1])\nprint(resultado)"),
    (2845, "first-next", "first · next", "Tomar el primer match con next.",
     "**primer match**\n\n`next(gen)` consume solo lo necesario hasta hallar el elemento.\n\n**Micro-reto:**\n1. `datos = [1, 4, 7]`\n2. `resultado = next(x for x in datos if x % 2 == 0)`\n3. Mostrá",
     "# datos = [1, 4, 7]\n# resultado = next(x for x in datos if x % 2 == 0)\n# print(resultado)\n",
     "assert ns['resultado'] == 4\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "4",
     "datos = [1, 4, 7]\nresultado = next(x for x in datos if x % 2 == 0)\nprint(resultado)"),
    (2846, "takewhile", "takewhile", "Cortar al inicio con itertools.takewhile.",
     "**takewhile**\n\n`itertools.takewhile(pred, it)` toma mientras el predicado sea verdadero.\n\n**Micro-reto:**\n1. `from itertools import takewhile`\n2. `resultado = list(takewhile(lambda x: x < 3, [1, 2, 3, 1]))`\n3. Mostrá",
     "# from itertools import takewhile\n# resultado = list(takewhile(lambda x: x < 3, [1, 2, 3, 1]))\n# print(resultado)\n",
     "assert ns['resultado'] == [1, 2]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[1, 2]",
     "from itertools import takewhile\nresultado = list(takewhile(lambda x: x < 3, [1, 2, 3, 1]))\nprint(resultado)"),
    (2847, "dropwhile", "dropwhile", "Saltar cabecera con itertools.dropwhile.",
     "**dropwhile**\n\nDescarta mientras el predicado sea verdadero; luego emite el resto.\n\n**Micro-reto:**\n1. `from itertools import dropwhile`\n2. `resultado = list(dropwhile(lambda x: x < 3, [1, 5, 2]))`\n3. Mostrá",
     "# from itertools import dropwhile\n# resultado = list(dropwhile(lambda x: x < 3, [1, 5, 2]))\n# print(resultado)\n",
     "assert ns['resultado'] == [5, 2]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[5, 2]",
     "from itertools import dropwhile\nresultado = list(dropwhile(lambda x: x < 3, [1, 5, 2]))\nprint(resultado)"),
    (2848, "early-check", "early · Suite", "Cerrar grupo: early termination.",
     "**Suite early**\n\n¿Hay un múltiplo de 7?\n\n**Micro-reto:**\n1. `datos = [1, 2, 14, 3]`\n2. `resultado = any(x % 7 == 0 for x in datos)`\n3. Mostrá",
     "# datos = [1, 2, 14, 3]\n# resultado = any(x % 7 == 0 for x in datos)\n# print(resultado)\n",
     "assert ns['resultado'] is True\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "True",
     "datos = [1, 2, 14, 3]\nresultado = any(x % 7 == 0 for x in datos)\nprint(resultado)"),

    # ---- Grupo 9: 2849-2854 agregación streaming (paradigms + data-model [3,1]) ----
    (2849, "stream-sum", "stream · sum", "Sumar elementos al vuelo.",
     "**sum en streaming**\n\n`sum(gen)` consume de a uno; no arma lista intermedia.\n\n**Micro-reto:**\n1. `resultado = sum(x * 2 for x in range(3))`\n2. Mostrá",
     "# resultado = sum(x * 2 for x in range(3))\n# print(resultado)\n",
     "assert ns['resultado'] == 6\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "6",
     "resultado = sum(x * 2 for x in range(3))\nprint(resultado)"),
    (2850, "stream-max", "stream · max", "Calcular máximo sin lista.",
     "**renumeración**\n\n`max(gen)` recuerda el mayor visto sin guardar todo.\n\n**Micro-reto:**\n1. `datos = [3, 9, 1]`\n2. `resultado = max(x for x in datos if x < 9)`\n3. Mostrá",
     "# datos = [3, 9, 1]\n# resultado = max(x for x in datos if x < 9)\n# print(resultado)\n",
     "assert ns['resultado'] == 3\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "3",
     "datos = [3, 9, 1]\nresultado = max(x for x in datos if x < 9)\nprint(resultado)"),
    (2851, "window-sum", "window · sum", "Sumar ventana deslizante.",
     "**ventana deslizante**\n\nSumá un bloque contiguo de tamaño fijo.\n\n**Micro-reto:**\n1. `datos = [1, 2, 3, 4]`\n2. `resultado = sum(datos[1:3])`\n3. Mostrá",
     "# datos = [1, 2, 3, 4]\n# resultado = sum(datos[1:3])\n# print(resultado)\n",
     "assert ns['resultado'] == 5\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "5",
     "datos = [1, 2, 3, 4]\nresultado = sum(datos[1:3])\nprint(resultado)"),
    (2852, "stream-avg", "stream · avg", "Promedio en un paso.",
     "**promedio streaming**\n\n`sum` y `len` combinados dan el promedio.\n\n**Micro-reto:**\n1. `datos = [2, 4, 6]`\n2. `resultado = sum(datos) / len(datos)`\n3. Mostrá",
     "# datos = [2, 4, 6]\n# resultado = sum(datos) / len(datos)\n# print(resultado)\n",
     "assert ns['resultado'] == 4.0\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "4.0",
     "datos = [2, 4, 6]\nresultado = sum(datos) / len(datos)\nprint(resultado)"),
    (2853, "stream-count", "stream · count", "Contar cumple-criterios.",
     "**conteo**\n\n`sum(1 for ... if ...)` cuenta sin materializar.\n\n**Micro-reto:**\n1. `datos = [1, 2, 3]`\n2. `resultado = sum(1 for x in datos if x % 2)`\n3. Mostrá",
     "# datos = [1, 2, 3]\n# resultado = sum(1 for x in datos if x % 2)\n# print(resultado)\n",
     "assert ns['resultado'] == 2\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "2",
     "datos = [1, 2, 3]\nresultado = sum(1 for x in datos if x % 2)\nprint(resultado)"),
    (2854, "stream-check", "stream · Suite", "Cerrar grupo: agregación streaming.",
     "**Suite streaming**\n\nSumá los valores mayores a 2, sin lista.\n\n**Micro-reto:**\n1. `datos = [1, 3, 2, 4]`\n2. `resultado = sum(x for x in datos if x > 2)`\n3. Mostrá",
     "# datos = [1, 3, 2, 4]\n# resultado = sum(x for x in datos if x > 2)\n# print(resultado)\n",
     "assert ns['resultado'] == 7\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "7",
     "datos = [1, 3, 2, 4]\nresultado = sum(x for x in datos if x > 2)\nprint(resultado)"),

    # ---- Grupo 10: 2855-2860 pipeline de scoring (application-domains + paradigms [5,3]) ----
    (2855, "score-rows", "scoring · filas", "Preparar filas de datos.",
     "**scoring**\n\nLos pipelines puntúan registros: primero ordená las filas.\n\n**Micro-reto:**\n1. `filas = [('a', 3), ('b', 1)]`\n2. `resultado = sorted(filas, key=lambda t: t[1])`\n3. Mostrá",
     "# filas = [('a', 3), ('b', 1)]\n# resultado = sorted(filas, key=lambda t: t[1])\n# print(resultado)\n",
     "assert ns['resultado'] == [('b', 1), ('a', 3)]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[('b', 1), ('a', 3)]",
     "filas = [('a', 3), ('b', 1)]\nresultado = sorted(filas, key=lambda t: t[1])\nprint(resultado)"),
    (2856, "score-map", "scoring · map", "Calcular puntaje con map.",
     "**calcular puntaje**\n\n`map` transforma cada fila en su valor de score.\n\n**Micro-reto:**\n1. `datos = [1, 2]`\n2. `resultado = list(map(lambda x: x * 10, datos))`\n3. Mostrá",
     "# datos = [1, 2]\n# resultado = list(map(lambda x: x * 10, datos))\n# print(resultado)\n",
     "assert ns['resultado'] == [10, 20]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[10, 20]",
     "datos = [1, 2]\nresultado = list(map(lambda x: x * 10, datos))\nprint(resultado)"),
    (2857, "score-filter", "scoring · filter", "Filtrar por umbral del score.",
     "**umbral**\n\n`filter` descarta registros bajo un puntaje mínimo.\n\n**Micro-reto:**\n1. `scores = [5, 8, 3]`\n2. `resultado = list(filter(lambda s: s >= 5, scores))`\n3. Mostrá",
     "# scores = [5, 8, 3]\n# resultado = list(filter(lambda s: s >= 5, scores))\n# print(resultado)\n",
     "assert ns['resultado'] == [5, 8]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[5, 8]",
     "scores = [5, 8, 3]\nresultado = list(filter(lambda s: s >= 5, scores))\nprint(resultado)"),
    (2858, "score-sort", "scoring · sort", "Ranking ordenado de scores.",
     "**ranking**\n\n`sorted(..., reverse=True)` ordena de mayor a menor.\n\n**Micro-reto:**\n1. `scores = [3, 9, 5]`\n2. `resultado = sorted(scores, reverse=True)`\n3. Mostrá",
     "# scores = [3, 9, 5]\n# resultado = sorted(scores, reverse=True)\n# print(resultado)\n",
     "assert ns['resultado'] == [9, 5, 3]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[9, 5, 3]",
     "scores = [3, 9, 5]\nresultado = sorted(scores, reverse=True)\nprint(resultado)"),
    (2859, "score-top", "scoring · top", "Ranking con top N.",
     "**top N**\n\nOrdená y cortá con slicing para el podio.\n\n**Micro-reto:**\n1. `scores = [4, 1, 9, 2]`\n2. `resultado = sorted(scores, reverse=True)[:3]`\n3. Mostrá",
     "# scores = [4, 1, 9, 2]\n# resultado = sorted(scores, reverse=True)[:3]\n# print(resultado)\n",
     "assert ns['resultado'] == [9, 4, 2]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[9, 4, 2]",
     "scores = [4, 1, 9, 2]\nresultado = sorted(scores, reverse=True)[:3]\nprint(resultado)"),
    (2860, "score-check", "scoring · Suite", "Cerrar ola: pipeline de ranking.",
     "**Suite scoring**\n\nTop 2 de los mayores a 3.\n\n**Micro-reto:**\n1. `scores = [2, 9, 4, 1]`\n2. Filtrá `> 3`, ordená desc, tomá top 2\n3. `resultado = ...`",
     "# scores = [2, 9, 4, 1]\n# aptos = filter(lambda s: s > 3, scores)\n# resultado = sorted(aptos, reverse=True)[:2]\n# print(resultado)\n",
     "assert ns['resultado'] == [9, 4]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
     "[9, 4]",
     "scores = [2, 9, 4, 1]\naptos = filter(lambda s: s > 3, scores)\nresultado = sorted(aptos, reverse=True)[:2]\nprint(resultado)"),
]


def build_raw(entries):
    """Convierte tuplas RAW -> list[dict] completas (sin next)."""
    out = []
    for e in entries:
        num, slug, title, objective, prompt, starter, pytest_tail, hint, solution = e
        test_name = slug.replace("-", "_")
        # Aspasan las ``\\n`` literales escritas como 2 chars a newlines reales,
        # de modo que el string Rust use ``\n`` (escape válido) y el pytest Python
        # compile con saltos de línea reales.
        pytest_tail = pytest_tail.replace("\\n", "\n")
        pytest = (
            "def test_{}(capsys):\n"
            "    ns = {{}}\n"
            "    exec(open('solution.py', encoding='utf-8').read(), ns)\n"
            "    {}\n".format(test_name, pytest_tail)
        )
        out.append(
            step(num, slug, title, objective, prompt, starter, pytest, hint, solution)
        )
    return out


def _rust_escape(s):
    """Convierte una cadena Python a la representación literal de Rust.

    - newline real  -> ``\\n`` (backslash + n, 2 chars) dentro del string Rust
    - doble backslash (de tests como ``\\n`` que son escapes de regex/str) -> ``\\\\n``
    - comillas simples se mantienen; dobles se escapan.
    """
    out = s.replace("\\", "\\\\")
    out = out.replace("\n", "\\n")
    out = out.replace("\"", "\\\"")
    return out


def emit_rust(step_list):
    """Emit la lista de CodingStep en Rust. El `next` enlaza al siguiente paso."""
    rust = []
    for i, s in enumerate(step_list):
        const_name = "PY{}_{}".format(s["num"], s["slug"].upper().replace("-", "_"))
        nxt = step_list[i + 1]["num"] if i + 1 < len(step_list) else None
        next_expr = 'Some("py-{}-{}")'.format(
            step_list[i + 1]["num"], step_list[i + 1]["slug"]
        ) if nxt else "None"
        block = (
            f'pub const {const_name}: CodingStep = CodingStep {{\n'
            f'    id: "py-{s["num"]}-{s["slug"]}", title: "{_rust_escape(s["title"])}", objective: "{_rust_escape(s["objective"])}",\n'
            f'    prompt_md: "{_rust_escape(s["prompt"])}",\n'
            f'    starter_code: "{_rust_escape(s["starter"])}",\n'
            f'    pytest: "{_rust_escape(s["pytest"])}",\n'
            f'    hint: "{_rust_escape(s["hint"])}",\n'
            f'    solution_example: "{_rust_escape(s["solution"])}",\n'
            f'    next: {next_expr}, show_type_chips: false, micro_step: {s["num"]},\n'
            f'}};'
        )
        rust.append(block)
    return "\n".join(rust)


def emit_refs(step_list):
    """Emite las líneas de referencias para el array CODING_STEPS."""
    return "\n".join(
        "    &PY{}_{},".format(s["num"], s["slug"].upper().replace("-", "_"))
        for s in step_list
    )


if __name__ == "__main__":
    import sys
    steps = build_raw(RAW)
    if len(sys.argv) > 1 and sys.argv[1] == "--refs":
        print(emit_refs(steps))
    else:
        print(emit_rust(steps))