"""Genera los micro-steps 2441-2500 de la Ola 25 (Pipelines avanzados y reducción).

Emite el bloque Rust de 60 `CodingStep` listo para insertar en `web/src/curriculum.rs`.

Reglas: stdlib-only, Wasm-safe, determinista (cero red/threads), 10x6,
explain->try->check, <=120 palabras, sin `input()`, pytest asserts con `resultado`.
"""

    10|def step(num, slug, title, objective, prompt, starter, pytest, hint, solution):
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


    20|# (num, slug, title, objective, prompt, starter, pytest_tail, hint, solution)
   50|RAW = [
   100|    # ---- Grupo 1: 2441-2446 map y filter con lambdas (paradigms [3]) ----
   110|    (2441, "map-lambda", "map · lambda", "Aplicar map(lambda, ...) a una lista.",
   120|     "**map(lambda, iterable)**\n\n`map` aplica una función a cada elemento sin escribir un `for`.\n\n**Micro-reto:**\n1. `resultado = list(map(lambda x: x * 2, [1, 2, 3]))`\n2. Mostrá",
   130|     "# resultado = list(map(lambda x: x * 2, [1, 2, 3]))\n# print(resultado)\n",
   140|     "assert ns['resultado'] == [2, 4, 6]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
   150|     "[2, 4, 6]",
   160|     "resultado = list(map(lambda x: x * 2, [1, 2, 3]))\nprint(resultado)"),
   170|    (2442, "filter-lambda", "filter · lambda", "Filtrar con filter(lambda, ...).",
   180|     "**filter(lambda, iterable)**\n\n`filter` conserva solo los elementos que cumplen el predicado.\n\n**Micro-reto:**\n1. `resultado = list(filter(lambda x: x > 2, [1, 2, 3]))`\n2. Mostrá",
   190|     "# resultado = list(filter(lambda x: x > 2, [1, 2, 3]))\n# print(resultado)\n",
   200|     "assert ns['resultado'] == [3]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
   210|     "[3]",
   220|     "resultado = list(filter(lambda x: x > 2, [1, 2, 3]))\nprint(resultado)"),
   230|    (2443, "mapfilter-combo", "map · filter combinado", "Encadenar filter y map.",
   240|     "**Combo funcional**\n\nPodés filtrar y transformar en cadena: primer `filter`, luego `map`.\n\n**Micro-reto:**\n1. De [1, 2, 3, 4] filtrá pares\n2. Elevá al cuadrado con `map`\n3. `resultado = list(...)`",
   250|     "# paso = filter(lambda x: x % 2 == 0, [1, 2, 3, 4])\n# resultado = list(map(lambda x: x * x, paso))\n# print(resultado)\n",
   260|     "assert ns['resultado'] == [4, 16]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
   270|     "[4, 16]",
   280|     "pares = filter(lambda x: x % 2 == 0, [1, 2, 3, 4])\nresultado = list(map(lambda x: x * x, pares))\nprint(resultado)"),
   290|    (2444, "lambda-sorted", "lambda · sorted", "Usar lambda como clave de orden.",
   300|     "**sorted(key=lambda)**\n\n`key=` define el criterio sin cambiar los valores.\n\n**Micro-reto:**\n1. `datos = [(1, 'b'), (3, 'a')]`\n2. `resultado = sorted(datos, key=lambda t: t[1])`\n3. Mostrá",
   310|     "# datos = [(1, 'b'), (3, 'a')]\n# resultado = sorted(datos, key=lambda t: t[1])\n# print(resultado)\n",
   320|     "assert ns['resultado'] == [(3, 'a'), (1, 'b')]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
   330|     "[(3, 'a'), (1, 'b')]",
   340|     "datos = [(1, 'b'), (3, 'a')]\nresultado = sorted(datos, key=lambda t: t[1])\nprint(resultado)"),
   350|    (2445, "map-multi", "map · varios iterables", "Aplicar map con dos listas.",
   360|     "**map multi-iterable**\n\n`map` acepta varios iterables: combina por posición.\n\n**Micro-reto:**\n1. `a = [1, 2]; b = [10, 20]`\n2. `resultado = list(map(lambda x, y: x + y, a, b))`\n3. Mostrá",
   370|     "# a = [1, 2]; b = [10, 20]\n# resultado = list(map(lambda x, y: x + y, a, b))\n# print(resultado)\n",
   380|     "assert ns['resultado'] == [11, 22]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
   390|     "[11, 22]",
   400|     "a = [1, 2]; b = [10, 20]\nresultado = list(map(lambda x, y: x + y, a, b))\nprint(resultado)"),
   410|    (2446, "mapfilter-check", "map/filter · Suite", "Cerrar grupo: map + filter.",
   420|     "**Suite funcional**\n\nFiltrá negativos y duplicá.\n\n**Micro-reto:**\n1. `datos = [1, -2, 3]`\n2. Filtrá los positivos\n3. `map` para duplicar; `resultado = list(...)`",
   430|     "# datos = [1, -2, 3]\n# positivos = filter(lambda x: x > 0, datos)\n# resultado = list(map(lambda x: x * 2, positivos))\n# print(resultado)",
   440|     "assert ns['resultado'] == [2, 6]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
   450|     "[2, 6]",
   460|     "datos = [1, -2, 3]\npositivos = filter(lambda x: x > 0, datos)\nresultado = list(map(lambda x: x * 2, positivos))\nprint(resultado)"),
   470|
   480|   # ---- Grupo 2: 2447-2452 funciones como datos / callbacks (paradigms + scope-legb [3,2]) ----
   490|    (2447, "fun-dato", "funciones · como datos", "Guardar una función en una variable.",
   500|     "**funciones como valores**\n\nEn Python las funciones son objetos: se asignan y llaman con `()`.\n\n**Micro-reto:**\n1. `def doble(x): return x * 2`\n2. `f = doble`\n3. `resultado = f(5)`",
   510|     "# def doble(x): return x * 2\n# f = doble\n# resultado = f(5)\n# print(resultado)\n",
   520|     "assert ns['resultado'] == 10\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
   530|     "10",
   540|     "def doble(x): return x * 2\nf = doble\nresultado = f(5)\nprint(resultado)"),
   550|    (2448, "sort-callback", "callback · sorted", "Usar una función como callback.",
   560|     "**callback**\n\n`key=` puede ser una función definida, no solo lambda.\n\n**Micro-reto:**\n1. `def largo(s): return len(s)`\n2. `resultado = sorted(['aa', 'b'], key=largo)`\n3. Mostrá",
   570|     "# def largo(s): return len(s)\n# resultado = sorted(['aa', 'b'], key=largo)\n# print(resultado)\n",
   580|     "assert ns['resultado'] == ['b', 'aa']\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
   590|     "['b', 'aa']",
   600|     "def largo(s): return len(s)\nresultado = sorted(['aa', 'b'], key=largo)\nprint(resultado)"),
   610|    (2449, "callback-filter", "callback · filter", "Pasar una función definida a filter.",
   620|     "**callback nombrado**\n\nUna función nombrada funciona igual que una lambda en `filter`.\n\n**Micro-reto:**\n1. `def es_par(x): return x % 2 == 0`\n2. `resultado = list(filter(es_par, [1, 2, 3, 4]))`\n3. Mostrá",
   630|     "# def es_par(x): return x % 2 == 0\n# resultado = list(filter(es_par, [1, 2, 3, 4]))\n# print(resultado)\n",
   640|     "assert ns['resultado'] == [2, 4]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
   650|     "[2, 4]",
   660|     "def es_par(x): return x % 2 == 0\nresultado = list(filter(es_par, [1, 2, 3, 4]))\nprint(resultado)"),
   670|    (2450, "higher-order", "orden · superior", "Escribir una función que reciba función.",
   680|     "**función de orden superior**\n\nUna función que recibe otra se llama de orden superior.\n\n**Micro-reto:**\n1. `def aplicar(f, n): return f(n)`\n2. `resultado = aplicar(lambda x: x + 1, 4)`\n3. Mostrá",
   690|     "# def aplicar(f, n): return f(n)\n# resultado = aplicar(lambda x: x + 1, 4)\n# print(resultado)\n",
   700|     "assert ns['resultado'] == 5\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
   710|     "5",
   720|     "def aplicar(f, n): return f(n)\nresultado = aplicar(lambda x: x + 1, 4)\nprint(resultado)"),
   730|   # ---- Grupo 3: 2453-2458 encadenar generadores lazy (paradigms + ecosystem [3,4]) ----
   740|    (2451, "callback-inline", "callback · inline", "Usar lambda como argumento.",
   750|     "**lambda como argumento**\n\nPasar una lambda directo a `sorted`/`filter`.\n\n**Micro-reto:**\n1. `resultado = sorted([3, 1, 2], key=lambda x: -x)`\n2. Mostrá",
   760|     "# resultado = sorted([3, 1, 2], key=lambda x: -x)\n# print(resultado)\n",
   770|     "assert ns['resultado'] == [3, 2, 1]\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
   780|     "[3, 2, 1]",
   780|     "resultado = sorted([3, 1, 2], key=lambda x: -x)\nprint(resultado)"),
   790|    (2452, "callback-check", "callback · Suite", "Cerrar grupo: callbacks.",
   800|     "**Suite callbacks**\n\nCada palabra por longitud, descendente.\n\n**Micro-reto:**\n1. `palabras = ['hola', 'y', 'mundo']`\n2. `sorted(palabras, key=len, reverse=True)`\n3. `resultado = ...`",
   800|     "# palabras = ['hola', 'y', 'mundo']\n# resultado = sorted(palabras, key=len, reverse=True)\n# print(resultado)",
   810|     "assert ns['resultado'] == ['mundo', 'hola', 'y']\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
   820|     "['mundo', 'hola', 'y']",
   830|     "palabras = ['hola', 'y', 'mundo']\nresultado = sorted(palabras, key=len, reverse=True)\nprint(resultado)"),
   830|  # ---- Grupo 4: 2459-2464 reduce / folding (paradigms + data-model [3,1]) ----
   840|    (2453, "accum-manual", "acumulador · manual", "Sumar con acumulador manual.",
   850|     "**acumulador**\n\nRecorrer y plegar: guardás el resultado parcial en una variable.\n\n**Micro-reto:**\n1. `total = 0`\n2. `for x in [1, 2, 3]: total += x`\n3. `resultado = total`",
   860|     "# total = 0\n# for x in [1, 2, 3]:\n#     total += x\n# resultado = total\n# print(resultado)",
   870|     "assert ns['resultado'] == 6\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
   880|     "6",
   890|     "total = 0\nfor x in [1, 2, 3]:\n    total += x\nresultado = total\nprint(resultado)"),
   900|   # ---- Grupo 5: 2465-2470 pipelines sobre logs (application-domains + data-model [5,1]) ----
   910|   (2465, "log-split", "log · split", "Dividir líneas de un log.",
   920|    "**pipelines de logs**\n\nLos logs son texto; `split` separa en líneas y campos.\n\n**Micro-reto:**\n1. `texto = 'INFO ok\\nERROR bad'`\n2. `resultado = texto.split('\\n')`\n3. Mostrá",
   930|    "# texto = 'INFO ok\\nERROR bad'\n# resultado = texto.split('\\n')\n# print(resultado)",
   940|    "assert ns['resultado'] == ['INFO ok', 'ERROR bad']\\n    assert capsys.readouterr().out.strip() == str(ns['resultado'])",
   950|    "['INFO ok', 'ERROR bad']",
   960|    "texto = 'INFO ok\\nERROR bad'\nresultado = texto.split('\\n')\nprint(resultado)"),
   970|   # ---- Grupo 6: 2471-2476 filter con predicados (paradigms + scope-legb [3,2]) ----
   980|   (2471, "pred-range", "predicado · rango", "Filtrar por rango.",
   990|    "**predicado de rango**\n\nLa condición puede combinar comparaciones.\n\n**Micro-reto:**\n1. `resultado = list(filter(lambda x: 1 <= x <= 3, [0, 2, 5]))`\n2. Mostrá",
  1000|   # ---- Grupo 7: 2477-2482 mapped/serial ETL (application-domains + paradigms [5,3]) ----
  1010|  # ---- Grupo 8: 2483-2488 early termination (paradigms + ecosystem [3,4]) ----
  1020|  # ---- Grupo 9: 2489-2494 agregación streaming (paradigms + data-model [3,1]) ----
  1030|  # ---- Grupo 10: 2495-2500 pipeline de scoring (application-domains + paradigms [5,3]) ----
  1040|   (2495, "score-rows", "scoring · filas", "Preparar filas de datos.",
  1050|    "**scoring**\n\nLos pipelines puntúan registros: primero ordená las filas.\n\n**Micro-reto:**\n1. `filas = [('a', 3), ('b', 1)]`\n2. `resultado = sorted(filas, key=lambda t: t[1])`\n3. Mostrá",
  1060|   # ---- Grupo 11: 2501-2505 ??? ----
  1070|  ]