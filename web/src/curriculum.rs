//! Coding micro-steps for Paso 2 (embedded from foundations seed).
//!
//! Source of truth: `docs/seeds/python-foundations-microsteps-v0.2.json`
//! Keep in sync until a JSON loader lands.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CodingStep {
    pub id: &'static str,
    pub title: &'static str,
    pub objective: &'static str,
    pub prompt_md: &'static str,
    pub starter_code: &'static str,
    pub pytest: &'static str,
    pub hint: &'static str,
    pub solution_example: &'static str,
    /// Next coding step id (`None` → return to workspace).
    pub next: Option<&'static str>,
    /// Show variable type chips under the enunciado.
    pub show_type_chips: bool,
    /// 1-based index on the workspace micro-step rail (1..=100).
    pub micro_step: i32,
}

pub const PY02_VARIABLES: CodingStep = CodingStep {
    id: "py-02-variables",
    title: "Variables (puente coding)",
    objective: "Primer micro-ejercicio de código tras el onboarding.",
    prompt_md: "Una variable guarda un valor. En Python se crea al asignar.\n\n**Micro-reto:**\n1. Crea `nombre` con un texto\n2. Crea `edad` con un entero\n3. Imprime ambas con `print(nombre, edad)`",
    starter_code: "# nombre = ...\n# edad = ...\n# print(...)\n",
    pytest: "def test_variables(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert 'nombre' in ns and isinstance(ns['nombre'], str)\n    assert 'edad' in ns and isinstance(ns['edad'], int)\n    out = ' '.join(capsys.readouterr().out.split())\n    assert str(ns['nombre']) in out and str(ns['edad']) in out\n",
    hint: "nombre = \"Ana\"\nedad = 25\nprint(nombre, edad)",
    solution_example: "nombre = \"Ana\"\nedad = 25\nprint(nombre, edad)",
    next: Some("py-02-intro"),
    show_type_chips: true,
    micro_step: 1,
};

pub const PY02_INTRO: CodingStep = CodingStep {
    id: "py-02-intro",
    title: "Python Intro",
    objective: "Reconocer sintaxis legible, indentación e intérprete mediante un Hola Mundo.",
    prompt_md: "**¿Qué es Python?**\n\nCreado por Guido van Rossum (1991). Destaca por:\n- sintaxis cercana al inglés,\n- ejecución con intérprete (prototipado rápido),\n- indentación para definir bloques.\n\n**Micro-reto:** Haz que el programa imprima exactamente `Hello, World!`",
    starter_code: "# Completa la línea\nprint()\n",
    pytest: "def test_hello(capsys):\n    exec(open('solution.py', encoding='utf-8').read())\n    assert capsys.readouterr().out.strip() == 'Hello, World!'\n",
    hint: "Usa comillas: print(\"Hello, World!\")",
    solution_example: "print(\"Hello, World!\")",
    next: Some("py-03-get-started"),
    show_type_chips: false,
    micro_step: 2,
};

pub const PY03_GET_STARTED: CodingStep = CodingStep {
    id: "py-03-get-started",
    title: "Python Get Started",
    objective: "Entender que el código se ejecuta en el navegador (Pyodide) sin instalar nada, y producir una salida propia.",
    prompt_md: "**Get Started**\n\nEn W3Schools (y aquí) puedes probar Python **sin instalar**. El editor corre en el navegador.\n\n**Micro-reto:** Imprime tu propio mensaje de bienvenida que contenga la palabra `Python`.",
    starter_code: "# Escribe tu mensaje\n",
    pytest: "def test_contains_python(capsys):\n    exec(open('solution.py', encoding='utf-8').read())\n    assert 'Python' in capsys.readouterr().out\n",
    hint: "Cualquier print(\"... Python ...\") válido alcanza.",
    solution_example: "print(\"Estoy aprendiendo Python\")",
    next: Some("py-04-syntax"),
    show_type_chips: false,
    micro_step: 3,
};

pub const PY04_SYNTAX: CodingStep = CodingStep {
    id: "py-04-syntax",
    title: "Python Syntax",
    objective: "Usar statements secuenciales (una instrucción por línea).",
    prompt_md: "**Sintaxis / Statements**\n\nUn programa es una lista de instrucciones (statements).\n\n**Challenge:**\n1. Imprime `Hello World!`\n2. Imprime `Have a good day.`\n3. Imprime `Learning Python is fun!`",
    starter_code: "# Tres statements, uno por línea\n",
    pytest: "def test_three_statements(capsys):\n    exec(open('solution.py', encoding='utf-8').read())\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == [\n        'Hello World!',\n        'Have a good day.',\n        'Learning Python is fun!',\n    ]\n",
    hint: "Tres llamadas a print(), en ese orden exacto.",
    solution_example: "print(\"Hello World!\")\nprint(\"Have a good day.\")\nprint(\"Learning Python is fun!\")",
    next: Some("py-05-output"),
    show_type_chips: false,
    micro_step: 4,
};

pub const PY05_OUTPUT: CodingStep = CodingStep {
    id: "py-05-output",
    title: "Python Output",
    objective: "Usar print() para combinar texto y números en una sola salida.",
    prompt_md: "**Output / Print**\n\n`print()` muestra texto o valores. Podés mezclar texto y números separándolos con comas.\n\n**Micro-reto:**\n1. Imprime el texto `I am` y el número `25` en **una** llamada a `print`",
    starter_code: "# print(...)\n",
    pytest: "def test_output_mix(capsys):\n    exec(open('solution.py', encoding='utf-8').read())\n    out = ' '.join(capsys.readouterr().out.split())\n    assert 'I am' in out and '25' in out\n",
    hint: "print(\"I am\", 25)",
    solution_example: "print(\"I am\", 25)",
    next: Some("py-06-comments"),
    show_type_chips: false,
    micro_step: 5,
};

pub const PY06_COMMENTS: CodingStep = CodingStep {
    id: "py-06-comments",
    title: "Python Comments",
    objective: "Documentar código con comentarios de línea y multilínea, y desactivar líneas temporales.",
    prompt_md: "**Comments**\n\nLos comentarios empiezan con `#`. También podés usar un string multilínea con comillas triples como comentario.\n\n**Micro-reto:**\n1. Agregá un comentario de una línea que diga exactamente `This is a comment`\n2. Comentá la línea `print(\"This should not run\")` para que **no** se ejecute\n3. Agregá un comentario multilínea (comillas triples) que contenga `This is`, `a multiline` y `comment`",
    starter_code: "# Completá el micro-reto\n\nprint(\"This should not run\")\n",
    pytest: "def test_comments(capsys):\n    src = open('solution.py', encoding='utf-8').read()\n    assert 'This is a comment' in src\n    assert any(\n        ln.lstrip().startswith('#') and 'print(\"This should not run\")' in ln\n        for ln in src.splitlines()\n    )\n    assert ('\"\"\"' in src) or (\"'''\" in src)\n    low = src.lower()\n    assert 'this is' in low and 'multiline' in low and 'comment' in low\n    exec(compile(src, 'solution.py', 'exec'))\n    assert 'This should not run' not in capsys.readouterr().out\n",
    hint: "# This is a comment\n# print(\"This should not run\")\n\"\"\"This is\na multiline\ncomment\"\"\"",
    solution_example: "# This is a comment\n# print(\"This should not run\")\n\"\"\"This is\na multiline\ncomment\"\"\"\n",
    next: Some("py-07-data-types"),
    show_type_chips: false,
    micro_step: 6,
};

pub const PY07_DATA_TYPES: CodingStep = CodingStep {
    id: "py-07-data-types",
    title: "Python Data Types",
    objective: "Crear variables de distintos tipos y revelar su tipo con type().",
    prompt_md: "**Data Types**\n\nPython asigna el tipo al crear la variable. `type()` muestra el tipo de un valor.\n\n**Micro-reto:**\n1. Creá `x` con el valor `5`\n2. Creá `y` con el valor `3.14`\n3. Creá `z` con el valor `\"Hello\"`\n4. Imprimí el tipo de cada variable con `type()`",
    starter_code: "# x = ...\n# y = ...\n# z = ...\n# print(type(...))\n",
    pytest: "def test_data_types(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('x') == 5 and isinstance(ns['x'], int)\n    assert ns.get('y') == 3.14 and isinstance(ns['y'], float)\n    assert ns.get('z') == 'Hello' and isinstance(ns['z'], str)\n    out = capsys.readouterr().out\n    assert 'int' in out and 'float' in out and 'str' in out\n",
    hint: "x = 5\ny = 3.14\nz = \"Hello\"\nprint(type(x))\nprint(type(y))\nprint(type(z))",
    solution_example: "x = 5\ny = 3.14\nz = \"Hello\"\nprint(type(x))\nprint(type(y))\nprint(type(z))\n",
    next: Some("py-08-numbers"),
    show_type_chips: false,
    micro_step: 7,
};

pub const PY08_NUMBERS: CodingStep = CodingStep {
    id: "py-08-numbers",
    title: "Python Numbers",
    objective: "Crear int, float y complex, y mostrar su tipo con type().",
    prompt_md: "**Numbers**\n\nPython tiene tres tipos numéricos: `int`, `float` y `complex`.\n\n**Micro-reto:**\n1. Creá `x` con el entero `5`\n2. Creá `y` con el float `3.14`\n3. Creá `z` con el complejo `2+3j`\n4. Imprimí el tipo de cada variable con `type()`",
    starter_code: "# x = ...\n# y = ...\n# z = ...\n# print(type(...))\n",
    pytest: "def test_numbers(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('x') == 5 and isinstance(ns['x'], int)\n    assert ns.get('y') == 3.14 and isinstance(ns['y'], float)\n    assert ns.get('z') == (2 + 3j) and isinstance(ns['z'], complex)\n    out = capsys.readouterr().out\n    assert 'int' in out and 'float' in out and 'complex' in out\n",
    hint: "x = 5\ny = 3.14\nz = 2 + 3j\nprint(type(x))\nprint(type(y))\nprint(type(z))",
    solution_example: "x = 5\ny = 3.14\nz = 2 + 3j\nprint(type(x))\nprint(type(y))\nprint(type(z))\n",
    next: Some("py-09-casting"),
    show_type_chips: false,
    micro_step: 8,
};

pub const PY09_CASTING: CodingStep = CodingStep {
    id: "py-09-casting",
    title: "Python Casting",
    objective: "Convertir tipos con float() y str(), y mostrar el resultado.",
    prompt_md: "**Casting**\n\nPodés forzar un tipo con constructores como `int()`, `float()` y `str()`.\n\n**Micro-reto:**\n1. Creá `x` con el valor entero `1`\n2. Convertí `x` a float y guardalo en `a`\n3. Convertí `x` a string y guardalo en `b`\n4. Imprimí `a` y `b`",
    starter_code: "# x = ...\n# a = ...\n# b = ...\n# print(...)\n",
    pytest: "def test_casting(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('x') == 1 and isinstance(ns['x'], int)\n    assert ns.get('a') == 1.0 and isinstance(ns['a'], float)\n    assert ns.get('b') == '1' and isinstance(ns['b'], str)\n    out = ' '.join(capsys.readouterr().out.split())\n    assert '1.0' in out and '1' in out\n",
    hint: "x = 1\na = float(x)\nb = str(x)\nprint(a)\nprint(b)",
    solution_example: "x = 1\na = float(x)\nb = str(x)\nprint(a)\nprint(b)\n",
    next: Some("py-10-strings"),
    show_type_chips: false,
    micro_step: 9,
};

pub const PY10_STRINGS: CodingStep = CodingStep {
    id: "py-10-strings",
    title: "Python Strings",
    objective: "Asignar un string, mostrarlo y medir su longitud con len().",
    prompt_md: "**Strings**\n\nLos strings van entre comillas simples o dobles.\n\n**Micro-reto:**\n1. Asigná a `a` el string `Hello, World!`\n2. Imprimí `a`\n3. Imprimí la longitud de `a` con `len()`",
    starter_code: "# a = ...\n# print(...)\n# print(len(...))\n",
    pytest: "def test_strings(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('a') == 'Hello, World!'\n    out = ' '.join(capsys.readouterr().out.split())\n    assert 'Hello, World!' in out and '13' in out\n",
    hint: "a = \"Hello, World!\"\nprint(a)\nprint(len(a))",
    solution_example: "a = \"Hello, World!\"\nprint(a)\nprint(len(a))\n",
    next: Some("py-11-slicing"),
    show_type_chips: false,
    micro_step: 10,
};

pub const PY11_SLICING: CodingStep = CodingStep {
    id: "py-11-slicing",
    title: "Python Slicing Strings",
    objective: "Extraer un rango de caracteres con la sintaxis de slice.",
    prompt_md: "**Slicing**\n\nPodés devolver un rango con `inicio:fin` (el fin no se incluye).\n\n**Micro-reto:**\n1. Asigná a `b` el string `Hello, World!`\n2. Guardá en `slice` el resultado de `b[2:5]`\n3. Imprimí `slice`",
    starter_code: "# b = ...\n# slice = ...\n# print(...)\n",
    pytest: "def test_slicing(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('b') == 'Hello, World!'\n    assert ns.get('slice') == 'llo'\n    out = capsys.readouterr().out\n    assert 'llo' in out\n",
    hint: "b = \"Hello, World!\"\nslice = b[2:5]\nprint(slice)",
    solution_example: "b = \"Hello, World!\"\nslice = b[2:5]\nprint(slice)\n",
    next: Some("py-12-modify-strings"),
    show_type_chips: false,
    micro_step: 11,
};

pub const PY12_MODIFY_STRINGS: CodingStep = CodingStep {
    id: "py-12-modify-strings",
    title: "Python Modify Strings",
    objective: "Usar upper() y lower() para transformar un string.",
    prompt_md: "**Modify Strings**\n\nPython trae métodos built-in para transformar strings.\n\n**Micro-reto:**\n1. Asigná a `a` el string `Hello, World!`\n2. Guardá en `u` el resultado de `a.upper()`\n3. Guardá en `l` el resultado de `a.lower()`\n4. Imprimí `u` y `l`",
    starter_code: "# a = ...\n# u = ...\n# l = ...\n# print(...)\n",
    pytest: "def test_modify_strings(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('a') == 'Hello, World!'\n    assert ns.get('u') == 'HELLO, WORLD!'\n    assert ns.get('l') == 'hello, world!'\n    out = capsys.readouterr().out\n    assert 'HELLO, WORLD!' in out and 'hello, world!' in out\n",
    hint: "a = \"Hello, World!\"\nu = a.upper()\nl = a.lower()\nprint(u)\nprint(l)",
    solution_example: "a = \"Hello, World!\"\nu = a.upper()\nl = a.lower()\nprint(u)\nprint(l)\n",
    next: Some("py-13-concatenate"),
    show_type_chips: false,
    micro_step: 12,
};

