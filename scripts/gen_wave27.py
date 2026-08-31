"""Generate the 60 original recursion/comprehension exercises in Wave 27."""


def exercise(num, slug, title, objective, solution, expected):
    prompt = (
        f"**{title}**\n\n{objective}\n\n"
        "**Micro-reto:** completá la función o comprensión indicada, guardá el valor "
        "final en `resultado` y mostralo."
    )
    starter = "\n".join(f"# {line}" for line in solution.splitlines()) + "\n"
    test_name = slug.replace("-", "_")
    pytest = (
        f"def test_{test_name}(capsys):\n"
        "    ns = {}\n"
        "    exec(open('solution.py', encoding='utf-8').read(), ns)\n"
        f"    assert ns['resultado'] == {expected!r}\n"
        "    assert capsys.readouterr().out.strip() == str(ns['resultado'])\n"
    )
    return {
        "num": num, "slug": slug, "title": title, "objective": objective,
        "prompt": prompt, "starter": starter, "pytest": pytest,
        "hint": f"El resultado esperado es {expected!r}.", "solution": solution,
    }


# Six exercises per family. Recursive inputs are deliberately small and bounded.
CASES = [
    # 1. Explicit base cases
    ("base-cero", "recursión · caso cero", "Definir un caso base para n igual a cero.", "def cuenta(n):\n    if n == 0:\n        return 'fin'\n    return cuenta(n - 1)\nresultado = cuenta(3)\nprint(resultado)", "fin"),
    ("base-lista-vacia", "recursión · lista vacía", "Detener el recorrido al llegar a una lista vacía.", "def largo(xs):\n    if not xs:\n        return 0\n    return 1 + largo(xs[1:])\nresultado = largo(['a', 'b', 'c'])\nprint(resultado)", 3),
    ("base-un-caracter", "recursión · un carácter", "Reconocer el caso indivisible de una cadena.", "def primero(s):\n    if len(s) == 1:\n        return s\n    return primero(s[:-1])\nresultado = primero('casa')\nprint(resultado)", "c"),
    ("base-uno", "recursión · caso uno", "Usar uno como caso base multiplicativo.", "def potencia_dos(n):\n    if n == 0:\n        return 1\n    return 2 * potencia_dos(n - 1)\nresultado = potencia_dos(4)\nprint(resultado)", 16),
    ("base-bool", "recursión · predicado base", "Cerrar un predicado recursivo sin consumir de más.", "def todos_positivos(xs):\n    if not xs:\n        return True\n    return xs[0] > 0 and todos_positivos(xs[1:])\nresultado = todos_positivos([2, 5, 1])\nprint(resultado)", True),
    ("base-suite", "recursión · suite base", "Combinar caso base y paso de reducción.", "def bajar(n):\n    if n <= 0:\n        return []\n    return [n] + bajar(n - 1)\nresultado = bajar(4)\nprint(resultado)", [4, 3, 2, 1]),
    # 2. Recursive sums and products
    ("suma-natural", "recursión · suma natural", "Sumar desde n hasta el caso base.", "def suma(n):\n    if n == 0:\n        return 0\n    return n + suma(n - 1)\nresultado = suma(5)\nprint(resultado)", 15),
    ("suma-lista", "recursión · suma lista", "Reducir una lista por cabeza y cola.", "def sumar(xs):\n    if not xs:\n        return 0\n    return xs[0] + sumar(xs[1:])\nresultado = sumar([4, 2, 7])\nprint(resultado)", 13),
    ("producto-lista", "recursión · producto", "Multiplicar una secuencia con identidad uno.", "def producto(xs):\n    if not xs:\n        return 1\n    return xs[0] * producto(xs[1:])\nresultado = producto([2, 3, 4])\nprint(resultado)", 24),
    ("factorial-acotado", "recursión · factorial", "Calcular un factorial con entrada pequeña.", "def factorial(n):\n    if n <= 1:\n        return 1\n    return n * factorial(n - 1)\nresultado = factorial(5)\nprint(resultado)", 120),
    ("suma-pares", "recursión · suma pares", "Consumir la lista y acumular solo pares.", "def suma_pares(xs):\n    if not xs:\n        return 0\n    aporte = xs[0] if xs[0] % 2 == 0 else 0\n    return aporte + suma_pares(xs[1:])\nresultado = suma_pares([1, 2, 4, 5])\nprint(resultado)", 6),
    ("producto-suite", "recursión · suite producto", "Combinar reducción y transformación.", "def producto_dobles(xs):\n    if not xs:\n        return 1\n    return (xs[0] * 2) * producto_dobles(xs[1:])\nresultado = producto_dobles([1, 2, 3])\nprint(resultado)", 48),
    # 3. Strings
    ("invertir-texto", "recursión · invertir texto", "Invertir una cadena consumiendo su cabeza.", "def invertir(s):\n    if not s:\n        return ''\n    return invertir(s[1:]) + s[0]\nresultado = invertir('luz')\nprint(resultado)", "zul"),
    ("contar-letra", "recursión · contar letra", "Contar coincidencias en una cadena.", "def contar(s, letra):\n    if not s:\n        return 0\n    return (s[0] == letra) + contar(s[1:], letra)\nresultado = contar('banana', 'a')\nprint(resultado)", 3),
    ("quitar-vocales", "recursión · quitar vocales", "Reconstruir texto omitiendo vocales.", "def sin_vocales(s):\n    if not s:\n        return ''\n    cabeza = '' if s[0] in 'aeiou' else s[0]\n    return cabeza + sin_vocales(s[1:])\nresultado = sin_vocales('codigo')\nprint(resultado)", "cdg"),
    ("palindromo", "recursión · palíndromo", "Comparar extremos y reducir el centro.", "def palindromo(s):\n    if len(s) <= 1:\n        return True\n    return s[0] == s[-1] and palindromo(s[1:-1])\nresultado = palindromo('reconocer')\nprint(resultado)", True),
    ("repetir-caracter", "recursión · repetir", "Construir una cadena con una cota decreciente.", "def repetir(c, n):\n    if n == 0:\n        return ''\n    return c + repetir(c, n - 1)\nresultado = repetir('*', 4)\nprint(resultado)", "****"),
    ("texto-suite", "recursión · suite texto", "Separar caracteres con una operación recursiva.", "def separar(s):\n    if len(s) <= 1:\n        return s\n    return s[0] + '-' + separar(s[1:])\nresultado = separar('abc')\nprint(resultado)", "a-b-c"),
    # 4. Lists
    ("maximo-lista", "recursión · máximo", "Comparar la cabeza con el máximo de la cola.", "def maximo(xs):\n    if len(xs) == 1:\n        return xs[0]\n    resto = maximo(xs[1:])\n    return xs[0] if xs[0] > resto else resto\nresultado = maximo([3, 9, 4])\nprint(resultado)", 9),
    ("buscar-lista", "recursión · buscar", "Detener la búsqueda al encontrar el objetivo.", "def contiene(xs, valor):\n    if not xs:\n        return False\n    return xs[0] == valor or contiene(xs[1:], valor)\nresultado = contiene([2, 7, 5], 7)\nprint(resultado)", True),
    ("duplicar-lista", "recursión · transformar lista", "Transformar cabeza y continuar con la cola.", "def duplicar(xs):\n    if not xs:\n        return []\n    return [xs[0] * 2] + duplicar(xs[1:])\nresultado = duplicar([1, 3, 5])\nprint(resultado)", [2, 6, 10]),
    ("filtrar-recursivo", "recursión · filtrar lista", "Elegir si la cabeza integra el resultado.", "def positivos(xs):\n    if not xs:\n        return []\n    cabeza = [xs[0]] if xs[0] > 0 else []\n    return cabeza + positivos(xs[1:])\nresultado = positivos([-1, 4, 0, 2])\nprint(resultado)", [4, 2]),
    ("intercalar-listas", "recursión · intercalar", "Consumir dos listas en paralelo.", "def intercalar(a, b):\n    if not a:\n        return b\n    if not b:\n        return a\n    return [a[0], b[0]] + intercalar(a[1:], b[1:])\nresultado = intercalar([1, 2], ['a', 'b'])\nprint(resultado)", [1, "a", 2, "b"]),
    ("listas-suite", "recursión · suite listas", "Eliminar duplicados preservando el primer orden.", "def unicos(xs):\n    if not xs:\n        return []\n    return [xs[0]] + unicos([x for x in xs[1:] if x != xs[0]])\nresultado = unicos([2, 1, 2, 3, 1])\nprint(resultado)", [2, 1, 3]),
    # 5. Nested structures
    ("aplanar-listas", "recursión · aplanar", "Recorrer listas anidadas con casos por tipo.", "def aplanar(xs):\n    if not xs:\n        return []\n    cabeza = aplanar(xs[0]) if isinstance(xs[0], list) else [xs[0]]\n    return cabeza + aplanar(xs[1:])\nresultado = aplanar([1, [2, [3]], 4])\nprint(resultado)", [1, 2, 3, 4]),
    ("profundidad-listas", "recursión · profundidad", "Medir niveles de anidación acotados.", "def profundidad(x):\n    if not isinstance(x, list):\n        return 0\n    if not x:\n        return 1\n    return 1 + max(profundidad(e) for e in x)\nresultado = profundidad([1, [2, [3]]])\nprint(resultado)", 3),
    ("sumar-anidado", "recursión · sumar anidado", "Sumar hojas numéricas de una estructura.", "def sumar(x):\n    if isinstance(x, int):\n        return x\n    if not x:\n        return 0\n    return sumar(x[0]) + sumar(x[1:])\nresultado = sumar([1, [2, 3], [4]])\nprint(resultado)", 10),
    ("contar-hojas", "recursión · contar hojas", "Distinguir contenedores de valores hoja.", "def hojas(x):\n    if not isinstance(x, list):\n        return 1\n    return sum(hojas(e) for e in x)\nresultado = hojas([1, [2, [3, 4]]])\nprint(resultado)", 4),
    ("mapear-anidado", "recursión · mapear anidado", "Conservar forma mientras se transforman hojas.", "def duplicar(x):\n    if isinstance(x, int):\n        return x * 2\n    return [duplicar(e) for e in x]\nresultado = duplicar([1, [2, 3]])\nprint(resultado)", [2, [4, 6]]),
    ("anidado-suite", "recursión · suite anidada", "Buscar un valor en cualquier nivel.", "def contiene(x, objetivo):\n    if not isinstance(x, list):\n        return x == objetivo\n    return any(contiene(e, objetivo) for e in x)\nresultado = contiene([1, [2, [8]]], 8)\nprint(resultado)", True),
    # 6. List comprehensions
    ("comp-cuadrados", "comprensión · cuadrados", "Transformar un rango con una list comprehension.", "resultado = [x * x for x in range(5)]\nprint(resultado)", [0, 1, 4, 9, 16]),
    ("comp-mayusculas", "comprensión · mayúsculas", "Transformar cada palabra sin bucle explícito.", "resultado = [s.upper() for s in ['sol', 'mar']]\nprint(resultado)", ["SOL", "MAR"]),
    ("comp-longitudes", "comprensión · longitudes", "Proyectar palabras a sus longitudes.", "resultado = [len(s) for s in ['a', 'casa', 'xy']]\nprint(resultado)", [1, 4, 2]),
    ("comp-enumerada", "comprensión · enumerada", "Combinar índice y valor en una expresión.", "resultado = [f'{i}:{v}' for i, v in enumerate(['a', 'b'])]\nprint(resultado)", ["0:a", "1:b"]),
    ("comp-producto", "comprensión · producto cartesiano", "Usar dos cláusulas for en orden.", "resultado = [(a, b) for a in [1, 2] for b in ['x', 'y']]\nprint(resultado)", [(1, "x"), (1, "y"), (2, "x"), (2, "y")]),
    ("comp-lista-suite", "comprensión · suite lista", "Aplanar un nivel con comprensión doble.", "matriz = [[1, 2], [3, 4]]\nresultado = [x for fila in matriz for x in fila]\nprint(resultado)", [1, 2, 3, 4]),
    # 7. Filter/transform comprehensions
    ("comp-pares", "comprensión · filtro par", "Filtrar pares dentro de una comprensión.", "resultado = [x for x in range(7) if x % 2 == 0]\nprint(resultado)", [0, 2, 4, 6]),
    ("comp-pares-cuadrados", "comprensión · filtrar y transformar", "Filtrar primero y transformar el valor elegido.", "resultado = [x * x for x in range(7) if x % 2 == 0]\nprint(resultado)", [0, 4, 16, 36]),
    ("comp-texto-no-vacio", "comprensión · limpiar texto", "Normalizar y descartar entradas vacías.", "datos = [' sol ', ' ', 'mar']\nresultado = [s.strip() for s in datos if s.strip()]\nprint(resultado)", ["sol", "mar"]),
    ("comp-condicional", "comprensión · expresión condicional", "Elegir una transformación para cada elemento.", "resultado = ['par' if x % 2 == 0 else 'impar' for x in [1, 2, 3]]\nprint(resultado)", ["impar", "par", "impar"]),
    ("comp-coordenadas", "comprensión · coordenadas", "Filtrar pares cartesianos con una condición.", "resultado = [(x, y) for x in range(3) for y in range(3) if x < y]\nprint(resultado)", [(0, 1), (0, 2), (1, 2)]),
    ("comp-filtro-suite", "comprensión · suite filtro", "Aplicar condición y transformación de cierre.", "datos = [-2, -1, 0, 3]\nresultado = [abs(x) for x in datos if x != 0]\nprint(resultado)", [2, 1, 3]),
    # 8. Dict comprehensions
    ("dict-cuadrados", "dict comprehension · cuadrados", "Construir claves y valores desde un rango.", "resultado = {x: x * x for x in range(1, 4)}\nprint(resultado)", {1: 1, 2: 4, 3: 9}),
    ("dict-longitudes", "dict comprehension · longitudes", "Indexar palabras por su longitud calculada.", "resultado = {s: len(s) for s in ['sol', 'universo']}\nprint(resultado)", {"sol": 3, "universo": 8}),
    ("dict-filtrado", "dict comprehension · filtro", "Conservar pares de un diccionario por condición.", "datos = {'a': 1, 'b': 4, 'c': 2}\nresultado = {k: v for k, v in datos.items() if v >= 2}\nprint(resultado)", {"b": 4, "c": 2}),
    ("dict-invertido", "dict comprehension · invertir", "Intercambiar claves y valores únicos.", "datos = {'rojo': 1, 'azul': 2}\nresultado = {v: k for k, v in datos.items()}\nprint(resultado)", {1: "rojo", 2: "azul"}),
    ("dict-enumerado", "dict comprehension · enumerar", "Crear un índice reproducible para palabras.", "resultado = {i: palabra for i, palabra in enumerate(['uno', 'dos'], start=1)}\nprint(resultado)", {1: "uno", 2: "dos"}),
    ("dict-suite", "dict comprehension · suite", "Normalizar claves y transformar valores.", "datos = {' Sol ': 2, ' MAR ': 3}\nresultado = {k.strip().lower(): v * 10 for k, v in datos.items()}\nprint(resultado)", {"sol": 20, "mar": 30}),
    # 9. Set comprehensions
    ("set-restos", "set comprehension · restos", "Deduplicar resultados de una transformación.", "resultado = {x % 3 for x in range(7)}\nprint(resultado)", {0, 1, 2}),
    ("set-iniciales", "set comprehension · iniciales", "Extraer iniciales únicas.", "resultado = {s[0] for s in ['sol', 'sal', 'mar']}\nprint(resultado)", {"s", "m"}),
    ("set-longitudes", "set comprehension · longitudes", "Calcular longitudes diferentes.", "resultado = {len(s) for s in ['a', 'sol', 'mar', 'universo']}\nprint(resultado)", {1, 3, 8}),
    ("set-filtrado", "set comprehension · filtro", "Filtrar antes de incorporar al conjunto.", "resultado = {x * x for x in range(-3, 4) if x > 0}\nprint(resultado)", {1, 4, 9}),
    ("set-normalizado", "set comprehension · normalizar", "Unificar variantes de texto.", "datos = [' Sol', 'sol ', 'MAR']\nresultado = {s.strip().lower() for s in datos}\nprint(resultado)", {"sol", "mar"}),
    ("set-suite", "set comprehension · suite", "Combinar dos fuentes y deduplicar.", "a = [1, 2, 2]\nb = [2, 3]\nresultado = {x for grupo in [a, b] for x in grupo}\nprint(resultado)", {1, 2, 3}),
    # 10. Combination and review
    ("combo-rec-comp", "combinación · recursión y comprensión", "Usar comprensión en un paso recursivo acotado.", "def capas(n):\n    if n == 0:\n        return []\n    return [[x for x in range(n)]] + capas(n - 1)\nresultado = capas(3)\nprint(resultado)", [[0, 1, 2], [0, 1], [0]]),
    ("combo-hojas-pares", "combinación · hojas pares", "Aplanar recursivamente y filtrar con comprensión.", "def aplanar(x):\n    if not isinstance(x, list):\n        return [x]\n    return [v for parte in x for v in aplanar(parte)]\nresultado = [x for x in aplanar([1, [2, 3], 4]) if x % 2 == 0]\nprint(resultado)", [2, 4]),
    ("combo-frecuencias", "combinación · frecuencias", "Construir frecuencias mediante una comprensión de diccionario.", "datos = ['a', 'b', 'a', 'c', 'b', 'a']\nresultado = {x: datos.count(x) for x in set(datos)}\nprint(resultado)", {"a": 3, "b": 2, "c": 1}),
    ("combo-arbol-mapeado", "combinación · árbol mapeado", "Transformar hojas y conservar una estructura anidada.", "def mapear(x):\n    if isinstance(x, int):\n        return x * x\n    return [mapear(e) for e in x]\nresultado = mapear([1, [2, 3]])\nprint(resultado)", [1, [4, 9]]),
    ("combo-palabras", "combinación · palabras", "Aplicar recursión y deduplicación normalizada.", "def normalizar(xs):\n    if not xs:\n        return []\n    return [xs[0].lower()] + normalizar(xs[1:])\nresultado = sorted({s for s in normalizar(['Sol', 'MAR', 'sol'])})\nprint(resultado)", ["mar", "sol"]),
    ("ola27-suite", "ola 27 · suite", "Cerrar la ola con recorrido, filtro y resumen determinista.", "def suma_hojas(x):\n    if isinstance(x, int):\n        return x\n    return sum(suma_hojas(e) for e in x)\ndatos = [[1, 2], [3, 4]]\nresultado = {i: suma_hojas(fila) for i, fila in enumerate(datos, start=1)}\nprint(resultado)", {1: 3, 2: 7}),
]


def build_steps():
    assert len(CASES) == 60
    return [exercise(2561 + i, *case) for i, case in enumerate(CASES)]


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