pub const PY13_CONCATENATE: CodingStep = CodingStep {
    id: "py-13-concatenate",
    title: "Python String Concatenation",
    objective: "Combinar strings con el operador +.",
    prompt_md: "**String Concatenation**\n\nPara unir strings usá el operador `+`.\n\n**Micro-reto:**\n1. Creá `a` con `Hello`\n2. Creá `b` con `World`\n3. Creá `c` como `a + \" \" + b`\n4. Imprimí `c`",
    starter_code: "# a = ...\n# b = ...\n# c = ...\n# print(...)\n",
    pytest: "def test_concatenate(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('a') == 'Hello'\n    assert ns.get('b') == 'World'\n    assert ns.get('c') == 'Hello World'\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'Hello World' or 'Hello World' in out\n",
    hint: "a = \"Hello\"\nb = \"World\"\nc = a + \" \" + b\nprint(c)",
    solution_example: "a = \"Hello\"\nb = \"World\"\nc = a + \" \" + b\nprint(c)\n",
    next: Some("py-14-format-strings"),
    show_type_chips: false,
    micro_step: 13,
};

pub const PY14_FORMAT_STRINGS: CodingStep = CodingStep {
    id: "py-14-format-strings",
    title: "Python Format Strings",
    objective: "Insertar variables en un string con f-strings.",
    prompt_md: "**Format Strings**\n\nLas f-strings (prefijo `f`) insertan variables dentro de `{}`.\n\n**Micro-reto:**\n1. Creá `age` con el entero `36`\n2. Creá `txt` con la f-string `My name is John, I am {age}`\n3. Imprimí `txt`",
    starter_code: "# age = ...\n# txt = ...\n# print(...)\n",
    pytest: "def test_format_strings(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('age') == 36\n    assert ns.get('txt') == 'My name is John, I am 36'\n    out = ' '.join(capsys.readouterr().out.split())\n    assert 'My name is John, I am 36' in out\n",
    hint: "age = 36\ntxt = f\"My name is John, I am {age}\"\nprint(txt)",
    solution_example: "age = 36\ntxt = f\"My name is John, I am {age}\"\nprint(txt)\n",
    next: Some("py-15-escape"),
    show_type_chips: false,
    micro_step: 14,
};

pub const PY15_ESCAPE: CodingStep = CodingStep {
    id: "py-15-escape",
    title: "Python Escape Characters",
    objective: "Insertar comillas dentro de un string con el escape \\\".",
    prompt_md: "**Escape Characters**\n\nUna barra invertida `\\` escapa caracteres ilegales (por ejemplo comillas).\n\n**Micro-reto:**\n1. Creá `txt` con el texto `We are the so-called \"Vikings\" from the north.` usando `\\\"`\n2. Imprimí `txt`",
    starter_code: "# txt = ...\n# print(...)\n",
    pytest: "def test_escape(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('txt') == 'We are the so-called \"Vikings\" from the north.'\n    out = capsys.readouterr().out\n    assert 'Vikings' in out\n",
    hint: "txt = \"We are the so-called \\\"Vikings\\\" from the north.\"\nprint(txt)",
    solution_example: "txt = \"We are the so-called \\\"Vikings\\\" from the north.\"\nprint(txt)\n",
    next: Some("py-16-booleans"),
    show_type_chips: false,
    micro_step: 15,
};

pub const PY16_BOOLEANS: CodingStep = CodingStep {
    id: "py-16-booleans",
    title: "Python Booleans",
    objective: "Evaluar comparaciones y obtener True o False.",
    prompt_md: "**Booleans**\n\nUna comparación en Python devuelve `True` o `False`.\n\n**Micro-reto:**\n1. Imprimí el resultado de `10 > 9`\n2. Imprimí el resultado de `10 == 9`\n3. Imprimí el resultado de `10 < 9`",
    starter_code: "# print(...)\n",
    pytest: "def test_booleans(capsys):\n    exec(open('solution.py', encoding='utf-8').read())\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['True', 'False', 'False']\n",
    hint: "print(10 > 9)\nprint(10 == 9)\nprint(10 < 9)",
    solution_example: "print(10 > 9)\nprint(10 == 9)\nprint(10 < 9)\n",
    next: Some("py-17-operators"),
    show_type_chips: false,
    micro_step: 16,
};

pub const PY17_OPERATORS: CodingStep = CodingStep {
    id: "py-17-operators",
    title: "Python Operators",
    objective: "Usar el operador + para sumar valores.",
    prompt_md: "**Operators**\n\nLos operadores realizan operaciones sobre variables y valores.\n\n**Micro-reto:**\n1. Imprimí el resultado de `10 + 5`",
    starter_code: "# print(...)\n",
    pytest: "def test_operators(capsys):\n    exec(open('solution.py', encoding='utf-8').read())\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '15'\n",
    hint: "print(10 + 5)",
    solution_example: "print(10 + 5)\n",
    next: Some("py-18-lists"),
    show_type_chips: false,
    micro_step: 17,
};

pub const PY18_LISTS: CodingStep = CodingStep {
    id: "py-18-lists",
    title: "Python Lists",
    objective: "Crear una lista, mostrarla y medir su longitud.",
    prompt_md: "**Lists**\n\nLas listas se crean con corchetes `[]`.\n\n**Micro-reto:**\n1. Creá `thislist` con `apple`, `banana` y `cherry`\n2. Imprimí `thislist`\n3. Imprimí `len(thislist)`",
    starter_code: "# thislist = ...\n# print(...)\n# print(len(...))\n",
    pytest: "def test_lists(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thislist') == ['apple', 'banana', 'cherry']\n    out = ' '.join(capsys.readouterr().out.split())\n    assert 'apple' in out and 'banana' in out and 'cherry' in out and '3' in out\n",
    hint: "thislist = [\"apple\", \"banana\", \"cherry\"]\nprint(thislist)\nprint(len(thislist))",
    solution_example: "thislist = [\"apple\", \"banana\", \"cherry\"]\nprint(thislist)\nprint(len(thislist))\n",
    next: Some("py-19-list-access"),
    show_type_chips: false,
    micro_step: 18,
};

pub const PY19_LIST_ACCESS: CodingStep = CodingStep {
    id: "py-19-list-access",
    title: "Python Access List Items",
    objective: "Acceder a un ítem de lista por índice.",
    prompt_md: "**Access List Items**\n\nLos ítems se indexan desde `0`.\n\n**Micro-reto:**\n1. Creá `thislist` con `apple`, `banana` y `cherry`\n2. Imprimí el segundo ítem (`thislist[1]`)",
    starter_code: "# thislist = ...\n# print(...)\n",
    pytest: "def test_list_access(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thislist') == ['apple', 'banana', 'cherry']\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'banana'\n",
    hint: "thislist = [\"apple\", \"banana\", \"cherry\"]\nprint(thislist[1])",
    solution_example: "thislist = [\"apple\", \"banana\", \"cherry\"]\nprint(thislist[1])\n",
    next: Some("py-20-list-change"),
    show_type_chips: false,
    micro_step: 19,
};

pub const PY20_LIST_CHANGE: CodingStep = CodingStep {
    id: "py-20-list-change",
    title: "Python Change List Items",
    objective: "Cambiar el valor de un ítem de lista por índice.",
    prompt_md: "**Change List Items**\n\nPara cambiar un ítem, asigná un nuevo valor en su índice.\n\n**Micro-reto:**\n1. Creá `thislist` con `apple`, `banana` y `cherry`\n2. Cambiá el segundo ítem a `blackcurrant`\n3. Imprimí `thislist`",
    starter_code: "# thislist = ...\n# thislist[...] = ...\n# print(...)\n",
    pytest: "def test_list_change(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thislist') == ['apple', 'blackcurrant', 'cherry']\n    out = ' '.join(capsys.readouterr().out.split())\n    assert 'blackcurrant' in out\n",
    hint: "thislist = [\"apple\", \"banana\", \"cherry\"]\nthislist[1] = \"blackcurrant\"\nprint(thislist)",
    solution_example: "thislist = [\"apple\", \"banana\", \"cherry\"]\nthislist[1] = \"blackcurrant\"\nprint(thislist)\n",
    next: Some("py-21-list-add"),
    show_type_chips: false,
    micro_step: 20,
};

pub const PY21_LIST_ADD: CodingStep = CodingStep {
    id: "py-21-list-add",
    title: "Python Add List Items",
    objective: "Agregar un ítem al final de la lista con append().",
    prompt_md: "**Add List Items**\n\n`append()` agrega un ítem al final de la lista.\n\n**Micro-reto:**\n1. Creá `thislist` con `apple`, `banana` y `cherry`\n2. Agregá `orange` con `append()`\n3. Imprimí `thislist`",
    starter_code: "# thislist = ...\n# thislist.append(...)\n# print(...)\n",
    pytest: "def test_list_add(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thislist') == ['apple', 'banana', 'cherry', 'orange']\n    out = ' '.join(capsys.readouterr().out.split())\n    assert 'orange' in out\n",
    hint: "thislist = [\"apple\", \"banana\", \"cherry\"]\nthislist.append(\"orange\")\nprint(thislist)",
    solution_example: "thislist = [\"apple\", \"banana\", \"cherry\"]\nthislist.append(\"orange\")\nprint(thislist)\n",
    next: Some("py-22-list-remove"),
    show_type_chips: false,
    micro_step: 21,
};

pub const PY22_LIST_REMOVE: CodingStep = CodingStep {
    id: "py-22-list-remove",
    title: "Python Remove List Items",
    objective: "Eliminar un ítem de la lista con remove().",
    prompt_md: "**Remove List Items**\n\n`remove()` elimina el ítem indicado (primera ocurrencia).\n\n**Micro-reto:**\n1. Creá `thislist` con `apple`, `banana` y `cherry`\n2. Remové `banana` con `remove()`\n3. Imprimí `thislist`",
    starter_code: "# thislist = ...\n# thislist.remove(...)\n# print(...)\n",
    pytest: "def test_list_remove(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thislist') == ['apple', 'cherry']\n    out = ' '.join(capsys.readouterr().out.split())\n    assert 'banana' not in out and 'apple' in out and 'cherry' in out\n",
    hint: "thislist = [\"apple\", \"banana\", \"cherry\"]\nthislist.remove(\"banana\")\nprint(thislist)",
    solution_example: "thislist = [\"apple\", \"banana\", \"cherry\"]\nthislist.remove(\"banana\")\nprint(thislist)\n",
    next: Some("py-23-list-loop"),
    show_type_chips: false,
    micro_step: 22,
};

pub const PY23_LIST_LOOP: CodingStep = CodingStep {
    id: "py-23-list-loop",
    title: "Python Loop Lists",
    objective: "Recorrer una lista con un for e imprimir cada ítem.",
    prompt_md: "**Loop Lists**\n\nPodés recorrer los ítems con un `for`.\n\n**Micro-reto:**\n1. Creá `thislist` con `apple`, `banana` y `cherry`\n2. Recorré la lista con `for x in thislist` e imprimí cada `x`",
    starter_code: "# thislist = ...\n# for x in thislist:\n#     print(x)\n",
    pytest: "def test_list_loop(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thislist') == ['apple', 'banana', 'cherry']\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['apple', 'banana', 'cherry']\n",
    hint: "thislist = [\"apple\", \"banana\", \"cherry\"]\nfor x in thislist:\n    print(x)",
    solution_example: "thislist = [\"apple\", \"banana\", \"cherry\"]\nfor x in thislist:\n    print(x)\n",
    next: Some("py-24-list-comprehension"),
    show_type_chips: false,
    micro_step: 23,
};

pub const PY24_LIST_COMPREHENSION: CodingStep = CodingStep {
    id: "py-24-list-comprehension",
    title: "Python List Comprehension",
    objective: "Crear una lista filtrada con list comprehension.",
    prompt_md: "**List Comprehension**\n\nSintaxis corta para crear una lista nueva a partir de otra.\n\n**Micro-reto:**\n1. Creá `fruits` con `apple`, `banana`, `cherry`, `kiwi`, `mango`\n2. Creá `newlist` con comprehension: ítems de `fruits` que contienen la letra `a`\n3. Imprimí `newlist`",
    starter_code: "# fruits = ...\n# newlist = [...]\n# print(...)\n",
    pytest: "def test_list_comprehension(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('fruits') == ['apple', 'banana', 'cherry', 'kiwi', 'mango']\n    assert ns.get('newlist') == ['apple', 'banana', 'mango']\n    out = ' '.join(capsys.readouterr().out.split())\n    assert 'apple' in out and 'banana' in out and 'mango' in out\n",
    hint: "fruits = [\"apple\", \"banana\", \"cherry\", \"kiwi\", \"mango\"]\nnewlist = [x for x in fruits if \"a\" in x]\nprint(newlist)",
    solution_example: "fruits = [\"apple\", \"banana\", \"cherry\", \"kiwi\", \"mango\"]\nnewlist = [x for x in fruits if \"a\" in x]\nprint(newlist)\n",
    next: Some("py-25-list-sort"),
    show_type_chips: false,
    micro_step: 24,
};

pub const PY25_LIST_SORT: CodingStep = CodingStep {
    id: "py-25-list-sort",
    title: "Python Sort Lists",
    objective: "Ordenar una lista alfabéticamente con sort().",
    prompt_md: "**Sort Lists**\n\n`sort()` ordena la lista alfanuméricamente (ascendente por defecto).\n\n**Micro-reto:**\n1. Creá `thislist` con `orange`, `mango`, `kiwi`, `pineapple`, `banana`\n2. Ordenála con `sort()`\n3. Imprimí `thislist`",
    starter_code: "# thislist = ...\n# thislist.sort()\n# print(...)\n",
    pytest: "def test_list_sort(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thislist') == ['banana', 'kiwi', 'mango', 'orange', 'pineapple']\n    out = ' '.join(capsys.readouterr().out.split())\n    assert 'banana' in out and 'pineapple' in out\n",
    hint: "thislist = [\"orange\", \"mango\", \"kiwi\", \"pineapple\", \"banana\"]\nthislist.sort()\nprint(thislist)",
    solution_example: "thislist = [\"orange\", \"mango\", \"kiwi\", \"pineapple\", \"banana\"]\nthislist.sort()\nprint(thislist)\n",
    next: Some("py-26-list-copy"),
    show_type_chips: false,
    micro_step: 25,
};

pub const PY26_LIST_COPY: CodingStep = CodingStep {
    id: "py-26-list-copy",
    title: "Python Copy Lists",
    objective: "Copiar una lista con el método copy().",
    prompt_md: "**Copy Lists**\n\n`list2 = list1` solo crea una referencia. Usá `copy()` para una copia real.\n\n**Micro-reto:**\n1. Creá `thislist` con `apple`, `banana` y `cherry`\n2. Creá `mylist` como copia con `thislist.copy()`\n3. Imprimí `mylist`",
    starter_code: "# thislist = ...\n# mylist = ...\n# print(...)\n",
    pytest: "def test_list_copy(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thislist') == ['apple', 'banana', 'cherry']\n    assert ns.get('mylist') == ['apple', 'banana', 'cherry']\n    assert ns.get('mylist') is not ns.get('thislist')\n    out = ' '.join(capsys.readouterr().out.split())\n    assert 'apple' in out\n",
    hint: "thislist = [\"apple\", \"banana\", \"cherry\"]\nmylist = thislist.copy()\nprint(mylist)",
    solution_example: "thislist = [\"apple\", \"banana\", \"cherry\"]\nmylist = thislist.copy()\nprint(mylist)\n",
    next: Some("py-27-list-join"),
    show_type_chips: false,
    micro_step: 26,
};

pub const PY27_LIST_JOIN: CodingStep = CodingStep {
    id: "py-27-list-join",
    title: "Python Join Lists",
    objective: "Unir dos listas con el operador +.",
    prompt_md: "**Join Lists**\n\nPodés unir listas con `+`.\n\n**Micro-reto:**\n1. Creá `list1` con `a`, `b`, `c`\n2. Creá `list2` con `1`, `2`, `3`\n3. Creá `list3` como `list1 + list2`\n4. Imprimí `list3`",
    starter_code: "# list1 = ...\n# list2 = ...\n# list3 = ...\n# print(...)\n",
    pytest: "def test_list_join(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('list1') == ['a', 'b', 'c']\n    assert ns.get('list2') == [1, 2, 3]\n    assert ns.get('list3') == ['a', 'b', 'c', 1, 2, 3]\n    out = ' '.join(capsys.readouterr().out.split())\n    assert 'a' in out and '3' in out\n",
    hint: "list1 = [\"a\", \"b\", \"c\"]\nlist2 = [1, 2, 3]\nlist3 = list1 + list2\nprint(list3)",
    solution_example: "list1 = [\"a\", \"b\", \"c\"]\nlist2 = [1, 2, 3]\nlist3 = list1 + list2\nprint(list3)\n",
    next: Some("py-28-tuples"),
    show_type_chips: false,
    micro_step: 27,
};

pub const PY28_TUPLES: CodingStep = CodingStep {
    id: "py-28-tuples",
    title: "Python Tuples",
    objective: "Crear una tupla, mostrarla y medir su longitud.",
    prompt_md: "**Tuples**\n\nLas tuplas se escriben con paréntesis `()` y son inmutables.\n\n**Micro-reto:**\n1. Creá `thistuple` con `apple`, `banana` y `cherry`\n2. Imprimí `thistuple`\n3. Imprimí `len(thistuple)`",
    starter_code: "# thistuple = ...\n# print(...)\n# print(len(...))\n",
    pytest: "def test_tuples(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thistuple') == ('apple', 'banana', 'cherry')\n    out = ' '.join(capsys.readouterr().out.split())\n    assert 'apple' in out and '3' in out\n",
    hint: "thistuple = (\"apple\", \"banana\", \"cherry\")\nprint(thistuple)\nprint(len(thistuple))",
    solution_example: "thistuple = (\"apple\", \"banana\", \"cherry\")\nprint(thistuple)\nprint(len(thistuple))\n",
    next: Some("py-29-tuple-access"),
    show_type_chips: false,
    micro_step: 28,
};

pub const PY29_TUPLE_ACCESS: CodingStep = CodingStep {
    id: "py-29-tuple-access",
    title: "Python Access Tuple Items",
    objective: "Acceder a un ítem de tupla por índice.",
    prompt_md: "**Access Tuple Items**\n\nLos ítems se indexan desde `0`.\n\n**Micro-reto:**\n1. Creá `thistuple` con `apple`, `banana` y `cherry`\n2. Imprimí el segundo ítem (`thistuple[1]`)",
    starter_code: "# thistuple = ...\n# print(...)\n",
    pytest: "def test_tuple_access(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thistuple') == ('apple', 'banana', 'cherry')\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'banana'\n",
    hint: "thistuple = (\"apple\", \"banana\", \"cherry\")\nprint(thistuple[1])",
    solution_example: "thistuple = (\"apple\", \"banana\", \"cherry\")\nprint(thistuple[1])\n",
    next: Some("py-30-tuple-update"),
    show_type_chips: false,
    micro_step: 29,
};

pub const PY30_TUPLE_UPDATE: CodingStep = CodingStep {
    id: "py-30-tuple-update",
    title: "Python Update Tuples",
    objective: "Actualizar una tupla convirtiéndola temporalmente a lista.",
    prompt_md: "**Update Tuples**\n\nLas tuplas son inmutables; el workaround es pasar por `list()`.\n\n**Micro-reto:**\n1. Creá `x` con `apple`, `banana`, `cherry`\n2. Convertí a lista en `y`, cambiá el índice `1` a `kiwi`, y reconvertí a tupla en `x`\n3. Imprimí `x`",
    starter_code: "# x = ...\n# y = list(x)\n# y[1] = ...\n# x = tuple(y)\n# print(x)\n",
    pytest: "def test_tuple_update(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('x') == ('apple', 'kiwi', 'cherry')\n    out = ' '.join(capsys.readouterr().out.split())\n    assert 'kiwi' in out\n",
    hint: "x = (\"apple\", \"banana\", \"cherry\")\ny = list(x)\ny[1] = \"kiwi\"\nx = tuple(y)\nprint(x)",
    solution_example: "x = (\"apple\", \"banana\", \"cherry\")\ny = list(x)\ny[1] = \"kiwi\"\nx = tuple(y)\nprint(x)\n",
    next: Some("py-31-tuple-unpack"),
    show_type_chips: false,
    micro_step: 30,
};

pub const PY31_TUPLE_UNPACK: CodingStep = CodingStep {
    id: "py-31-tuple-unpack",
    title: "Python Unpack Tuples",
    objective: "Desempaquetar valores de una tupla en variables.",
    prompt_md: "**Unpack Tuples**\n\nPodés extraer valores de una tupla en variables.\n\n**Micro-reto:**\n1. Creá `fruits` con `apple`, `banana`, `cherry`\n2. Desempaquetá en `green`, `yellow`, `red`\n3. Imprimí las tres variables",
    starter_code: "# fruits = ...\n# (green, yellow, red) = fruits\n# print(...)\n",
    pytest: "def test_tuple_unpack(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('fruits') == ('apple', 'banana', 'cherry')\n    assert ns.get('green') == 'apple'\n    assert ns.get('yellow') == 'banana'\n    assert ns.get('red') == 'cherry'\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['apple', 'banana', 'cherry']\n",
    hint: "fruits = (\"apple\", \"banana\", \"cherry\")\n(green, yellow, red) = fruits\nprint(green)\nprint(yellow)\nprint(red)",
    solution_example: "fruits = (\"apple\", \"banana\", \"cherry\")\n(green, yellow, red) = fruits\nprint(green)\nprint(yellow)\nprint(red)\n",
    next: Some("py-32-tuple-loop"),
    show_type_chips: false,
    micro_step: 31,
};

pub const PY32_TUPLE_LOOP: CodingStep = CodingStep {
    id: "py-32-tuple-loop",
    title: "Python Loop Tuples",
    objective: "Recorrer una tupla con for e imprimir cada ítem.",
    prompt_md: "**Loop Tuples**\n\nPodés recorrer los ítems con un `for`.\n\n**Micro-reto:**\n1. Creá `thistuple` con `apple`, `banana` y `cherry`\n2. Recorré con `for x in thistuple` e imprimí cada `x`",
    starter_code: "# thistuple = ...\n# for x in thistuple:\n#     print(x)\n",
    pytest: "def test_tuple_loop(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thistuple') == ('apple', 'banana', 'cherry')\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['apple', 'banana', 'cherry']\n",
    hint: "thistuple = (\"apple\", \"banana\", \"cherry\")\nfor x in thistuple:\n    print(x)",
    solution_example: "thistuple = (\"apple\", \"banana\", \"cherry\")\nfor x in thistuple:\n    print(x)\n",
    next: Some("py-33-tuple-join"),
    show_type_chips: false,
    micro_step: 32,
};

pub const PY33_TUPLE_JOIN: CodingStep = CodingStep {
    id: "py-33-tuple-join",
    title: "Python Join Tuples",
    objective: "Unir dos tuplas con el operador +.",
    prompt_md: "**Join Tuples**\n\nPodés unir tuplas con `+`.\n\n**Micro-reto:**\n1. Creá `tuple1` con `a`, `b`, `c`\n2. Creá `tuple2` con `1`, `2`, `3`\n3. Creá `tuple3` como `tuple1 + tuple2`\n4. Imprimí `tuple3`",
    starter_code: "# tuple1 = ...\n# tuple2 = ...\n# tuple3 = ...\n# print(...)\n",
    pytest: "def test_tuple_join(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('tuple1') == ('a', 'b', 'c')\n    assert ns.get('tuple2') == (1, 2, 3)\n    assert ns.get('tuple3') == ('a', 'b', 'c', 1, 2, 3)\n    out = ' '.join(capsys.readouterr().out.split())\n    assert 'a' in out and '3' in out\n",
    hint: "tuple1 = (\"a\", \"b\", \"c\")\ntuple2 = (1, 2, 3)\ntuple3 = tuple1 + tuple2\nprint(tuple3)",
    solution_example: "tuple1 = (\"a\", \"b\", \"c\")\ntuple2 = (1, 2, 3)\ntuple3 = tuple1 + tuple2\nprint(tuple3)\n",
    next: Some("py-34-sets"),
    show_type_chips: false,
    micro_step: 33,
};

pub const PY34_SETS: CodingStep = CodingStep {
    id: "py-34-sets",
    title: "Python Sets",
    objective: "Crear un set y medir su longitud con len().",
    prompt_md: "**Sets**\n\nLos sets usan llaves `{}` y no permiten duplicados.\n\n**Micro-reto:**\n1. Creá `thisset` con `apple`, `banana` y `cherry`\n2. Imprimí `len(thisset)`",
    starter_code: "# thisset = ...\n# print(len(...))\n",
    pytest: "def test_sets(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thisset') == {'apple', 'banana', 'cherry'}\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '3'\n",
    hint: "thisset = {\"apple\", \"banana\", \"cherry\"}\nprint(len(thisset))",
    solution_example: "thisset = {\"apple\", \"banana\", \"cherry\"}\nprint(len(thisset))\n",
    next: Some("py-35-set-access"),
    show_type_chips: false,
    micro_step: 34,
};

pub const PY35_SET_ACCESS: CodingStep = CodingStep {
    id: "py-35-set-access",
    title: "Python Access Set Items",
    objective: "Comprobar pertenencia en un set con in.",
    prompt_md: "**Access Set Items**\n\nNo hay índice; usá `in` para preguntar si un valor está presente.\n\n**Micro-reto:**\n1. Creá `thisset` con `apple`, `banana` y `cherry`\n2. Imprimí el resultado de `\"banana\" in thisset`",
    starter_code: "# thisset = ...\n# print(... in ...)\n",
    pytest: "def test_set_access(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thisset') == {'apple', 'banana', 'cherry'}\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'True'\n",
    hint: "thisset = {\"apple\", \"banana\", \"cherry\"}\nprint(\"banana\" in thisset)",
    solution_example: "thisset = {\"apple\", \"banana\", \"cherry\"}\nprint(\"banana\" in thisset)\n",
    next: Some("py-36-set-add"),
    show_type_chips: false,
    micro_step: 35,
};

pub const PY36_SET_ADD: CodingStep = CodingStep {
    id: "py-36-set-add",
    title: "Python Add Set Items",
    objective: "Agregar un ítem al set con add().",
    prompt_md: "**Add Set Items**\n\n`add()` agrega un ítem al set.\n\n**Micro-reto:**\n1. Creá `thisset` con `apple`, `banana` y `cherry`\n2. Agregá `orange` con `add()`\n3. Imprimí `\"orange\" in thisset`",
    starter_code: "# thisset = ...\n# thisset.add(...)\n# print(...)\n",
    pytest: "def test_set_add(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thisset') == {'apple', 'banana', 'cherry', 'orange'}\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'True'\n",
    hint: "thisset = {\"apple\", \"banana\", \"cherry\"}\nthisset.add(\"orange\")\nprint(\"orange\" in thisset)",
    solution_example: "thisset = {\"apple\", \"banana\", \"cherry\"}\nthisset.add(\"orange\")\nprint(\"orange\" in thisset)\n",
    next: Some("py-37-set-remove"),
    show_type_chips: false,
    micro_step: 36,
};

pub const PY37_SET_REMOVE: CodingStep = CodingStep {
    id: "py-37-set-remove",
    title: "Python Remove Set Items",
    objective: "Eliminar un ítem del set con remove().",
    prompt_md: "**Remove Set Items**\n\n`remove()` elimina el ítem indicado.\n\n**Micro-reto:**\n1. Creá `thisset` con `apple`, `banana` y `cherry`\n2. Remové `banana` con `remove()`\n3. Imprimí `\"banana\" in thisset`",
    starter_code: "# thisset = ...\n# thisset.remove(...)\n# print(...)\n",
    pytest: "def test_set_remove(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thisset') == {'apple', 'cherry'}\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'False'\n",
    hint: "thisset = {\"apple\", \"banana\", \"cherry\"}\nthisset.remove(\"banana\")\nprint(\"banana\" in thisset)",
    solution_example: "thisset = {\"apple\", \"banana\", \"cherry\"}\nthisset.remove(\"banana\")\nprint(\"banana\" in thisset)\n",
    next: Some("py-38-set-loop"),
    show_type_chips: false,
    micro_step: 37,
};

pub const PY38_SET_LOOP: CodingStep = CodingStep {
    id: "py-38-set-loop",
    title: "Python Loop Sets",
    objective: "Recorrer un set con for e imprimir cada ítem.",
    prompt_md: "**Loop Sets**\n\nPodés recorrer los ítems con un `for`.\n\n**Micro-reto:**\n1. Creá `thisset` con `apple`, `banana` y `cherry`\n2. Recorré con `for x in thisset` e imprimí cada `x`",
    starter_code: "# thisset = ...\n# for x in thisset:\n#     print(x)\n",
    pytest: "def test_set_loop(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thisset') == {'apple', 'banana', 'cherry'}\n    lines = {ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()}\n    assert lines == {'apple', 'banana', 'cherry'}\n",
    hint: "thisset = {\"apple\", \"banana\", \"cherry\"}\nfor x in thisset:\n    print(x)",
    solution_example: "thisset = {\"apple\", \"banana\", \"cherry\"}\nfor x in thisset:\n    print(x)\n",
    next: Some("py-39-set-join"),
    show_type_chips: false,
    micro_step: 38,
};

pub const PY39_SET_JOIN: CodingStep = CodingStep {
    id: "py-39-set-join",
    title: "Python Join Sets",
    objective: "Unir dos sets con union().",
    prompt_md: "**Join Sets**\n\n`union()` devuelve un set nuevo con ítems de ambos.\n\n**Micro-reto:**\n1. Creá `set1` con `a`, `b`, `c`\n2. Creá `set2` con `1`, `2`, `3`\n3. Creá `set3` como `set1.union(set2)`\n4. Imprimí `len(set3)`",
    starter_code: "# set1 = ...\n# set2 = ...\n# set3 = ...\n# print(len(...))\n",
    pytest: "def test_set_join(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('set1') == {'a', 'b', 'c'}\n    assert ns.get('set2') == {1, 2, 3}\n    assert ns.get('set3') == {'a', 'b', 'c', 1, 2, 3}\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '6'\n",
    hint: "set1 = {\"a\", \"b\", \"c\"}\nset2 = {1, 2, 3}\nset3 = set1.union(set2)\nprint(len(set3))",
    solution_example: "set1 = {\"a\", \"b\", \"c\"}\nset2 = {1, 2, 3}\nset3 = set1.union(set2)\nprint(len(set3))\n",
    next: Some("py-40-dictionaries"),
    show_type_chips: false,
    micro_step: 39,
};

pub const PY40_DICTIONARIES: CodingStep = CodingStep {
    id: "py-40-dictionaries",
    title: "Python Dictionaries",
    objective: "Crear un dictionary y medir su longitud con len().",
    prompt_md: "**Dictionaries**\n\nLos dictionaries guardan pares `clave:valor` entre llaves `{}`.\n\n**Micro-reto:**\n1. Creá `thisdict` con `brand`=`Ford`, `model`=`Mustang`, `year`=`1964`\n2. Imprimí `len(thisdict)`",
    starter_code: "# thisdict = {...}\n# print(len(...))\n",
    pytest: "def test_dictionaries(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thisdict') == {'brand': 'Ford', 'model': 'Mustang', 'year': 1964}\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '3'\n",
    hint: "thisdict = {\"brand\": \"Ford\", \"model\": \"Mustang\", \"year\": 1964}\nprint(len(thisdict))",
    solution_example: "thisdict = {\"brand\": \"Ford\", \"model\": \"Mustang\", \"year\": 1964}\nprint(len(thisdict))\n",
    next: Some("py-41-dict-access"),
    show_type_chips: false,
    micro_step: 40,
};

pub const PY41_DICT_ACCESS: CodingStep = CodingStep {
    id: "py-41-dict-access",
    title: "Python Access Dictionary Items",
    objective: "Acceder al valor de una clave con corchetes.",
    prompt_md: "**Access Dictionary Items**\n\nAccedé a un valor con `dict[\"clave\"]`.\n\n**Micro-reto:**\n1. Creá `thisdict` con `brand`=`Ford`, `model`=`Mustang`, `year`=`1964`\n2. Imprimí `thisdict[\"model\"]`",
    starter_code: "# thisdict = {...}\n# print(...)\n",
    pytest: "def test_dict_access(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thisdict') == {'brand': 'Ford', 'model': 'Mustang', 'year': 1964}\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'Mustang'\n",
    hint: "thisdict = {\"brand\": \"Ford\", \"model\": \"Mustang\", \"year\": 1964}\nprint(thisdict[\"model\"])",
    solution_example: "thisdict = {\"brand\": \"Ford\", \"model\": \"Mustang\", \"year\": 1964}\nprint(thisdict[\"model\"])\n",
    next: Some("py-42-dict-change"),
    show_type_chips: false,
    micro_step: 41,
};

pub const PY42_DICT_CHANGE: CodingStep = CodingStep {
    id: "py-42-dict-change",
    title: "Python Change Dictionary Items",
    objective: "Cambiar el valor de una clave existente.",
    prompt_md: "**Change Dictionary Items**\n\nAsigná un nuevo valor a una clave existente.\n\n**Micro-reto:**\n1. Creá `thisdict` con `brand`=`Ford`, `model`=`Mustang`, `year`=`1964`\n2. Cambiá `year` a `2018`\n3. Imprimí `thisdict[\"year\"]`",
    starter_code: "# thisdict = {...}\n# thisdict[\"year\"] = ...\n# print(...)\n",
    pytest: "def test_dict_change(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thisdict') == {'brand': 'Ford', 'model': 'Mustang', 'year': 2018}\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '2018'\n",
    hint: "thisdict = {\"brand\": \"Ford\", \"model\": \"Mustang\", \"year\": 1964}\nthisdict[\"year\"] = 2018\nprint(thisdict[\"year\"])",
    solution_example: "thisdict = {\"brand\": \"Ford\", \"model\": \"Mustang\", \"year\": 1964}\nthisdict[\"year\"] = 2018\nprint(thisdict[\"year\"])\n",
    next: Some("py-43-dict-add"),
    show_type_chips: false,
    micro_step: 42,
};

pub const PY43_DICT_ADD: CodingStep = CodingStep {
    id: "py-43-dict-add",
    title: "Python Add Dictionary Items",
    objective: "Agregar un par clave:valor nuevo al dictionary.",
    prompt_md: "**Add Dictionary Items**\n\nAgregá una clave nueva asignándole un valor.\n\n**Micro-reto:**\n1. Creá `thisdict` con `brand`=`Ford`, `model`=`Mustang`, `year`=`1964`\n2. Agregá `color`=`red`\n3. Imprimí `thisdict[\"color\"]`",
    starter_code: "# thisdict = {...}\n# thisdict[\"color\"] = ...\n# print(...)\n",
    pytest: "def test_dict_add(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thisdict') == {'brand': 'Ford', 'model': 'Mustang', 'year': 1964, 'color': 'red'}\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'red'\n",
    hint: "thisdict = {\"brand\": \"Ford\", \"model\": \"Mustang\", \"year\": 1964}\nthisdict[\"color\"] = \"red\"\nprint(thisdict[\"color\"])",
    solution_example: "thisdict = {\"brand\": \"Ford\", \"model\": \"Mustang\", \"year\": 1964}\nthisdict[\"color\"] = \"red\"\nprint(thisdict[\"color\"])\n",
    next: Some("py-44-dict-remove"),
    show_type_chips: false,
    micro_step: 43,
};

pub const PY44_DICT_REMOVE: CodingStep = CodingStep {
    id: "py-44-dict-remove",
    title: "Python Remove Dictionary Items",
    objective: "Eliminar una clave con pop().",
    prompt_md: "**Remove Dictionary Items**\n\n`pop()` elimina el ítem con la clave indicada.\n\n**Micro-reto:**\n1. Creá `thisdict` con `brand`=`Ford`, `model`=`Mustang`, `year`=`1964`\n2. Remové `model` con `pop()`\n3. Imprimí `\"model\" in thisdict`",
    starter_code: "# thisdict = {...}\n# thisdict.pop(...)\n# print(...)\n",
    pytest: "def test_dict_remove(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thisdict') == {'brand': 'Ford', 'year': 1964}\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'False'\n",
    hint: "thisdict = {\"brand\": \"Ford\", \"model\": \"Mustang\", \"year\": 1964}\nthisdict.pop(\"model\")\nprint(\"model\" in thisdict)",
    solution_example: "thisdict = {\"brand\": \"Ford\", \"model\": \"Mustang\", \"year\": 1964}\nthisdict.pop(\"model\")\nprint(\"model\" in thisdict)\n",
    next: Some("py-45-dict-loop"),
    show_type_chips: false,
    micro_step: 44,
};

pub const PY45_DICT_LOOP: CodingStep = CodingStep {
    id: "py-45-dict-loop",
    title: "Python Loop Dictionaries",
    objective: "Recorrer las claves de un dictionary con for.",
    prompt_md: "**Loop Dictionaries**\n\nAl recorrer un dictionary con `for`, obtenés las claves.\n\n**Micro-reto:**\n1. Creá `thisdict` con `brand`=`Ford`, `model`=`Mustang`, `year`=`1964`\n2. Recorré con `for x in thisdict` e imprimí cada `x`",
    starter_code: "# thisdict = {...}\n# for x in thisdict:\n#     print(x)\n",
    pytest: "def test_dict_loop(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thisdict') == {'brand': 'Ford', 'model': 'Mustang', 'year': 1964}\n    lines = {ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()}\n    assert lines == {'brand', 'model', 'year'}\n",
    hint: "thisdict = {\"brand\": \"Ford\", \"model\": \"Mustang\", \"year\": 1964}\nfor x in thisdict:\n    print(x)",
    solution_example: "thisdict = {\"brand\": \"Ford\", \"model\": \"Mustang\", \"year\": 1964}\nfor x in thisdict:\n    print(x)\n",
    next: Some("py-46-dict-copy"),
    show_type_chips: false,
    micro_step: 45,
};

pub const PY46_DICT_COPY: CodingStep = CodingStep {
    id: "py-46-dict-copy",
    title: "Python Copy Dictionaries",
    objective: "Copiar un dictionary con copy().",
    prompt_md: "**Copy Dictionaries**\n\n`dict2 = dict1` solo crea una referencia. Usá `copy()`.\n\n**Micro-reto:**\n1. Creá `thisdict` con `brand`=`Ford`, `model`=`Mustang`, `year`=`1964`\n2. Creá `mydict` con `thisdict.copy()`\n3. Imprimí `mydict[\"brand\"]`",
    starter_code: "# thisdict = {...}\n# mydict = ...\n# print(...)\n",
    pytest: "def test_dict_copy(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('thisdict') == {'brand': 'Ford', 'model': 'Mustang', 'year': 1964}\n    assert ns.get('mydict') == {'brand': 'Ford', 'model': 'Mustang', 'year': 1964}\n    assert ns.get('mydict') is not ns.get('thisdict')\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'Ford'\n",
    hint: "thisdict = {\"brand\": \"Ford\", \"model\": \"Mustang\", \"year\": 1964}\nmydict = thisdict.copy()\nprint(mydict[\"brand\"])",
    solution_example: "thisdict = {\"brand\": \"Ford\", \"model\": \"Mustang\", \"year\": 1964}\nmydict = thisdict.copy()\nprint(mydict[\"brand\"])\n",
    next: Some("py-47-dict-nested"),
    show_type_chips: false,
    micro_step: 46,
};

pub const PY47_DICT_NESTED: CodingStep = CodingStep {
    id: "py-47-dict-nested",
    title: "Python Nested Dictionaries",
    objective: "Acceder a un valor en un dictionary anidado.",
    prompt_md: "**Nested Dictionaries**\n\nUn dictionary puede contener otros dictionaries.\n\n**Micro-reto:**\n1. Creá `myfamily` con `child1`, `child2` y `child3` (nombres Emil/Tobias/Linus y años 2004/2007/2011)\n2. Imprimí `myfamily[\"child2\"][\"name\"]`",
    starter_code: "# myfamily = {...}\n# print(...)\n",
    pytest: "def test_dict_nested(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    fam = ns.get('myfamily')\n    assert isinstance(fam, dict)\n    assert fam['child2']['name'] == 'Tobias'\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'Tobias'\n",
    hint: "myfamily = {\n  \"child1\": {\"name\": \"Emil\", \"year\": 2004},\n  \"child2\": {\"name\": \"Tobias\", \"year\": 2007},\n  \"child3\": {\"name\": \"Linus\", \"year\": 2011}\n}\nprint(myfamily[\"child2\"][\"name\"])",
    solution_example: "myfamily = {\n  \"child1\": {\"name\": \"Emil\", \"year\": 2004},\n  \"child2\": {\"name\": \"Tobias\", \"year\": 2007},\n  \"child3\": {\"name\": \"Linus\", \"year\": 2011}\n}\nprint(myfamily[\"child2\"][\"name\"])\n",
    next: Some("py-48-if"),
    show_type_chips: false,
    micro_step: 47,
};

pub const PY48_IF: CodingStep = CodingStep {
    id: "py-48-if",
    title: "Python If Statement",
    objective: "Usar if para ejecutar código cuando una condición es True.",
    prompt_md: "**If Statement**\n\n`if` ejecuta un bloque cuando la condición es verdadera.\n\n**Micro-reto:**\n1. Creá `a` con `33` y `b` con `200`\n2. Si `b > a`, imprimí `b is greater than a`",
    starter_code: "# a = ...\n# b = ...\n# if ...:\n#     print(...)\n",
    pytest: "def test_if(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('a') == 33 and ns.get('b') == 200\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'b is greater than a'\n",
    hint: "a = 33\nb = 200\nif b > a:\n    print(\"b is greater than a\")",
    solution_example: "a = 33\nb = 200\nif b > a:\n    print(\"b is greater than a\")\n",
    next: Some("py-49-elif"),
    show_type_chips: false,
    micro_step: 48,
};

pub const PY49_ELIF: CodingStep = CodingStep {
    id: "py-49-elif",
    title: "Python Elif",
    objective: "Usar elif para una condición alternativa.",
    prompt_md: "**Elif**\n\n`elif` se evalúa si el `if` anterior fue falso.\n\n**Micro-reto:**\n1. Creá `a` con `33` y `b` con `33`\n2. Si `b > a` imprimí `b is greater than a`\n3. Sino, si `a == b`, imprimí `a and b are equal`",
    starter_code: "# a = ...\n# b = ...\n# if ...:\n#     ...\n# elif ...:\n#     ...\n",
    pytest: "def test_elif(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('a') == 33 and ns.get('b') == 33\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'a and b are equal'\n",
    hint: "a = 33\nb = 33\nif b > a:\n    print(\"b is greater than a\")\nelif a == b:\n    print(\"a and b are equal\")",
    solution_example: "a = 33\nb = 33\nif b > a:\n    print(\"b is greater than a\")\nelif a == b:\n    print(\"a and b are equal\")\n",
    next: Some("py-50-while"),
    show_type_chips: false,
    micro_step: 49,
};

pub const PY50_WHILE: CodingStep = CodingStep {
    id: "py-50-while",
    title: "Python While Loops",
    objective: "Repetir con while mientras la condición sea True.",
    prompt_md: "**While Loops**\n\n`while` ejecuta un bloque mientras la condición sea verdadera.\n\n**Micro-reto:**\n1. Inicializá `i` en `1`\n2. Mientras `i < 6`, imprimí `i` e incrementá `i`",
    starter_code: "# i = ...\n# while ...:\n#     print(i)\n#     i += 1\n",
    pytest: "def test_while(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['1', '2', '3', '4', '5']\n",
    hint: "i = 1\nwhile i < 6:\n    print(i)\n    i += 1",
    solution_example: "i = 1\nwhile i < 6:\n    print(i)\n    i += 1\n",
    next: Some("py-51-for"),
    show_type_chips: false,
    micro_step: 50,
};

pub const PY51_FOR: CodingStep = CodingStep {
    id: "py-51-for",
    title: "Python For Loops",
    objective: "Iterar una lista con for.",
    prompt_md: "**For Loops**\n\n`for` itera sobre una secuencia.\n\n**Micro-reto:**\n1. Creá `fruits` con `apple`, `banana`, `cherry`\n2. Recorré con `for x in fruits` e imprimí cada `x`",
    starter_code: "# fruits = ...\n# for x in fruits:\n#     print(x)\n",
    pytest: "def test_for(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('fruits') == ['apple', 'banana', 'cherry']\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['apple', 'banana', 'cherry']\n",
    hint: "fruits = [\"apple\", \"banana\", \"cherry\"]\nfor x in fruits:\n    print(x)",
    solution_example: "fruits = [\"apple\", \"banana\", \"cherry\"]\nfor x in fruits:\n    print(x)\n",
    next: Some("py-52-functions"),
    show_type_chips: false,
    micro_step: 51,
};

pub const PY52_FUNCTIONS: CodingStep = CodingStep {
    id: "py-52-functions",
    title: "Python Functions",
    objective: "Definir una función con def y llamarla.",
    prompt_md: "**Functions**\n\nUna función se define con `def` y se ejecuta al llamarla.\n\n**Micro-reto:**\n1. Definí `my_function` que imprima `Hello from a function`\n2. Llamá `my_function()`",
    starter_code: "# def my_function():\n#     ...\n# my_function()\n",
    pytest: "def test_functions(capsys):\n    exec(open('solution.py', encoding='utf-8').read())\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'Hello from a function'\n",
    hint: "def my_function():\n    print(\"Hello from a function\")\nmy_function()",
    solution_example: "def my_function():\n    print(\"Hello from a function\")\nmy_function()\n",
    next: Some("py-53-function-args"),
    show_type_chips: false,
    micro_step: 52,
};

pub const PY53_FUNCTION_ARGS: CodingStep = CodingStep {
    id: "py-53-function-args",
    title: "Python Function Arguments",
    objective: "Pasar un argumento a una función.",
    prompt_md: "**Function Arguments**\n\nPodés pasar información a la función como argumentos.\n\n**Micro-reto:**\n1. Definí `my_function(fname)` que imprima `fname + \" Refsnes\"`\n2. Llamá `my_function(\"Emil\")`",
    starter_code: "# def my_function(fname):\n#     ...\n# my_function(...)\n",
    pytest: "def test_function_args(capsys):\n    exec(open('solution.py', encoding='utf-8').read())\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'Emil Refsnes'\n",
    hint: "def my_function(fname):\n    print(fname + \" Refsnes\")\nmy_function(\"Emil\")",
    solution_example: "def my_function(fname):\n    print(fname + \" Refsnes\")\nmy_function(\"Emil\")\n",
    next: Some("py-54-function-return"),
    show_type_chips: false,
    micro_step: 53,
};

pub const PY54_FUNCTION_RETURN: CodingStep = CodingStep {
    id: "py-54-function-return",
    title: "Python Function Return",
    objective: "Devolver un valor con return.",
    prompt_md: "**Return Values**\n\n`return` envía un resultado al código que llamó la función.\n\n**Micro-reto:**\n1. Definí `my_function(x)` que retorne `5 * x`\n2. Imprimí `my_function(3)`",
    starter_code: "# def my_function(x):\n#     return ...\n# print(...)\n",
    pytest: "def test_function_return(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['my_function'](3) == 15\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '15'\n",
    hint: "def my_function(x):\n    return 5 * x\nprint(my_function(3))",
    solution_example: "def my_function(x):\n    return 5 * x\nprint(my_function(3))\n",
    next: Some("py-55-lambda"),
    show_type_chips: false,
    micro_step: 54,
};

pub const PY55_LAMBDA: CodingStep = CodingStep {
    id: "py-55-lambda",
    title: "Python Lambda",
    objective: "Crear una función anónima con lambda.",
    prompt_md: "**Lambda**\n\n`lambda` crea una función pequeña de una sola expresión.\n\n**Micro-reto:**\n1. Creá `x` como `lambda a : a + 10`\n2. Imprimí `x(5)`",
    starter_code: "# x = lambda ...\n# print(...)\n",
    pytest: "def test_lambda(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['x'](5) == 15\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '15'\n",
    hint: "x = lambda a : a + 10\nprint(x(5))",
    solution_example: "x = lambda a : a + 10\nprint(x(5))\n",
    next: Some("py-56-arrays"),
    show_type_chips: false,
    micro_step: 55,
};

pub const PY56_ARRAYS: CodingStep = CodingStep {
    id: "py-56-arrays",
    title: "Python Arrays",
    objective: "Usar una list como array: acceso, append y len.",
    prompt_md: "**Arrays**\n\nPython no tiene arrays nativos; se usan lists.\n\n**Micro-reto:**\n1. Creá `cars` con `Ford`, `Volvo`, `BMW`\n2. Agregá `Honda` con `append()`\n3. Imprimí `len(cars)`",
    starter_code: "# cars = ...\n# cars.append(...)\n# print(len(...))\n",
    pytest: "def test_arrays(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('cars') == ['Ford', 'Volvo', 'BMW', 'Honda']\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '4'\n",
    hint: "cars = [\"Ford\", \"Volvo\", \"BMW\"]\ncars.append(\"Honda\")\nprint(len(cars))",
    solution_example: "cars = [\"Ford\", \"Volvo\", \"BMW\"]\ncars.append(\"Honda\")\nprint(len(cars))\n",
    next: Some("py-57-classes"),
    show_type_chips: false,
    micro_step: 56,
};

pub const PY57_CLASSES: CodingStep = CodingStep {
    id: "py-57-classes",
    title: "Python Classes/Objects",
    objective: "Crear una clase, un objeto y leer una propiedad.",
    prompt_md: "**Classes/Objects**\n\nUna class es un blueprint; un object es una instancia.\n\n**Micro-reto:**\n1. Creá la clase `MyClass` con propiedad `x = 5`\n2. Creá el object `p1 = MyClass()`\n3. Imprimí `p1.x`",
    starter_code: "# class MyClass:\n#     x = ...\n# p1 = ...\n# print(...)\n",
    pytest: "def test_classes(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['p1'].x == 5\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '5'\n",
    hint: "class MyClass:\n    x = 5\np1 = MyClass()\nprint(p1.x)",
    solution_example: "class MyClass:\n    x = 5\np1 = MyClass()\nprint(p1.x)\n",
    next: Some("py-58-init"),
    show_type_chips: false,
    micro_step: 57,
};

pub const PY58_INIT: CodingStep = CodingStep {
    id: "py-58-init",
    title: "Python __init__",
    objective: "Inicializar propiedades de un object con __init__.",
    prompt_md: "**The __init__() Function**\n\n`__init__` se ejecuta al crear un object.\n\n**Micro-reto:**\n1. Creá la clase `Person` con `__init__(self, name, age)` que asigne `self.name` y `self.age`\n2. Creá `p1 = Person(\"John\", 36)`\n3. Imprimí `p1.name` y `p1.age`",
    starter_code: "# class Person:\n#     def __init__(...):\n#         ...\n# p1 = ...\n# print(...)\n",
    pytest: "def test_init(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['p1'].name == 'John' and ns['p1'].age == 36\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['John', '36']\n",
    hint: "class Person:\n    def __init__(self, name, age):\n        self.name = name\n        self.age = age\np1 = Person(\"John\", 36)\nprint(p1.name)\nprint(p1.age)",
    solution_example: "class Person:\n    def __init__(self, name, age):\n        self.name = name\n        self.age = age\np1 = Person(\"John\", 36)\nprint(p1.name)\nprint(p1.age)\n",
    next: Some("py-59-inheritance"),
    show_type_chips: false,
    micro_step: 58,
};

pub const PY59_INHERITANCE: CodingStep = CodingStep {
    id: "py-59-inheritance",
    title: "Python Inheritance",
    objective: "Heredar métodos de una clase padre.",
    prompt_md: "**Inheritance**\n\nUna child class hereda de una parent class.\n\n**Micro-reto:**\n1. Creá `Person` con `__init__(self, fname, lname)` y método `printname`\n2. Creá `Student(Person)` con `pass`\n3. Creá `x = Student(\"Mike\", \"Olsen\")` y llamá `x.printname()`",
    starter_code: "# class Person:\n#     ...\n# class Student(Person):\n#     pass\n# x = ...\n# x.printname()\n",
    pytest: "def test_inheritance(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert type(ns['x']).__name__ == 'Student'\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'Mike Olsen'\n",
    hint: "class Person:\n    def __init__(self, fname, lname):\n        self.firstname = fname\n        self.lastname = lname\n    def printname(self):\n        print(self.firstname, self.lastname)\nclass Student(Person):\n    pass\nx = Student(\"Mike\", \"Olsen\")\nx.printname()",
    solution_example: "class Person:\n    def __init__(self, fname, lname):\n        self.firstname = fname\n        self.lastname = lname\n    def printname(self):\n        print(self.firstname, self.lastname)\nclass Student(Person):\n    pass\nx = Student(\"Mike\", \"Olsen\")\nx.printname()\n",
    next: Some("py-60-iterators"),
    show_type_chips: false,
    micro_step: 59,
};

pub const PY60_ITERATORS: CodingStep = CodingStep {
    id: "py-60-iterators",
    title: "Python Iterators",
    objective: "Obtener un iterator y avanzar con next().",
    prompt_md: "**Iterators**\n\n`iter()` crea un iterator; `next()` obtiene el siguiente valor.\n\n**Micro-reto:**\n1. Creá `mytuple` con `apple`, `banana`, `cherry`\n2. Creá `myit = iter(mytuple)`\n3. Imprimí `next(myit)` tres veces",
    starter_code: "# mytuple = ...\n# myit = ...\n# print(next(...))\n",
    pytest: "def test_iterators(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('mytuple') == ('apple', 'banana', 'cherry')\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['apple', 'banana', 'cherry']\n",
    hint: "mytuple = (\"apple\", \"banana\", \"cherry\")\nmyit = iter(mytuple)\nprint(next(myit))\nprint(next(myit))\nprint(next(myit))",
    solution_example: "mytuple = (\"apple\", \"banana\", \"cherry\")\nmyit = iter(mytuple)\nprint(next(myit))\nprint(next(myit))\nprint(next(myit))\n",
    next: Some("py-61-polymorphism"),
    show_type_chips: false,
    micro_step: 60,
};

pub const PY61_POLYMORPHISM: CodingStep = CodingStep {
    id: "py-61-polymorphism",
    title: "Python Polymorphism",
    objective: "Usar len() sobre distintos tipos (polimorfismo).",
    prompt_md: "**Polymorphism**\n\n`len()` funciona con strings, tuples y dictionaries.\n\n**Micro-reto:**\n1. Creá `x` con el string `Hello World!`\n2. Creá `mytuple` con `apple`, `banana`, `cherry`\n3. Imprimí `len(x)` y `len(mytuple)`",
    starter_code: "# x = ...\n# mytuple = ...\n# print(len(...))\n",
    pytest: "def test_polymorphism(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('x') == 'Hello World!'\n    assert ns.get('mytuple') == ('apple', 'banana', 'cherry')\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['12', '3']\n",
    hint: "x = \"Hello World!\"\nmytuple = (\"apple\", \"banana\", \"cherry\")\nprint(len(x))\nprint(len(mytuple))",
    solution_example: "x = \"Hello World!\"\nmytuple = (\"apple\", \"banana\", \"cherry\")\nprint(len(x))\nprint(len(mytuple))\n",
    next: Some("py-62-scope"),
    show_type_chips: false,
    micro_step: 61,
};

pub const PY62_SCOPE: CodingStep = CodingStep {
    id: "py-62-scope",
    title: "Python Scope",
    objective: "Usar una variable local dentro de una función.",
    prompt_md: "**Scope**\n\nUna variable creada dentro de una función es local a esa función.\n\n**Micro-reto:**\n1. Definí `myfunc` que cree `x = 300` e imprima `x`\n2. Llamá `myfunc()`",
    starter_code: "# def myfunc():\n#     ...\n# myfunc()\n",
    pytest: "def test_scope(capsys):\n    exec(open('solution.py', encoding='utf-8').read())\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '300'\n",
    hint: "def myfunc():\n    x = 300\n    print(x)\nmyfunc()",
    solution_example: "def myfunc():\n    x = 300\n    print(x)\nmyfunc()\n",
    next: Some("py-63-modules"),
    show_type_chips: false,
    micro_step: 62,
};

pub const PY63_MODULES: CodingStep = CodingStep {
    id: "py-63-modules",
    title: "Python Modules",
    objective: "Importar un módulo built-in y usarlo.",
    prompt_md: "**Modules**\n\nPodés importar módulos built-in como `platform`.\n\n**Micro-reto:**\n1. `import platform`\n2. Guardá en `x` el resultado de `platform.system()`\n3. Imprimí `type(x).__name__` (debe ser `str`)",
    starter_code: "# import platform\n# x = ...\n# print(...)\n",
    pytest: "def test_modules(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert isinstance(ns.get('x'), str)\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'str'\n",
    hint: "import platform\nx = platform.system()\nprint(type(x).__name__)",
    solution_example: "import platform\nx = platform.system()\nprint(type(x).__name__)\n",
    next: Some("py-64-dates"),
    show_type_chips: false,
    micro_step: 63,
};

pub const PY64_DATES: CodingStep = CodingStep {
    id: "py-64-dates",
    title: "Python Dates",
    objective: "Crear un objeto datetime con año, mes y día.",
    prompt_md: "**Python Dates**\n\nEl módulo `datetime` trabaja con fechas.\n\n**Micro-reto:**\n1. `import datetime`\n2. Creá `x = datetime.datetime(2020, 5, 17)`\n3. Imprimí `x`",
    starter_code: "# import datetime\n# x = ...\n# print(x)\n",
    pytest: "def test_dates(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    import datetime as _dt\n    assert ns.get('x') == _dt.datetime(2020, 5, 17)\n    out = ' '.join(capsys.readouterr().out.split())\n    assert '2020-05-17' in out\n",
    hint: "import datetime\nx = datetime.datetime(2020, 5, 17)\nprint(x)",
    solution_example: "import datetime\nx = datetime.datetime(2020, 5, 17)\nprint(x)\n",
    next: Some("py-65-math"),
    show_type_chips: false,
    micro_step: 64,
};

pub const PY65_MATH: CodingStep = CodingStep {
    id: "py-65-math",
    title: "Python Math",
    objective: "Usar math.sqrt para la raíz cuadrada.",
    prompt_md: "**Python Math**\n\nEl módulo `math` extiende las funciones matemáticas.\n\n**Micro-reto:**\n1. `import math`\n2. Guardá en `x` el resultado de `math.sqrt(64)`\n3. Imprimí `x`",
    starter_code: "# import math\n# x = ...\n# print(x)\n",
    pytest: "def test_math(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('x') == 8.0\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '8.0'\n",
    hint: "import math\nx = math.sqrt(64)\nprint(x)",
    solution_example: "import math\nx = math.sqrt(64)\nprint(x)\n",
    next: Some("py-66-json"),
    show_type_chips: false,
    micro_step: 65,
};

pub const PY66_JSON: CodingStep = CodingStep {
    id: "py-66-json",
    title: "Python JSON",
    objective: "Convertir un string JSON a un dict de Python.",
    prompt_md: "**Python JSON**\n\n`json.loads` convierte un string JSON a un objeto Python.\n\n**Micro-reto:**\n1. `import json`\n2. Creá `x` con el string JSON `{\"name\":\"John\", \"age\":30}`\n3. `y = json.loads(x)` e imprimí `y[\"name\"]`",
    starter_code: "# import json\n# x = ...\n# y = ...\n# print(...)\n",
    pytest: "def test_json(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('y', {}).get('name') == 'John'\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'John'\n",
    hint: "import json\nx = '{\"name\":\"John\", \"age\":30}'\ny = json.loads(x)\nprint(y[\"name\"])",
    solution_example: "import json\nx = '{\"name\":\"John\", \"age\":30}'\ny = json.loads(x)\nprint(y[\"name\"])\n",
    next: Some("py-67-regex"),
    show_type_chips: false,
    micro_step: 66,
};

pub const PY67_REGEX: CodingStep = CodingStep {
    id: "py-67-regex",
    title: "Python RegEx",
    objective: "Buscar un patrón con re.search.",
    prompt_md: "**Python RegEx**\n\nEl módulo `re` busca patrones en strings.\n\n**Micro-reto:**\n1. `import re`\n2. Creá `txt` con `The rain in Spain`\n3. `x = re.search(\"Spain\", txt)` e imprimí `x.group()`",
    starter_code: "# import re\n# txt = ...\n# x = ...\n# print(...)\n",
    pytest: "def test_regex(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('txt') == 'The rain in Spain'\n    assert ns['x'].group() == 'Spain'\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'Spain'\n",
    hint: "import re\ntxt = \"The rain in Spain\"\nx = re.search(\"Spain\", txt)\nprint(x.group())",
    solution_example: "import re\ntxt = \"The rain in Spain\"\nx = re.search(\"Spain\", txt)\nprint(x.group())\n",
    next: Some("py-68-try-except"),
    show_type_chips: false,
    micro_step: 67,
};

pub const PY68_TRY_EXCEPT: CodingStep = CodingStep {
    id: "py-68-try-except",
    title: "Python Try Except",
    objective: "Capturar un error con try/except.",
    prompt_md: "**Try...Except**\n\n`try` prueba un bloque; `except` maneja el error.\n\n**Micro-reto:**\n1. En un `try`, ejecutá `print(x)` donde `x` no está definido\n2. En el `except`, imprimí exactamente `An exception occurred`",
    starter_code: "# try:\n#     ...\n# except:\n#     ...\n",
    pytest: "def test_try_except(capsys):\n    exec(open('solution.py', encoding='utf-8').read())\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'An exception occurred'\n",
    hint: "try:\n    print(x)\nexcept:\n    print(\"An exception occurred\")",
    solution_example: "try:\n    print(x)\nexcept:\n    print(\"An exception occurred\")\n",
    next: Some("py-69-string-formatting"),
    show_type_chips: false,
    micro_step: 68,
};

pub const PY69_STRING_FORMATTING: CodingStep = CodingStep {
    id: "py-69-string-formatting",
    title: "Python String Formatting",
    objective: "Usar un f-string para interpolar un valor.",
    prompt_md: "**String Formatting**\n\nLos f-strings interpolan variables en un string.\n\n**Micro-reto:**\n1. Creá `price = 49`\n2. Creá `txt = f\"The price is {price} dollars\"`\n3. Imprimí `txt`",
    starter_code: "# price = ...\n# txt = ...\n# print(txt)\n",
    pytest: "def test_string_formatting(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('price') == 49\n    assert ns.get('txt') == 'The price is 49 dollars'\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'The price is 49 dollars'\n",
    hint: "price = 49\ntxt = f\"The price is {price} dollars\"\nprint(txt)",
    solution_example: "price = 49\ntxt = f\"The price is {price} dollars\"\nprint(txt)\n",
    next: Some("py-70-file-write"),
    show_type_chips: false,
    micro_step: 69,
};

pub const PY70_FILE_WRITE: CodingStep = CodingStep {
    id: "py-70-file-write",
    title: "Python Write Files",
    objective: "Crear un archivo de texto con open(..., \"w\").",
    prompt_md: "**Write/Create Files**\n\n`open(nombre, \"w\")` crea o sobrescribe un archivo.\n\n**Micro-reto:**\n1. Abrí `demofile.txt` en modo `\"w\"`\n2. Escribí exactamente `Hello! Welcome to demofile.txt`\n3. Cerrá el archivo (o usá `with`)",
    starter_code: "# with open(\"demofile.txt\", \"w\") as f:\n#     ...\n",
    pytest: "def test_file_write():\n    exec(open('solution.py', encoding='utf-8').read())\n    with open('demofile.txt', encoding='utf-8') as f:\n        assert f.read() == 'Hello! Welcome to demofile.txt'\n",
    hint: "with open(\"demofile.txt\", \"w\") as f:\n    f.write(\"Hello! Welcome to demofile.txt\")",
    solution_example: "with open(\"demofile.txt\", \"w\") as f:\n    f.write(\"Hello! Welcome to demofile.txt\")\n",
    next: Some("py-71-file-read"),
    show_type_chips: false,
    micro_step: 70,
};

pub const PY71_FILE_READ: CodingStep = CodingStep {
    id: "py-71-file-read",
    title: "Python Read Files",
    objective: "Leer el contenido completo de un archivo.",
    prompt_md: "**Read Files**\n\n`f.read()` devuelve todo el texto del archivo.\n\n**Micro-reto:**\n1. Creá `demofile.txt` con contenido `Hello Python`\n2. Abrilo y imprimí `f.read()`",
    starter_code: "# with open(\"demofile.txt\", \"w\") as f:\n#     ...\n# with open(\"demofile.txt\") as f:\n#     print(...)\n",
    pytest: "def test_file_read(capsys):\n    exec(open('solution.py', encoding='utf-8').read())\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'Hello Python'\n",
    hint: "with open(\"demofile.txt\", \"w\") as f:\n    f.write(\"Hello Python\")\nwith open(\"demofile.txt\") as f:\n    print(f.read())",
    solution_example: "with open(\"demofile.txt\", \"w\") as f:\n    f.write(\"Hello Python\")\nwith open(\"demofile.txt\") as f:\n    print(f.read())\n",
    next: Some("py-72-file-readline"),
    show_type_chips: false,
    micro_step: 71,
};

pub const PY72_FILE_READLINE: CodingStep = CodingStep {
    id: "py-72-file-readline",
    title: "Python File Readline",
    objective: "Leer la primera línea con readline().",
    prompt_md: "**Read Lines**\n\n`readline()` lee una línea del archivo.\n\n**Micro-reto:**\n1. Creá `demofile.txt` con dos líneas: `First` y `Second` (con salto de línea entre ellas)\n2. Abrilo e imprimí `f.readline().strip()`",
    starter_code: "# with open(\"demofile.txt\", \"w\") as f:\n#     ...\n# with open(\"demofile.txt\") as f:\n#     print(...)\n",
    pytest: "def test_file_readline(capsys):\n    exec(open('solution.py', encoding='utf-8').read())\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'First'\n",
    hint: "with open(\"demofile.txt\", \"w\") as f:\n    f.write(\"First\\nSecond\\n\")\nwith open(\"demofile.txt\") as f:\n    print(f.readline().strip())",
    solution_example: "with open(\"demofile.txt\", \"w\") as f:\n    f.write(\"First\\nSecond\\n\")\nwith open(\"demofile.txt\") as f:\n    print(f.readline().strip())\n",
    next: Some("py-73-file-append"),
    show_type_chips: false,
    micro_step: 72,
};

pub const PY73_FILE_APPEND: CodingStep = CodingStep {
    id: "py-73-file-append",
    title: "Python File Append",
    objective: "Agregar texto al final con modo a.",
    prompt_md: "**Append Files**\n\nEl modo `\"a\"` agrega texto al final sin borrar lo existente.\n\n**Micro-reto:**\n1. Creá `demofile.txt` con `Hello`\n2. Abrilo en modo `\"a\"` y agregá ` World`\n3. Leé e imprimí el contenido completo",
    starter_code: "# with open(\"demofile.txt\", \"w\") as f:\n#     ...\n# with open(\"demofile.txt\", \"a\") as f:\n#     ...\n# with open(\"demofile.txt\") as f:\n#     print(...)\n",
    pytest: "def test_file_append(capsys):\n    exec(open('solution.py', encoding='utf-8').read())\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'Hello World'\n",
    hint: "with open(\"demofile.txt\", \"w\") as f:\n    f.write(\"Hello\")\nwith open(\"demofile.txt\", \"a\") as f:\n    f.write(\" World\")\nwith open(\"demofile.txt\") as f:\n    print(f.read())",
    solution_example: "with open(\"demofile.txt\", \"w\") as f:\n    f.write(\"Hello\")\nwith open(\"demofile.txt\", \"a\") as f:\n    f.write(\" World\")\nwith open(\"demofile.txt\") as f:\n    print(f.read())\n",
    next: Some("py-74-file-delete"),
    show_type_chips: false,
    micro_step: 73,
};

pub const PY74_FILE_DELETE: CodingStep = CodingStep {
    id: "py-74-file-delete",
    title: "Python Delete Files",
    objective: "Borrar un archivo con os.remove.",
    prompt_md: "**Delete Files**\n\n`os.remove(path)` elimina un archivo.\n\n**Micro-reto:**\n1. `import os`\n2. Creá `demofile.txt` con cualquier contenido\n3. Borrálo con `os.remove(\"demofile.txt\")`\n4. Imprimí `os.path.exists(\"demofile.txt\")`",
    starter_code: "# import os\n# ...\n# print(...)\n",
    pytest: "def test_file_delete(capsys):\n    import os\n    exec(open('solution.py', encoding='utf-8').read())\n    assert not os.path.exists('demofile.txt')\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'False'\n",
    hint: "import os\nwith open(\"demofile.txt\", \"w\") as f:\n    f.write(\"bye\")\nos.remove(\"demofile.txt\")\nprint(os.path.exists(\"demofile.txt\"))",
    solution_example: "import os\nwith open(\"demofile.txt\", \"w\") as f:\n    f.write(\"bye\")\nos.remove(\"demofile.txt\")\nprint(os.path.exists(\"demofile.txt\"))\n",
    next: Some("py-75-user-input"),
    show_type_chips: false,
    micro_step: 74,
};

pub const PY75_USER_INPUT: CodingStep = CodingStep {
    id: "py-75-user-input",
    title: "Python User Input",
    objective: "Guardar un valor como si viniera de input() y usarlo.",
    prompt_md: "**User Input**\n\nEn la plataforma no hay teclado interactivo; simulamos `input()` asignando el valor.\n\n**Micro-reto:**\n1. Asigná `username = \"Alice\"` (como si viniera de `input()`)\n2. Imprimí `Hello, ` seguido de `username` (concatená o f-string)\n\nResultado esperado: `Hello, Alice`",
    starter_code: "# username = ...\n# print(...)\n",
    pytest: "def test_user_input(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('username') == 'Alice'\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'Hello, Alice'\n",
    hint: "username = \"Alice\"\nprint(\"Hello, \" + username)",
    solution_example: "username = \"Alice\"\nprint(\"Hello, \" + username)\n",
    next: Some("py-76-reverse-string"),
    show_type_chips: false,
    micro_step: 75,
};

pub const PY76_REVERSE_STRING: CodingStep = CodingStep {
    id: "py-76-reverse-string",
    title: "How To Reverse a String",
    objective: "Invertir un string con slicing [::-1].",
    prompt_md: "**How to Reverse a String**\n\nNo hay función built-in; usá slicing con paso `-1`.\n\n**Micro-reto:**\n1. Creá `txt = \"Hello World\"[::-1]`\n2. Imprimí `txt`",
    starter_code: "# txt = ...\n# print(txt)\n",
    pytest: "def test_reverse_string(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('txt') == 'dlroW olleH'\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'dlroW olleH'\n",
    hint: "txt = \"Hello World\"[::-1]\nprint(txt)",
    solution_example: "txt = \"Hello World\"[::-1]\nprint(txt)\n",
    next: Some("py-77-remove-duplicates"),
    show_type_chips: false,
    micro_step: 76,
};

pub const PY77_REMOVE_DUPLICATES: CodingStep = CodingStep {
    id: "py-77-remove-duplicates",
    title: "How To Remove List Duplicates",
    objective: "Eliminar duplicados convirtiendo a set y de vuelta a list.",
    prompt_md: "**Remove List Duplicates**\n\nUn `set` no permite duplicados.\n\n**Micro-reto:**\n1. Creá `mylist` con `a`, `b`, `a`, `c`, `c`\n2. Convertí a set y de nuevo a list en `mylist`\n3. Imprimí `sorted(mylist)`",
    starter_code: "# mylist = ...\n# mylist = list(set(mylist))\n# print(...)\n",
    pytest: "def test_remove_duplicates(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert sorted(ns.get('mylist', [])) == ['a', 'b', 'c']\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == \"['a', 'b', 'c']\"\n",
    hint: "mylist = [\"a\", \"b\", \"a\", \"c\", \"c\"]\nmylist = list(set(mylist))\nprint(sorted(mylist))",
    solution_example: "mylist = [\"a\", \"b\", \"a\", \"c\", \"c\"]\nmylist = list(set(mylist))\nprint(sorted(mylist))\n",
    next: Some("py-78-add-two-numbers"),
    show_type_chips: false,
    micro_step: 77,
};

pub const PY78_ADD_TWO_NUMBERS: CodingStep = CodingStep {
    id: "py-78-add-two-numbers",
    title: "How To Add Two Numbers",
    objective: "Sumar dos números e imprimir el resultado.",
    prompt_md: "**Add Two Numbers**\n\nSumá dos variables numéricas.\n\n**Micro-reto:**\n1. Creá `x = 5` y `y = 10`\n2. Guardá la suma en `z`\n3. Imprimí `z`",
    starter_code: "# x = ...\n# y = ...\n# z = ...\n# print(z)\n",
    pytest: "def test_add_two_numbers(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('z') == 15\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '15'\n",
    hint: "x = 5\ny = 10\nz = x + y\nprint(z)",
    solution_example: "x = 5\ny = 10\nz = x + y\nprint(z)\n",
    next: Some("py-79-self"),
    show_type_chips: false,
    micro_step: 78,
};

pub const PY79_SELF: CodingStep = CodingStep {
    id: "py-79-self",
    title: "Python self Parameter",
    objective: "Usar self para acceder a propiedades del object.",
    prompt_md: "**The self Parameter**\n\n`self` es una referencia a la instancia actual.\n\n**Micro-reto:**\n1. Creá la clase `Person` con `__init__(self, name)` que asigne `self.name = name`\n2. Agregá método `myfunc(self)` que imprima `Hello my name is` seguido de `self.name`\n3. Creá `p1 = Person(\"John\")` y llamá `p1.myfunc()`",
    starter_code: "# class Person:\n#     ...\n# p1 = ...\n# p1.myfunc()\n",
    pytest: "def test_self(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['p1'].name == 'John'\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'Hello my name is John'\n",
    hint: "class Person:\n    def __init__(self, name):\n        self.name = name\n    def myfunc(self):\n        print(\"Hello my name is \" + self.name)\np1 = Person(\"John\")\np1.myfunc()",
    solution_example: "class Person:\n    def __init__(self, name):\n        self.name = name\n    def myfunc(self):\n        print(\"Hello my name is \" + self.name)\np1 = Person(\"John\")\np1.myfunc()\n",
    next: Some("py-80-class-properties"),
    show_type_chips: false,
    micro_step: 79,
};

pub const PY80_CLASS_PROPERTIES: CodingStep = CodingStep {
    id: "py-80-class-properties",
    title: "Python Class Properties",
    objective: "Modificar una propiedad de un object.",
    prompt_md: "**Modify Object Properties**\n\nPodés cambiar propiedades después de crear el object.\n\n**Micro-reto:**\n1. Creá `Person` con `__init__(self, name, age)`\n2. Creá `p1 = Person(\"John\", 36)`\n3. Cambiá `p1.age = 40` e imprimí `p1.age`",
    starter_code: "# class Person:\n#     ...\n# p1 = ...\n# p1.age = ...\n# print(...)\n",
    pytest: "def test_class_properties(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['p1'].age == 40\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '40'\n",
    hint: "class Person:\n    def __init__(self, name, age):\n        self.name = name\n        self.age = age\np1 = Person(\"John\", 36)\np1.age = 40\nprint(p1.age)",
    solution_example: "class Person:\n    def __init__(self, name, age):\n        self.name = name\n        self.age = age\np1 = Person(\"John\", 36)\np1.age = 40\nprint(p1.age)\n",
    next: Some("py-81-class-methods"),
    show_type_chips: false,
    micro_step: 80,
};

pub const PY81_CLASS_METHODS: CodingStep = CodingStep {
    id: "py-81-class-methods",
    title: "Python Class Methods",
    objective: "Definir e invocar un método de instancia.",
    prompt_md: "**Object Methods**\n\nLos métodos son funciones que pertenecen al object.\n\n**Micro-reto:**\n1. Creá `Person` con `__init__(self, name)` y método `greet(self)` que imprima `Hi, I am` + `self.name`\n2. Creá `p1 = Person(\"Emil\")` y llamá `p1.greet()`",
    starter_code: "# class Person:\n#     ...\n# p1 = ...\n# p1.greet()\n",
    pytest: "def test_class_methods(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['p1'].name == 'Emil'\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'Hi, I am Emil'\n",
    hint: "class Person:\n    def __init__(self, name):\n        self.name = name\n    def greet(self):\n        print(\"Hi, I am \" + self.name)\np1 = Person(\"Emil\")\np1.greet()",
    solution_example: "class Person:\n    def __init__(self, name):\n        self.name = name\n    def greet(self):\n        print(\"Hi, I am \" + self.name)\np1 = Person(\"Emil\")\np1.greet()\n",
    next: Some("py-82-stack"),
    show_type_chips: false,
    micro_step: 81,
};

pub const PY82_STACK: CodingStep = CodingStep {
    id: "py-82-stack",
    title: "DSA Stack (list)",
    objective: "Usar una list como stack LIFO (append/pop).",
    prompt_md: "**Stacks**\n\nUna stack es LIFO: último en entrar, primero en salir.\n\n**Micro-reto:**\n1. Creá `stack = []`\n2. Hacé `append` de `A`, `B`, `C`\n3. Imprimí `stack.pop()`",
    starter_code: "# stack = []\n# ...\n# print(...)\n",
    pytest: "def test_stack(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('stack') == ['A', 'B']\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'C'\n",
    hint: "stack = []\nstack.append('A')\nstack.append('B')\nstack.append('C')\nprint(stack.pop())",
    solution_example: "stack = []\nstack.append('A')\nstack.append('B')\nstack.append('C')\nprint(stack.pop())\n",
    next: Some("py-83-stack-peek"),
    show_type_chips: false,
    micro_step: 82,
};

pub const PY83_STACK_PEEK: CodingStep = CodingStep {
    id: "py-83-stack-peek",
    title: "DSA Stack Peek",
    objective: "Mirar el tope de la stack sin hacer pop.",
    prompt_md: "**Stack Peek**\n\nPeek mira el último elemento sin removerlo (`stack[-1]`).\n\n**Micro-reto:**\n1. Creá `stack` con `A`, `B`, `C` vía append\n2. Guardá en `top` el peek `stack[-1]`\n3. Imprimí `top`",
    starter_code: "# stack = []\n# ...\n# top = ...\n# print(top)\n",
    pytest: "def test_stack_peek(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('stack') == ['A', 'B', 'C']\n    assert ns.get('top') == 'C'\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'C'\n",
    hint: "stack = []\nstack.append('A')\nstack.append('B')\nstack.append('C')\ntop = stack[-1]\nprint(top)",
    solution_example: "stack = []\nstack.append('A')\nstack.append('B')\nstack.append('C')\ntop = stack[-1]\nprint(top)\n",
    next: Some("py-84-queue"),
    show_type_chips: false,
    micro_step: 83,
};

pub const PY84_QUEUE: CodingStep = CodingStep {
    id: "py-84-queue",
    title: "DSA Queue (list)",
    objective: "Usar una list como queue FIFO (append/pop(0)).",
    prompt_md: "**Queues**\n\nUna queue es FIFO: primero en entrar, primero en salir.\n\n**Micro-reto:**\n1. Creá `queue = []`\n2. Encolá `A`, `B`, `C` con append\n3. Imprimí `queue.pop(0)`",
    starter_code: "# queue = []\n# ...\n# print(...)\n",
    pytest: "def test_queue(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('queue') == ['B', 'C']\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'A'\n",
    hint: "queue = []\nqueue.append('A')\nqueue.append('B')\nqueue.append('C')\nprint(queue.pop(0))",
    solution_example: "queue = []\nqueue.append('A')\nqueue.append('B')\nqueue.append('C')\nprint(queue.pop(0))\n",
    next: Some("py-85-queue-peek"),
    show_type_chips: false,
    micro_step: 84,
};

pub const PY85_QUEUE_PEEK: CodingStep = CodingStep {
    id: "py-85-queue-peek",
    title: "DSA Queue Peek",
    objective: "Mirar el frente de la queue sin dequeue.",
    prompt_md: "**Queue Peek**\n\nPeek mira el primer elemento (`queue[0]`).\n\n**Micro-reto:**\n1. Creá `queue` con `A`, `B`, `C`\n2. Guardá en `front` el peek `queue[0]`\n3. Imprimí `front`",
    starter_code: "# queue = []\n# ...\n# front = ...\n# print(front)\n",
    pytest: "def test_queue_peek(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('queue') == ['A', 'B', 'C']\n    assert ns.get('front') == 'A'\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'A'\n",
    hint: "queue = []\nqueue.append('A')\nqueue.append('B')\nqueue.append('C')\nfront = queue[0]\nprint(front)",
    solution_example: "queue = []\nqueue.append('A')\nqueue.append('B')\nqueue.append('C')\nfront = queue[0]\nprint(front)\n",
    next: Some("py-86-stack-class"),
    show_type_chips: false,
    micro_step: 85,
};

pub const PY86_STACK_CLASS: CodingStep = CodingStep {
    id: "py-86-stack-class",
    title: "DSA Stack Class",
    objective: "Implementar push/pop en una clase Stack.",
    prompt_md: "**Stack Class**\n\nEncapsulá la stack en una clase.\n\n**Micro-reto:**\n1. Creá `class Stack` con `__init__` que haga `self.stack = []`\n2. Métodos `push(self, element)` (append) y `pop(self)` (pop)\n3. Creá `myStack`, hacé push de `A` y `B`, imprimí `myStack.pop()`",
    starter_code: "# class Stack:\n#     ...\n# myStack = Stack()\n# ...\n# print(...)\n",
    pytest: "def test_stack_class(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['myStack'].stack == ['A']\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'B'\n",
    hint: "class Stack:\n    def __init__(self):\n        self.stack = []\n    def push(self, element):\n        self.stack.append(element)\n    def pop(self):\n        return self.stack.pop()\nmyStack = Stack()\nmyStack.push('A')\nmyStack.push('B')\nprint(myStack.pop())",
    solution_example: "class Stack:\n    def __init__(self):\n        self.stack = []\n    def push(self, element):\n        self.stack.append(element)\n    def pop(self):\n        return self.stack.pop()\nmyStack = Stack()\nmyStack.push('A')\nmyStack.push('B')\nprint(myStack.pop())\n",
    next: Some("py-87-queue-class"),
    show_type_chips: false,
    micro_step: 86,
};

pub const PY87_QUEUE_CLASS: CodingStep = CodingStep {
    id: "py-87-queue-class",
    title: "DSA Queue Class",
    objective: "Implementar enqueue/dequeue en una clase Queue.",
    prompt_md: "**Queue Class**\n\nEncapsulá la queue en una clase.\n\n**Micro-reto:**\n1. Creá `class Queue` con `self.queue = []`\n2. `enqueue` con append; `dequeue` con `pop(0)`\n3. Creá `myQueue`, encolá `A` y `B`, imprimí `myQueue.dequeue()`",
    starter_code: "# class Queue:\n#     ...\n# myQueue = Queue()\n# ...\n# print(...)\n",
    pytest: "def test_queue_class(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['myQueue'].queue == ['B']\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'A'\n",
    hint: "class Queue:\n    def __init__(self):\n        self.queue = []\n    def enqueue(self, element):\n        self.queue.append(element)\n    def dequeue(self):\n        return self.queue.pop(0)\nmyQueue = Queue()\nmyQueue.enqueue('A')\nmyQueue.enqueue('B')\nprint(myQueue.dequeue())",
    solution_example: "class Queue:\n    def __init__(self):\n        self.queue = []\n    def enqueue(self, element):\n        self.queue.append(element)\n    def dequeue(self):\n        return self.queue.pop(0)\nmyQueue = Queue()\nmyQueue.enqueue('A')\nmyQueue.enqueue('B')\nprint(myQueue.dequeue())\n",
    next: None,
    show_type_chips: false,
    micro_step: 87,
};

pub const CODING_STEPS: &[&CodingStep] = &[
    &PY02_VARIABLES,
    &PY02_INTRO,
    &PY03_GET_STARTED,
    &PY04_SYNTAX,
    &PY05_OUTPUT,
    &PY06_COMMENTS,
    &PY07_DATA_TYPES,
    &PY08_NUMBERS,
    &PY09_CASTING,
    &PY10_STRINGS,
    &PY11_SLICING,
    &PY12_MODIFY_STRINGS,
    &PY13_CONCATENATE,
    &PY14_FORMAT_STRINGS,
    &PY15_ESCAPE,
    &PY16_BOOLEANS,
    &PY17_OPERATORS,
    &PY18_LISTS,
    &PY19_LIST_ACCESS,
    &PY20_LIST_CHANGE,
    &PY21_LIST_ADD,
    &PY22_LIST_REMOVE,
    &PY23_LIST_LOOP,
    &PY24_LIST_COMPREHENSION,
    &PY25_LIST_SORT,
    &PY26_LIST_COPY,
    &PY27_LIST_JOIN,
    &PY28_TUPLES,
    &PY29_TUPLE_ACCESS,
    &PY30_TUPLE_UPDATE,
    &PY31_TUPLE_UNPACK,
    &PY32_TUPLE_LOOP,
    &PY33_TUPLE_JOIN,
    &PY34_SETS,
    &PY35_SET_ACCESS,
    &PY36_SET_ADD,
    &PY37_SET_REMOVE,
    &PY38_SET_LOOP,
    &PY39_SET_JOIN,
    &PY40_DICTIONARIES,
    &PY41_DICT_ACCESS,
    &PY42_DICT_CHANGE,
    &PY43_DICT_ADD,
    &PY44_DICT_REMOVE,
    &PY45_DICT_LOOP,
    &PY46_DICT_COPY,
    &PY47_DICT_NESTED,
    &PY48_IF,
    &PY49_ELIF,
    &PY50_WHILE,
    &PY51_FOR,
    &PY52_FUNCTIONS,
    &PY53_FUNCTION_ARGS,
    &PY54_FUNCTION_RETURN,
    &PY55_LAMBDA,
    &PY56_ARRAYS,
    &PY57_CLASSES,
    &PY58_INIT,
    &PY59_INHERITANCE,
    &PY60_ITERATORS,
    &PY61_POLYMORPHISM,
    &PY62_SCOPE,
    &PY63_MODULES,
    &PY64_DATES,
    &PY65_MATH,
    &PY66_JSON,
    &PY67_REGEX,
    &PY68_TRY_EXCEPT,
    &PY69_STRING_FORMATTING,
    &PY70_FILE_WRITE,
    &PY71_FILE_READ,
    &PY72_FILE_READLINE,
    &PY73_FILE_APPEND,
    &PY74_FILE_DELETE,
    &PY75_USER_INPUT,
    &PY76_REVERSE_STRING,
    &PY77_REMOVE_DUPLICATES,
    &PY78_ADD_TWO_NUMBERS,
    &PY79_SELF,
    &PY80_CLASS_PROPERTIES,
    &PY81_CLASS_METHODS,
    &PY82_STACK,
    &PY83_STACK_PEEK,
    &PY84_QUEUE,
    &PY85_QUEUE_PEEK,
    &PY86_STACK_CLASS,
    &PY87_QUEUE_CLASS,
];

pub const DEFAULT_CODING_STEP_ID: &str = "py-02-variables";

pub fn coding_step_by_id(id: &str) -> Option<&'static CodingStep> {
    CODING_STEPS.iter().copied().find(|s| s.id == id)
}

pub fn coding_step_by_micro_step(n: i32) -> Option<&'static CodingStep> {
    CODING_STEPS.iter().copied().find(|s| s.micro_step == n)
}

/// True when the rail cell is playable (completed or current cursor).
pub fn micro_step_unlocked(current_level: i32, n: i32) -> bool {
    n > 0 && n <= current_level
}

pub fn coding_step_or_default(id: &str) -> &'static CodingStep {
    coding_step_by_id(id).unwrap_or(&PY02_VARIABLES)
}

/// First coding bridge after onboarding (seed step `py-02-variables`).
pub fn first_coding_step() -> &'static CodingStep {
    &PY02_VARIABLES
}

/// Extremely light markdown → HTML for the enunciado panel (bold + newlines).
pub fn prompt_to_html(md: &str) -> String {
    prompt_to_html_with_flash(md, None)
}

/// Same as [`prompt_to_html`], marking `nombre`/`edad` idents and flashing one for 5s UX.
pub fn prompt_to_html_with_flash(md: &str, flash_ident: Option<&str>) -> String {
    let mut out = String::new();
    for line in md.lines() {
        let mut escaped = html_escape(line);
        while let Some(start) = escaped.find("**") {
            if let Some(rel_end) = escaped[start + 2..].find("**") {
                let end = start + 2 + rel_end;
                let inner = escaped[start + 2..end].to_string();
                let replacement = format!("<strong>{inner}</strong>");
                escaped.replace_range(start..end + 2, &replacement);
            } else {
                break;
            }
        }
        while let Some(start) = escaped.find('`') {
            if let Some(rel_end) = escaped[start + 1..].find('`') {
                let end = start + 1 + rel_end;
                let inner = escaped[start + 1..end].to_string();
                let replacement = code_span_html(&inner, flash_ident);
                escaped.replace_range(start..end + 1, &replacement);
            } else {
                break;
            }
        }
        out.push_str("<p>");
        out.push_str(&escaped);
        out.push_str("</p>");
    }
    out
}

fn code_span_html(inner: &str, flash_ident: Option<&str>) -> String {
    if inner == "nombre" || inner == "edad" {
        // Primary occurrence (lines 1–2): keep stable DOM ids for e2e / a11y.
        return format!(
            r#"<code class="{}" id="learn-ident-{inner}" data-ident="{inner}">{inner}</code>"#,
            ident_class(flash_ident == Some(inner))
        );
    }
    // Compound snippets (e.g. print(nombre, edad)): flash every ident occurrence.
    format!(
        "<code>{}</code>",
        wrap_explore_idents(inner, flash_ident)
    )
}

fn ident_class(flashing: bool) -> &'static str {
    if flashing {
        "learn__ident learn__ident--flash"
    } else {
        "learn__ident"
    }
}

fn wrap_explore_idents(text: &str, flash_ident: Option<&str>) -> String {
    const NEEDLES: &[&str] = &["nombre", "edad"];
    let bytes = text.as_bytes();
    let mut out = String::new();
    let mut i = 0;
    while i < text.len() {
        let mut hit: Option<&str> = None;
        for needle in NEEDLES {
            if text[i..].starts_with(needle) {
                let end = i + needle.len();
                let before_ok = i == 0 || !is_ident_char(bytes[i - 1]);
                let after_ok = end >= bytes.len() || !is_ident_char(bytes[end]);
                if before_ok && after_ok {
                    hit = Some(needle);
                    break;
                }
            }
        }
        if let Some(ident) = hit {
            out.push_str(&format!(
                r#"<span class="{}" data-ident="{ident}">{ident}</span>"#,
                ident_class(flash_ident == Some(ident))
            ));
            i += ident.len();
        } else {
            let ch = text[i..].chars().next().expect("char at i");
            out.push(ch);
            i += ch.len_utf8();
        }
    }
    out
}

fn is_ident_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_step_matches_seed_id() {
        assert_eq!(first_coding_step().id, "py-02-variables");
        assert!(first_coding_step().pytest.contains("test_variables"));
        assert!(first_coding_step().starter_code.contains("nombre"));
        assert_eq!(first_coding_step().next, Some("py-02-intro"));
        assert_eq!(first_coding_step().micro_step, 1);
    }

    #[test]
    fn coding_steps_have_unique_micro_steps() {
        let mut seen = std::collections::BTreeSet::new();
        for step in CODING_STEPS {
            assert!(step.micro_step >= 1 && step.micro_step <= 100);
            assert!(
                seen.insert(step.micro_step),
                "duplicate micro_step {}",
                step.micro_step
            );
        }
    }

    #[test]
    fn py05_output_chained_from_syntax() {
        let syntax = coding_step_by_id("py-04-syntax").expect("syntax");
        assert_eq!(syntax.next, Some("py-05-output"));
        let out = coding_step_by_micro_step(5).expect("py-05");
        assert_eq!(out.id, "py-05-output");
        assert_eq!(out.micro_step, 5);
        assert_eq!(out.next, Some("py-06-comments"));
        assert!(out.pytest.contains("test_output_mix"));
    }

    #[test]
    fn py06_comments_chained_from_output() {
        let comments = coding_step_by_micro_step(6).expect("py-06");
        assert_eq!(comments.id, "py-06-comments");
        assert_eq!(comments.next, Some("py-07-data-types"));
        assert!(comments.pytest.contains("test_comments"));
        assert!(comments.starter_code.contains("This should not run"));
    }

    #[test]
    fn py07_data_types_chained_from_comments() {
        let step = coding_step_by_micro_step(7).expect("py-07");
        assert_eq!(step.id, "py-07-data-types");
        assert_eq!(step.next, Some("py-08-numbers"));
        assert!(step.pytest.contains("test_data_types"));
    }

    #[test]
    fn py08_numbers_chained_from_data_types() {
        let step = coding_step_by_micro_step(8).expect("py-08");
        assert_eq!(step.id, "py-08-numbers");
        assert_eq!(step.next, Some("py-09-casting"));
        assert!(step.pytest.contains("test_numbers"));
    }

    #[test]
    fn py09_casting_chained_from_numbers() {
        let step = coding_step_by_micro_step(9).expect("py-09");
        assert_eq!(step.id, "py-09-casting");
        assert_eq!(step.next, Some("py-10-strings"));
        assert!(step.pytest.contains("test_casting"));
    }

    #[test]
    fn py10_to_py15_strings_family_chain() {
        let ids = [
            (10, "py-10-strings", Some("py-11-slicing")),
            (11, "py-11-slicing", Some("py-12-modify-strings")),
            (12, "py-12-modify-strings", Some("py-13-concatenate")),
            (13, "py-13-concatenate", Some("py-14-format-strings")),
            (14, "py-14-format-strings", Some("py-15-escape")),
            (15, "py-15-escape", Some("py-16-booleans")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("strings family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py16_to_py21_bool_ops_lists_chain() {
        let ids = [
            (16, "py-16-booleans", Some("py-17-operators")),
            (17, "py-17-operators", Some("py-18-lists")),
            (18, "py-18-lists", Some("py-19-list-access")),
            (19, "py-19-list-access", Some("py-20-list-change")),
            (20, "py-20-list-change", Some("py-21-list-add")),
            (21, "py-21-list-add", Some("py-22-list-remove")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("bool/ops/lists step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py22_to_py27_list_ops_chain() {
        let ids = [
            (22, "py-22-list-remove", Some("py-23-list-loop")),
            (23, "py-23-list-loop", Some("py-24-list-comprehension")),
            (24, "py-24-list-comprehension", Some("py-25-list-sort")),
            (25, "py-25-list-sort", Some("py-26-list-copy")),
            (26, "py-26-list-copy", Some("py-27-list-join")),
            (27, "py-27-list-join", Some("py-28-tuples")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("list ops step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py28_to_py33_tuples_chain() {
        let ids = [
            (28, "py-28-tuples", Some("py-29-tuple-access")),
            (29, "py-29-tuple-access", Some("py-30-tuple-update")),
            (30, "py-30-tuple-update", Some("py-31-tuple-unpack")),
            (31, "py-31-tuple-unpack", Some("py-32-tuple-loop")),
            (32, "py-32-tuple-loop", Some("py-33-tuple-join")),
            (33, "py-33-tuple-join", Some("py-34-sets")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("tuples step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py34_to_py39_sets_chain() {
        let ids = [
            (34, "py-34-sets", Some("py-35-set-access")),
            (35, "py-35-set-access", Some("py-36-set-add")),
            (36, "py-36-set-add", Some("py-37-set-remove")),
            (37, "py-37-set-remove", Some("py-38-set-loop")),
            (38, "py-38-set-loop", Some("py-39-set-join")),
            (39, "py-39-set-join", Some("py-40-dictionaries")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("sets step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py40_to_py45_dicts_chain() {
        let ids = [
            (40, "py-40-dictionaries", Some("py-41-dict-access")),
            (41, "py-41-dict-access", Some("py-42-dict-change")),
            (42, "py-42-dict-change", Some("py-43-dict-add")),
            (43, "py-43-dict-add", Some("py-44-dict-remove")),
            (44, "py-44-dict-remove", Some("py-45-dict-loop")),
            (45, "py-45-dict-loop", Some("py-46-dict-copy")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("dicts step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py46_to_py51_control_flow_chain() {
        let ids = [
            (46, "py-46-dict-copy", Some("py-47-dict-nested")),
            (47, "py-47-dict-nested", Some("py-48-if")),
            (48, "py-48-if", Some("py-49-elif")),
            (49, "py-49-elif", Some("py-50-while")),
            (50, "py-50-while", Some("py-51-for")),
            (51, "py-51-for", Some("py-52-functions")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("control flow step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py52_to_py57_functions_classes_chain() {
        let ids = [
            (52, "py-52-functions", Some("py-53-function-args")),
            (53, "py-53-function-args", Some("py-54-function-return")),
            (54, "py-54-function-return", Some("py-55-lambda")),
            (55, "py-55-lambda", Some("py-56-arrays")),
            (56, "py-56-arrays", Some("py-57-classes")),
            (57, "py-57-classes", Some("py-58-init")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("functions/classes step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py58_to_py63_oop_chain() {
        let ids = [
            (58, "py-58-init", Some("py-59-inheritance")),
            (59, "py-59-inheritance", Some("py-60-iterators")),
            (60, "py-60-iterators", Some("py-61-polymorphism")),
            (61, "py-61-polymorphism", Some("py-62-scope")),
            (62, "py-62-scope", Some("py-63-modules")),
            (63, "py-63-modules", Some("py-64-dates")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("oop family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py64_to_py69_stdlib_chain() {
        let ids = [
            (64, "py-64-dates", Some("py-65-math")),
            (65, "py-65-math", Some("py-66-json")),
            (66, "py-66-json", Some("py-67-regex")),
            (67, "py-67-regex", Some("py-68-try-except")),
            (68, "py-68-try-except", Some("py-69-string-formatting")),
            (69, "py-69-string-formatting", Some("py-70-file-write")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("stdlib family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py70_to_py75_files_chain() {
        let ids = [
            (70, "py-70-file-write", Some("py-71-file-read")),
            (71, "py-71-file-read", Some("py-72-file-readline")),
            (72, "py-72-file-readline", Some("py-73-file-append")),
            (73, "py-73-file-append", Some("py-74-file-delete")),
            (74, "py-74-file-delete", Some("py-75-user-input")),
            (75, "py-75-user-input", Some("py-76-reverse-string")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("files family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py76_to_py81_howto_oop_chain() {
        let ids = [
            (76, "py-76-reverse-string", Some("py-77-remove-duplicates")),
            (77, "py-77-remove-duplicates", Some("py-78-add-two-numbers")),
            (78, "py-78-add-two-numbers", Some("py-79-self")),
            (79, "py-79-self", Some("py-80-class-properties")),
            (80, "py-80-class-properties", Some("py-81-class-methods")),
            (81, "py-81-class-methods", Some("py-82-stack")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("howto/oop family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py82_to_py87_dsa_chain() {
        let ids = [
            (82, "py-82-stack", Some("py-83-stack-peek")),
            (83, "py-83-stack-peek", Some("py-84-queue")),
            (84, "py-84-queue", Some("py-85-queue-peek")),
            (85, "py-85-queue-peek", Some("py-86-stack-class")),
            (86, "py-86-stack-class", Some("py-87-queue-class")),
            (87, "py-87-queue-class", None),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("dsa family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn micro_step_unlocked_uses_cursor() {
        assert!(!micro_step_unlocked(1, 2));
        assert!(micro_step_unlocked(5, 5));
        assert!(micro_step_unlocked(5, 1));
        assert!(!micro_step_unlocked(5, 6));
    }

    #[test]
    fn chain_reaches_intro() {
        let intro = coding_step_by_id("py-02-intro").expect("intro");
        assert_eq!(intro.next, Some("py-03-get-started"));
        assert!(!intro.show_type_chips);
    }

    #[test]
    fn prompt_html_bold_and_code() {
        let html = prompt_to_html("**Micro-reto:**\nUsa `print`");
        assert!(html.contains("<strong>Micro-reto:</strong>"));
        assert!(html.contains("<code>print</code>"));
        assert!(!html.contains("<script"));
    }

    #[test]
    fn prompt_body_no_longer_duplicates_variables_heading() {
        let html = prompt_to_html(first_coding_step().prompt_md);
        assert!(html.contains("Una variable guarda un valor"));
        assert!(!html.contains("<strong>Variables</strong>"));
        assert!(html.contains(r#"id="learn-ident-nombre""#));
        assert!(html.contains(r#"id="learn-ident-edad""#));
        // Line 3 `print(nombre, edad)` also marks both idents (no duplicate ids).
        assert_eq!(html.matches(r#"data-ident="nombre""#).count(), 2);
        assert_eq!(html.matches(r#"data-ident="edad""#).count(), 2);
        assert!(html.contains("print("));
    }

    #[test]
    fn prompt_flash_marks_nombre_on_all_occurrences() {
        let html = prompt_to_html_with_flash(first_coding_step().prompt_md, Some("nombre"));
        assert_eq!(
            html.matches(r#"class="learn__ident learn__ident--flash" data-ident="nombre""#)
                .count()
                + html
                    .matches(
                        r#"class="learn__ident learn__ident--flash" id="learn-ident-nombre""#
                    )
                    .count(),
            2
        );
        assert!(html.contains(r#"class="learn__ident" id="learn-ident-edad""#));
        assert!(html.contains(r#"class="learn__ident" data-ident="edad""#));
    }
}
