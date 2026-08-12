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
    /// 1-based index on the workspace micro-step rail (1..=300).
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
    next: Some("py-88-linear-in"),
    show_type_chips: false,
    micro_step: 87,
};

pub const PY88_LINEAR_IN: CodingStep = CodingStep {
    id: "py-88-linear-in",
    title: "DSA Linear Search (in)",
    objective: "Comprobar pertenencia con el operador in.",
    prompt_md: "**Linear Search**\n\nLa forma rápida de chequear si un valor existe es el operador `in`.\n\n**Micro-reto:**\n1. Creá `mylist` con `3, 7, 2, 9, 5, 1, 8, 4, 6`\n2. Si `4` está en `mylist`, imprimí `Found!`; si no, `Not found!`",
    starter_code: "# mylist = ...\n# if ...:\n#     ...\n",
    pytest: "def test_linear_in(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('mylist') == [3, 7, 2, 9, 5, 1, 8, 4, 6]\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == 'Found!'\n",
    hint: "mylist = [3, 7, 2, 9, 5, 1, 8, 4, 6]\nif 4 in mylist:\n    print(\"Found!\")\nelse:\n    print(\"Not found!\")",
    solution_example: "mylist = [3, 7, 2, 9, 5, 1, 8, 4, 6]\nif 4 in mylist:\n    print(\"Found!\")\nelse:\n    print(\"Not found!\")\n",
    next: Some("py-89-linear-search"),
    show_type_chips: false,
    micro_step: 88,
};

pub const PY89_LINEAR_SEARCH: CodingStep = CodingStep {
    id: "py-89-linear-search",
    title: "DSA Linear Search Index",
    objective: "Devolver el índice del target con linear search.",
    prompt_md: "**Linear Search (index)**\n\nRecorré el array y devolvé el índice del valor buscado.\n\n**Micro-reto:**\n1. Definí `linearSearch(arr, targetVal)` que recorra `arr` y devuelva el índice o `-1`\n2. Con `mylist = [3, 7, 2, 9, 5, 1, 8, 4, 6]` y `x = 4`\n3. Imprimí `linearSearch(mylist, x)`",
    starter_code: "# def linearSearch(arr, targetVal):\n#     ...\n# mylist = ...\n# print(...)\n",
    pytest: "def test_linear_search(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['linearSearch']([3, 7, 2, 9, 5, 1, 8, 4, 6], 4) == 7\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '7'\n",
    hint: "def linearSearch(arr, targetVal):\n    for i in range(len(arr)):\n        if arr[i] == targetVal:\n            return i\n    return -1\nmylist = [3, 7, 2, 9, 5, 1, 8, 4, 6]\nx = 4\nprint(linearSearch(mylist, x))",
    solution_example: "def linearSearch(arr, targetVal):\n    for i in range(len(arr)):\n        if arr[i] == targetVal:\n            return i\n    return -1\nmylist = [3, 7, 2, 9, 5, 1, 8, 4, 6]\nx = 4\nprint(linearSearch(mylist, x))\n",
    next: Some("py-90-bubble-sort"),
    show_type_chips: false,
    micro_step: 89,
};

pub const PY90_BUBBLE_SORT: CodingStep = CodingStep {
    id: "py-90-bubble-sort",
    title: "DSA Bubble Sort",
    objective: "Ordenar una list con Bubble Sort.",
    prompt_md: "**Bubble Sort**\n\nCompará pares vecinos e intercambiá si están desordenados.\n\n**Micro-reto:**\n1. Creá `mylist = [64, 34, 25, 12, 22, 11, 90, 5]`\n2. Implementá Bubble Sort (doble loop con swap)\n3. Imprimí `mylist`",
    starter_code: "# mylist = ...\n# n = len(mylist)\n# for i in range(n-1):\n#     ...\n# print(mylist)\n",
    pytest: "def test_bubble_sort(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('mylist') == [5, 11, 12, 22, 25, 34, 64, 90]\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '[5, 11, 12, 22, 25, 34, 64, 90]'\n",
    hint: "mylist = [64, 34, 25, 12, 22, 11, 90, 5]\nn = len(mylist)\nfor i in range(n-1):\n    for j in range(n-i-1):\n        if mylist[j] > mylist[j+1]:\n            mylist[j], mylist[j+1] = mylist[j+1], mylist[j]\nprint(mylist)",
    solution_example: "mylist = [64, 34, 25, 12, 22, 11, 90, 5]\nn = len(mylist)\nfor i in range(n-1):\n    for j in range(n-i-1):\n        if mylist[j] > mylist[j+1]:\n            mylist[j], mylist[j+1] = mylist[j+1], mylist[j]\nprint(mylist)\n",
    next: Some("py-91-binary-search"),
    show_type_chips: false,
    micro_step: 90,
};

pub const PY91_BINARY_SEARCH: CodingStep = CodingStep {
    id: "py-91-binary-search",
    title: "DSA Binary Search",
    objective: "Buscar en un array ordenado con binary search.",
    prompt_md: "**Binary Search**\n\nEn un array ordenado, mirá el medio y descartá mitad.\n\n**Micro-reto:**\n1. Definí `binarySearch(arr, targetVal)` que devuelva el índice o `-1`\n2. Usá `mylist = [1, 3, 5, 7, 9, 11, 13, 15]` y `x = 11`\n3. Imprimí `binarySearch(mylist, x)`",
    starter_code: "# def binarySearch(arr, targetVal):\n#     ...\n# mylist = ...\n# print(...)\n",
    pytest: "def test_binary_search(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['binarySearch']([1, 3, 5, 7, 9, 11, 13, 15], 11) == 5\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '5'\n",
    hint: "def binarySearch(arr, targetVal):\n    left = 0\n    right = len(arr) - 1\n    while left <= right:\n        mid = (left + right) // 2\n        if arr[mid] == targetVal:\n            return mid\n        if arr[mid] < targetVal:\n            left = mid + 1\n        else:\n            right = mid - 1\n    return -1\nmylist = [1, 3, 5, 7, 9, 11, 13, 15]\nx = 11\nprint(binarySearch(mylist, x))",
    solution_example: "def binarySearch(arr, targetVal):\n    left = 0\n    right = len(arr) - 1\n    while left <= right:\n        mid = (left + right) // 2\n        if arr[mid] == targetVal:\n            return mid\n        if arr[mid] < targetVal:\n            left = mid + 1\n        else:\n            right = mid - 1\n    return -1\nmylist = [1, 3, 5, 7, 9, 11, 13, 15]\nx = 11\nprint(binarySearch(mylist, x))\n",
    next: Some("py-92-selection-sort"),
    show_type_chips: false,
    micro_step: 91,
};

pub const PY92_SELECTION_SORT: CodingStep = CodingStep {
    id: "py-92-selection-sort",
    title: "DSA Selection Sort",
    objective: "Ordenar eligiendo el mínimo en cada pasada.",
    prompt_md: "**Selection Sort**\n\nEn cada pasada, ubicá el mínimo del resto y swapéalo al frente.\n\n**Micro-reto:**\n1. Creá `mylist = [64, 34, 25, 12, 22, 11, 90, 5]`\n2. Implementá Selection Sort\n3. Imprimí `mylist`",
    starter_code: "# mylist = ...\n# n = len(mylist)\n# for i in range(n):\n#     ...\n# print(mylist)\n",
    pytest: "def test_selection_sort(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('mylist') == [5, 11, 12, 22, 25, 34, 64, 90]\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '[5, 11, 12, 22, 25, 34, 64, 90]'\n",
    hint: "mylist = [64, 34, 25, 12, 22, 11, 90, 5]\nn = len(mylist)\nfor i in range(n):\n    min_idx = i\n    for j in range(i+1, n):\n        if mylist[j] < mylist[min_idx]:\n            min_idx = j\n    mylist[i], mylist[min_idx] = mylist[min_idx], mylist[i]\nprint(mylist)",
    solution_example: "mylist = [64, 34, 25, 12, 22, 11, 90, 5]\nn = len(mylist)\nfor i in range(n):\n    min_idx = i\n    for j in range(i+1, n):\n        if mylist[j] < mylist[min_idx]:\n            min_idx = j\n    mylist[i], mylist[min_idx] = mylist[min_idx], mylist[i]\nprint(mylist)\n",
    next: Some("py-93-insertion-sort"),
    show_type_chips: false,
    micro_step: 92,
};

pub const PY93_INSERTION_SORT: CodingStep = CodingStep {
    id: "py-93-insertion-sort",
    title: "DSA Insertion Sort",
    objective: "Ordenar insertando cada elemento en su lugar.",
    prompt_md: "**Insertion Sort**\n\nInsertá cada valor en la posición correcta del prefijo ordenado.\n\n**Micro-reto:**\n1. Creá `mylist = [64, 34, 25, 12, 22, 11, 90, 5]`\n2. Implementá Insertion Sort\n3. Imprimí `mylist`",
    starter_code: "# mylist = ...\n# n = len(mylist)\n# for i in range(1, n):\n#     ...\n# print(mylist)\n",
    pytest: "def test_insertion_sort(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('mylist') == [5, 11, 12, 22, 25, 34, 64, 90]\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '[5, 11, 12, 22, 25, 34, 64, 90]'\n",
    hint: "mylist = [64, 34, 25, 12, 22, 11, 90, 5]\nn = len(mylist)\nfor i in range(1, n):\n    key = mylist[i]\n    j = i - 1\n    while j >= 0 and mylist[j] > key:\n        mylist[j + 1] = mylist[j]\n        j -= 1\n    mylist[j + 1] = key\nprint(mylist)",
    solution_example: "mylist = [64, 34, 25, 12, 22, 11, 90, 5]\nn = len(mylist)\nfor i in range(1, n):\n    key = mylist[i]\n    j = i - 1\n    while j >= 0 and mylist[j] > key:\n        mylist[j + 1] = mylist[j]\n        j -= 1\n    mylist[j + 1] = key\nprint(mylist)\n",
    next: Some("py-94-linked-node"),
    show_type_chips: false,
    micro_step: 93,
};

pub const PY94_LINKED_NODE: CodingStep = CodingStep {
    id: "py-94-linked-node",
    title: "DSA Linked List Node",
    objective: "Crear nodos y enlazarlos con next.",
    prompt_md: "**Linked Lists**\n\nCada Node guarda `data` y un puntero `next`.\n\n**Micro-reto:**\n1. Creá `class Node` con `__init__(self, data)` que asigne `self.data` y `self.next = None`\n2. Creá `node1 = Node(7)` y `node2 = Node(11)`\n3. Enlazá `node1.next = node2` e imprimí `node1.next.data`",
    starter_code: "# class Node:\n#     ...\n# node1 = ...\n# node2 = ...\n# ...\n# print(...)\n",
    pytest: "def test_linked_node(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['node1'].data == 7 and ns['node2'].data == 11\n    assert ns['node1'].next is ns['node2']\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '11'\n",
    hint: "class Node:\n    def __init__(self, data):\n        self.data = data\n        self.next = None\nnode1 = Node(7)\nnode2 = Node(11)\nnode1.next = node2\nprint(node1.next.data)",
    solution_example: "class Node:\n    def __init__(self, data):\n        self.data = data\n        self.next = None\nnode1 = Node(7)\nnode2 = Node(11)\nnode1.next = node2\nprint(node1.next.data)\n",
    next: Some("py-95-linked-traverse"),
    show_type_chips: false,
    micro_step: 94,
};

pub const PY95_LINKED_TRAVERSE: CodingStep = CodingStep {
    id: "py-95-linked-traverse",
    title: "DSA Linked List Traverse",
    objective: "Recorrer una linked list e imprimir los valores.",
    prompt_md: "**Traversal**\n\nSeguí `next` desde el head hasta `None`.\n\n**Micro-reto:**\n1. Creá `Node` como antes\n2. Definí `traverse(head)` que imprima cada `data` separado por espacio\n3. Enlazá `7 -> 11 -> 3` y llamá `traverse(node1)`",
    starter_code: "# class Node:\n#     ...\n# def traverse(head):\n#     ...\n# ...\n# traverse(node1)\n",
    pytest: "def test_linked_traverse(capsys):\n    exec(open('solution.py', encoding='utf-8').read())\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '7 11 3'\n",
    hint: "class Node:\n    def __init__(self, data):\n        self.data = data\n        self.next = None\ndef traverse(head):\n    current = head\n    while current:\n        print(current.data, end=\" \")\n        current = current.next\n    print()\nnode1 = Node(7)\nnode2 = Node(11)\nnode3 = Node(3)\nnode1.next = node2\nnode2.next = node3\ntraverse(node1)",
    solution_example: "class Node:\n    def __init__(self, data):\n        self.data = data\n        self.next = None\ndef traverse(head):\n    current = head\n    while current:\n        print(current.data, end=\" \")\n        current = current.next\n    print()\nnode1 = Node(7)\nnode2 = Node(11)\nnode3 = Node(3)\nnode1.next = node2\nnode2.next = node3\ntraverse(node1)\n",
    next: Some("py-96-linked-lowest"),
    show_type_chips: false,
    micro_step: 95,
};

pub const PY96_LINKED_LOWEST: CodingStep = CodingStep {
    id: "py-96-linked-lowest",
    title: "DSA Linked List Lowest",
    objective: "Encontrar el valor mínimo en una linked list.",
    prompt_md: "**Find Lowest Value**\n\nRecorré la lista y guardá el mínimo.\n\n**Micro-reto:**\n1. Creá `Node` y la cadena `7 -> 11 -> 3 -> 2 -> 9`\n2. Definí `findLowestValue(head)` que devuelva el mínimo\n3. Imprimí `findLowestValue(node1)`",
    starter_code: "# class Node:\n#     ...\n# def findLowestValue(head):\n#     ...\n# ...\n# print(...)\n",
    pytest: "def test_linked_lowest(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['findLowestValue'](ns['node1']) == 2\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '2'\n",
    hint: "class Node:\n    def __init__(self, data):\n        self.data = data\n        self.next = None\ndef findLowestValue(head):\n    minValue = head.data\n    current = head.next\n    while current:\n        if current.data < minValue:\n            minValue = current.data\n        current = current.next\n    return minValue\nnode1 = Node(7)\nnode2 = Node(11)\nnode3 = Node(3)\nnode4 = Node(2)\nnode5 = Node(9)\nnode1.next = node2\nnode2.next = node3\nnode3.next = node4\nnode4.next = node5\nprint(findLowestValue(node1))",
    solution_example: "class Node:\n    def __init__(self, data):\n        self.data = data\n        self.next = None\ndef findLowestValue(head):\n    minValue = head.data\n    current = head.next\n    while current:\n        if current.data < minValue:\n            minValue = current.data\n        current = current.next\n    return minValue\nnode1 = Node(7)\nnode2 = Node(11)\nnode3 = Node(3)\nnode4 = Node(2)\nnode5 = Node(9)\nnode1.next = node2\nnode2.next = node3\nnode3.next = node4\nnode4.next = node5\nprint(findLowestValue(node1))\n",
    next: Some("py-97-recursion"),
    show_type_chips: false,
    micro_step: 96,
};

pub const PY97_RECURSION: CodingStep = CodingStep {
    id: "py-97-recursion",
    title: "Python Recursion (factorial)",
    objective: "Calcular factorial con una función recursiva.",
    prompt_md: "**Recursion**\n\nUna función recursiva se llama a sí misma.\n\n**Micro-reto:**\n1. Definí `factorial(n)` que devuelva `1` si `n == 1`, si no `n * factorial(n - 1)`\n2. Imprimí `factorial(5)`",
    starter_code: "# def factorial(n):\n#     ...\n# print(...)\n",
    pytest: "def test_recursion(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['factorial'](5) == 120\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '120'\n",
    hint: "def factorial(n):\n    if n == 1:\n        return 1\n    else:\n        return n * factorial(n - 1)\nprint(factorial(5))",
    solution_example: "def factorial(n):\n    if n == 1:\n        return 1\n    else:\n        return n * factorial(n - 1)\nprint(factorial(5))\n",
    next: Some("py-98-fibonacci"),
    show_type_chips: false,
    micro_step: 97,
};

pub const PY98_FIBONACCI: CodingStep = CodingStep {
    id: "py-98-fibonacci",
    title: "Python Recursion (Fibonacci)",
    objective: "Calcular Fibonacci con recursion.",
    prompt_md: "**Fibonacci**\n\n`fib(n) = fib(n-1) + fib(n-2)` con base `0` y `1`.\n\n**Micro-reto:**\n1. Definí `fib(n)` recursivo (`n == 0` → `0`, `n == 1` → `1`)\n2. Imprimí `fib(7)`",
    starter_code: "# def fib(n):\n#     ...\n# print(...)\n",
    pytest: "def test_fibonacci(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['fib'](7) == 13\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '13'\n",
    hint: "def fib(n):\n    if n == 0:\n        return 0\n    if n == 1:\n        return 1\n    return fib(n - 1) + fib(n - 2)\nprint(fib(7))",
    solution_example: "def fib(n):\n    if n == 0:\n        return 0\n    if n == 1:\n        return 1\n    return fib(n - 1) + fib(n - 2)\nprint(fib(7))\n",
    next: Some("py-99-quicksort"),
    show_type_chips: false,
    micro_step: 98,
};

pub const PY99_QUICKSORT: CodingStep = CodingStep {
    id: "py-99-quicksort",
    title: "DSA Quicksort",
    objective: "Ordenar una list con Quicksort recursivo.",
    prompt_md: "**Quicksort**\n\nParticioná alrededor de un pivot y ordená recursivamente.\n\n**Micro-reto:**\n1. Implementá `partition` y `quicksort` como en W3S (pivot = último)\n2. Ordená `mylist = [64, 34, 25, 5, 22, 11, 90, 12]`\n3. Imprimí `mylist`",
    starter_code: "# def partition(array, low, high):\n#     ...\n# def quicksort(array, low=0, high=None):\n#     ...\n# mylist = ...\n# quicksort(mylist)\n# print(mylist)\n",
    pytest: "def test_quicksort(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('mylist') == [5, 11, 12, 22, 25, 34, 64, 90]\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '[5, 11, 12, 22, 25, 34, 64, 90]'\n",
    hint: "def partition(array, low, high):\n    pivot = array[high]\n    i = low - 1\n    for j in range(low, high):\n        if array[j] <= pivot:\n            i += 1\n            array[i], array[j] = array[j], array[i]\n    array[i+1], array[high] = array[high], array[i+1]\n    return i+1\ndef quicksort(array, low=0, high=None):\n    if high is None:\n        high = len(array) - 1\n    if low < high:\n        pivot_index = partition(array, low, high)\n        quicksort(array, low, pivot_index-1)\n        quicksort(array, pivot_index+1, high)\nmylist = [64, 34, 25, 5, 22, 11, 90, 12]\nquicksort(mylist)\nprint(mylist)",
    solution_example: "def partition(array, low, high):\n    pivot = array[high]\n    i = low - 1\n    for j in range(low, high):\n        if array[j] <= pivot:\n            i += 1\n            array[i], array[j] = array[j], array[i]\n    array[i+1], array[high] = array[high], array[i+1]\n    return i+1\ndef quicksort(array, low=0, high=None):\n    if high is None:\n        high = len(array) - 1\n    if low < high:\n        pivot_index = partition(array, low, high)\n        quicksort(array, low, pivot_index-1)\n        quicksort(array, pivot_index+1, high)\nmylist = [64, 34, 25, 5, 22, 11, 90, 12]\nquicksort(mylist)\nprint(mylist)\n",
    next: Some("py-100-hash-count"),
    show_type_chips: false,
    micro_step: 99,
};

pub const PY100_HASH_COUNT: CodingStep = CodingStep {
    id: "py-100-hash-count",
    title: "DSA Hash Tables (count)",
    objective: "Contar frecuencias con un dict (hash table).",
    prompt_md: "**Hash Tables**\n\nEn Python, un `dict` es la hash table práctica para contar.\n\n**Micro-reto:**\n1. Creá `mylist = [\"apple\", \"banana\", \"apple\", \"cherry\", \"banana\", \"apple\"]`\n2. Contá apariciones en `counts` (dict)\n3. Imprimí `counts[\"apple\"]`",
    starter_code: "# mylist = ...\n# counts = {}\n# for x in mylist:\n#     ...\n# print(...)\n",
    pytest: "def test_hash_count(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns.get('counts', {}).get('apple') == 3\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '3'\n",
    hint: "mylist = [\"apple\", \"banana\", \"apple\", \"cherry\", \"banana\", \"apple\"]\ncounts = {}\nfor x in mylist:\n    counts[x] = counts.get(x, 0) + 1\nprint(counts[\"apple\"])",
    solution_example: "mylist = [\"apple\", \"banana\", \"apple\", \"cherry\", \"banana\", \"apple\"]\ncounts = {}\nfor x in mylist:\n    counts[x] = counts.get(x, 0) + 1\nprint(counts[\"apple\"])\n",
    next: Some("py-101-linked-delete"),
    show_type_chips: false,
    micro_step: 100,
};

pub const PY101_LINKED_DELETE: CodingStep = CodingStep {
    id: "py-101-linked-delete",
    title: "DSA Linked List Delete",
    objective: "Borrar un nodo reconectando next.",
    prompt_md: "**Delete a Node**\n\nAntes de borrar, conectá el nodo previo con el siguiente.\n\n**Micro-reto:**\n1. Creá `Node` y la cadena `7 -> 11 -> 3`\n2. Definí `deleteNext(node)` que elimine el nodo inmediatamente siguiente a `node`\n3. Llamá `deleteNext(node1)` (borra el `11`) y imprimí `node1.next.data`",
    starter_code: "# class Node:\n#     ...\n# def deleteNext(node):\n#     ...\n# ...\n# print(...)\n",
    pytest: "def test_linked_delete(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['node1'].next.data == 3\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '3'\n",
    hint: "class Node:\n    def __init__(self, data):\n        self.data = data\n        self.next = None\ndef deleteNext(node):\n    if node.next is not None:\n        node.next = node.next.next\nnode1 = Node(7)\nnode2 = Node(11)\nnode3 = Node(3)\nnode1.next = node2\nnode2.next = node3\ndeleteNext(node1)\nprint(node1.next.data)",
    solution_example: "class Node:\n    def __init__(self, data):\n        self.data = data\n        self.next = None\ndef deleteNext(node):\n    if node.next is not None:\n        node.next = node.next.next\nnode1 = Node(7)\nnode2 = Node(11)\nnode3 = Node(3)\nnode1.next = node2\nnode2.next = node3\ndeleteNext(node1)\nprint(node1.next.data)\n",
    next: Some("py-102-linked-insert"),
    show_type_chips: false,
    micro_step: 101,
};

pub const PY102_LINKED_INSERT: CodingStep = CodingStep {
    id: "py-102-linked-insert",
    title: "DSA Linked List Insert",
    objective: "Insertar un nodo después de otro.",
    prompt_md: "**Insert a Node**\n\nAjustá los punteros `next` al insertar.\n\n**Micro-reto:**\n1. Creá la cadena `7 -> 3`\n2. Definí `insertAfter(node, newNode)` que inserte `newNode` justo después de `node`\n3. Insertá `Node(97)` después de `node1` e imprimí `node1.next.data`",
    starter_code: "# class Node:\n#     ...\n# def insertAfter(node, newNode):\n#     ...\n# ...\n# print(...)\n",
    pytest: "def test_linked_insert(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['node1'].next.data == 97\n    assert ns['node1'].next.next.data == 3\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '97'\n",
    hint: "class Node:\n    def __init__(self, data):\n        self.data = data\n        self.next = None\ndef insertAfter(node, newNode):\n    newNode.next = node.next\n    node.next = newNode\nnode1 = Node(7)\nnode2 = Node(3)\nnode1.next = node2\ninsertAfter(node1, Node(97))\nprint(node1.next.data)",
    solution_example: "class Node:\n    def __init__(self, data):\n        self.data = data\n        self.next = None\ndef insertAfter(node, newNode):\n    newNode.next = node.next\n    node.next = newNode\nnode1 = Node(7)\nnode2 = Node(3)\nnode1.next = node2\ninsertAfter(node1, Node(97))\nprint(node1.next.data)\n",
    next: Some("py-103-merge-sort"),
    show_type_chips: false,
    micro_step: 102,
};

pub const PY103_MERGE_SORT: CodingStep = CodingStep {
    id: "py-103-merge-sort",
    title: "DSA Merge Sort",
    objective: "Ordenar con Merge Sort recursivo.",
    prompt_md: "**Merge Sort**\n\nDividí, ordená cada mitad y mergeá.\n\n**Micro-reto:**\n1. Implementá `merge` y `mergeSort` como en W3S\n2. Ordená `mylist = [3, 7, 6, -10, 15, 23.5, 55, -13]`\n3. Imprimí el resultado de `mergeSort(mylist)`",
    starter_code: "# def merge(left, right):\n#     ...\n# def mergeSort(arr):\n#     ...\n# mylist = ...\n# print(...)\n",
    pytest: "def test_merge_sort(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '[-13, -10, 3, 6, 7, 15, 23.5, 55]'\n",
    hint: "def merge(left, right):\n    result = []\n    i = j = 0\n    while i < len(left) and j < len(right):\n        if left[i] < right[j]:\n            result.append(left[i])\n            i += 1\n        else:\n            result.append(right[j])\n            j += 1\n    result.extend(left[i:])\n    result.extend(right[j:])\n    return result\ndef mergeSort(arr):\n    if len(arr) <= 1:\n        return arr\n    mid = len(arr) // 2\n    return merge(mergeSort(arr[:mid]), mergeSort(arr[mid:]))\nmylist = [3, 7, 6, -10, 15, 23.5, 55, -13]\nprint(mergeSort(mylist))",
    solution_example: "def merge(left, right):\n    result = []\n    i = j = 0\n    while i < len(left) and j < len(right):\n        if left[i] < right[j]:\n            result.append(left[i])\n            i += 1\n        else:\n            result.append(right[j])\n            j += 1\n    result.extend(left[i:])\n    result.extend(right[j:])\n    return result\ndef mergeSort(arr):\n    if len(arr) <= 1:\n        return arr\n    mid = len(arr) // 2\n    return merge(mergeSort(arr[:mid]), mergeSort(arr[mid:]))\nmylist = [3, 7, 6, -10, 15, 23.5, 55, -13]\nprint(mergeSort(mylist))\n",
    next: Some("py-104-counting-sort"),
    show_type_chips: false,
    micro_step: 103,
};

pub const PY104_COUNTING_SORT: CodingStep = CodingStep {
    id: "py-104-counting-sort",
    title: "DSA Counting Sort",
    objective: "Ordenar enteros no negativos contando frecuencias.",
    prompt_md: "**Counting Sort**\n\nContá cuántas veces aparece cada valor y reconstruí el array ordenado.\n\n**Micro-reto:**\n1. Definí `countingSort(arr)` para enteros `>= 0`\n2. Ordená `mylist = [4, 2, 2, 8, 3, 3, 1]`\n3. Imprimí el resultado",
    starter_code: "# def countingSort(arr):\n#     ...\n# mylist = ...\n# print(...)\n",
    pytest: "def test_counting_sort(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['countingSort']([4, 2, 2, 8, 3, 3, 1]) == [1, 2, 2, 3, 3, 4, 8]\n    out = ' '.join(capsys.readouterr().out.split())\n    assert out == '[1, 2, 2, 3, 3, 4, 8]'\n",
    hint: "def countingSort(arr):\n    if not arr:\n        return []\n    size = max(arr) + 1\n    count = [0] * size\n    for x in arr:\n        count[x] += 1\n    out = []\n    for value, freq in enumerate(count):\n        out.extend([value] * freq)\n    return out\nmylist = [4, 2, 2, 8, 3, 3, 1]\nprint(countingSort(mylist))",
    solution_example: "def countingSort(arr):\n    if not arr:\n        return []\n    size = max(arr) + 1\n    count = [0] * size\n    for x in arr:\n        count[x] += 1\n    out = []\n    for value, freq in enumerate(count):\n        out.extend([value] * freq)\n    return out\nmylist = [4, 2, 2, 8, 3, 3, 1]\nprint(countingSort(mylist))\n",
    next: Some("py-105-tree-node"),
    show_type_chips: false,
    micro_step: 104,
};

pub const PY105_TREE_NODE: CodingStep = CodingStep {
    id: "py-105-tree-node",
    title: "DSA Binary Tree Node",
    objective: "Crear un árbol binario con left y right.",
    prompt_md: "**Binary Trees**\n\nUn TreeNode tiene `data`, `left` y `right`.\n\n**Micro-reto:**\n1. Creá `class TreeNode` con `data`, `left=None`, `right=None`\n2. Creá `root = TreeNode(1)` con `left=TreeNode(2)` y `right=TreeNode(3)`\n3. Imprimí `root.left.data` y `root.right.data`",
    starter_code: "# class TreeNode:\n#     ...\n# root = ...\n# print(...)\n",
    pytest: "def test_tree_node(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['root'].data == 1\n    assert ns['root'].left.data == 2 and ns['root'].right.data == 3\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['2', '3']\n",
    hint: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\nroot = TreeNode(1)\nroot.left = TreeNode(2)\nroot.right = TreeNode(3)\nprint(root.left.data)\nprint(root.right.data)",
    solution_example: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\nroot = TreeNode(1)\nroot.left = TreeNode(2)\nroot.right = TreeNode(3)\nprint(root.left.data)\nprint(root.right.data)\n",
    next: Some("py-106-tree-preorder"),
    show_type_chips: false,
    micro_step: 105,
};

pub const PY106_TREE_PREORDER: CodingStep = CodingStep {
    id: "py-106-tree-preorder",
    title: "DSA Tree Preorder",
    objective: "Recorrer un árbol en preorder (root-left-right).",
    prompt_md: "**Tree Traversal (Preorder)**\n\nPreorder: visitar root, luego left, luego right.\n\n**Micro-reto:**\n1. Creá el árbol `1` con left `2` y right `3`\n2. Definí `preorder(node)` que imprima cada `data` en su propia línea\n3. Llamá `preorder(root)`",
    starter_code: "# class TreeNode:\n#     ...\n# def preorder(node):\n#     ...\n# ...\n# preorder(root)\n",
    pytest: "def test_tree_preorder(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['1', '2', '3']\n    assert callable(ns.get('preorder'))\n",
    hint: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\ndef preorder(node):\n    if node is None:\n        return\n    print(node.data)\n    preorder(node.left)\n    preorder(node.right)\nroot = TreeNode(1)\nroot.left = TreeNode(2)\nroot.right = TreeNode(3)\npreorder(root)",
    solution_example: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\ndef preorder(node):\n    if node is None:\n        return\n    print(node.data)\n    preorder(node.left)\n    preorder(node.right)\nroot = TreeNode(1)\nroot.left = TreeNode(2)\nroot.right = TreeNode(3)\npreorder(root)\n",
    next: Some("py-107-tree-inorder"),
    show_type_chips: false,
    micro_step: 106,
};

pub const PY107_TREE_INORDER: CodingStep = CodingStep {
    id: "py-107-tree-inorder",
    title: "DSA Tree Inorder",
    objective: "Recorrer un árbol en inorder (left-root-right).",
    prompt_md: "**Tree Traversal (Inorder)**\n\nInorder: left, luego root, luego right.\n\n**Micro-reto:**\n1. Creá el árbol `1` con left `2` y right `3`\n2. Definí `inorder(node)` que imprima cada `data` en su propia línea\n3. Llamá `inorder(root)` (salida esperada: `2`, `1`, `3`)",
    starter_code: "# class TreeNode:\n#     ...\n# def inorder(node):\n#     ...\n# ...\n# inorder(root)\n",
    pytest: "def test_tree_inorder(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['2', '1', '3']\n    assert callable(ns.get('inorder'))\n",
    hint: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\ndef inorder(node):\n    if node is None:\n        return\n    inorder(node.left)\n    print(node.data)\n    inorder(node.right)\nroot = TreeNode(1)\nroot.left = TreeNode(2)\nroot.right = TreeNode(3)\ninorder(root)",
    solution_example: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\ndef inorder(node):\n    if node is None:\n        return\n    inorder(node.left)\n    print(node.data)\n    inorder(node.right)\nroot = TreeNode(1)\nroot.left = TreeNode(2)\nroot.right = TreeNode(3)\ninorder(root)\n",
    next: Some("py-108-tree-postorder"),
    show_type_chips: false,
    micro_step: 107,
};

pub const PY108_TREE_POSTORDER: CodingStep = CodingStep {
    id: "py-108-tree-postorder",
    title: "DSA Tree Postorder",
    objective: "Recorrer un árbol en postorder (left-right-root).",
    prompt_md: "**Tree Traversal (Postorder)**\n\nPostorder: left, luego right, luego root.\n\n**Micro-reto:**\n1. Creá el árbol `1` con left `2` y right `3`\n2. Definí `postorder(node)` que imprima cada `data` en su propia línea\n3. Llamá `postorder(root)` (salida esperada: `2`, `3`, `1`)",
    starter_code: "# class TreeNode:\n#     ...\n# def postorder(node):\n#     ...\n# ...\n# postorder(root)\n",
    pytest: "def test_tree_postorder(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['2', '3', '1']\n    assert callable(ns.get('postorder'))\n",
    hint: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\ndef postorder(node):\n    if node is None:\n        return\n    postorder(node.left)\n    postorder(node.right)\n    print(node.data)\nroot = TreeNode(1)\nroot.left = TreeNode(2)\nroot.right = TreeNode(3)\npostorder(root)",
    solution_example: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\ndef postorder(node):\n    if node is None:\n        return\n    postorder(node.left)\n    postorder(node.right)\n    print(node.data)\nroot = TreeNode(1)\nroot.left = TreeNode(2)\nroot.right = TreeNode(3)\npostorder(root)\n",
    next: Some("py-109-graph-dfs"),
    show_type_chips: false,
    micro_step: 108,
};

pub const PY109_GRAPH_DFS: CodingStep = CodingStep {
    id: "py-109-graph-dfs",
    title: "DSA Graph DFS",
    objective: "Recorrer un grafo con Depth First Search.",
    prompt_md: "**Graphs · DFS**\n\nDFS explora tan profundo como puede antes de backtrack.\n\n**Micro-reto:**\n1. Definí el grafo `{'A': ['B', 'C'], 'B': ['D'], 'C': ['E'], 'D': [], 'E': []}`\n2. Definí `dfs(graph, node, visited=None)` que imprima cada nodo visitado en su línea\n3. Llamá `dfs(graph, 'A')` (salida: `A`, `B`, `D`, `C`, `E`)",
    starter_code: "# graph = {...}\n# def dfs(graph, node, visited=None):\n#     ...\n# dfs(graph, 'A')\n",
    pytest: "def test_graph_dfs(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['A', 'B', 'D', 'C', 'E']\n    assert callable(ns.get('dfs'))\n",
    hint: "graph = {'A': ['B', 'C'], 'B': ['D'], 'C': ['E'], 'D': [], 'E': []}\ndef dfs(graph, node, visited=None):\n    if visited is None:\n        visited = set()\n    if node in visited:\n        return\n    visited.add(node)\n    print(node)\n    for neighbor in graph[node]:\n        dfs(graph, neighbor, visited)\ndfs(graph, 'A')",
    solution_example: "graph = {'A': ['B', 'C'], 'B': ['D'], 'C': ['E'], 'D': [], 'E': []}\ndef dfs(graph, node, visited=None):\n    if visited is None:\n        visited = set()\n    if node in visited:\n        return\n    visited.add(node)\n    print(node)\n    for neighbor in graph[node]:\n        dfs(graph, neighbor, visited)\ndfs(graph, 'A')\n",
    next: Some("py-110-graph-bfs"),
    show_type_chips: false,
    micro_step: 109,
};

pub const PY110_GRAPH_BFS: CodingStep = CodingStep {
    id: "py-110-graph-bfs",
    title: "DSA Graph BFS",
    objective: "Recorrer un grafo con Breadth First Search.",
    prompt_md: "**Graphs · BFS**\n\nBFS visita nivel por nivel usando una cola.\n\n**Micro-reto:**\n1. Usá el mismo grafo del DFS: `A→B,C` · `B→D` · `C→E`\n2. Definí `bfs(graph, start)` que imprima cada nodo visitado en su línea\n3. Llamá `bfs(graph, 'A')` (salida: `A`, `B`, `C`, `D`, `E`)",
    starter_code: "# from collections import deque\n# graph = {...}\n# def bfs(graph, start):\n#     ...\n# bfs(graph, 'A')\n",
    pytest: "def test_graph_bfs(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['A', 'B', 'C', 'D', 'E']\n    assert callable(ns.get('bfs'))\n",
    hint: "from collections import deque\ngraph = {'A': ['B', 'C'], 'B': ['D'], 'C': ['E'], 'D': [], 'E': []}\ndef bfs(graph, start):\n    visited = set([start])\n    q = deque([start])\n    while q:\n        node = q.popleft()\n        print(node)\n        for neighbor in graph[node]:\n            if neighbor not in visited:\n                visited.add(neighbor)\n                q.append(neighbor)\nbfs(graph, 'A')",
    solution_example: "from collections import deque\ngraph = {'A': ['B', 'C'], 'B': ['D'], 'C': ['E'], 'D': [], 'E': []}\ndef bfs(graph, start):\n    visited = set([start])\n    q = deque([start])\n    while q:\n        node = q.popleft()\n        print(node)\n        for neighbor in graph[node]:\n            if neighbor not in visited:\n                visited.add(neighbor)\n                q.append(neighbor)\nbfs(graph, 'A')\n",
    next: Some("py-111-tree-height"),
    show_type_chips: false,
    micro_step: 110,
};

pub const PY111_TREE_HEIGHT: CodingStep = CodingStep {
    id: "py-111-tree-height",
    title: "DSA Tree Height",
    objective: "Calcular la altura de un árbol binario.",
    prompt_md: "**Tree Height**\n\nLa altura de un nodo es `1 + max(altura left, altura right)`. Un árbol vacío tiene altura `0`.\n\n**Micro-reto:**\n1. Creá el árbol `1` con left `2` y right `3`\n2. Definí `treeHeight(node)` recursiva\n3. Imprimí `treeHeight(root)` (debe ser `2`)",
    starter_code: "# class TreeNode:\n#     ...\n# def treeHeight(node):\n#     ...\n# ...\n# print(treeHeight(root))\n",
    pytest: "def test_tree_height(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('treeHeight'))\n    assert ns['treeHeight'](ns['root']) == 2\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['2']\n",
    hint: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\ndef treeHeight(node):\n    if node is None:\n        return 0\n    return 1 + max(treeHeight(node.left), treeHeight(node.right))\nroot = TreeNode(1)\nroot.left = TreeNode(2)\nroot.right = TreeNode(3)\nprint(treeHeight(root))",
    solution_example: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\ndef treeHeight(node):\n    if node is None:\n        return 0\n    return 1 + max(treeHeight(node.left), treeHeight(node.right))\nroot = TreeNode(1)\nroot.left = TreeNode(2)\nroot.right = TreeNode(3)\nprint(treeHeight(root))\n",
    next: Some("py-112-dijkstra"),
    show_type_chips: false,
    micro_step: 111,
};

pub const PY112_DIJKSTRA: CodingStep = CodingStep {
    id: "py-112-dijkstra",
    title: "DSA Dijkstra Intro",
    objective: "Calcular distancias mínimas con Dijkstra (sin heap, dict).",
    prompt_md: "**Dijkstra (intro)**\n\nDistancias mínimas desde un origen en un grafo ponderado no negativo.\n\n**Micro-reto:**\n1. Grafo: `{'A': {'B': 4, 'C': 2}, 'B': {'C': 1, 'D': 5}, 'C': {'D': 8}, 'D': {}}`\n2. Definí `dijkstra(graph, start)` que devuelva un dict de distancias\n3. Imprimí `dijkstra(graph, 'A')` (esperado: `{'A': 0, 'B': 4, 'C': 2, 'D': 9}`)",
    starter_code: "# graph = {...}\n# def dijkstra(graph, start):\n#     ...\n# print(dijkstra(graph, 'A'))\n",
    pytest: "def test_dijkstra(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('dijkstra'))\n    assert ns['dijkstra'](ns['graph'], 'A') == {'A': 0, 'B': 4, 'C': 2, 'D': 9}\n    out = capsys.readouterr().out\n    assert \"'A': 0\" in out and \"'B': 4\" in out and \"'C': 2\" in out and \"'D': 9\" in out\n",
    hint: "graph = {'A': {'B': 4, 'C': 2}, 'B': {'C': 1, 'D': 5}, 'C': {'D': 8}, 'D': {}}\ndef dijkstra(graph, start):\n    dist = {n: float('inf') for n in graph}\n    dist[start] = 0\n    unvisited = set(graph)\n    while unvisited:\n        u = min(unvisited, key=lambda n: dist[n])\n        unvisited.remove(u)\n        for v, w in graph[u].items():\n            alt = dist[u] + w\n            if alt < dist[v]:\n                dist[v] = alt\n    return dist\nprint(dijkstra(graph, 'A'))",
    solution_example: "graph = {'A': {'B': 4, 'C': 2}, 'B': {'C': 1, 'D': 5}, 'C': {'D': 8}, 'D': {}}\ndef dijkstra(graph, start):\n    dist = {n: float('inf') for n in graph}\n    dist[start] = 0\n    unvisited = set(graph)\n    while unvisited:\n        u = min(unvisited, key=lambda n: dist[n])\n        unvisited.remove(u)\n        for v, w in graph[u].items():\n            alt = dist[u] + w\n            if alt < dist[v]:\n                dist[v] = alt\n    return dist\nprint(dijkstra(graph, 'A'))\n",
    next: Some("py-113-heap"),
    show_type_chips: false,
    micro_step: 112,
};

pub const PY113_HEAP: CodingStep = CodingStep {
    id: "py-113-heap",
    title: "DSA Heap (heapq)",
    objective: "Usar un min-heap con heapq (push/pop).",
    prompt_md: "**Heaps**\n\nUn min-heap saca siempre el menor con `heappop`.\n\n**Micro-reto:**\n1. Importá `heapq`\n2. Hacé push de `5`, `3`, `8`, `1`\n3. Hacé pop hasta vaciar e imprimí cada valor en su línea (salida: `1`, `3`, `5`, `8`)",
    starter_code: "# import heapq\n# h = []\n# ...\n",
    pytest: "def test_heap(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['1', '3', '5', '8']\n",
    hint: "import heapq\nh = []\nfor x in [5, 3, 8, 1]:\n    heapq.heappush(h, x)\nwhile h:\n    print(heapq.heappop(h))",
    solution_example: "import heapq\nh = []\nfor x in [5, 3, 8, 1]:\n    heapq.heappush(h, x)\nwhile h:\n    print(heapq.heappop(h))\n",
    next: Some("py-114-priority-queue"),
    show_type_chips: false,
    micro_step: 113,
};

pub const PY114_PRIORITY_QUEUE: CodingStep = CodingStep {
    id: "py-114-priority-queue",
    title: "DSA Priority Queue",
    objective: "Priorizar tareas con un heap de tuplas (prioridad, tarea).",
    prompt_md: "**Priority Queue**\n\nEn Python se modela con `heapq` y tuplas `(prioridad, valor)`.\n\n**Micro-reto:**\n1. Encolá `(2, 'code')`, `(1, 'eat')`, `(3, 'sleep')`\n2. Desencolá e imprimí solo el nombre de cada tarea (salida: `eat`, `code`, `sleep`)",
    starter_code: "# import heapq\n# pq = []\n# ...\n",
    pytest: "def test_priority_queue(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['eat', 'code', 'sleep']\n",
    hint: "import heapq\npq = []\nheapq.heappush(pq, (2, 'code'))\nheapq.heappush(pq, (1, 'eat'))\nheapq.heappush(pq, (3, 'sleep'))\nwhile pq:\n    print(heapq.heappop(pq)[1])",
    solution_example: "import heapq\npq = []\nheapq.heappush(pq, (2, 'code'))\nheapq.heappush(pq, (1, 'eat'))\nheapq.heappush(pq, (3, 'sleep'))\nwhile pq:\n    print(heapq.heappop(pq)[1])\n",
    next: Some("py-115-union-find"),
    show_type_chips: false,
    micro_step: 114,
};

pub const PY115_UNION_FIND: CodingStep = CodingStep {
    id: "py-115-union-find",
    title: "DSA Union-Find",
    objective: "Implementar find/union (disjoint set) sobre parent[].",
    prompt_md: "**Union-Find**\n\nDetecta componentes conexas: `find` sigue al root; `union` fusiona trees.\n\n**Micro-reto:**\n1. `parent = [0, 1, 2, 3]`\n2. Definí `find(x)` y `union(a, b)`\n3. Hacé `union(0,1)`, `union(2,3)`, `union(1,2)` e imprimí `find(0) == find(3)`",
    starter_code: "# parent = [0, 1, 2, 3]\n# def find(x):\n#     ...\n# def union(a, b):\n#     ...\n# ...\n# print(...)\n",
    pytest: "def test_union_find(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('find')) and callable(ns.get('union'))\n    assert ns['find'](0) == ns['find'](3)\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['True']\n",
    hint: "parent = [0, 1, 2, 3]\ndef find(x):\n    while parent[x] != x:\n        x = parent[x]\n    return x\ndef union(a, b):\n    ra, rb = find(a), find(b)\n    if ra != rb:\n        parent[rb] = ra\nunion(0, 1)\nunion(2, 3)\nunion(1, 2)\nprint(find(0) == find(3))",
    solution_example: "parent = [0, 1, 2, 3]\ndef find(x):\n    while parent[x] != x:\n        x = parent[x]\n    return x\ndef union(a, b):\n    ra, rb = find(a), find(b)\n    if ra != rb:\n        parent[rb] = ra\nunion(0, 1)\nunion(2, 3)\nunion(1, 2)\nprint(find(0) == find(3))\n",
    next: Some("py-116-kruskal"),
    show_type_chips: false,
    micro_step: 115,
};

pub const PY116_KRUSKAL: CodingStep = CodingStep {
    id: "py-116-kruskal",
    title: "DSA Kruskal MST",
    objective: "Calcular el peso del MST con Kruskal + Union-Find.",
    prompt_md: "**Kruskal**\n\nOrdená aristas por peso y uní extremos si no forman ciclo.\n\n**Micro-reto:**\n1. `edges = [(1, 'A', 'B'), (2, 'B', 'C'), (3, 'A', 'C'), (4, 'C', 'D')]`\n2. Definí `kruskal(edges, nodes)` que devuelva el peso total del MST\n3. Imprimí `kruskal(edges, ['A', 'B', 'C', 'D'])` (esperado: `7`)",
    starter_code: "# edges = [...]\n# def kruskal(edges, nodes):\n#     ...\n# print(kruskal(edges, ['A', 'B', 'C', 'D']))\n",
    pytest: "def test_kruskal(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('kruskal'))\n    assert ns['kruskal'](ns['edges'], ['A', 'B', 'C', 'D']) == 7\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['7']\n",
    hint: "edges = [(1, 'A', 'B'), (2, 'B', 'C'), (3, 'A', 'C'), (4, 'C', 'D')]\ndef kruskal(edges, nodes):\n    parent = {n: n for n in nodes}\n    def find(x):\n        while parent[x] != x:\n            x = parent[x]\n        return x\n    total = 0\n    for w, u, v in sorted(edges, key=lambda e: e[0]):\n        if find(u) != find(v):\n            parent[find(v)] = find(u)\n            total += w\n    return total\nprint(kruskal(edges, ['A', 'B', 'C', 'D']))",
    solution_example: "edges = [(1, 'A', 'B'), (2, 'B', 'C'), (3, 'A', 'C'), (4, 'C', 'D')]\ndef kruskal(edges, nodes):\n    parent = {n: n for n in nodes}\n    def find(x):\n        while parent[x] != x:\n            x = parent[x]\n        return x\n    total = 0\n    for w, u, v in sorted(edges, key=lambda e: e[0]):\n        if find(u) != find(v):\n            parent[find(v)] = find(u)\n            total += w\n    return total\nprint(kruskal(edges, ['A', 'B', 'C', 'D']))\n",
    next: Some("py-117-prim"),
    show_type_chips: false,
    micro_step: 116,
};

pub const PY117_PRIM: CodingStep = CodingStep {
    id: "py-117-prim",
    title: "DSA Prim MST",
    objective: "Calcular el peso del MST creciendo desde un vértice (Prim + heap).",
    prompt_md: "**Prim**\n\nDesde un start, sumá la arista más barata hacia un nodo no visitado.\n\n**Micro-reto:**\n1. Grafo no dirigido ponderado A–B–C–D (pesos como en el hint)\n2. Definí `prim(graph, start='A')` que devuelva el peso del MST\n3. Imprimí el resultado (esperado: `7`)",
    starter_code: "# import heapq\n# graph = {...}\n# def prim(graph, start='A'):\n#     ...\n# print(prim(graph))\n",
    pytest: "def test_prim(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('prim'))\n    assert ns['prim'](ns['graph']) == 7\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['7']\n",
    hint: "import heapq\ngraph = {'A': {'B': 1, 'C': 3}, 'B': {'A': 1, 'C': 2, 'D': 4}, 'C': {'A': 3, 'B': 2, 'D': 5}, 'D': {'B': 4, 'C': 5}}\ndef prim(graph, start='A'):\n    visited = set()\n    pq = [(0, start)]\n    total = 0\n    while pq and len(visited) < len(graph):\n        w, u = heapq.heappop(pq)\n        if u in visited:\n            continue\n        visited.add(u)\n        total += w\n        for v, vw in graph[u].items():\n            if v not in visited:\n                heapq.heappush(pq, (vw, v))\n    return total\nprint(prim(graph))",
    solution_example: "import heapq\ngraph = {'A': {'B': 1, 'C': 3}, 'B': {'A': 1, 'C': 2, 'D': 4}, 'C': {'A': 3, 'B': 2, 'D': 5}, 'D': {'B': 4, 'C': 5}}\ndef prim(graph, start='A'):\n    visited = set()\n    pq = [(0, start)]\n    total = 0\n    while pq and len(visited) < len(graph):\n        w, u = heapq.heappop(pq)\n        if u in visited:\n            continue\n        visited.add(u)\n        total += w\n        for v, vw in graph[u].items():\n            if v not in visited:\n                heapq.heappush(pq, (vw, v))\n    return total\nprint(prim(graph))\n",
    next: Some("py-118-topo-sort"),
    show_type_chips: false,
    micro_step: 117,
};

pub const PY118_TOPO_SORT: CodingStep = CodingStep {
    id: "py-118-topo-sort",
    title: "DSA Topological Sort",
    objective: "Ordenar un DAG con Kahn (indegree + cola).",
    prompt_md: "**Topological Sort**\n\nEn un DAG, Kahn saca nodos con indegree 0.\n\n**Micro-reto:**\n1. `graph = {'A': ['B', 'C'], 'B': ['D'], 'C': ['D'], 'D': []}`\n2. Definí `topo(graph)` que devuelva la lista ordenada\n3. Imprimí `topo(graph)` (esperado: `['A', 'B', 'C', 'D']`)",
    starter_code: "# from collections import deque\n# graph = {...}\n# def topo(graph):\n#     ...\n# print(topo(graph))\n",
    pytest: "def test_topo_sort(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('topo'))\n    assert ns['topo'](ns['graph']) == ['A', 'B', 'C', 'D']\n    out = ' '.join(capsys.readouterr().out.split())\n    assert \"'A'\" in out and \"'B'\" in out and \"'C'\" in out and \"'D'\" in out\n",
    hint: "from collections import deque\ngraph = {'A': ['B', 'C'], 'B': ['D'], 'C': ['D'], 'D': []}\ndef topo(graph):\n    indeg = {n: 0 for n in graph}\n    for u in graph:\n        for v in graph[u]:\n            indeg[v] += 1\n    q = deque([n for n in graph if indeg[n] == 0])\n    order = []\n    while q:\n        u = q.popleft()\n        order.append(u)\n        for v in graph[u]:\n            indeg[v] -= 1\n            if indeg[v] == 0:\n                q.append(v)\n    return order\nprint(topo(graph))",
    solution_example: "from collections import deque\ngraph = {'A': ['B', 'C'], 'B': ['D'], 'C': ['D'], 'D': []}\ndef topo(graph):\n    indeg = {n: 0 for n in graph}\n    for u in graph:\n        for v in graph[u]:\n            indeg[v] += 1\n    q = deque([n for n in graph if indeg[n] == 0])\n    order = []\n    while q:\n        u = q.popleft()\n        order.append(u)\n        for v in graph[u]:\n            indeg[v] -= 1\n            if indeg[v] == 0:\n                q.append(v)\n    return order\nprint(topo(graph))\n",
    next: Some("py-119-bellman-ford"),
    show_type_chips: false,
    micro_step: 118,
};

pub const PY119_BELLMAN_FORD: CodingStep = CodingStep {
    id: "py-119-bellman-ford",
    title: "DSA Bellman-Ford",
    objective: "Calcular distancias con Bellman-Ford (aristas negativas OK).",
    prompt_md: "**Bellman-Ford**\n\nRelajá todas las aristas `V-1` veces; admite pesos negativos.\n\n**Micro-reto:**\n1. `edges = [('A','B',4), ('A','C',2), ('B','C',-1), ('B','D',5), ('C','D',3)]`\n2. Definí `bellman_ford(edges, nodes, start)` → dict de distancias\n3. Imprimí desde `'A'` (esperado: `A:0 B:4 C:2 D:5`)",
    starter_code: "# edges = [...]\n# nodes = ['A', 'B', 'C', 'D']\n# def bellman_ford(edges, nodes, start):\n#     ...\n# print(bellman_ford(edges, nodes, 'A'))\n",
    pytest: "def test_bellman_ford(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('bellman_ford'))\n    assert ns['bellman_ford'](ns['edges'], ns['nodes'], 'A') == {'A': 0, 'B': 4, 'C': 2, 'D': 5}\n    out = capsys.readouterr().out\n    assert \"'A': 0\" in out and \"'B': 4\" in out and \"'C': 2\" in out and \"'D': 5\" in out\n",
    hint: "edges = [('A', 'B', 4), ('A', 'C', 2), ('B', 'C', -1), ('B', 'D', 5), ('C', 'D', 3)]\nnodes = ['A', 'B', 'C', 'D']\ndef bellman_ford(edges, nodes, start):\n    dist = {n: float('inf') for n in nodes}\n    dist[start] = 0\n    for _ in range(len(nodes) - 1):\n        for u, v, w in edges:\n            if dist[u] + w < dist[v]:\n                dist[v] = dist[u] + w\n    return dist\nprint(bellman_ford(edges, nodes, 'A'))",
    solution_example: "edges = [('A', 'B', 4), ('A', 'C', 2), ('B', 'C', -1), ('B', 'D', 5), ('C', 'D', 3)]\nnodes = ['A', 'B', 'C', 'D']\ndef bellman_ford(edges, nodes, start):\n    dist = {n: float('inf') for n in nodes}\n    dist[start] = 0\n    for _ in range(len(nodes) - 1):\n        for u, v, w in edges:\n            if dist[u] + w < dist[v]:\n                dist[v] = dist[u] + w\n    return dist\nprint(bellman_ford(edges, nodes, 'A'))\n",
    next: Some("py-120-memo-fib"),
    show_type_chips: false,
    micro_step: 119,
};

pub const PY120_MEMO_FIB: CodingStep = CodingStep {
    id: "py-120-memo-fib",
    title: "DSA Memoization (Fib)",
    objective: "Calcular Fibonacci con memoization (top-down).",
    prompt_md: "**Memoization**\n\nGuardá subproblemas en un dict para no recomputar.\n\n**Micro-reto:**\n1. Definí `fib(n)` con `memo = {}`\n2. Imprimí `fib(6)` (esperado: `8`)",
    starter_code: "# memo = {}\n# def fib(n):\n#     ...\n# print(fib(6))\n",
    pytest: "def test_memo_fib(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('fib'))\n    assert ns['fib'](6) == 8\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['8']\n",
    hint: "memo = {}\ndef fib(n):\n    if n in memo:\n        return memo[n]\n    if n <= 1:\n        return n\n    memo[n] = fib(n - 1) + fib(n - 2)\n    return memo[n]\nprint(fib(6))",
    solution_example: "memo = {}\ndef fib(n):\n    if n in memo:\n        return memo[n]\n    if n <= 1:\n        return n\n    memo[n] = fib(n - 1) + fib(n - 2)\n    return memo[n]\nprint(fib(6))\n",
    next: Some("py-121-tab-fib"),
    show_type_chips: false,
    micro_step: 120,
};

pub const PY121_TAB_FIB: CodingStep = CodingStep {
    id: "py-121-tab-fib",
    title: "DSA Tabulation (Fib)",
    objective: "Calcular Fibonacci con tabulación (bottom-up).",
    prompt_md: "**Tabulation**\n\nLlená una tabla `dp` desde lo más chico hacia `n`.\n\n**Micro-reto:**\n1. Definí `fib_tab(n)` con array/lista DP\n2. Imprimí `fib_tab(6)` (esperado: `8`)",
    starter_code: "# def fib_tab(n):\n#     ...\n# print(fib_tab(6))\n",
    pytest: "def test_tab_fib(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('fib_tab'))\n    assert ns['fib_tab'](6) == 8\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['8']\n",
    hint: "def fib_tab(n):\n    if n <= 1:\n        return n\n    dp = [0] * (n + 1)\n    dp[1] = 1\n    for i in range(2, n + 1):\n        dp[i] = dp[i - 1] + dp[i - 2]\n    return dp[n]\nprint(fib_tab(6))",
    solution_example: "def fib_tab(n):\n    if n <= 1:\n        return n\n    dp = [0] * (n + 1)\n    dp[1] = 1\n    for i in range(2, n + 1):\n        dp[i] = dp[i - 1] + dp[i - 2]\n    return dp[n]\nprint(fib_tab(6))\n",
    next: Some("py-122-knapsack"),
    show_type_chips: false,
    micro_step: 121,
};

pub const PY122_KNAPSACK: CodingStep = CodingStep {
    id: "py-122-knapsack",
    title: "DSA 0/1 Knapsack",
    objective: "Maximizar valor bajo un peso límite (tabla DP).",
    prompt_md: "**0/1 Knapsack**\n\nCada ítem se toma o no; DP `dp[i][w]`.\n\n**Micro-reto:**\n1. `weights = [1, 3, 4]`, `values = [15, 20, 30]`, `capacity = 4`\n2. Definí `knapsack(weights, values, capacity)`\n3. Imprimí el valor óptimo (esperado: `35`)",
    starter_code: "# weights = [1, 3, 4]\n# values = [15, 20, 30]\n# def knapsack(weights, values, capacity):\n#     ...\n# print(knapsack(weights, values, 4))\n",
    pytest: "def test_knapsack(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('knapsack'))\n    assert ns['knapsack'](ns['weights'], ns['values'], 4) == 35\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['35']\n",
    hint: "weights = [1, 3, 4]\nvalues = [15, 20, 30]\ndef knapsack(weights, values, capacity):\n    n = len(weights)\n    dp = [[0] * (capacity + 1) for _ in range(n + 1)]\n    for i in range(1, n + 1):\n        for w in range(capacity + 1):\n            dp[i][w] = dp[i - 1][w]\n            if weights[i - 1] <= w:\n                dp[i][w] = max(dp[i][w], dp[i - 1][w - weights[i - 1]] + values[i - 1])\n    return dp[n][capacity]\nprint(knapsack(weights, values, 4))",
    solution_example: "weights = [1, 3, 4]\nvalues = [15, 20, 30]\ndef knapsack(weights, values, capacity):\n    n = len(weights)\n    dp = [[0] * (capacity + 1) for _ in range(n + 1)]\n    for i in range(1, n + 1):\n        for w in range(capacity + 1):\n            dp[i][w] = dp[i - 1][w]\n            if weights[i - 1] <= w:\n                dp[i][w] = max(dp[i][w], dp[i - 1][w - weights[i - 1]] + values[i - 1])\n    return dp[n][capacity]\nprint(knapsack(weights, values, 4))\n",
    next: Some("py-123-euclidean"),
    show_type_chips: false,
    micro_step: 122,
};

pub const PY123_EUCLIDEAN: CodingStep = CodingStep {
    id: "py-123-euclidean",
    title: "DSA Euclidean GCD",
    objective: "Calcular el MCD con el algoritmo de Euclides.",
    prompt_md: "**Euclidean Algorithm**\n\n`gcd(a, b) = gcd(b, a % b)` hasta `b == 0`.\n\n**Micro-reto:**\n1. Definí `gcd(a, b)`\n2. Imprimí `gcd(48, 18)` (esperado: `6`)",
    starter_code: "# def gcd(a, b):\n#     ...\n# print(gcd(48, 18))\n",
    pytest: "def test_euclidean(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('gcd'))\n    assert ns['gcd'](48, 18) == 6\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['6']\n",
    hint: "def gcd(a, b):\n    while b:\n        a, b = b, a % b\n    return a\nprint(gcd(48, 18))",
    solution_example: "def gcd(a, b):\n    while b:\n        a, b = b, a % b\n    return a\nprint(gcd(48, 18))\n",
    next: Some("py-124-huffman-cost"),
    show_type_chips: false,
    micro_step: 123,
};

pub const PY124_HUFFMAN_COST: CodingStep = CodingStep {
    id: "py-124-huffman-cost",
    title: "DSA Huffman Intro",
    objective: "Sumar el costo de fusionar frecuencias (intro Huffman).",
    prompt_md: "**Huffman (intro)**\n\nSacá los dos menores, sumalos, reinsertá; el costo es la suma de cada fusión.\n\n**Micro-reto:**\n1. Definí `huffman_cost(freqs)` usando `heapq`\n2. Imprimí `huffman_cost([1, 1, 1, 1])` (esperado: `8`)",
    starter_code: "# import heapq\n# def huffman_cost(freqs):\n#     ...\n# print(huffman_cost([1, 1, 1, 1]))\n",
    pytest: "def test_huffman_cost(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('huffman_cost'))\n    assert ns['huffman_cost']([1, 1, 1, 1]) == 8\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['8']\n",
    hint: "import heapq\ndef huffman_cost(freqs):\n    h = list(freqs)\n    heapq.heapify(h)\n    cost = 0\n    while len(h) > 1:\n        a = heapq.heappop(h)\n        b = heapq.heappop(h)\n        s = a + b\n        cost += s\n        heapq.heappush(h, s)\n    return cost\nprint(huffman_cost([1, 1, 1, 1]))",
    solution_example: "import heapq\ndef huffman_cost(freqs):\n    h = list(freqs)\n    heapq.heapify(h)\n    cost = 0\n    while len(h) > 1:\n        a = heapq.heappop(h)\n        b = heapq.heappop(h)\n        s = a + b\n        cost += s\n        heapq.heappush(h, s)\n    return cost\nprint(huffman_cost([1, 1, 1, 1]))\n",
    next: Some("py-125-greedy-coin"),
    show_type_chips: false,
    micro_step: 124,
};

pub const PY125_GREEDY_COIN: CodingStep = CodingStep {
    id: "py-125-greedy-coin",
    title: "DSA Greedy Coin",
    objective: "Cambiar monto con monedas greedy (denominaciones canónicas).",
    prompt_md: "**Greedy Coin Change**\n\nTomá siempre la moneda más grande que quepa.\n\n**Micro-reto:**\n1. Definí `greedy_coin(coins, amount)` que devuelva la cantidad de monedas\n2. Imprimí `greedy_coin([25, 10, 5, 1], 63)` (esperado: `6`)",
    starter_code: "# def greedy_coin(coins, amount):\n#     ...\n# print(greedy_coin([25, 10, 5, 1], 63))\n",
    pytest: "def test_greedy_coin(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('greedy_coin'))\n    assert ns['greedy_coin']([25, 10, 5, 1], 63) == 6\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['6']\n",
    hint: "def greedy_coin(coins, amount):\n    coins = sorted(coins, reverse=True)\n    count = 0\n    for c in coins:\n        count += amount // c\n        amount %= c\n    return count\nprint(greedy_coin([25, 10, 5, 1], 63))",
    solution_example: "def greedy_coin(coins, amount):\n    coins = sorted(coins, reverse=True)\n    count = 0\n    for c in coins:\n        count += amount // c\n        amount %= c\n    return count\nprint(greedy_coin([25, 10, 5, 1], 63))\n",
    next: Some("py-126-activity-select"),
    show_type_chips: false,
    micro_step: 125,
};

pub const PY126_ACTIVITY_SELECT: CodingStep = CodingStep {
    id: "py-126-activity-select",
    title: "DSA Activity Selection",
    objective: "Elegir el máximo de actividades sin solapamiento (greedy por fin).",
    prompt_md: "**Activity Selection**\n\nOrdená por tiempo de fin y tomá la siguiente que empiece ≥ fin actual.\n\n**Micro-reto:**\n1. Definí `activity_select(intervals)` → lista de pares elegidos\n2. Con `[(1,4),(3,5),(0,6),(5,7),(8,9),(5,9)]` imprimí el resultado (esperado: `[(1, 4), (5, 7), (8, 9)]`)",
    starter_code: "# def activity_select(intervals):\n#     ...\n# print(activity_select([(1, 4), (3, 5), (0, 6), (5, 7), (8, 9), (5, 9)]))\n",
    pytest: "def test_activity_select(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('activity_select'))\n    assert ns['activity_select']([(1, 4), (3, 5), (0, 6), (5, 7), (8, 9), (5, 9)]) == [(1, 4), (5, 7), (8, 9)]\n    out = ' '.join(capsys.readouterr().out.split())\n    assert '(1, 4)' in out and '(5, 7)' in out and '(8, 9)' in out\n",
    hint: "def activity_select(intervals):\n    intervals = sorted(intervals, key=lambda x: x[1])\n    picked = []\n    end = -1\n    for s, e in intervals:\n        if s >= end:\n            picked.append((s, e))\n            end = e\n    return picked\nprint(activity_select([(1, 4), (3, 5), (0, 6), (5, 7), (8, 9), (5, 9)]))",
    solution_example: "def activity_select(intervals):\n    intervals = sorted(intervals, key=lambda x: x[1])\n    picked = []\n    end = -1\n    for s, e in intervals:\n        if s >= end:\n            picked.append((s, e))\n            end = e\n    return picked\nprint(activity_select([(1, 4), (3, 5), (0, 6), (5, 7), (8, 9), (5, 9)]))\n",
    next: Some("py-127-tsp-nearest"),
    show_type_chips: false,
    micro_step: 126,
};

pub const PY127_TSP_NEAREST: CodingStep = CodingStep {
    id: "py-127-tsp-nearest",
    title: "DSA TSP Nearest Neighbor",
    objective: "Aproximar TSP con nearest-neighbor (greedy).",
    prompt_md: "**Traveling Salesman (nearest neighbor)**\n\nDesde 0, siempre visitá la ciudad no visitada más cercana y volvé al origen.\n\n**Micro-reto:**\n1. Matrix 4×4 como en el hint\n2. Definí `nearest_neighbor_tsp(distances)` → `(route, total)`\n3. Imprimí `total` (esperado: `33`)",
    starter_code: "# distances = [...]\n# def nearest_neighbor_tsp(distances):\n#     ...\n# route, total = nearest_neighbor_tsp(distances)\n# print(total)\n",
    pytest: "def test_tsp_nearest(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('nearest_neighbor_tsp'))\n    route, total = ns['nearest_neighbor_tsp'](ns['distances'])\n    assert total == 33\n    assert route[0] == 0 and route[-1] == 0\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['33']\n",
    hint: "distances = [[0, 2, 9, 10], [1, 0, 6, 4], [15, 7, 0, 8], [6, 3, 12, 0]]\ndef nearest_neighbor_tsp(distances):\n    n = len(distances)\n    visited = [False] * n\n    route = [0]\n    visited[0] = True\n    total = 0\n    for _ in range(1, n):\n        last = route[-1]\n        nearest = min((i for i in range(n) if not visited[i]), key=lambda i: distances[last][i])\n        total += distances[last][nearest]\n        route.append(nearest)\n        visited[nearest] = True\n    total += distances[route[-1]][0]\n    route.append(0)\n    return route, total\nroute, total = nearest_neighbor_tsp(distances)\nprint(total)",
    solution_example: "distances = [[0, 2, 9, 10], [1, 0, 6, 4], [15, 7, 0, 8], [6, 3, 12, 0]]\ndef nearest_neighbor_tsp(distances):\n    n = len(distances)\n    visited = [False] * n\n    route = [0]\n    visited[0] = True\n    total = 0\n    for _ in range(1, n):\n        last = route[-1]\n        nearest = min((i for i in range(n) if not visited[i]), key=lambda i: distances[last][i])\n        total += distances[last][nearest]\n        route.append(nearest)\n        visited[nearest] = True\n    total += distances[route[-1]][0]\n    route.append(0)\n    return route, total\nroute, total = nearest_neighbor_tsp(distances)\nprint(total)\n",
    next: Some("py-128-lcs"),
    show_type_chips: false,
    micro_step: 127,
};

pub const PY128_LCS: CodingStep = CodingStep {
    id: "py-128-lcs",
    title: "DSA LCS",
    objective: "Calcular la longitud de la Longest Common Subsequence.",
    prompt_md: "**Longest Common Subsequence**\n\nDP: si `a[i]==b[j]` sumá 1 al diagonal; si no, tomá el max del vecino.\n\n**Micro-reto:**\n1. Definí `lcs(a, b)` → longitud\n2. Imprimí `lcs('ABCBDAB', 'BDCABA')` (esperado: `4`)",
    starter_code: "# def lcs(a, b):\n#     ...\n# print(lcs('ABCBDAB', 'BDCABA'))\n",
    pytest: "def test_lcs(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('lcs'))\n    assert ns['lcs']('ABCBDAB', 'BDCABA') == 4\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['4']\n",
    hint: "def lcs(a, b):\n    m, n = len(a), len(b)\n    dp = [[0] * (n + 1) for _ in range(m + 1)]\n    for i in range(1, m + 1):\n        for j in range(1, n + 1):\n            if a[i - 1] == b[j - 1]:\n                dp[i][j] = dp[i - 1][j - 1] + 1\n            else:\n                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])\n    return dp[m][n]\nprint(lcs('ABCBDAB', 'BDCABA'))",
    solution_example: "def lcs(a, b):\n    m, n = len(a), len(b)\n    dp = [[0] * (n + 1) for _ in range(m + 1)]\n    for i in range(1, m + 1):\n        for j in range(1, n + 1):\n            if a[i - 1] == b[j - 1]:\n                dp[i][j] = dp[i - 1][j - 1] + 1\n            else:\n                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1])\n    return dp[m][n]\nprint(lcs('ABCBDAB', 'BDCABA'))\n",
    next: Some("py-129-coin-change-dp"),
    show_type_chips: false,
    micro_step: 128,
};

pub const PY129_COIN_CHANGE_DP: CodingStep = CodingStep {
    id: "py-129-coin-change-dp",
    title: "DSA Coin Change DP",
    objective: "Mínimo de monedas con DP (denominaciones no canónicas).",
    prompt_md: "**Coin Change (DP)**\n\n`dp[a] = min(dp[a - c] + 1)` para cada moneda.\n\n**Micro-reto:**\n1. Definí `coin_change(coins, amount)`\n2. Imprimí `coin_change([1, 3, 4], 6)` (esperado: `2`)",
    starter_code: "# def coin_change(coins, amount):\n#     ...\n# print(coin_change([1, 3, 4], 6))\n",
    pytest: "def test_coin_change_dp(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('coin_change'))\n    assert ns['coin_change']([1, 3, 4], 6) == 2\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['2']\n",
    hint: "def coin_change(coins, amount):\n    dp = [0] + [float('inf')] * amount\n    for a in range(1, amount + 1):\n        for c in coins:\n            if c <= a:\n                dp[a] = min(dp[a], dp[a - c] + 1)\n    return int(dp[amount]) if dp[amount] != float('inf') else -1\nprint(coin_change([1, 3, 4], 6))",
    solution_example: "def coin_change(coins, amount):\n    dp = [0] + [float('inf')] * amount\n    for a in range(1, amount + 1):\n        for c in coins:\n            if c <= a:\n                dp[a] = min(dp[a], dp[a - c] + 1)\n    return int(dp[amount]) if dp[amount] != float('inf') else -1\nprint(coin_change([1, 3, 4], 6))\n",
    next: Some("py-130-floyd-warshall"),
    show_type_chips: false,
    micro_step: 129,
};

pub const PY130_FLOYD_WARSHALL: CodingStep = CodingStep {
    id: "py-130-floyd-warshall",
    title: "DSA Floyd-Warshall",
    objective: "Calcular todas las distancias mínimas entre pares (APSP).",
    prompt_md: "**Floyd-Warshall**\n\nPara cada `k`, relajá `dist[i][j]` vía `k`.\n\n**Micro-reto:**\n1. Matriz `INF=999` como en el hint\n2. Definí `floyd(graph)` → matriz de distancias\n3. Imprimí la fila `0` (esperado: `[0, 3, 5, 6]`)",
    starter_code: "# INF = 999\n# graph = [...]\n# def floyd(graph):\n#     ...\n# print(floyd(graph)[0])\n",
    pytest: "def test_floyd_warshall(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('floyd'))\n    assert ns['floyd'](ns['graph'])[0] == [0, 3, 5, 6]\n    out = ' '.join(capsys.readouterr().out.split())\n    assert '[0, 3, 5, 6]' in out\n",
    hint: "INF = 999\ngraph = [[0, 3, INF, 7], [8, 0, 2, INF], [5, INF, 0, 1], [2, INF, INF, 0]]\ndef floyd(graph):\n    n = len(graph)\n    dist = [row[:] for row in graph]\n    for k in range(n):\n        for i in range(n):\n            for j in range(n):\n                if dist[i][k] + dist[k][j] < dist[i][j]:\n                    dist[i][j] = dist[i][k] + dist[k][j]\n    return dist\nprint(floyd(graph)[0])",
    solution_example: "INF = 999\ngraph = [[0, 3, INF, 7], [8, 0, 2, INF], [5, INF, 0, 1], [2, INF, INF, 0]]\ndef floyd(graph):\n    n = len(graph)\n    dist = [row[:] for row in graph]\n    for k in range(n):\n        for i in range(n):\n            for j in range(n):\n                if dist[i][k] + dist[k][j] < dist[i][j]:\n                    dist[i][j] = dist[i][k] + dist[k][j]\n    return dist\nprint(floyd(graph)[0])\n",
    next: Some("py-131-two-pointers"),
    show_type_chips: false,
    micro_step: 130,
};

pub const PY131_TWO_POINTERS: CodingStep = CodingStep {
    id: "py-131-two-pointers",
    title: "DSA Two Pointers",
    objective: "Encontrar un par con suma objetivo en una lista ordenada.",
    prompt_md: "**Two Pointers**\n\nEn array ordenado, mové left/right según la suma.\n\n**Micro-reto:**\n1. Definí `two_sum(nums, target)` → tupla de índices o `None`\n2. Imprimí `two_sum([2, 7, 11, 15], 9)` (esperado: `(0, 1)`)",
    starter_code: "# def two_sum(nums, target):\n#     ...\n# print(two_sum([2, 7, 11, 15], 9))\n",
    pytest: "def test_two_pointers(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('two_sum'))\n    assert ns['two_sum']([2, 7, 11, 15], 9) == (0, 1)\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['(0, 1)']\n",
    hint: "def two_sum(nums, target):\n    left, right = 0, len(nums) - 1\n    while left < right:\n        s = nums[left] + nums[right]\n        if s == target:\n            return (left, right)\n        if s < target:\n            left += 1\n        else:\n            right -= 1\n    return None\nprint(two_sum([2, 7, 11, 15], 9))",
    solution_example: "def two_sum(nums, target):\n    left, right = 0, len(nums) - 1\n    while left < right:\n        s = nums[left] + nums[right]\n        if s == target:\n            return (left, right)\n        if s < target:\n            left += 1\n        else:\n            right -= 1\n    return None\nprint(two_sum([2, 7, 11, 15], 9))\n",
    next: Some("py-132-sliding-window"),
    show_type_chips: false,
    micro_step: 131,
};

pub const PY132_SLIDING_WINDOW: CodingStep = CodingStep {
    id: "py-132-sliding-window",
    title: "DSA Sliding Window",
    objective: "Máxima suma de una ventana fija de tamaño k.",
    prompt_md: "**Sliding Window**\n\nMantené la suma de k elementos y avanzá restando el que sale.\n\n**Micro-reto:**\n1. Definí `max_window(nums, k)`\n2. Imprimí `max_window([2, 1, 5, 1, 3, 2], 3)` (esperado: `9`)",
    starter_code: "# def max_window(nums, k):\n#     ...\n# print(max_window([2, 1, 5, 1, 3, 2], 3))\n",
    pytest: "def test_sliding_window(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('max_window'))\n    assert ns['max_window']([2, 1, 5, 1, 3, 2], 3) == 9\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['9']\n",
    hint: "def max_window(nums, k):\n    window = sum(nums[:k])\n    best = window\n    for i in range(k, len(nums)):\n        window += nums[i] - nums[i - k]\n        best = max(best, window)\n    return best\nprint(max_window([2, 1, 5, 1, 3, 2], 3))",
    solution_example: "def max_window(nums, k):\n    window = sum(nums[:k])\n    best = window\n    for i in range(k, len(nums)):\n        window += nums[i] - nums[i - k]\n        best = max(best, window)\n    return best\nprint(max_window([2, 1, 5, 1, 3, 2], 3))\n",
    next: Some("py-133-permutations"),
    show_type_chips: false,
    micro_step: 132,
};

pub const PY133_PERMUTATIONS: CodingStep = CodingStep {
    id: "py-133-permutations",
    title: "DSA Permutations",
    objective: "Generar permutaciones con backtracking.",
    prompt_md: "**Backtracking · Permutations**\n\nConstruí el path y backtrackeá al sacar el último elemento.\n\n**Micro-reto:**\n1. Definí `permute(nums)` → lista de permutaciones\n2. Imprimí `sorted(permute([1, 2, 3]))` (esperado: `[[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]`)",
    starter_code: "# def permute(nums):\n#     ...\n# print(sorted(permute([1, 2, 3])))\n",
    pytest: "def test_permutations(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('permute'))\n    assert sorted(ns['permute']([1, 2, 3])) == [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]\n    out = ' '.join(capsys.readouterr().out.split())\n    assert '[1, 2, 3]' in out and '[3, 2, 1]' in out\n",
    hint: "def permute(nums):\n    res = []\n    def bt(path, unused):\n        if not unused:\n            res.append(path[:])\n            return\n        for i, x in enumerate(unused):\n            path.append(x)\n            bt(path, unused[:i] + unused[i + 1:])\n            path.pop()\n    bt([], list(nums))\n    return res\nprint(sorted(permute([1, 2, 3])))",
    solution_example: "def permute(nums):\n    res = []\n    def bt(path, unused):\n        if not unused:\n            res.append(path[:])\n            return\n        for i, x in enumerate(unused):\n            path.append(x)\n            bt(path, unused[:i] + unused[i + 1:])\n            path.pop()\n    bt([], list(nums))\n    return res\nprint(sorted(permute([1, 2, 3])))\n",
    next: Some("py-134-nqueens-count"),
    show_type_chips: false,
    micro_step: 133,
};

pub const PY134_NQUEENS_COUNT: CodingStep = CodingStep {
    id: "py-134-nqueens-count",
    title: "DSA N-Queens Count",
    objective: "Contar soluciones al problema de las N reinas.",
    prompt_md: "**N-Queens**\n\nColocá n reinas sin atacarse; contá soluciones válidas.\n\n**Micro-reto:**\n1. Definí `nqueens_count(n)`\n2. Imprimí `nqueens_count(4)` (esperado: `2`)",
    starter_code: "# def nqueens_count(n):\n#     ...\n# print(nqueens_count(4))\n",
    pytest: "def test_nqueens_count(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('nqueens_count'))\n    assert ns['nqueens_count'](4) == 2\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['2']\n",
    hint: "def nqueens_count(n):\n    cols = set()\n    diag1 = set()\n    diag2 = set()\n    def bt(row):\n        if row == n:\n            return 1\n        total = 0\n        for c in range(n):\n            if c in cols or row - c in diag1 or row + c in diag2:\n                continue\n            cols.add(c); diag1.add(row - c); diag2.add(row + c)\n            total += bt(row + 1)\n            cols.remove(c); diag1.remove(row - c); diag2.remove(row + c)\n        return total\n    return bt(0)\nprint(nqueens_count(4))",
    solution_example: "def nqueens_count(n):\n    cols = set()\n    diag1 = set()\n    diag2 = set()\n    def bt(row):\n        if row == n:\n            return 1\n        total = 0\n        for c in range(n):\n            if c in cols or row - c in diag1 or row + c in diag2:\n                continue\n            cols.add(c); diag1.add(row - c); diag2.add(row + c)\n            total += bt(row + 1)\n            cols.remove(c); diag1.remove(row - c); diag2.remove(row + c)\n        return total\n    return bt(0)\nprint(nqueens_count(4))\n",
    next: Some("py-135-trie"),
    show_type_chips: false,
    micro_step: 134,
};

pub const PY135_TRIE: CodingStep = CodingStep {
    id: "py-135-trie",
    title: "DSA Trie",
    objective: "Insertar y buscar palabras en un Trie.",
    prompt_md: "**Trie (prefix tree)**\n\nCada nodo es un dict de hijos; `end` marca fin de palabra.\n\n**Micro-reto:**\n1. Definí `Trie` con `insert(word)` y `search(word)`\n2. Insertá `cat` y `car`; imprimí `search('cat')`, `search('car')`, `search('cap')`",
    starter_code: "# class Trie:\n#     ...\n# t = Trie()\n# ...\n# print(...)\n",
    pytest: "def test_trie(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert 't' in ns\n    assert ns['t'].search('cat') is True\n    assert ns['t'].search('car') is True\n    assert ns['t'].search('cap') is False\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['True', 'True', 'False']\n",
    hint: "class Trie:\n    def __init__(self):\n        self.root = {}\n    def insert(self, word):\n        node = self.root\n        for ch in word:\n            node = node.setdefault(ch, {})\n        node['#'] = True\n    def search(self, word):\n        node = self.root\n        for ch in word:\n            if ch not in node:\n                return False\n            node = node[ch]\n        return '#' in node\nt = Trie()\nt.insert('cat')\nt.insert('car')\nprint(t.search('cat'))\nprint(t.search('car'))\nprint(t.search('cap'))",
    solution_example: "class Trie:\n    def __init__(self):\n        self.root = {}\n    def insert(self, word):\n        node = self.root\n        for ch in word:\n            node = node.setdefault(ch, {})\n        node['#'] = True\n    def search(self, word):\n        node = self.root\n        for ch in word:\n            if ch not in node:\n                return False\n            node = node[ch]\n        return '#' in node\nt = Trie()\nt.insert('cat')\nt.insert('car')\nprint(t.search('cat'))\nprint(t.search('car'))\nprint(t.search('cap'))\n",
    next: Some("py-136-bit-count"),
    show_type_chips: false,
    micro_step: 135,
};

pub const PY136_BIT_COUNT: CodingStep = CodingStep {
    id: "py-136-bit-count",
    title: "DSA Bit Count",
    objective: "Contar bits en 1 con Brian Kernighan (n &= n-1).",
    prompt_md: "**Bit Count**\n\nCada `n = n & (n - 1)` apaga el bit 1 menos significativo.\n\n**Micro-reto:**\n1. Definí `bit_count(n)`\n2. Imprimí `bit_count(13)` (esperado: `3`, porque `1101`)",
    starter_code: "# def bit_count(n):\n#     ...\n# print(bit_count(13))\n",
    pytest: "def test_bit_count(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('bit_count'))\n    assert ns['bit_count'](13) == 3\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['3']\n",
    hint: "def bit_count(n):\n    count = 0\n    while n:\n        n &= n - 1\n        count += 1\n    return count\nprint(bit_count(13))",
    solution_example: "def bit_count(n):\n    count = 0\n    while n:\n        n &= n - 1\n        count += 1\n    return count\nprint(bit_count(13))\n",
    next: Some("py-137-kadane"),
    show_type_chips: false,
    micro_step: 136,
};

pub const PY137_KADANE: CodingStep = CodingStep {
    id: "py-137-kadane",
    title: "DSA Kadane",
    objective: "Máxima suma de subarray contiguo (Kadane).",
    prompt_md: "**Kadane**\n\nMantené `best` y `cur`; si `cur` cae bajo 0, reiniciá.\n\n**Micro-reto:**\n1. Definí `max_subarray(nums)`\n2. Imprimí `max_subarray([-2, 1, -3, 4, -1, 2, 1, -5, 4])` (esperado: `6`)",
    starter_code: "# def max_subarray(nums):\n#     ...\n# print(max_subarray([-2, 1, -3, 4, -1, 2, 1, -5, 4]))\n",
    pytest: "def test_kadane(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('max_subarray'))\n    assert ns['max_subarray']([-2, 1, -3, 4, -1, 2, 1, -5, 4]) == 6\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['6']\n",
    hint: "def max_subarray(nums):\n    best = cur = nums[0]\n    for x in nums[1:]:\n        cur = max(x, cur + x)\n        best = max(best, cur)\n    return best\nprint(max_subarray([-2, 1, -3, 4, -1, 2, 1, -5, 4]))",
    solution_example: "def max_subarray(nums):\n    best = cur = nums[0]\n    for x in nums[1:]:\n        cur = max(x, cur + x)\n        best = max(best, cur)\n    return best\nprint(max_subarray([-2, 1, -3, 4, -1, 2, 1, -5, 4]))\n",
    next: Some("py-138-merge-intervals"),
    show_type_chips: false,
    micro_step: 137,
};

pub const PY138_MERGE_INTERVALS: CodingStep = CodingStep {
    id: "py-138-merge-intervals",
    title: "DSA Merge Intervals",
    objective: "Fusionar intervalos solapados.",
    prompt_md: "**Merge Intervals**\n\nOrdená por inicio y extendé el último si se solapa.\n\n**Micro-reto:**\n1. Definí `merge_intervals(intervals)`\n2. Imprimí `merge_intervals([[1,3],[2,6],[8,10],[15,18]])` (esperado: `[[1, 6], [8, 10], [15, 18]]`)",
    starter_code: "# def merge_intervals(intervals):\n#     ...\n# print(merge_intervals([[1, 3], [2, 6], [8, 10], [15, 18]]))\n",
    pytest: "def test_merge_intervals(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('merge_intervals'))\n    assert ns['merge_intervals']([[1, 3], [2, 6], [8, 10], [15, 18]]) == [[1, 6], [8, 10], [15, 18]]\n    out = ' '.join(capsys.readouterr().out.split())\n    assert '[1, 6]' in out and '[8, 10]' in out and '[15, 18]' in out\n",
    hint: "def merge_intervals(intervals):\n    intervals = sorted(intervals, key=lambda x: x[0])\n    out = [intervals[0][:]]\n    for s, e in intervals[1:]:\n        if s <= out[-1][1]:\n            out[-1][1] = max(out[-1][1], e)\n        else:\n            out.append([s, e])\n    return out\nprint(merge_intervals([[1, 3], [2, 6], [8, 10], [15, 18]]))",
    solution_example: "def merge_intervals(intervals):\n    intervals = sorted(intervals, key=lambda x: x[0])\n    out = [intervals[0][:]]\n    for s, e in intervals[1:]:\n        if s <= out[-1][1]:\n            out[-1][1] = max(out[-1][1], e)\n        else:\n            out.append([s, e])\n    return out\nprint(merge_intervals([[1, 3], [2, 6], [8, 10], [15, 18]]))\n",
    next: Some("py-139-lower-bound"),
    show_type_chips: false,
    micro_step: 138,
};

pub const PY139_LOWER_BOUND: CodingStep = CodingStep {
    id: "py-139-lower-bound",
    title: "DSA Lower Bound",
    objective: "Primer índice donde nums[i] >= target (binary search).",
    prompt_md: "**Lower Bound**\n\nBinary search del primer elemento ≥ target.\n\n**Micro-reto:**\n1. Definí `lower_bound(nums, target)`\n2. Imprimí `lower_bound([1, 3, 3, 5, 7], 3)` (esperado: `1`)",
    starter_code: "# def lower_bound(nums, target):\n#     ...\n# print(lower_bound([1, 3, 3, 5, 7], 3))\n",
    pytest: "def test_lower_bound(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('lower_bound'))\n    assert ns['lower_bound']([1, 3, 3, 5, 7], 3) == 1\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['1']\n",
    hint: "def lower_bound(nums, target):\n    lo, hi = 0, len(nums)\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if nums[mid] < target:\n            lo = mid + 1\n        else:\n            hi = mid\n    return lo\nprint(lower_bound([1, 3, 3, 5, 7], 3))",
    solution_example: "def lower_bound(nums, target):\n    lo, hi = 0, len(nums)\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if nums[mid] < target:\n            lo = mid + 1\n        else:\n            hi = mid\n    return lo\nprint(lower_bound([1, 3, 3, 5, 7], 3))\n",
    next: Some("py-140-rotate-matrix"),
    show_type_chips: false,
    micro_step: 139,
};

pub const PY140_ROTATE_MATRIX: CodingStep = CodingStep {
    id: "py-140-rotate-matrix",
    title: "DSA Rotate Matrix",
    objective: "Rotar una matriz cuadrada 90° en sentido horario.",
    prompt_md: "**Rotate Matrix**\n\nTraspuesta + reverse de cada fila = rotación 90° CW.\n\n**Micro-reto:**\n1. Definí `rotate(matrix)` in-place (modifica y también devolvé)\n2. Con `[[1,2,3],[4,5,6],[7,8,9]]` imprimí el resultado (esperado: `[[7, 4, 1], [8, 5, 2], [9, 6, 3]]`)",
    starter_code: "# def rotate(matrix):\n#     ...\n# m = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]\n# print(rotate(m))\n",
    pytest: "def test_rotate_matrix(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('rotate'))\n    assert ns['m'] == [[7, 4, 1], [8, 5, 2], [9, 6, 3]]\n    out = ' '.join(capsys.readouterr().out.split())\n    assert '[7, 4, 1]' in out and '[9, 6, 3]' in out\n",
    hint: "def rotate(matrix):\n    n = len(matrix)\n    for i in range(n):\n        for j in range(i + 1, n):\n            matrix[i][j], matrix[j][i] = matrix[j][i], matrix[i][j]\n    for row in matrix:\n        row.reverse()\n    return matrix\nm = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]\nprint(rotate(m))",
    solution_example: "def rotate(matrix):\n    n = len(matrix)\n    for i in range(n):\n        for j in range(i + 1, n):\n            matrix[i][j], matrix[j][i] = matrix[j][i], matrix[i][j]\n    for row in matrix:\n        row.reverse()\n    return matrix\nm = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]\nprint(rotate(m))\n",
    next: Some("py-141-valid-parens"),
    show_type_chips: false,
    micro_step: 140,
};

pub const PY141_VALID_PARENS: CodingStep = CodingStep {
    id: "py-141-valid-parens",
    title: "DSA Valid Parentheses",
    objective: "Validar paréntesis/brackets balanceados con stack.",
    prompt_md: "**Valid Parentheses**\n\nPush opens; pop must match el cierre.\n\n**Micro-reto:**\n1. Definí `valid_parens(s)` → bool\n2. Imprimí `valid_parens('()[]{}')` y `valid_parens('(]')` (esperado: `True` / `False`)",
    starter_code: "# def valid_parens(s):\n#     ...\n# print(valid_parens('()[]{}'))\n# print(valid_parens('(]'))\n",
    pytest: "def test_valid_parens(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('valid_parens'))\n    assert ns['valid_parens']('()[]{}') is True\n    assert ns['valid_parens']('(]') is False\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['True', 'False']\n",
    hint: "def valid_parens(s):\n    pairs = {')': '(', ']': '[', '}': '{'}\n    stack = []\n    for ch in s:\n        if ch in '([{':\n            stack.append(ch)\n        elif not stack or stack.pop() != pairs[ch]:\n            return False\n    return not stack\nprint(valid_parens('()[]{}'))\nprint(valid_parens('(]'))",
    solution_example: "def valid_parens(s):\n    pairs = {')': '(', ']': '[', '}': '{'}\n    stack = []\n    for ch in s:\n        if ch in '([{':\n            stack.append(ch)\n        elif not stack or stack.pop() != pairs[ch]:\n            return False\n    return not stack\nprint(valid_parens('()[]{}'))\nprint(valid_parens('(]'))\n",
    next: Some("py-142-anagram"),
    show_type_chips: false,
    micro_step: 141,
};

pub const PY142_ANAGRAM: CodingStep = CodingStep {
    id: "py-142-anagram",
    title: "DSA Anagram Check",
    objective: "Decidir si dos strings son anagramas (conteo).",
    prompt_md: "**Anagram**\n\nMisma frecuencia de caracteres.\n\n**Micro-reto:**\n1. Definí `is_anagram(a, b)`\n2. Imprimí `is_anagram('listen', 'silent')` y `is_anagram('hello', 'world')`",
    starter_code: "# def is_anagram(a, b):\n#     ...\n# print(is_anagram('listen', 'silent'))\n# print(is_anagram('hello', 'world'))\n",
    pytest: "def test_anagram(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('is_anagram'))\n    assert ns['is_anagram']('listen', 'silent') is True\n    assert ns['is_anagram']('hello', 'world') is False\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['True', 'False']\n",
    hint: "from collections import Counter\ndef is_anagram(a, b):\n    return Counter(a) == Counter(b)\nprint(is_anagram('listen', 'silent'))\nprint(is_anagram('hello', 'world'))",
    solution_example: "from collections import Counter\ndef is_anagram(a, b):\n    return Counter(a) == Counter(b)\nprint(is_anagram('listen', 'silent'))\nprint(is_anagram('hello', 'world'))\n",
    next: Some("py-143-climb-stairs"),
    show_type_chips: false,
    micro_step: 142,
};

pub const PY143_CLIMB_STAIRS: CodingStep = CodingStep {
    id: "py-143-climb-stairs",
    title: "DSA Climb Stairs",
    objective: "Contar formas de subir n escalones (1 o 2).",
    prompt_md: "**Climbing Stairs**\n\n`ways(n) = ways(n-1) + ways(n-2)`.\n\n**Micro-reto:**\n1. Definí `climb_stairs(n)`\n2. Imprimí `climb_stairs(5)` (esperado: `8`)",
    starter_code: "# def climb_stairs(n):\n#     ...\n# print(climb_stairs(5))\n",
    pytest: "def test_climb_stairs(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('climb_stairs'))\n    assert ns['climb_stairs'](5) == 8\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['8']\n",
    hint: "def climb_stairs(n):\n    if n <= 2:\n        return n\n    a, b = 1, 2\n    for _ in range(3, n + 1):\n        a, b = b, a + b\n    return b\nprint(climb_stairs(5))",
    solution_example: "def climb_stairs(n):\n    if n <= 2:\n        return n\n    a, b = 1, 2\n    for _ in range(3, n + 1):\n        a, b = b, a + b\n    return b\nprint(climb_stairs(5))\n",
    next: Some("py-144-house-robber"),
    show_type_chips: false,
    micro_step: 143,
};

pub const PY144_HOUSE_ROBBER: CodingStep = CodingStep {
    id: "py-144-house-robber",
    title: "DSA House Robber",
    objective: "Máximo botín sin robar casas adyacentes.",
    prompt_md: "**House Robber**\n\n`dp[i] = max(dp[i-1], dp[i-2] + nums[i])`.\n\n**Micro-reto:**\n1. Definí `rob(nums)`\n2. Imprimí `rob([2, 7, 9, 3, 1])` (esperado: `12`)",
    starter_code: "# def rob(nums):\n#     ...\n# print(rob([2, 7, 9, 3, 1]))\n",
    pytest: "def test_house_robber(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('rob'))\n    assert ns['rob']([2, 7, 9, 3, 1]) == 12\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['12']\n",
    hint: "def rob(nums):\n    prev2 = prev1 = 0\n    for x in nums:\n        prev2, prev1 = prev1, max(prev1, prev2 + x)\n    return prev1\nprint(rob([2, 7, 9, 3, 1]))",
    solution_example: "def rob(nums):\n    prev2 = prev1 = 0\n    for x in nums:\n        prev2, prev1 = prev1, max(prev1, prev2 + x)\n    return prev1\nprint(rob([2, 7, 9, 3, 1]))\n",
    next: Some("py-145-unique-paths"),
    show_type_chips: false,
    micro_step: 144,
};

pub const PY145_UNIQUE_PATHS: CodingStep = CodingStep {
    id: "py-145-unique-paths",
    title: "DSA Unique Paths",
    objective: "Caminos en grilla m×n solo derecha/abajo.",
    prompt_md: "**Unique Paths**\n\n`dp[i][j] = dp[i-1][j] + dp[i][j-1]`.\n\n**Micro-reto:**\n1. Definí `unique_paths(m, n)`\n2. Imprimí `unique_paths(3, 7)` (esperado: `28`)",
    starter_code: "# def unique_paths(m, n):\n#     ...\n# print(unique_paths(3, 7))\n",
    pytest: "def test_unique_paths(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('unique_paths'))\n    assert ns['unique_paths'](3, 7) == 28\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['28']\n",
    hint: "def unique_paths(m, n):\n    dp = [[1] * n for _ in range(m)]\n    for i in range(1, m):\n        for j in range(1, n):\n            dp[i][j] = dp[i - 1][j] + dp[i][j - 1]\n    return dp[-1][-1]\nprint(unique_paths(3, 7))",
    solution_example: "def unique_paths(m, n):\n    dp = [[1] * n for _ in range(m)]\n    for i in range(1, m):\n        for j in range(1, n):\n            dp[i][j] = dp[i - 1][j] + dp[i][j - 1]\n    return dp[-1][-1]\nprint(unique_paths(3, 7))\n",
    next: Some("py-146-majority"),
    show_type_chips: false,
    micro_step: 145,
};

pub const PY146_MAJORITY: CodingStep = CodingStep {
    id: "py-146-majority",
    title: "DSA Majority Element",
    objective: "Encontrar el elemento mayoritario (Boyer-Moore).",
    prompt_md: "**Majority Element**\n\nVoto: si count==0 tomá candidato; ±1 según match.\n\n**Micro-reto:**\n1. Definí `majority(nums)`\n2. Imprimí `majority([2, 2, 1, 1, 1, 2, 2])` (esperado: `2`)",
    starter_code: "# def majority(nums):\n#     ...\n# print(majority([2, 2, 1, 1, 1, 2, 2]))\n",
    pytest: "def test_majority(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('majority'))\n    assert ns['majority']([2, 2, 1, 1, 1, 2, 2]) == 2\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['2']\n",
    hint: "def majority(nums):\n    cand = None\n    count = 0\n    for x in nums:\n        if count == 0:\n            cand = x\n        count += 1 if x == cand else -1\n    return cand\nprint(majority([2, 2, 1, 1, 1, 2, 2]))",
    solution_example: "def majority(nums):\n    cand = None\n    count = 0\n    for x in nums:\n        if count == 0:\n            cand = x\n        count += 1 if x == cand else -1\n    return cand\nprint(majority([2, 2, 1, 1, 1, 2, 2]))\n",
    next: Some("py-147-missing-number"),
    show_type_chips: false,
    micro_step: 146,
};

pub const PY147_MISSING_NUMBER: CodingStep = CodingStep {
    id: "py-147-missing-number",
    title: "DSA Missing Number",
    objective: "Hallar el faltante en 0..n con XOR.",
    prompt_md: "**Missing Number**\n\nXOR de índices y valores cancela pares; queda el faltante.\n\n**Micro-reto:**\n1. Definí `missing_number(nums)` para nums con n números de `0..n`\n2. Imprimí `missing_number([3, 0, 1])` (esperado: `2`)",
    starter_code: "# def missing_number(nums):\n#     ...\n# print(missing_number([3, 0, 1]))\n",
    pytest: "def test_missing_number(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('missing_number'))\n    assert ns['missing_number']([3, 0, 1]) == 2\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['2']\n",
    hint: "def missing_number(nums):\n    missing = len(nums)\n    for i, x in enumerate(nums):\n        missing ^= i ^ x\n    return missing\nprint(missing_number([3, 0, 1]))",
    solution_example: "def missing_number(nums):\n    missing = len(nums)\n    for i, x in enumerate(nums):\n        missing ^= i ^ x\n    return missing\nprint(missing_number([3, 0, 1]))\n",
    next: Some("py-148-single-number"),
    show_type_chips: false,
    micro_step: 147,
};

pub const PY148_SINGLE_NUMBER: CodingStep = CodingStep {
    id: "py-148-single-number",
    title: "DSA Single Number",
    objective: "Hallar el único no-duplicado con XOR.",
    prompt_md: "**Single Number**\n\n`a ^ a = 0`; XOR de todos deja el único.\n\n**Micro-reto:**\n1. Definí `single_number(nums)`\n2. Imprimí `single_number([4, 1, 2, 1, 2])` (esperado: `4`)",
    starter_code: "# def single_number(nums):\n#     ...\n# print(single_number([4, 1, 2, 1, 2]))\n",
    pytest: "def test_single_number(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('single_number'))\n    assert ns['single_number']([4, 1, 2, 1, 2]) == 4\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['4']\n",
    hint: "def single_number(nums):\n    x = 0\n    for n in nums:\n        x ^= n\n    return x\nprint(single_number([4, 1, 2, 1, 2]))",
    solution_example: "def single_number(nums):\n    x = 0\n    for n in nums:\n        x ^= n\n    return x\nprint(single_number([4, 1, 2, 1, 2]))\n",
    next: Some("py-149-lis"),
    show_type_chips: false,
    micro_step: 148,
};

pub const PY149_LIS: CodingStep = CodingStep {
    id: "py-149-lis",
    title: "DSA LIS Length",
    objective: "Longitud de la Longest Increasing Subsequence (DP O(n²)).",
    prompt_md: "**LIS**\n\n`dp[i] = 1 + max(dp[j])` para j < i con nums[j] < nums[i].\n\n**Micro-reto:**\n1. Definí `lis_length(nums)`\n2. Imprimí `lis_length([10, 9, 2, 5, 3, 7, 101, 18])` (esperado: `4`)",
    starter_code: "# def lis_length(nums):\n#     ...\n# print(lis_length([10, 9, 2, 5, 3, 7, 101, 18]))\n",
    pytest: "def test_lis(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('lis_length'))\n    assert ns['lis_length']([10, 9, 2, 5, 3, 7, 101, 18]) == 4\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['4']\n",
    hint: "def lis_length(nums):\n    if not nums:\n        return 0\n    dp = [1] * len(nums)\n    for i in range(len(nums)):\n        for j in range(i):\n            if nums[j] < nums[i]:\n                dp[i] = max(dp[i], dp[j] + 1)\n    return max(dp)\nprint(lis_length([10, 9, 2, 5, 3, 7, 101, 18]))",
    solution_example: "def lis_length(nums):\n    if not nums:\n        return 0\n    dp = [1] * len(nums)\n    for i in range(len(nums)):\n        for j in range(i):\n            if nums[j] < nums[i]:\n                dp[i] = max(dp[i], dp[j] + 1)\n    return max(dp)\nprint(lis_length([10, 9, 2, 5, 3, 7, 101, 18]))\n",
    next: Some("py-150-edit-distance"),
    show_type_chips: false,
    micro_step: 149,
};

pub const PY150_EDIT_DISTANCE: CodingStep = CodingStep {
    id: "py-150-edit-distance",
    title: "DSA Edit Distance",
    objective: "Distancia de Levenshtein (insert/delete/replace).",
    prompt_md: "**Edit Distance**\n\nDP clásico sobre dos strings.\n\n**Micro-reto:**\n1. Definí `edit_distance(a, b)`\n2. Imprimí `edit_distance('horse', 'ros')` (esperado: `3`)",
    starter_code: "# def edit_distance(a, b):\n#     ...\n# print(edit_distance('horse', 'ros'))\n",
    pytest: "def test_edit_distance(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('edit_distance'))\n    assert ns['edit_distance']('horse', 'ros') == 3\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['3']\n",
    hint: "def edit_distance(a, b):\n    m, n = len(a), len(b)\n    dp = [[0] * (n + 1) for _ in range(m + 1)]\n    for i in range(m + 1):\n        dp[i][0] = i\n    for j in range(n + 1):\n        dp[0][j] = j\n    for i in range(1, m + 1):\n        for j in range(1, n + 1):\n            if a[i - 1] == b[j - 1]:\n                dp[i][j] = dp[i - 1][j - 1]\n            else:\n                dp[i][j] = 1 + min(dp[i - 1][j], dp[i][j - 1], dp[i - 1][j - 1])\n    return dp[m][n]\nprint(edit_distance('horse', 'ros'))",
    solution_example: "def edit_distance(a, b):\n    m, n = len(a), len(b)\n    dp = [[0] * (n + 1) for _ in range(m + 1)]\n    for i in range(m + 1):\n        dp[i][0] = i\n    for j in range(n + 1):\n        dp[0][j] = j\n    for i in range(1, m + 1):\n        for j in range(1, n + 1):\n            if a[i - 1] == b[j - 1]:\n                dp[i][j] = dp[i - 1][j - 1]\n            else:\n                dp[i][j] = 1 + min(dp[i - 1][j], dp[i][j - 1], dp[i - 1][j - 1])\n    return dp[m][n]\nprint(edit_distance('horse', 'ros'))\n",
    next: Some("py-151-word-break"),
    show_type_chips: false,
    micro_step: 150,
};

pub const PY151_WORD_BREAK: CodingStep = CodingStep {
    id: "py-151-word-break",
    title: "DSA Word Break",
    objective: "¿Se puede segmentar s con palabras del dict?",
    prompt_md: "**Word Break**\n\n`dp[i]` true si algún corte j < i con dp[j] y s[j:i] en el dict.\n\n**Micro-reto:**\n1. Definí `word_break(s, word_dict)`\n2. Imprimí `word_break('leetcode', ['leet', 'code'])` (esperado: `True`)",
    starter_code: "# def word_break(s, word_dict):\n#     ...\n# print(word_break('leetcode', ['leet', 'code']))\n",
    pytest: "def test_word_break(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('word_break'))\n    assert ns['word_break']('leetcode', ['leet', 'code']) is True\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['True']\n",
    hint: "def word_break(s, word_dict):\n    words = set(word_dict)\n    dp = [False] * (len(s) + 1)\n    dp[0] = True\n    for i in range(1, len(s) + 1):\n        for j in range(i):\n            if dp[j] and s[j:i] in words:\n                dp[i] = True\n                break\n    return dp[-1]\nprint(word_break('leetcode', ['leet', 'code']))",
    solution_example: "def word_break(s, word_dict):\n    words = set(word_dict)\n    dp = [False] * (len(s) + 1)\n    dp[0] = True\n    for i in range(1, len(s) + 1):\n        for j in range(i):\n            if dp[j] and s[j:i] in words:\n                dp[i] = True\n                break\n    return dp[-1]\nprint(word_break('leetcode', ['leet', 'code']))\n",
    next: Some("py-152-min-path-sum"),
    show_type_chips: false,
    micro_step: 151,
};

pub const PY152_MIN_PATH_SUM: CodingStep = CodingStep {
    id: "py-152-min-path-sum",
    title: "DSA Min Path Sum",
    objective: "Camino mínimo en grilla solo derecha/abajo.",
    prompt_md: "**Min Path Sum**\n\n`dp[i][j] = grid[i][j] + min(arriba, izquierda)`.\n\n**Micro-reto:**\n1. Definí `min_path_sum(grid)`\n2. Imprimí con `[[1,3,1],[1,5,1],[4,2,1]]` (esperado: `7`)",
    starter_code: "# def min_path_sum(grid):\n#     ...\n# print(min_path_sum([[1, 3, 1], [1, 5, 1], [4, 2, 1]]))\n",
    pytest: "def test_min_path_sum(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('min_path_sum'))\n    assert ns['min_path_sum']([[1, 3, 1], [1, 5, 1], [4, 2, 1]]) == 7\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['7']\n",
    hint: "def min_path_sum(grid):\n    m, n = len(grid), len(grid[0])\n    dp = [row[:] for row in grid]\n    for i in range(1, m):\n        dp[i][0] += dp[i - 1][0]\n    for j in range(1, n):\n        dp[0][j] += dp[0][j - 1]\n    for i in range(1, m):\n        for j in range(1, n):\n            dp[i][j] += min(dp[i - 1][j], dp[i][j - 1])\n    return dp[-1][-1]\nprint(min_path_sum([[1, 3, 1], [1, 5, 1], [4, 2, 1]]))",
    solution_example: "def min_path_sum(grid):\n    m, n = len(grid), len(grid[0])\n    dp = [row[:] for row in grid]\n    for i in range(1, m):\n        dp[i][0] += dp[i - 1][0]\n    for j in range(1, n):\n        dp[0][j] += dp[0][j - 1]\n    for i in range(1, m):\n        for j in range(1, n):\n            dp[i][j] += min(dp[i - 1][j], dp[i][j - 1])\n    return dp[-1][-1]\nprint(min_path_sum([[1, 3, 1], [1, 5, 1], [4, 2, 1]]))\n",
    next: Some("py-153-decode-ways"),
    show_type_chips: false,
    micro_step: 152,
};

pub const PY153_DECODE_WAYS: CodingStep = CodingStep {
    id: "py-153-decode-ways",
    title: "DSA Decode Ways",
    objective: "Formas de decodificar dígitos a letras A-Z (1..26).",
    prompt_md: "**Decode Ways**\n\nDP: single digit válido y/o par 10–26.\n\n**Micro-reto:**\n1. Definí `decode_ways(s)`\n2. Imprimí `decode_ways('226')` (esperado: `3`)",
    starter_code: "# def decode_ways(s):\n#     ...\n# print(decode_ways('226'))\n",
    pytest: "def test_decode_ways(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('decode_ways'))\n    assert ns['decode_ways']('226') == 3\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['3']\n",
    hint: "def decode_ways(s):\n    if not s or s[0] == '0':\n        return 0\n    n = len(s)\n    dp = [0] * (n + 1)\n    dp[0] = dp[1] = 1\n    for i in range(2, n + 1):\n        if s[i - 1] != '0':\n            dp[i] += dp[i - 1]\n        two = int(s[i - 2:i])\n        if 10 <= two <= 26:\n            dp[i] += dp[i - 2]\n    return dp[n]\nprint(decode_ways('226'))",
    solution_example: "def decode_ways(s):\n    if not s or s[0] == '0':\n        return 0\n    n = len(s)\n    dp = [0] * (n + 1)\n    dp[0] = dp[1] = 1\n    for i in range(2, n + 1):\n        if s[i - 1] != '0':\n            dp[i] += dp[i - 1]\n        two = int(s[i - 2:i])\n        if 10 <= two <= 26:\n            dp[i] += dp[i - 2]\n    return dp[n]\nprint(decode_ways('226'))\n",
    next: Some("py-154-longest-palindrome"),
    show_type_chips: false,
    micro_step: 153,
};

pub const PY154_LONGEST_PALINDROME: CodingStep = CodingStep {
    id: "py-154-longest-palindrome",
    title: "DSA Longest Palindrome",
    objective: "Longitud del palíndromo más largo expandiendo desde centros.",
    prompt_md: "**Longest Palindromic Substring (length)**\n\nExpandí alrededor de cada centro (impar y par).\n\n**Micro-reto:**\n1. Definí `longest_palindrome_len(s)`\n2. Imprimí `longest_palindrome_len('babad')` (esperado: `3`)",
    starter_code: "# def longest_palindrome_len(s):\n#     ...\n# print(longest_palindrome_len('babad'))\n",
    pytest: "def test_longest_palindrome(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('longest_palindrome_len'))\n    assert ns['longest_palindrome_len']('babad') == 3\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['3']\n",
    hint: "def longest_palindrome_len(s):\n    def expand(l, r):\n        while l >= 0 and r < len(s) and s[l] == s[r]:\n            l -= 1\n            r += 1\n        return r - l - 1\n    best = 0\n    for i in range(len(s)):\n        best = max(best, expand(i, i), expand(i, i + 1))\n    return best\nprint(longest_palindrome_len('babad'))",
    solution_example: "def longest_palindrome_len(s):\n    def expand(l, r):\n        while l >= 0 and r < len(s) and s[l] == s[r]:\n            l -= 1\n            r += 1\n        return r - l - 1\n    best = 0\n    for i in range(len(s)):\n        best = max(best, expand(i, i), expand(i, i + 1))\n    return best\nprint(longest_palindrome_len('babad'))\n",
    next: Some("py-155-contains-dup"),
    show_type_chips: false,
    micro_step: 154,
};

pub const PY155_CONTAINS_DUP: CodingStep = CodingStep {
    id: "py-155-contains-dup",
    title: "DSA Contains Duplicate",
    objective: "Detectar valores repetidos con un conjunto.",
    prompt_md: "**Contains Duplicate**\n\nUn `set` guarda cada valor visto: si uno ya estaba, hay duplicado.\n\n**Micro-reto:**\n1. Definí `contains_duplicate(nums)`\n2. Imprimí `contains_duplicate([1, 2, 3, 1])` (esperado: `True`)",
    starter_code: "# def contains_duplicate(nums):\n#     ...\n# print(contains_duplicate([1, 2, 3, 1]))\n",
    pytest: "def test_contains_duplicate(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('contains_duplicate'))\n    assert ns['contains_duplicate']([1, 2, 3, 1]) is True\n    assert ns['contains_duplicate']([1, 2, 3, 4]) is False\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['True']\n",
    hint: "def contains_duplicate(nums):\n    seen = set()\n    for n in nums:\n        if n in seen:\n            return True\n        seen.add(n)\n    return False\nprint(contains_duplicate([1, 2, 3, 1]))",
    solution_example: "def contains_duplicate(nums):\n    seen = set()\n    for n in nums:\n        if n in seen:\n            return True\n        seen.add(n)\n    return False\nprint(contains_duplicate([1, 2, 3, 1]))\n",
    next: Some("py-156-best-stock"),
    show_type_chips: false,
    micro_step: 155,
};

pub const PY156_BEST_STOCK: CodingStep = CodingStep {
    id: "py-156-best-stock",
    title: "DSA Best Stock Profit",
    objective: "Maximizar la ganancia con una compra y una venta.",
    prompt_md: "**Best Time to Buy and Sell Stock**\n\nConservá el menor precio visto y compará la ganancia de vender hoy.\n\n**Micro-reto:**\n1. Definí `max_profit(prices)`\n2. Imprimí `max_profit([7, 1, 5, 3, 6, 4])` (esperado: `5`)",
    starter_code: "# def max_profit(prices):\n#     ...\n# print(max_profit([7, 1, 5, 3, 6, 4]))\n",
    pytest: "def test_max_profit(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('max_profit'))\n    assert ns['max_profit']([7, 1, 5, 3, 6, 4]) == 5\n    assert ns['max_profit']([7, 6, 4, 3, 1]) == 0\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['5']\n",
    hint: "def max_profit(prices):\n    lowest = float('inf')\n    best = 0\n    for price in prices:\n        lowest = min(lowest, price)\n        best = max(best, price - lowest)\n    return best\nprint(max_profit([7, 1, 5, 3, 6, 4]))",
    solution_example: "def max_profit(prices):\n    lowest = float('inf')\n    best = 0\n    for price in prices:\n        lowest = min(lowest, price)\n        best = max(best, price - lowest)\n    return best\nprint(max_profit([7, 1, 5, 3, 6, 4]))\n",
    next: Some("py-157-move-zeroes"),
    show_type_chips: false,
    micro_step: 156,
};

pub const PY157_MOVE_ZEROES: CodingStep = CodingStep {
    id: "py-157-move-zeroes",
    title: "DSA Move Zeroes",
    objective: "Mover ceros al final preservando los no-cero.",
    prompt_md: "**Move Zeroes**\n\nEscribí los valores no-cero al frente y completá el resto con ceros.\n\n**Micro-reto:**\n1. Definí `move_zeroes(nums)` y retorná la lista mutada\n2. Imprimí `move_zeroes([0, 1, 0, 3, 12])` (esperado: `[1, 3, 12, 0, 0]`)",
    starter_code: "# def move_zeroes(nums):\n#     ...\n# print(move_zeroes([0, 1, 0, 3, 12]))\n",
    pytest: "def test_move_zeroes(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('move_zeroes'))\n    nums = [0, 1, 0, 3, 12]\n    assert ns['move_zeroes'](nums) == [1, 3, 12, 0, 0]\n    assert nums == [1, 3, 12, 0, 0]\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[1, 3, 12, 0, 0]']\n",
    hint: "def move_zeroes(nums):\n    write = 0\n    for n in nums:\n        if n != 0:\n            nums[write] = n\n            write += 1\n    for i in range(write, len(nums)):\n        nums[i] = 0\n    return nums\nprint(move_zeroes([0, 1, 0, 3, 12]))",
    solution_example: "def move_zeroes(nums):\n    write = 0\n    for n in nums:\n        if n != 0:\n            nums[write] = n\n            write += 1\n    for i in range(write, len(nums)):\n        nums[i] = 0\n    return nums\nprint(move_zeroes([0, 1, 0, 3, 12]))\n",
    next: Some("py-158-product-except"),
    show_type_chips: false,
    micro_step: 157,
};

pub const PY158_PRODUCT_EXCEPT: CodingStep = CodingStep {
    id: "py-158-product-except",
    title: "DSA Product Except Self",
    objective: "Calcular productos salvo el propio índice sin división.",
    prompt_md: "**Product of Array Except Self**\n\nConstruí prefijos y sufijos: cada posición combina lo de antes y después, sin dividir.\n\n**Micro-reto:**\n1. Definí `product_except_self(nums)` sin división\n2. Imprimí `product_except_self([1, 2, 3, 4])` (esperado: `[24, 12, 8, 6]`)",
    starter_code: "# def product_except_self(nums):\n#     ...\n# print(product_except_self([1, 2, 3, 4]))\n",
    pytest: "def test_product_except_self(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('product_except_self'))\n    assert ns['product_except_self']([1, 2, 3, 4]) == [24, 12, 8, 6]\n    assert ns['product_except_self']([-1, 1, 0, -3, 3]) == [0, 0, 9, 0, 0]\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[24, 12, 8, 6]']\n",
    hint: "def product_except_self(nums):\n    result = [1] * len(nums)\n    prefix = 1\n    for i, n in enumerate(nums):\n        result[i] = prefix\n        prefix *= n\n    suffix = 1\n    for i in range(len(nums) - 1, -1, -1):\n        result[i] *= suffix\n        suffix *= nums[i]\n    return result\nprint(product_except_self([1, 2, 3, 4]))",
    solution_example: "def product_except_self(nums):\n    result = [1] * len(nums)\n    prefix = 1\n    for i, n in enumerate(nums):\n        result[i] = prefix\n        prefix *= n\n    suffix = 1\n    for i in range(len(nums) - 1, -1, -1):\n        result[i] *= suffix\n        suffix *= nums[i]\n    return result\nprint(product_except_self([1, 2, 3, 4]))\n",
    next: Some("py-159-first-unique"),
    show_type_chips: false,
    micro_step: 158,
};

pub const PY159_FIRST_UNIQUE: CodingStep = CodingStep {
    id: "py-159-first-unique",
    title: "DSA First Unique Character",
    objective: "Encontrar el primer carácter cuya frecuencia sea uno.",
    prompt_md: "**First Unique Character**\n\nContá frecuencias y recorré el string para hallar el primer carácter único.\n\n**Micro-reto:**\n1. Definí `first_uniq_char(s)`\n2. Imprimí `first_uniq_char('leetcode')` (esperado: `0`)",
    starter_code: "# def first_uniq_char(s):\n#     ...\n# print(first_uniq_char('leetcode'))\n",
    pytest: "def test_first_uniq_char(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('first_uniq_char'))\n    assert ns['first_uniq_char']('leetcode') == 0\n    assert ns['first_uniq_char']('loveleetcode') == 2\n    assert ns['first_uniq_char']('aabb') == -1\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['0']\n",
    hint: "def first_uniq_char(s):\n    counts = {}\n    for char in s:\n        counts[char] = counts.get(char, 0) + 1\n    for i, char in enumerate(s):\n        if counts[char] == 1:\n            return i\n    return -1\nprint(first_uniq_char('leetcode'))",
    solution_example: "def first_uniq_char(s):\n    counts = {}\n    for char in s:\n        counts[char] = counts.get(char, 0) + 1\n    for i, char in enumerate(s):\n        if counts[char] == 1:\n            return i\n    return -1\nprint(first_uniq_char('leetcode'))\n",
    next: Some("py-160-happy-number"),
    show_type_chips: false,
    micro_step: 159,
};

pub const PY160_HAPPY_NUMBER: CodingStep = CodingStep {
    id: "py-160-happy-number",
    title: "DSA Happy Number",
    objective: "Detectar si la suma de cuadrados de dígitos llega a uno.",
    prompt_md: "**Happy Number**\n\nGuardá los valores vistos; repetir uno implica ciclo, llegar a `1` implica éxito.\n\n**Micro-reto:**\n1. Definí `is_happy(n)`\n2. Imprimí `is_happy(19)` (esperado: `True`)",
    starter_code: "# def is_happy(n):\n#     ...\n# print(is_happy(19))\n",
    pytest: "def test_is_happy(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('is_happy'))\n    assert ns['is_happy'](19) is True\n    assert ns['is_happy'](2) is False\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['True']\n",
    hint: "def is_happy(n):\n    seen = set()\n    while n != 1 and n not in seen:\n        seen.add(n)\n        n = sum(int(digit) ** 2 for digit in str(n))\n    return n == 1\nprint(is_happy(19))",
    solution_example: "def is_happy(n):\n    seen = set()\n    while n != 1 and n not in seen:\n        seen.add(n)\n        n = sum(int(digit) ** 2 for digit in str(n))\n    return n == 1\nprint(is_happy(19))\n",
    next: Some("py-161-reverse-list"),
    show_type_chips: false,
    micro_step: 160,
};

pub const PY161_REVERSE_LIST: CodingStep = CodingStep {
    id: "py-161-reverse-list",
    title: "DSA Reverse List",
    objective: "Invertir una lista devolviendo sus elementos en orden inverso.",
    prompt_md: "**Reverse List**\n\nDevolvé una nueva lista con los elementos en orden inverso.\n\n**Micro-reto:**\n1. Definí `reverse_list(nums: list)`\n2. Devolvé la lista invertida sin modificar `nums`\n3. Imprimí `reverse_list([1, 2, 3, 4])` (esperado: `[4, 3, 2, 1]`)",
    starter_code: "# def reverse_list(nums: list):\n#     ...\n# print(reverse_list([1, 2, 3, 4]))\n",
    pytest: "def test_reverse_list(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('reverse_list'))\n    nums = [1, 2, 3, 4]\n    assert ns['reverse_list'](nums) == [4, 3, 2, 1]\n    assert nums == [1, 2, 3, 4]\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[4, 3, 2, 1]']\n",
    hint: "def reverse_list(nums: list):\n    return nums[::-1]\nprint(reverse_list([1, 2, 3, 4]))",
    solution_example: "def reverse_list(nums: list):\n    return nums[::-1]\nprint(reverse_list([1, 2, 3, 4]))\n",
    next: Some("py-162-merge-sorted"),
    show_type_chips: false,
    micro_step: 161,
};

pub const PY162_MERGE_SORTED: CodingStep = CodingStep {
    id: "py-162-merge-sorted",
    title: "DSA Merge Sorted Lists",
    objective: "Unir dos listas ordenadas preservando el orden ascendente.",
    prompt_md: "**Merge Sorted Lists**\n\nUsá dos índices: elegí el menor elemento disponible de cada lista.\n\n**Micro-reto:**\n1. Definí `merge_sorted(a, b)`\n2. Devolvé una nueva lista ordenada\n3. Imprimí `merge_sorted([1, 2, 4], [1, 3, 4])` (esperado: `[1, 1, 2, 3, 4, 4]`)",
    starter_code: "# def merge_sorted(a, b):\n#     ...\n# print(merge_sorted([1, 2, 4], [1, 3, 4]))\n",
    pytest: "def test_merge_sorted(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('merge_sorted'))\n    assert ns['merge_sorted']([1, 2, 4], [1, 3, 4]) == [1, 1, 2, 3, 4, 4]\n    assert ns['merge_sorted']([], [2]) == [2]\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[1, 1, 2, 3, 4, 4]']\n",
    hint: "def merge_sorted(a, b):\n    result = []\n    i = j = 0\n    while i < len(a) and j < len(b):\n        if a[i] <= b[j]:\n            result.append(a[i])\n            i += 1\n        else:\n            result.append(b[j])\n            j += 1\n    return result + a[i:] + b[j:]\nprint(merge_sorted([1, 2, 4], [1, 3, 4]))",
    solution_example: "def merge_sorted(a, b):\n    result = []\n    i = j = 0\n    while i < len(a) and j < len(b):\n        if a[i] <= b[j]:\n            result.append(a[i])\n            i += 1\n        else:\n            result.append(b[j])\n            j += 1\n    return result + a[i:] + b[j:]\nprint(merge_sorted([1, 2, 4], [1, 3, 4]))\n",
    next: Some("py-163-linked-cycle"),
    show_type_chips: false,
    micro_step: 162,
};

pub const PY163_LINKED_CYCLE: CodingStep = CodingStep {
    id: "py-163-linked-cycle",
    title: "DSA Linked List Cycle",
    objective: "Detectar un ciclo en nodos enlazados con los punteros lento y rápido.",
    prompt_md: "**Linked List Cycle**\n\nUsá Floyd: `slow` avanza un nodo y `fast` dos. Si se encuentran, hay ciclo.\n\n**Micro-reto:**\n1. Definí `class Node` con `data` y `next`\n2. Definí `has_cycle(head)`\n3. Construí `3 -> 2 -> 0 -> -4` y conectá `-4.next` con el nodo `2`\n4. Imprimí `has_cycle(node1)` (esperado: `True`)",
    starter_code: "# class Node:\n#     ...\n# def has_cycle(head):\n#     ...\n# node1 = ...\n# ...\n# print(has_cycle(node1))\n",
    pytest: "def test_linked_cycle(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('has_cycle'))\n    assert ns['has_cycle'](ns['node1']) is True\n    no_cycle = ns['Node'](1)\n    no_cycle.next = ns['Node'](2)\n    assert ns['has_cycle'](no_cycle) is False\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['True']\n",
    hint: "class Node:\n    def __init__(self, data):\n        self.data = data\n        self.next = None\n\ndef has_cycle(head):\n    slow = fast = head\n    while fast is not None and fast.next is not None:\n        slow = slow.next\n        fast = fast.next.next\n        if slow is fast:\n            return True\n    return False\n\nnode1 = Node(3)\nnode2 = Node(2)\nnode3 = Node(0)\nnode4 = Node(-4)\nnode1.next = node2\nnode2.next = node3\nnode3.next = node4\nnode4.next = node2\nprint(has_cycle(node1))",
    solution_example: "class Node:\n    def __init__(self, data):\n        self.data = data\n        self.next = None\n\ndef has_cycle(head):\n    slow = fast = head\n    while fast is not None and fast.next is not None:\n        slow = slow.next\n        fast = fast.next.next\n        if slow is fast:\n            return True\n    return False\n\nnode1 = Node(3)\nnode2 = Node(2)\nnode3 = Node(0)\nnode4 = Node(-4)\nnode1.next = node2\nnode2.next = node3\nnode3.next = node4\nnode4.next = node2\nprint(has_cycle(node1))\n",
    next: Some("py-164-valid-palindrome"),
    show_type_chips: false,
    micro_step: 163,
};

pub const PY164_VALID_PALINDROME: CodingStep = CodingStep {
    id: "py-164-valid-palindrome",
    title: "DSA Valid Palindrome",
    objective: "Verificar un palíndromo ignorando caracteres no alfanuméricos y mayúsculas.",
    prompt_md: "**Valid Palindrome**\n\nFiltrá solo caracteres alfanuméricos, compará sin distinguir mayúsculas y minúsculas.\n\n**Micro-reto:**\n1. Definí `is_palindrome(s)`\n2. Ignorá espacios, puntuación y case\n3. Imprimí `is_palindrome('A man, a plan, a canal: Panama')` (esperado: `True`)",
    starter_code: "# def is_palindrome(s):\n#     ...\n# print(is_palindrome('A man, a plan, a canal: Panama'))\n",
    pytest: "def test_valid_palindrome(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('is_palindrome'))\n    assert ns['is_palindrome']('A man, a plan, a canal: Panama') is True\n    assert ns['is_palindrome']('race a car') is False\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['True']\n",
    hint: "def is_palindrome(s):\n    cleaned = ''.join(ch.casefold() for ch in s if ch.isalnum())\n    return cleaned == cleaned[::-1]\nprint(is_palindrome('A man, a plan, a canal: Panama'))",
    solution_example: "def is_palindrome(s):\n    cleaned = ''.join(ch.casefold() for ch in s if ch.isalnum())\n    return cleaned == cleaned[::-1]\nprint(is_palindrome('A man, a plan, a canal: Panama'))\n",
    next: Some("py-165-common-prefix"),
    show_type_chips: false,
    micro_step: 164,
};

pub const PY165_COMMON_PREFIX: CodingStep = CodingStep {
    id: "py-165-common-prefix",
    title: "DSA Longest Common Prefix",
    objective: "Encontrar el prefijo compartido más largo entre strings.",
    prompt_md: "**Longest Common Prefix**\n\nPartí del primer string y acortalo hasta que todos compartan el prefijo.\n\n**Micro-reto:**\n1. Definí `longest_common_prefix(strs)`\n2. Devolvé `''` si no hay prefijo o la lista está vacía\n3. Imprimí `longest_common_prefix(['flower', 'flow', 'flight'])` (esperado: `fl`)",
    starter_code: "# def longest_common_prefix(strs):\n#     ...\n# print(longest_common_prefix(['flower', 'flow', 'flight']))\n",
    pytest: "def test_common_prefix(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('longest_common_prefix'))\n    assert ns['longest_common_prefix'](['flower', 'flow', 'flight']) == 'fl'\n    assert ns['longest_common_prefix'](['dog', 'racecar', 'car']) == ''\n    assert ns['longest_common_prefix']([]) == ''\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['fl']\n",
    hint: "def longest_common_prefix(strs):\n    if not strs:\n        return ''\n    prefix = strs[0]\n    for word in strs[1:]:\n        while not word.startswith(prefix):\n            prefix = prefix[:-1]\n            if not prefix:\n                return ''\n    return prefix\nprint(longest_common_prefix(['flower', 'flow', 'flight']))",
    solution_example: "def longest_common_prefix(strs):\n    if not strs:\n        return ''\n    prefix = strs[0]\n    for word in strs[1:]:\n        while not word.startswith(prefix):\n            prefix = prefix[:-1]\n            if not prefix:\n                return ''\n    return prefix\nprint(longest_common_prefix(['flower', 'flow', 'flight']))\n",
    next: Some("py-166-roman-to-int"),
    show_type_chips: false,
    micro_step: 165,
};

pub const PY166_ROMAN_TO_INT: CodingStep = CodingStep {
    id: "py-166-roman-to-int",
    title: "DSA Roman to Integer",
    objective: "Convertir un número romano aplicando la regla sustractiva.",
    prompt_md: "**Roman to Integer**\n\nRecorré de derecha a izquierda: restá un símbolo menor si aparece antes de uno mayor.\n\n**Micro-reto:**\n1. Definí `roman_to_int(s)`\n2. Usá los valores `I, V, X, L, C, D, M`\n3. Imprimí `roman_to_int('MCMXCIV')` (esperado: `1994`)",
    starter_code: "# def roman_to_int(s):\n#     ...\n# print(roman_to_int('MCMXCIV'))\n",
    pytest: "def test_roman_to_int(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('roman_to_int'))\n    assert ns['roman_to_int']('MCMXCIV') == 1994\n    assert ns['roman_to_int']('III') == 3\n    assert ns['roman_to_int']('LVIII') == 58\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['1994']\n",
    hint: "def roman_to_int(s):\n    values = {'I': 1, 'V': 5, 'X': 10, 'L': 50, 'C': 100, 'D': 500, 'M': 1000}\n    total = 0\n    previous = 0\n    for symbol in reversed(s):\n        value = values[symbol]\n        if value < previous:\n            total -= value\n        else:\n            total += value\n            previous = value\n    return total\nprint(roman_to_int('MCMXCIV'))",
    solution_example: "def roman_to_int(s):\n    values = {'I': 1, 'V': 5, 'X': 10, 'L': 50, 'C': 100, 'D': 500, 'M': 1000}\n    total = 0\n    previous = 0\n    for symbol in reversed(s):\n        value = values[symbol]\n        if value < previous:\n            total -= value\n        else:\n            total += value\n            previous = value\n    return total\nprint(roman_to_int('MCMXCIV'))\n",
    next: Some("py-167-invert-tree"),
    show_type_chips: false,
    micro_step: 166,
};

pub const PY167_INVERT_TREE: CodingStep = CodingStep {
    id: "py-167-invert-tree",
    title: "DSA Invert Binary Tree",
    objective: "Espejar un árbol binario intercambiando recursivamente left y right.",
    prompt_md: "**Invert Binary Tree**\n\nUn `TreeNode` tiene `data`, `left` y `right` (como en py-105). Invertir = intercambiar subárboles.\n\n**Micro-reto:**\n1. Definí `class TreeNode` con `data`, `left=None`, `right=None`\n2. Definí `invert_tree(root)` que muta y devuelve la raíz\n3. Definí `level_order(root)` que devuelve la lista level-order de valores\n4. Construí el árbol `4` → left `2` (1, 3) / right `7` (6, 9)\n5. Invertí e imprimí `level_order(root)` (esperado: `[4, 7, 2, 9, 6, 3, 1]`)",
    starter_code: "# class TreeNode:\n#     ...\n# def invert_tree(root):\n#     ...\n# def level_order(root):\n#     ...\n# root = ...\n# ...\n# invert_tree(root)\n# print(level_order(root))\n",
    pytest: "def test_invert_tree(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('invert_tree'))\n    assert callable(ns.get('level_order'))\n    TreeNode = ns['TreeNode']\n    root = TreeNode(4)\n    root.left = TreeNode(2)\n    root.right = TreeNode(7)\n    root.left.left = TreeNode(1)\n    root.left.right = TreeNode(3)\n    root.right.left = TreeNode(6)\n    root.right.right = TreeNode(9)\n    ns['invert_tree'](root)\n    assert ns['level_order'](root) == [4, 7, 2, 9, 6, 3, 1]\n    assert ns['invert_tree'](None) is None\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[4, 7, 2, 9, 6, 3, 1]']\n",
    hint: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef invert_tree(root):\n    if root is None:\n        return None\n    root.left, root.right = invert_tree(root.right), invert_tree(root.left)\n    return root\n\ndef level_order(root):\n    if root is None:\n        return []\n    result = []\n    queue = [root]\n    while queue:\n        node = queue.pop(0)\n        result.append(node.data)\n        if node.left:\n            queue.append(node.left)\n        if node.right:\n            queue.append(node.right)\n    return result\n\nroot = TreeNode(4)\nroot.left = TreeNode(2)\nroot.right = TreeNode(7)\nroot.left.left = TreeNode(1)\nroot.left.right = TreeNode(3)\nroot.right.left = TreeNode(6)\nroot.right.right = TreeNode(9)\ninvert_tree(root)\nprint(level_order(root))",
    solution_example: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef invert_tree(root):\n    if root is None:\n        return None\n    root.left, root.right = invert_tree(root.right), invert_tree(root.left)\n    return root\n\ndef level_order(root):\n    if root is None:\n        return []\n    result = []\n    queue = [root]\n    while queue:\n        node = queue.pop(0)\n        result.append(node.data)\n        if node.left:\n            queue.append(node.left)\n        if node.right:\n            queue.append(node.right)\n    return result\n\nroot = TreeNode(4)\nroot.left = TreeNode(2)\nroot.right = TreeNode(7)\nroot.left.left = TreeNode(1)\nroot.left.right = TreeNode(3)\nroot.right.left = TreeNode(6)\nroot.right.right = TreeNode(9)\ninvert_tree(root)\nprint(level_order(root))\n",
    next: Some("py-168-same-tree"),
    show_type_chips: false,
    micro_step: 167,
};

pub const PY168_SAME_TREE: CodingStep = CodingStep {
    id: "py-168-same-tree",
    title: "DSA Same Tree",
    objective: "Decidir si dos árboles binarios son estructuralmente idénticos.",
    prompt_md: "**Same Tree**\n\nDos árboles son iguales si coinciden valor y forma en cada nodo.\n\n**Micro-reto:**\n1. Definí `class TreeNode` con `data`, `left=None`, `right=None`\n2. Definí `is_same_tree(p, q)`\n3. Construí dos árboles iguales `1 → left 2 / right 3`\n4. Imprimí `is_same_tree(p, q)` (esperado: `True`)",
    starter_code: "# class TreeNode:\n#     ...\n# def is_same_tree(p, q):\n#     ...\n# p = ...\n# q = ...\n# print(is_same_tree(p, q))\n",
    pytest: "def test_same_tree(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('is_same_tree'))\n    TreeNode = ns['TreeNode']\n    p = TreeNode(1)\n    p.left = TreeNode(2)\n    p.right = TreeNode(3)\n    q = TreeNode(1)\n    q.left = TreeNode(2)\n    q.right = TreeNode(3)\n    assert ns['is_same_tree'](p, q) is True\n    other = TreeNode(1)\n    other.left = TreeNode(2)\n    assert ns['is_same_tree'](p, other) is False\n    assert ns['is_same_tree'](None, None) is True\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['True']\n",
    hint: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef is_same_tree(p, q):\n    if p is None and q is None:\n        return True\n    if p is None or q is None or p.data != q.data:\n        return False\n    return is_same_tree(p.left, q.left) and is_same_tree(p.right, q.right)\n\np = TreeNode(1)\np.left = TreeNode(2)\np.right = TreeNode(3)\nq = TreeNode(1)\nq.left = TreeNode(2)\nq.right = TreeNode(3)\nprint(is_same_tree(p, q))",
    solution_example: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef is_same_tree(p, q):\n    if p is None and q is None:\n        return True\n    if p is None or q is None or p.data != q.data:\n        return False\n    return is_same_tree(p.left, q.left) and is_same_tree(p.right, q.right)\n\np = TreeNode(1)\np.left = TreeNode(2)\np.right = TreeNode(3)\nq = TreeNode(1)\nq.left = TreeNode(2)\nq.right = TreeNode(3)\nprint(is_same_tree(p, q))\n",
    next: Some("py-169-max-depth"),
    show_type_chips: false,
    micro_step: 168,
};

pub const PY169_MAX_DEPTH: CodingStep = CodingStep {
    id: "py-169-max-depth",
    title: "DSA Maximum Depth",
    objective: "Calcular la profundidad máxima de un árbol binario.",
    prompt_md: "**Maximum Depth**\n\nLa profundidad es 1 + el máximo entre left y right; `None` vale 0.\n\n**Micro-reto:**\n1. Definí `class TreeNode` con `data`, `left=None`, `right=None`\n2. Definí `max_depth(root)`\n3. Construí el árbol clásico: `3` → left `9` / right `20` (15, 7)\n4. Imprimí `max_depth(root)` (esperado: `3`)",
    starter_code: "# class TreeNode:\n#     ...\n# def max_depth(root):\n#     ...\n# root = ...\n# print(max_depth(root))\n",
    pytest: "def test_max_depth(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('max_depth'))\n    TreeNode = ns['TreeNode']\n    root = TreeNode(3)\n    root.left = TreeNode(9)\n    root.right = TreeNode(20)\n    root.right.left = TreeNode(15)\n    root.right.right = TreeNode(7)\n    assert ns['max_depth'](root) == 3\n    assert ns['max_depth'](None) == 0\n    leaf = TreeNode(1)\n    assert ns['max_depth'](leaf) == 1\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['3']\n",
    hint: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef max_depth(root):\n    if root is None:\n        return 0\n    return 1 + max(max_depth(root.left), max_depth(root.right))\n\nroot = TreeNode(3)\nroot.left = TreeNode(9)\nroot.right = TreeNode(20)\nroot.right.left = TreeNode(15)\nroot.right.right = TreeNode(7)\nprint(max_depth(root))",
    solution_example: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef max_depth(root):\n    if root is None:\n        return 0\n    return 1 + max(max_depth(root.left), max_depth(root.right))\n\nroot = TreeNode(3)\nroot.left = TreeNode(9)\nroot.right = TreeNode(20)\nroot.right.left = TreeNode(15)\nroot.right.right = TreeNode(7)\nprint(max_depth(root))\n",
    next: Some("py-170-spiral-matrix"),
    show_type_chips: false,
    micro_step: 169,
};

pub const PY170_SPIRAL_MATRIX: CodingStep = CodingStep {
    id: "py-170-spiral-matrix",
    title: "DSA Spiral Matrix",
    objective: "Recorrer una matriz en espiral desde la esquina superior izquierda.",
    prompt_md: "**Spiral Matrix**\n\nRecorré capas: derecha → abajo → izquierda → arriba, cerrando bordes.\n\n**Micro-reto:**\n1. Definí `spiral_order(matrix)`\n2. Devolvé la lista de valores en orden espiral\n3. Imprimí `spiral_order([[1, 2, 3], [4, 5, 6], [7, 8, 9]])` (esperado: `[1, 2, 3, 6, 9, 8, 7, 4, 5]`)",
    starter_code: "# def spiral_order(matrix):\n#     ...\n# print(spiral_order([[1, 2, 3], [4, 5, 6], [7, 8, 9]]))\n",
    pytest: "def test_spiral_matrix(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('spiral_order'))\n    assert ns['spiral_order']([[1, 2, 3], [4, 5, 6], [7, 8, 9]]) == [1, 2, 3, 6, 9, 8, 7, 4, 5]\n    assert ns['spiral_order']([[1, 2], [3, 4]]) == [1, 2, 4, 3]\n    assert ns['spiral_order']([]) == []\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[1, 2, 3, 6, 9, 8, 7, 4, 5]']\n",
    hint: "def spiral_order(matrix):\n    if not matrix:\n        return []\n    result = []\n    top, bottom = 0, len(matrix) - 1\n    left, right = 0, len(matrix[0]) - 1\n    while top <= bottom and left <= right:\n        for j in range(left, right + 1):\n            result.append(matrix[top][j])\n        top += 1\n        for i in range(top, bottom + 1):\n            result.append(matrix[i][right])\n        right -= 1\n        if top <= bottom:\n            for j in range(right, left - 1, -1):\n                result.append(matrix[bottom][j])\n            bottom -= 1\n        if left <= right:\n            for i in range(bottom, top - 1, -1):\n                result.append(matrix[i][left])\n            left += 1\n    return result\nprint(spiral_order([[1, 2, 3], [4, 5, 6], [7, 8, 9]]))",
    solution_example: "def spiral_order(matrix):\n    if not matrix:\n        return []\n    result = []\n    top, bottom = 0, len(matrix) - 1\n    left, right = 0, len(matrix[0]) - 1\n    while top <= bottom and left <= right:\n        for j in range(left, right + 1):\n            result.append(matrix[top][j])\n        top += 1\n        for i in range(top, bottom + 1):\n            result.append(matrix[i][right])\n        right -= 1\n        if top <= bottom:\n            for j in range(right, left - 1, -1):\n                result.append(matrix[bottom][j])\n            bottom -= 1\n        if left <= right:\n            for i in range(bottom, top - 1, -1):\n                result.append(matrix[i][left])\n            left += 1\n    return result\nprint(spiral_order([[1, 2, 3], [4, 5, 6], [7, 8, 9]]))\n",
    next: Some("py-171-set-zeroes"),
    show_type_chips: false,
    micro_step: 170,
};

pub const PY171_SET_ZEROES: CodingStep = CodingStep {
    id: "py-171-set-zeroes",
    title: "DSA Set Matrix Zeroes",
    objective: "Si una celda es 0, poner en cero toda su fila y columna.",
    prompt_md: "**Set Matrix Zeroes**\n\nMarcá filas/columnas que contienen un 0 y aplicá después (no en el mismo paso del scan).\n\n**Micro-reto:**\n1. Definí `set_zeroes(matrix)` que muta la matriz y la devuelve\n2. Imprimí `set_zeroes([[1, 1, 1], [1, 0, 1], [1, 1, 1]])` (esperado: `[[1, 0, 1], [0, 0, 0], [1, 0, 1]]`)",
    starter_code: "# def set_zeroes(matrix):\n#     ...\n# print(set_zeroes([[1, 1, 1], [1, 0, 1], [1, 1, 1]]))\n",
    pytest: "def test_set_zeroes(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('set_zeroes'))\n    matrix = [[1, 1, 1], [1, 0, 1], [1, 1, 1]]\n    out = ns['set_zeroes'](matrix)\n    assert out == [[1, 0, 1], [0, 0, 0], [1, 0, 1]]\n    assert matrix == [[1, 0, 1], [0, 0, 0], [1, 0, 1]]\n    other = [[0, 1], [1, 1]]\n    assert ns['set_zeroes'](other) == [[0, 0], [0, 1]]\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[[1, 0, 1], [0, 0, 0], [1, 0, 1]]']\n",
    hint: "def set_zeroes(matrix):\n    rows = set()\n    cols = set()\n    for i in range(len(matrix)):\n        for j in range(len(matrix[0])):\n            if matrix[i][j] == 0:\n                rows.add(i)\n                cols.add(j)\n    for i in range(len(matrix)):\n        for j in range(len(matrix[0])):\n            if i in rows or j in cols:\n                matrix[i][j] = 0\n    return matrix\nprint(set_zeroes([[1, 1, 1], [1, 0, 1], [1, 1, 1]]))",
    solution_example: "def set_zeroes(matrix):\n    rows = set()\n    cols = set()\n    for i in range(len(matrix)):\n        for j in range(len(matrix[0])):\n            if matrix[i][j] == 0:\n                rows.add(i)\n                cols.add(j)\n    for i in range(len(matrix)):\n        for j in range(len(matrix[0])):\n            if i in rows or j in cols:\n                matrix[i][j] = 0\n    return matrix\nprint(set_zeroes([[1, 1, 1], [1, 0, 1], [1, 1, 1]]))\n",
    next: Some("py-172-subsets"),
    show_type_chips: false,
    micro_step: 171,
};

pub const PY172_SUBSETS: CodingStep = CodingStep {
    id: "py-172-subsets",
    title: "DSA Subsets",
    objective: "Generar todos los subsets de una lista de enteros distintos.",
    prompt_md: "**Subsets**\n\nPartí de `[[]]` y, por cada número, agregá una copia de cada subset existente extendida con ese número.\n\n**Micro-reto:**\n1. Definí `subsets(nums)` que devolvé una lista de listas\n2. Imprimí `sorted(subsets([1, 2]))` (esperado: `[[], [1], [1, 2], [2]]`)",
    starter_code: "# def subsets(nums):\n#     ...\n# print(sorted(subsets([1, 2])))\n",
    pytest: "def test_subsets(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('subsets'))\n    got = [sorted(s) for s in ns['subsets']([1, 2])]\n    assert sorted(got) == [[], [1], [1, 2], [2]]\n    got3 = [sorted(s) for s in ns['subsets']([1, 2, 3])]\n    assert sorted(got3) == [[], [1], [1, 2], [1, 2, 3], [1, 3], [2], [2, 3], [3]]\n    assert ns['subsets']([]) == [[]]\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[[], [1], [1, 2], [2]]']\n",
    hint: "def subsets(nums):\n    result = [[]]\n    for n in nums:\n        result += [subset + [n] for subset in result]\n    return result\nprint(sorted(subsets([1, 2])))",
    solution_example: "def subsets(nums):\n    result = [[]]\n    for n in nums:\n        result += [subset + [n] for subset in result]\n    return result\nprint(sorted(subsets([1, 2])))\n",
    next: Some("py-173-jump-game"),
    show_type_chips: false,
    micro_step: 172,
};

pub const PY173_JUMP_GAME: CodingStep = CodingStep {
    id: "py-173-jump-game",
    title: "DSA Jump Game",
    objective: "Decidir si se puede llegar al último índice con saltos máximos.",
    prompt_md: "**Jump Game**\n\nLlevá el alcance máximo (`reach`) y avanzá solo mientras `i <= reach`.\n\n**Micro-reto:**\n1. Definí `can_jump(nums)`\n2. Devolvé `True` si podés llegar al final\n3. Imprimí `can_jump([2, 3, 1, 1, 4])` (esperado: `True`)",
    starter_code: "# def can_jump(nums):\n#     ...\n# print(can_jump([2, 3, 1, 1, 4]))\n",
    pytest: "def test_jump_game(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('can_jump'))\n    assert ns['can_jump']([2, 3, 1, 1, 4]) is True\n    assert ns['can_jump']([3, 2, 1, 0, 4]) is False\n    assert ns['can_jump']([0]) is True\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['True']\n",
    hint: "def can_jump(nums):\n    reach = 0\n    for i, n in enumerate(nums):\n        if i > reach:\n            return False\n        reach = max(reach, i + n)\n    return True\nprint(can_jump([2, 3, 1, 1, 4]))",
    solution_example: "def can_jump(nums):\n    reach = 0\n    for i, n in enumerate(nums):\n        if i > reach:\n            return False\n        reach = max(reach, i + n)\n    return True\nprint(can_jump([2, 3, 1, 1, 4]))\n",
    next: Some("py-174-gas-station"),
    show_type_chips: false,
    micro_step: 173,
};

pub const PY174_GAS_STATION: CodingStep = CodingStep {
    id: "py-174-gas-station",
    title: "DSA Gas Station",
    objective: "Encontrar el índice de partida para completar el circuito de gas.",
    prompt_md: "**Gas Station**\n\nSi el total de gas es insuficiente, devolvé `-1`. Si no, el único start válido es donde el tank deja de ir negativo.\n\n**Micro-reto:**\n1. Definí `can_complete_circuit(gas, cost)`\n2. Imprimí `can_complete_circuit([1, 2, 3, 4, 5], [3, 4, 5, 1, 2])` (esperado: `3`)",
    starter_code: "# def can_complete_circuit(gas, cost):\n#     ...\n# print(can_complete_circuit([1, 2, 3, 4, 5], [3, 4, 5, 1, 2]))\n",
    pytest: "def test_gas_station(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('can_complete_circuit'))\n    assert ns['can_complete_circuit']([1, 2, 3, 4, 5], [3, 4, 5, 1, 2]) == 3\n    assert ns['can_complete_circuit']([2, 3, 4], [3, 4, 3]) == -1\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['3']\n",
    hint: "def can_complete_circuit(gas, cost):\n    if sum(gas) < sum(cost):\n        return -1\n    tank = 0\n    start = 0\n    for i in range(len(gas)):\n        tank += gas[i] - cost[i]\n        if tank < 0:\n            tank = 0\n            start = i + 1\n    return start\nprint(can_complete_circuit([1, 2, 3, 4, 5], [3, 4, 5, 1, 2]))",
    solution_example: "def can_complete_circuit(gas, cost):\n    if sum(gas) < sum(cost):\n        return -1\n    tank = 0\n    start = 0\n    for i in range(len(gas)):\n        tank += gas[i] - cost[i]\n        if tank < 0:\n            tank = 0\n            start = i + 1\n    return start\nprint(can_complete_circuit([1, 2, 3, 4, 5], [3, 4, 5, 1, 2]))\n",
    next: Some("py-175-container-water"),
    show_type_chips: false,
    micro_step: 174,
};

pub const PY175_CONTAINER_WATER: CodingStep = CodingStep {
    id: "py-175-container-water",
    title: "DSA Container With Most Water",
    objective: "Maximizar el área entre dos líneas con dos punteros.",
    prompt_md: "**Container With Most Water**\n\nEmpezá en los extremos: mové el puntero de menor altura.\n\n**Micro-reto:**\n1. Definí `max_area(height)`\n2. Imprimí `max_area([1, 8, 6, 2, 5, 4, 8, 3, 7])` (esperado: `49`)",
    starter_code: "# def max_area(height):\n#     ...\n# print(max_area([1, 8, 6, 2, 5, 4, 8, 3, 7]))\n",
    pytest: "def test_container_water(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('max_area'))\n    assert ns['max_area']([1, 8, 6, 2, 5, 4, 8, 3, 7]) == 49\n    assert ns['max_area']([1, 1]) == 1\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['49']\n",
    hint: "def max_area(height):\n    left, right = 0, len(height) - 1\n    best = 0\n    while left < right:\n        best = max(best, min(height[left], height[right]) * (right - left))\n        if height[left] < height[right]:\n            left += 1\n        else:\n            right -= 1\n    return best\nprint(max_area([1, 8, 6, 2, 5, 4, 8, 3, 7]))",
    solution_example: "def max_area(height):\n    left, right = 0, len(height) - 1\n    best = 0\n    while left < right:\n        best = max(best, min(height[left], height[right]) * (right - left))\n        if height[left] < height[right]:\n            left += 1\n        else:\n            right -= 1\n    return best\nprint(max_area([1, 8, 6, 2, 5, 4, 8, 3, 7]))\n",
    next: Some("py-176-three-sum"),
    show_type_chips: false,
    micro_step: 175,
};

pub const PY176_THREE_SUM: CodingStep = CodingStep {
    id: "py-176-three-sum",
    title: "DSA 3Sum",
    objective: "Encontrar triplets únicos que sumen cero.",
    prompt_md: "**3Sum**\n\nOrdená, fijá un índice y usá two-pointers; saltá duplicados.\n\n**Micro-reto:**\n1. Definí `three_sum(nums)` que devolvé triplets ordenados y únicos\n2. Imprimí `three_sum([-1, 0, 1, 2, -1, -4])` (esperado: `[[-1, -1, 2], [-1, 0, 1]]`)",
    starter_code: "# def three_sum(nums):\n#     ...\n# print(three_sum([-1, 0, 1, 2, -1, -4]))\n",
    pytest: "def test_three_sum(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('three_sum'))\n    assert ns['three_sum']([-1, 0, 1, 2, -1, -4]) == [[-1, -1, 2], [-1, 0, 1]]\n    assert ns['three_sum']([0, 1, 1]) == []\n    assert ns['three_sum']([0, 0, 0]) == [[0, 0, 0]]\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[[-1, -1, 2], [-1, 0, 1]]']\n",
    hint: "def three_sum(nums):\n    nums = sorted(nums)\n    result = []\n    for i in range(len(nums)):\n        if i and nums[i] == nums[i - 1]:\n            continue\n        lo, hi = i + 1, len(nums) - 1\n        while lo < hi:\n            total = nums[i] + nums[lo] + nums[hi]\n            if total == 0:\n                result.append([nums[i], nums[lo], nums[hi]])\n                lo += 1\n                hi -= 1\n                while lo < hi and nums[lo] == nums[lo - 1]:\n                    lo += 1\n                while lo < hi and nums[hi] == nums[hi + 1]:\n                    hi -= 1\n            elif total < 0:\n                lo += 1\n            else:\n                hi -= 1\n    return result\nprint(three_sum([-1, 0, 1, 2, -1, -4]))",
    solution_example: "def three_sum(nums):\n    nums = sorted(nums)\n    result = []\n    for i in range(len(nums)):\n        if i and nums[i] == nums[i - 1]:\n            continue\n        lo, hi = i + 1, len(nums) - 1\n        while lo < hi:\n            total = nums[i] + nums[lo] + nums[hi]\n            if total == 0:\n                result.append([nums[i], nums[lo], nums[hi]])\n                lo += 1\n                hi -= 1\n                while lo < hi and nums[lo] == nums[lo - 1]:\n                    lo += 1\n                while lo < hi and nums[hi] == nums[hi + 1]:\n                    hi -= 1\n            elif total < 0:\n                lo += 1\n            else:
                hi -= 1\n    return result\nprint(three_sum([-1, 0, 1, 2, -1, -4]))\n",
    next: Some("py-177-trapping-rain"),
    show_type_chips: false,
    micro_step: 176,
};

pub const PY177_TRAPPING_RAIN: CodingStep = CodingStep {
    id: "py-177-trapping-rain",
    title: "DSA Trapping Rain Water",
    objective: "Calcular cuánta agua queda atrapada entre barras.",
    prompt_md: "**Trapping Rain Water**\n\nDos punteros + máximos laterales: el agua en un lado es `max_lado - altura`.\n\n**Micro-reto:**\n1. Definí `trap(height)`\n2. Imprimí `trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1])` (esperado: `6`)",
    starter_code: "# def trap(height):\n#     ...\n# print(trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]))\n",
    pytest: "def test_trapping_rain(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('trap'))\n    assert ns['trap']([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]) == 6\n    assert ns['trap']([4, 2, 0, 3, 2, 5]) == 9\n    assert ns['trap']([]) == 0\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['6']\n",
    hint: "def trap(height):\n    if not height:\n        return 0\n    left, right = 0, len(height) - 1\n    left_max = right_max = water = 0\n    while left < right:\n        if height[left] < height[right]:\n            if height[left] >= left_max:\n                left_max = height[left]\n            else:\n                water += left_max - height[left]\n            left += 1\n        else:\n            if height[right] >= right_max:\n                right_max = height[right]\n            else:\n                water += right_max - height[right]\n            right -= 1\n    return water\nprint(trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]))",
    solution_example: "def trap(height):\n    if not height:\n        return 0\n    left, right = 0, len(height) - 1\n    left_max = right_max = water = 0\n    while left < right:\n        if height[left] < height[right]:\n            if height[left] >= left_max:\n                left_max = height[left]\n            else:\n                water += left_max - height[left]\n            left += 1\n        else:\n            if height[right] >= right_max:\n                right_max = height[right]\n            else:\n                water += right_max - height[right]\n            right -= 1\n    return water\nprint(trap([0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]))\n",
    next: Some("py-178-group-anagrams"),
    show_type_chips: false,
    micro_step: 177,
};

pub const PY178_GROUP_ANAGRAMS: CodingStep = CodingStep {
    id: "py-178-group-anagrams",
    title: "DSA Group Anagrams",
    objective: "Agrupar strings que son anagramas entre sí.",
    prompt_md: "**Group Anagrams**\n\nUsá la firma ordenada de cada string como clave del grupo.\n\n**Micro-reto:**\n1. Definí `group_anagrams(strs)`\n2. Imprimí `sorted([sorted(g) for g in group_anagrams(['eat', 'tea', 'tan', 'ate', 'nat', 'bat'])])` (esperado: `[['ate', 'eat', 'tea'], ['bat'], ['nat', 'tan']]`)",
    starter_code: "# def group_anagrams(strs):\n#     ...\n# print(sorted([sorted(g) for g in group_anagrams(['eat', 'tea', 'tan', 'ate', 'nat', 'bat'])]))\n",
    pytest: "def test_group_anagrams(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('group_anagrams'))\n    groups = [sorted(g) for g in ns['group_anagrams'](['eat', 'tea', 'tan', 'ate', 'nat', 'bat'])]\n    assert sorted(groups) == [['ate', 'eat', 'tea'], ['bat'], ['nat', 'tan']]\n    assert sorted([sorted(g) for g in ns['group_anagrams']([''])]) == [['']]\n    assert sorted([sorted(g) for g in ns['group_anagrams'](['a'])]) == [['a']]\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == [\"[['ate', 'eat', 'tea'], ['bat'], ['nat', 'tan']]\"]\n",
    hint: "def group_anagrams(strs):\n    groups = {}\n    for s in strs:\n        key = ''.join(sorted(s))\n        groups.setdefault(key, []).append(s)\n    return list(groups.values())\nprint(sorted([sorted(g) for g in group_anagrams(['eat', 'tea', 'tan', 'ate', 'nat', 'bat'])]))",
    solution_example: "def group_anagrams(strs):\n    groups = {}\n    for s in strs:\n        key = ''.join(sorted(s))\n        groups.setdefault(key, []).append(s)\n    return list(groups.values())\nprint(sorted([sorted(g) for g in group_anagrams(['eat', 'tea', 'tan', 'ate', 'nat', 'bat'])]))\n",
    next: Some("py-179-daily-temps"),
    show_type_chips: false,
    micro_step: 178,
};

pub const PY179_DAILY_TEMPS: CodingStep = CodingStep {
    id: "py-179-daily-temps",
    title: "DSA Daily Temperatures",
    objective: "Días hasta una temperatura más cálida (mono-stack).",
    prompt_md: "**Daily Temperatures**\n\nStack monotónico decreciente de índices.\n\n**Micro-reto:**\n1. Definí `daily_temperatures(temps)`\n2. Imprimí `daily_temperatures([73, 74, 75, 71, 69, 72, 76, 73])` (esperado: `[1, 1, 4, 2, 1, 1, 0, 0]`)",
    starter_code: "# def daily_temperatures(temps):\n#     n = len(temps)\n#     ans = [0] * n\n#     stack = []\n#     for i, t in enumerate(temps):\n#         while stack and temps[stack[-1]] < t:\n#             j = stack.pop()\n#             ans[j] = i - j\n#         stack.append(i)\n#     return ans\n# print(daily_temperatures([73, 74, 75, 71, 69, 72, 76, 73]))\n",
    pytest: "def test_daily_temperatures(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('daily_temperatures'))\n    assert ns['daily_temperatures']([73, 74, 75, 71, 69, 72, 76, 73]) == [1, 1, 4, 2, 1, 1, 0, 0]\n    assert ns['daily_temperatures']([30, 40, 50, 60]) == [1, 1, 1, 0]\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[1, 1, 4, 2, 1, 1, 0, 0]']\n",
    hint: "def daily_temperatures(temps):\n    n = len(temps)\n    ans = [0] * n\n    stack = []\n    for i, t in enumerate(temps):\n        while stack and temps[stack[-1]] < t:\n            j = stack.pop()\n            ans[j] = i - j\n        stack.append(i)\n    return ans\nprint(daily_temperatures([73, 74, 75, 71, 69, 72, 76, 73]))",
    solution_example: "def daily_temperatures(temps):\n    n = len(temps)\n    ans = [0] * n\n    stack = []\n    for i, t in enumerate(temps):\n        while stack and temps[stack[-1]] < t:\n            j = stack.pop()\n            ans[j] = i - j\n        stack.append(i)\n    return ans\nprint(daily_temperatures([73, 74, 75, 71, 69, 72, 76, 73]))\n",
    next: Some("py-180-next-greater"),
    show_type_chips: false,
    micro_step: 179,
};

pub const PY180_NEXT_GREATER: CodingStep = CodingStep {
    id: "py-180-next-greater",
    title: "DSA Next Greater Element",
    objective: "Próximo mayor a la derecha con stack monotónico.",
    prompt_md: "**Next Greater Element**\n\nPara cada índice, el primer valor mayor a la derecha (o -1).\n\n**Micro-reto:**\n1. Definí `next_greater(nums)`\n2. Imprimí `next_greater([2, 1, 2, 4, 3])` (esperado: `[4, 2, 4, -1, -1]`)",
    starter_code: "# def next_greater(nums):\n#     n = len(nums)\n#     ans = [-1] * n\n#     stack = []\n#     for i, x in enumerate(nums):\n#         while stack and nums[stack[-1]] < x:\n#             ans[stack.pop()] = x\n#         stack.append(i)\n#     return ans\n# print(next_greater([2, 1, 2, 4, 3]))\n",
    pytest: "def test_next_greater(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('next_greater'))\n    assert ns['next_greater']([2, 1, 2, 4, 3]) == [4, 2, 4, -1, -1]\n    assert ns['next_greater']([1, 2, 3]) == [2, 3, -1]\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[4, 2, 4, -1, -1]']\n",
    hint: "def next_greater(nums):\n    n = len(nums)\n    ans = [-1] * n\n    stack = []\n    for i, x in enumerate(nums):\n        while stack and nums[stack[-1]] < x:\n            ans[stack.pop()] = x\n        stack.append(i)\n    return ans\nprint(next_greater([2, 1, 2, 4, 3]))",
    solution_example: "def next_greater(nums):\n    n = len(nums)\n    ans = [-1] * n\n    stack = []\n    for i, x in enumerate(nums):\n        while stack and nums[stack[-1]] < x:\n            ans[stack.pop()] = x\n        stack.append(i)\n    return ans\nprint(next_greater([2, 1, 2, 4, 3]))\n",
    next: Some("py-181-eval-rpn"),
    show_type_chips: false,
    micro_step: 180,
};

pub const PY181_EVAL_RPN: CodingStep = CodingStep {
    id: "py-181-eval-rpn",
    title: "DSA Evaluate RPN",
    objective: "Evaluar expresión en notación polaca inversa con stack.",
    prompt_md: "**Evaluate Reverse Polish Notation**\n\nOperá con stack; división trunca hacia 0.\n\n**Micro-reto:**\n1. Definí `eval_rpn(tokens)`\n2. Imprimí `eval_rpn(['2', '1', '+', '3', '*'])` (esperado: `9`)",
    starter_code: "# def eval_rpn(tokens):\n#     stack = []\n#     for t in tokens:\n#         if t in '+-*/':\n#             b, a = stack.pop(), stack.pop()\n#             if t == '+':\n#                 stack.append(a + b)\n#             elif t == '-':\n#                 stack.append(a - b)\n#             elif t == '*':\n#                 stack.append(a * b)\n#             else:\n#                 stack.append(int(a / b))\n#         else:\n#             stack.append(int(t))\n#     return stack[0]\n# print(eval_rpn(['2', '1', '+', '3', '*']))\n",
    pytest: "def test_eval_rpn(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('eval_rpn'))\n    assert ns['eval_rpn'](['2', '1', '+', '3', '*']) == 9\n    assert ns['eval_rpn'](['4', '13', '5', '/', '+']) == 6\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['9']\n",
    hint: "def eval_rpn(tokens):\n    stack = []\n    for t in tokens:\n        if t in '+-*/':\n            b, a = stack.pop(), stack.pop()\n            if t == '+':\n                stack.append(a + b)\n            elif t == '-':\n                stack.append(a - b)\n            elif t == '*':\n                stack.append(a * b)\n            else:\n                stack.append(int(a / b))\n        else:\n            stack.append(int(t))\n    return stack[0]\nprint(eval_rpn(['2', '1', '+', '3', '*']))",
    solution_example: "def eval_rpn(tokens):\n    stack = []\n    for t in tokens:\n        if t in '+-*/':\n            b, a = stack.pop(), stack.pop()\n            if t == '+':\n                stack.append(a + b)\n            elif t == '-':\n                stack.append(a - b)\n            elif t == '*':\n                stack.append(a * b)\n            else:\n                stack.append(int(a / b))\n        else:\n            stack.append(int(t))\n    return stack[0]\nprint(eval_rpn(['2', '1', '+', '3', '*']))\n",
    next: Some("py-182-queue-stacks"),
    show_type_chips: false,
    micro_step: 181,
};

pub const PY182_QUEUE_STACKS: CodingStep = CodingStep {
    id: "py-182-queue-stacks",
    title: "DSA Queue With Stacks",
    objective: "Implementar cola FIFO con dos stacks.",
    prompt_md: "**Implement Queue using Stacks**\n\n`push` en stack in; `pop`/`peek` desde out (volcá when empty).\n\n**Micro-reto:**\n1. Definí `class MyQueue` con `push`, `pop`, `peek`, `empty`\n2. Ejecutá push(1), push(2) e imprimí `[peek(), pop(), empty()]` (esperado: `[1, 1, False]`)",
    starter_code: "# class MyQueue:\n#     def __init__(self):\n#         self.inn = []\n#         self.out = []\n# \n#     def push(self, x):\n#         self.inn.append(x)\n# \n#     def pop(self):\n#         self.peek()\n#         return self.out.pop()\n# \n#     def peek(self):\n#         if not self.out:\n#             while self.inn:\n#                 self.out.append(self.inn.pop())\n#         return self.out[-1]\n# \n#     def empty(self):\n#         return not self.inn and not self.out\n# \n# q = MyQueue()\n# q.push(1)\n# q.push(2)\n# print([q.peek(), q.pop(), q.empty()])\n",
    pytest: "def test_queue_stacks(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert 'MyQueue' in ns\n    q = ns['MyQueue']()\n    q.push(1)\n    q.push(2)\n    assert q.peek() == 1\n    assert q.pop() == 1\n    assert q.empty() is False\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[1, 1, False]']\n",
    hint: "class MyQueue:\n    def __init__(self):\n        self.inn = []\n        self.out = []\n\n    def push(self, x):\n        self.inn.append(x)\n\n    def pop(self):\n        self.peek()\n        return self.out.pop()\n\n    def peek(self):\n        if not self.out:\n            while self.inn:\n                self.out.append(self.inn.pop())\n        return self.out[-1]\n\n    def empty(self):\n        return not self.inn and not self.out\n\nq = MyQueue()\nq.push(1)\nq.push(2)\nprint([q.peek(), q.pop(), q.empty()])",
    solution_example: "class MyQueue:\n    def __init__(self):\n        self.inn = []\n        self.out = []\n\n    def push(self, x):\n        self.inn.append(x)\n\n    def pop(self):\n        self.peek()\n        return self.out.pop()\n\n    def peek(self):\n        if not self.out:\n            while self.inn:\n                self.out.append(self.inn.pop())\n        return self.out[-1]\n\n    def empty(self):\n        return not self.inn and not self.out\n\nq = MyQueue()\nq.push(1)\nq.push(2)\nprint([q.peek(), q.pop(), q.empty()])\n",
    next: Some("py-183-sliding-max"),
    show_type_chips: false,
    micro_step: 182,
};

pub const PY183_SLIDING_MAX: CodingStep = CodingStep {
    id: "py-183-sliding-max",
    title: "DSA Sliding Window Maximum",
    objective: "Máximo de cada ventana de tamaño k (deque).",
    prompt_md: "**Sliding Window Maximum**\n\nDeque de índices decreciente en valor.\n\n**Micro-reto:**\n1. Definí `max_sliding_window(nums, k)`\n2. Imprimí `max_sliding_window([1, 3, -1, -3, 5, 3, 6, 7], 3)` (esperado: `[3, 3, 5, 5, 6, 7]`)",
    starter_code: "# from collections import deque\n# \n# def max_sliding_window(nums, k):\n#     dq = deque()\n#     out = []\n#     for i, x in enumerate(nums):\n#         while dq and dq[0] <= i - k:\n#             dq.popleft()\n#         while dq and nums[dq[-1]] <= x:\n#             dq.pop()\n#         dq.append(i)\n#         if i >= k - 1:\n#             out.append(nums[dq[0]])\n#     return out\n# print(max_sliding_window([1, 3, -1, -3, 5, 3, 6, 7], 3))\n",
    pytest: "def test_max_sliding_window(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('max_sliding_window'))\n    assert ns['max_sliding_window']([1, 3, -1, -3, 5, 3, 6, 7], 3) == [3, 3, 5, 5, 6, 7]\n    assert ns['max_sliding_window']([1], 1) == [1]\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[3, 3, 5, 5, 6, 7]']\n",
    hint: "from collections import deque\n\ndef max_sliding_window(nums, k):\n    dq = deque()\n    out = []\n    for i, x in enumerate(nums):\n        while dq and dq[0] <= i - k:\n            dq.popleft()\n        while dq and nums[dq[-1]] <= x:\n            dq.pop()\n        dq.append(i)\n        if i >= k - 1:\n            out.append(nums[dq[0]])\n    return out\nprint(max_sliding_window([1, 3, -1, -3, 5, 3, 6, 7], 3))",
    solution_example: "from collections import deque\n\ndef max_sliding_window(nums, k):\n    dq = deque()\n    out = []\n    for i, x in enumerate(nums):\n        while dq and dq[0] <= i - k:\n            dq.popleft()\n        while dq and nums[dq[-1]] <= x:\n            dq.pop()\n        dq.append(i)\n        if i >= k - 1:\n            out.append(nums[dq[0]])\n    return out\nprint(max_sliding_window([1, 3, -1, -3, 5, 3, 6, 7], 3))\n",
    next: Some("py-184-min-stack"),
    show_type_chips: false,
    micro_step: 183,
};

pub const PY184_MIN_STACK: CodingStep = CodingStep {
    id: "py-184-min-stack",
    title: "DSA Min Stack",
    objective: "Stack con getMin en O(1) con stack auxiliar.",
    prompt_md: "**Min Stack**\n\nMantené un stack de mínimos paralelos.\n\n**Micro-reto:**\n1. Definí `class MinStack` con `push`, `pop`, `top`, `get_min`\n2. push(-2), push(0), push(-3); imprimí `[get_min(), top tras pop, get_min]` tras esa secuencia (esperado: `[-3, 0, -2]`)",
    starter_code: "# class MinStack:\n#     def __init__(self):\n#         self.stack = []\n#         self.mins = []\n# \n#     def push(self, val):\n#         self.stack.append(val)\n#         self.mins.append(val if not self.mins else min(val, self.mins[-1]))\n# \n#     def pop(self):\n#         self.stack.pop()\n#         self.mins.pop()\n# \n#     def top(self):\n#         return self.stack[-1]\n# \n#     def get_min(self):\n#         return self.mins[-1]\n# \n# s = MinStack()\n# s.push(-2)\n# s.push(0)\n# s.push(-3)\n# a = s.get_min()\n# s.pop()\n# b = s.top()\n# c = s.get_min()\n# print([a, b, c])\n",
    pytest: "def test_min_stack(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    s = ns['MinStack']()\n    s.push(-2)\n    s.push(0)\n    s.push(-3)\n    assert s.get_min() == -3\n    s.pop()\n    assert s.top() == 0\n    assert s.get_min() == -2\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[-3, 0, -2]']\n",
    hint: "class MinStack:\n    def __init__(self):\n        self.stack = []\n        self.mins = []\n\n    def push(self, val):\n        self.stack.append(val)\n        self.mins.append(val if not self.mins else min(val, self.mins[-1]))\n\n    def pop(self):\n        self.stack.pop()\n        self.mins.pop()\n\n    def top(self):\n        return self.stack[-1]\n\n    def get_min(self):\n        return self.mins[-1]\n\ns = MinStack()\ns.push(-2)\ns.push(0)\ns.push(-3)\na = s.get_min()\ns.pop()\nb = s.top()\nc = s.get_min()\nprint([a, b, c])",
    solution_example: "class MinStack:\n    def __init__(self):\n        self.stack = []\n        self.mins = []\n\n    def push(self, val):\n        self.stack.append(val)\n        self.mins.append(val if not self.mins else min(val, self.mins[-1]))\n\n    def pop(self):\n        self.stack.pop()\n        self.mins.pop()\n\n    def top(self):\n        return self.stack[-1]\n\n    def get_min(self):\n        return self.mins[-1]\n\ns = MinStack()\ns.push(-2)\ns.push(0)\ns.push(-3)\na = s.get_min()\ns.pop()\nb = s.top()\nc = s.get_min()\nprint([a, b, c])\n",
    next: Some("py-185-first-last"),
    show_type_chips: false,
    micro_step: 184,
};

pub const PY185_FIRST_LAST: CodingStep = CodingStep {
    id: "py-185-first-last",
    title: "DSA First Last Position",
    objective: "Primera y última posición de un target en un array ordenado.",
    prompt_md: "**Find First and Last Position**\n\nDos binary searches (lower/upper bound) sobre el mismo array ordenado.\n\n**Micro-reto:**\n1. Definí `search_range(nums, target)` → `[lo, hi]` o `[-1, -1]`\n2. Imprimí `search_range([5, 7, 7, 8, 8, 10], 8)` (esperado: `[3, 4]`)",
    starter_code: "# def search_range(nums, target):\n#     ...\n# print(search_range([5, 7, 7, 8, 8, 10], 8))\n",
    pytest: "def test_search_range(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('search_range'))\n    assert ns['search_range']([5, 7, 7, 8, 8, 10], 8) == [3, 4]\n    assert ns['search_range']([5, 7, 7, 8, 8, 10], 6) == [-1, -1]\n    assert ns['search_range']([], 0) == [-1, -1]\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[3, 4]']\n",
    hint: "def search_range(nums, target):\n    def bound(first):\n        lo, hi = 0, len(nums)\n        while lo < hi:\n            mid = (lo + hi) // 2\n            if nums[mid] > target or (first and nums[mid] == target):\n                hi = mid\n            else:\n                lo = mid + 1\n        return lo\n    left = bound(True)\n    if left == len(nums) or nums[left] != target:\n        return [-1, -1]\n    return [left, bound(False) - 1]\nprint(search_range([5, 7, 7, 8, 8, 10], 8))",
    solution_example: "def search_range(nums, target):\n    def bound(first):\n        lo, hi = 0, len(nums)\n        while lo < hi:\n            mid = (lo + hi) // 2\n            if nums[mid] > target or (first and nums[mid] == target):\n                hi = mid\n            else:\n                lo = mid + 1\n        return lo\n    left = bound(True)\n    if left == len(nums) or nums[left] != target:\n        return [-1, -1]\n    return [left, bound(False) - 1]\nprint(search_range([5, 7, 7, 8, 8, 10], 8))\n",
    next: Some("py-186-peak-element"),
    show_type_chips: false,
    micro_step: 185,
};

pub const PY186_PEAK_ELEMENT: CodingStep = CodingStep {
    id: "py-186-peak-element",
    title: "DSA Peak Element",
    objective: "Encontrar un índice pico con binary search en O(log n).",
    prompt_md: "**Find Peak Element**\n\nSi `nums[mid] < nums[mid+1]`, el pico está a la derecha; si no, a la izquierda (incluido mid).\n\n**Micro-reto:**\n1. Definí `find_peak_element(nums)` → índice de un pico\n2. Imprimí `find_peak_element([1, 2, 3, 1])` (esperado: `2`)",
    starter_code: "# def find_peak_element(nums):\n#     ...\n# print(find_peak_element([1, 2, 3, 1]))\n",
    pytest: "def test_peak_element(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('find_peak_element'))\n    assert ns['find_peak_element']([1, 2, 3, 1]) == 2\n    assert ns['find_peak_element']([1, 2, 1, 3, 5, 6, 4]) in (1, 5)\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['2']\n",
    hint: "def find_peak_element(nums):\n    lo, hi = 0, len(nums) - 1\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if nums[mid] < nums[mid + 1]:\n            lo = mid + 1\n        else:\n            hi = mid\n    return lo\nprint(find_peak_element([1, 2, 3, 1]))",
    solution_example: "def find_peak_element(nums):\n    lo, hi = 0, len(nums) - 1\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if nums[mid] < nums[mid + 1]:\n            lo = mid + 1\n        else:\n            hi = mid\n    return lo\nprint(find_peak_element([1, 2, 3, 1]))\n",
    next: Some("py-187-search-rotated"),
    show_type_chips: false,
    micro_step: 186,
};

pub const PY187_SEARCH_ROTATED: CodingStep = CodingStep {
    id: "py-187-search-rotated",
    title: "DSA Search Rotated Array",
    objective: "Buscar en un array rotado ordenado en O(log n).",
    prompt_md: "**Search in Rotated Sorted Array**\n\nIdentificá qué mitad está ordenada y descartá la otra según el target.\n\n**Micro-reto:**\n1. Definí `search_rotated(nums, target)`\n2. Imprimí `search_rotated([4, 5, 6, 7, 0, 1, 2], 0)` (esperado: `4`)",
    starter_code: "# def search_rotated(nums, target):\n#     ...\n# print(search_rotated([4, 5, 6, 7, 0, 1, 2], 0))\n",
    pytest: "def test_search_rotated(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('search_rotated'))\n    assert ns['search_rotated']([4, 5, 6, 7, 0, 1, 2], 0) == 4\n    assert ns['search_rotated']([4, 5, 6, 7, 0, 1, 2], 3) == -1\n    assert ns['search_rotated']([1], 0) == -1\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['4']\n",
    hint: "def search_rotated(nums, target):\n    lo, hi = 0, len(nums) - 1\n    while lo <= hi:\n        mid = (lo + hi) // 2\n        if nums[mid] == target:\n            return mid\n        if nums[lo] <= nums[mid]:\n            if nums[lo] <= target < nums[mid]:\n                hi = mid - 1\n            else:\n                lo = mid + 1\n        else:\n            if nums[mid] < target <= nums[hi]:\n                lo = mid + 1\n            else:\n                hi = mid - 1\n    return -1\nprint(search_rotated([4, 5, 6, 7, 0, 1, 2], 0))",
    solution_example: "def search_rotated(nums, target):\n    lo, hi = 0, len(nums) - 1\n    while lo <= hi:\n        mid = (lo + hi) // 2\n        if nums[mid] == target:\n            return mid\n        if nums[lo] <= nums[mid]:\n            if nums[lo] <= target < nums[mid]:\n                hi = mid - 1\n            else:\n                lo = mid + 1\n        else:\n            if nums[mid] < target <= nums[hi]:\n                lo = mid + 1\n            else:\n                hi = mid - 1\n    return -1\nprint(search_rotated([4, 5, 6, 7, 0, 1, 2], 0))\n",
    next: Some("py-188-sqrt"),
    show_type_chips: false,
    micro_step: 187,
};

pub const PY188_SQRT: CodingStep = CodingStep {
    id: "py-188-sqrt",
    title: "DSA Sqrt Integer",
    objective: "Raíz entera por binary search (piso de sqrt).",
    prompt_md: "**Sqrt(x)**\n\nBuscá el mayor entero `m` con `m*m <= x`.\n\n**Micro-reto:**\n1. Definí `my_sqrt(x)`\n2. Imprimí `my_sqrt(8)` (esperado: `2`)",
    starter_code: "# def my_sqrt(x):\n#     ...\n# print(my_sqrt(8))\n",
    pytest: "def test_my_sqrt(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('my_sqrt'))\n    assert ns['my_sqrt'](8) == 2\n    assert ns['my_sqrt'](4) == 2\n    assert ns['my_sqrt'](0) == 0\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['2']\n",
    hint: "def my_sqrt(x):\n    if x < 2:\n        return x\n    lo, hi = 1, x // 2\n    while lo <= hi:\n        mid = (lo + hi) // 2\n        square = mid * mid\n        if square == x:\n            return mid\n        if square < x:\n            lo = mid + 1\n        else:\n            hi = mid - 1\n    return hi\nprint(my_sqrt(8))",
    solution_example: "def my_sqrt(x):\n    if x < 2:\n        return x\n    lo, hi = 1, x // 2\n    while lo <= hi:\n        mid = (lo + hi) // 2\n        square = mid * mid\n        if square == x:\n            return mid\n        if square < x:\n            lo = mid + 1\n        else:\n            hi = mid - 1\n    return hi\nprint(my_sqrt(8))\n",
    next: Some("py-189-ship-capacity"),
    show_type_chips: false,
    micro_step: 188,
};

pub const PY189_SHIP_CAPACITY: CodingStep = CodingStep {
    id: "py-189-ship-capacity",
    title: "DSA Ship Capacity",
    objective: "Capacidad mínima del barco para despachar en D días (binary search on answer).",
    prompt_md: "**Capacity To Ship Packages Within D Days**\n\nBinary search sobre la capacidad: lo = max(peso), hi = suma.\n\n**Micro-reto:**\n1. Definí `ship_within_days(weights, days)`\n2. Imprimí `ship_within_days([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5)` (esperado: `15`)",
    starter_code: "# def ship_within_days(weights, days):\n#     ...\n# print(ship_within_days([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5))\n",
    pytest: "def test_ship_capacity(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('ship_within_days'))\n    assert ns['ship_within_days']([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5) == 15\n    assert ns['ship_within_days']([3, 2, 2, 4, 1, 4], 3) == 6\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['15']\n",
    hint: "def ship_within_days(weights, days):\n    def feasible(capacity):\n        needed = 1\n        current = 0\n        for weight in weights:\n            if current + weight > capacity:\n                needed += 1\n                current = weight\n                if needed > days:\n                    return False\n            else:\n                current += weight\n        return True\n\n    lo, hi = max(weights), sum(weights)\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if feasible(mid):\n            hi = mid\n        else:\n            lo = mid + 1\n    return lo\nprint(ship_within_days([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5))",
    solution_example: "def ship_within_days(weights, days):\n    def feasible(capacity):\n        needed = 1\n        current = 0\n        for weight in weights:\n            if current + weight > capacity:\n                needed += 1\n                current = weight\n                if needed > days:\n                    return False\n            else:\n                current += weight\n        return True\n\n    lo, hi = max(weights), sum(weights)\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if feasible(mid):\n            hi = mid\n        else:\n            lo = mid + 1\n    return lo\nprint(ship_within_days([1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5))\n",
    next: Some("py-190-min-rotated"),
    show_type_chips: false,
    micro_step: 189,
};

pub const PY190_MIN_ROTATED: CodingStep = CodingStep {
    id: "py-190-min-rotated",
    title: "DSA Min Rotated Array",
    objective: "Mínimo en array rotado ordenado (sin search_insert: ya cubierto por lower_bound).",
    prompt_md: "**Find Minimum in Rotated Sorted Array**\n\nSi `nums[mid] > nums[hi]`, el mínimo está a la derecha; si no, a la izquierda (incluido mid).\n\n**Micro-reto:**\n1. Definí `find_min_rotated(nums)`\n2. Imprimí `find_min_rotated([3, 4, 5, 1, 2])` (esperado: `1`)",
    starter_code: "# def find_min_rotated(nums):\n#     ...\n# print(find_min_rotated([3, 4, 5, 1, 2]))\n",
    pytest: "def test_min_rotated(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('find_min_rotated'))\n    assert ns['find_min_rotated']([3, 4, 5, 1, 2]) == 1\n    assert ns['find_min_rotated']([4, 5, 6, 7, 0, 1, 2]) == 0\n    assert ns['find_min_rotated']([11, 13, 15, 17]) == 11\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['1']\n",
    hint: "def find_min_rotated(nums):\n    lo, hi = 0, len(nums) - 1\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if nums[mid] > nums[hi]:\n            lo = mid + 1\n        else:\n            hi = mid\n    return nums[lo]\nprint(find_min_rotated([3, 4, 5, 1, 2]))",
    solution_example: "def find_min_rotated(nums):\n    lo, hi = 0, len(nums) - 1\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if nums[mid] > nums[hi]:\n            lo = mid + 1\n        else:\n            hi = mid\n    return nums[lo]\nprint(find_min_rotated([3, 4, 5, 1, 2]))\n",
    next: Some("py-191-kth-largest"),
    show_type_chips: false,
    micro_step: 190,
};
pub const PY191_KTH_LARGEST: CodingStep = CodingStep {
    id: "py-191-kth-largest",
    title: "DSA Kth Largest",
    objective: "K-ésimo mayor con heap (nlargest).",
    prompt_md: "**Kth Largest Element**\n\nUsá un heap (o `heapq.nlargest`) para obtener el k-ésimo sin ordenar todo a mano.\n\n**Micro-reto:**\n1. Definí `find_kth_largest(nums, k)`\n2. Imprimí `find_kth_largest([3, 2, 1, 5, 6, 4], 2)` (esperado: `5`)",
    starter_code: "# import heapq\n# def find_kth_largest(nums, k):\n#     ...\n# print(find_kth_largest([3, 2, 1, 5, 6, 4], 2))\n",
    pytest: "def test_kth_largest(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('find_kth_largest'))\n    assert ns['find_kth_largest']([3, 2, 1, 5, 6, 4], 2) == 5\n    assert ns['find_kth_largest']([3, 2, 3, 1, 2, 4, 5, 5, 6], 4) == 4\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['5']\n",
    hint: "import heapq\n\ndef find_kth_largest(nums, k):\n    return heapq.nlargest(k, nums)[-1]\nprint(find_kth_largest([3, 2, 1, 5, 6, 4], 2))",
    solution_example: "import heapq\n\ndef find_kth_largest(nums, k):\n    return heapq.nlargest(k, nums)[-1]\nprint(find_kth_largest([3, 2, 1, 5, 6, 4], 2))\n",
    next: Some("py-192-top-k-frequent"),
    show_type_chips: false,
    micro_step: 191,
};

pub const PY192_TOP_K_FREQ: CodingStep = CodingStep {
    id: "py-192-top-k-frequent",
    title: "DSA Top K Frequent",
    objective: "K elementos más frecuentes (Counter + heap/most_common).",
    prompt_md: "**Top K Frequent Elements**\n\nContá frecuencias y devolvé los k más frecuentes **ordenados** (determinista).\n\n**Micro-reto:**\n1. Definí `top_k_frequent(nums, k)` → lista ordenada\n2. Imprimí `top_k_frequent([1, 1, 1, 2, 2, 3], 2)` (esperado: `[1, 2]`)",
    starter_code: "# from collections import Counter\n# def top_k_frequent(nums, k):\n#     ...\n# print(top_k_frequent([1, 1, 1, 2, 2, 3], 2))\n",
    pytest: "def test_top_k_frequent(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('top_k_frequent'))\n    assert sorted(ns['top_k_frequent']([1, 1, 1, 2, 2, 3], 2)) == [1, 2]\n    assert sorted(ns['top_k_frequent']([1], 1)) == [1]\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[1, 2]']\n",
    hint: "from collections import Counter\n\ndef top_k_frequent(nums, k):\n    counts = Counter(nums)\n    return sorted(n for n, _ in counts.most_common(k))\nprint(top_k_frequent([1, 1, 1, 2, 2, 3], 2))",
    solution_example: "from collections import Counter\n\ndef top_k_frequent(nums, k):\n    counts = Counter(nums)\n    return sorted(n for n, _ in counts.most_common(k))\nprint(top_k_frequent([1, 1, 1, 2, 2, 3], 2))\n",
    next: Some("py-193-merge-k-lists"),
    show_type_chips: false,
    micro_step: 192,
};

pub const PY193_MERGE_K_LISTS: CodingStep = CodingStep {
    id: "py-193-merge-k-lists",
    title: "DSA Merge K Lists",
    objective: "Merge de k listas ordenadas (list-of-lists) con heap.",
    prompt_md: "**Merge k Sorted Lists**\n\nEntrada: lista de listas ya ordenadas. Heap de `(valor, índice_lista, índice_en_lista)`.\n\n**Micro-reto:**\n1. Definí `merge_k_lists(lists)`\n2. Imprimí `merge_k_lists([[1, 4, 5], [1, 3, 4], [2, 6]])` (esperado: `[1, 1, 2, 3, 4, 4, 5, 6]`)",
    starter_code: "# import heapq\n# def merge_k_lists(lists):\n#     ...\n# print(merge_k_lists([[1, 4, 5], [1, 3, 4], [2, 6]]))\n",
    pytest: "def test_merge_k_lists(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('merge_k_lists'))\n    assert ns['merge_k_lists']([[1, 4, 5], [1, 3, 4], [2, 6]]) == [1, 1, 2, 3, 4, 4, 5, 6]\n    assert ns['merge_k_lists']([[]]) == []\n    assert ns['merge_k_lists']([]) == []\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[1, 1, 2, 3, 4, 4, 5, 6]']\n",
    hint: "import heapq\n\ndef merge_k_lists(lists):\n    heap = []\n    for i, lst in enumerate(lists):\n        if lst:\n            heapq.heappush(heap, (lst[0], i, 0))\n    result = []\n    while heap:\n        value, list_i, idx = heapq.heappop(heap)\n        result.append(value)\n        if idx + 1 < len(lists[list_i]):\n            heapq.heappush(heap, (lists[list_i][idx + 1], list_i, idx + 1))\n    return result\nprint(merge_k_lists([[1, 4, 5], [1, 3, 4], [2, 6]]))",
    solution_example: "import heapq\n\ndef merge_k_lists(lists):\n    heap = []\n    for i, lst in enumerate(lists):\n        if lst:\n            heapq.heappush(heap, (lst[0], i, 0))\n    result = []\n    while heap:\n        value, list_i, idx = heapq.heappop(heap)\n        result.append(value)\n        if idx + 1 < len(lists[list_i]):\n            heapq.heappush(heap, (lists[list_i][idx + 1], list_i, idx + 1))\n    return result\nprint(merge_k_lists([[1, 4, 5], [1, 3, 4], [2, 6]]))\n",
    next: Some("py-194-meeting-rooms"),
    show_type_chips: false,
    micro_step: 193,
};

pub const PY194_MEETING_ROOMS: CodingStep = CodingStep {
    id: "py-194-meeting-rooms",
    title: "DSA Meeting Rooms",
    objective: "Decidir si una persona puede asistir a todas las reuniones.",
    prompt_md: "**Meeting Rooms**\n\nOrdená por inicio; si alguna empieza antes de que termine la anterior, hay conflicto.\n\n**Micro-reto:**\n1. Definí `can_attend_meetings(intervals)`\n2. Imprimí `can_attend_meetings([[0, 30], [5, 10], [15, 20]])` (esperado: `False`)",
    starter_code: "# def can_attend_meetings(intervals):\n#     ...\n# print(can_attend_meetings([[0, 30], [5, 10], [15, 20]]))\n",
    pytest: "def test_meeting_rooms(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('can_attend_meetings'))\n    assert ns['can_attend_meetings']([[0, 30], [5, 10], [15, 20]]) is False\n    assert ns['can_attend_meetings']([[7, 10], [2, 4]]) is True\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['False']\n",
    hint: "def can_attend_meetings(intervals):\n    intervals = sorted(intervals)\n    for i in range(1, len(intervals)):\n        if intervals[i][0] < intervals[i - 1][1]:\n            return False\n    return True\nprint(can_attend_meetings([[0, 30], [5, 10], [15, 20]]))",
    solution_example: "def can_attend_meetings(intervals):\n    intervals = sorted(intervals)\n    for i in range(1, len(intervals)):\n        if intervals[i][0] < intervals[i - 1][1]:\n            return False\n    return True\nprint(can_attend_meetings([[0, 30], [5, 10], [15, 20]]))\n",
    next: Some("py-195-ugly-number"),
    show_type_chips: false,
    micro_step: 194,
};

pub const PY195_UGLY_NUMBER: CodingStep = CodingStep {
    id: "py-195-ugly-number",
    title: "DSA Ugly Number",
    objective: "n-ésimo ugly number (factores 2/3/5) con tres punteros.",
    prompt_md: "**Ugly Number II**\n\nGenerá candidatos `*2`, `*3`, `*5` desde la secuencia ya construida.\n\n**Micro-reto:**\n1. Definí `nth_ugly_number(n)`\n2. Imprimí `nth_ugly_number(10)` (esperado: `12`)",
    starter_code: "# def nth_ugly_number(n):\n#     ...\n# print(nth_ugly_number(10))\n",
    pytest: "def test_ugly_number(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('nth_ugly_number'))\n    assert ns['nth_ugly_number'](10) == 12\n    assert ns['nth_ugly_number'](1) == 1\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['12']\n",
    hint: "def nth_ugly_number(n):\n    ugly = [1]\n    i2 = i3 = i5 = 0\n    while len(ugly) < n:\n        n2, n3, n5 = ugly[i2] * 2, ugly[i3] * 3, ugly[i5] * 5\n        nxt = min(n2, n3, n5)\n        ugly.append(nxt)\n        if nxt == n2:\n            i2 += 1\n        if nxt == n3:\n            i3 += 1\n        if nxt == n5:\n            i5 += 1\n    return ugly[-1]\nprint(nth_ugly_number(10))",
    solution_example: "def nth_ugly_number(n):\n    ugly = [1]\n    i2 = i3 = i5 = 0\n    while len(ugly) < n:\n        n2, n3, n5 = ugly[i2] * 2, ugly[i3] * 3, ugly[i5] * 5\n        nxt = min(n2, n3, n5)\n        ugly.append(nxt)\n        if nxt == n2:\n            i2 += 1\n        if nxt == n3:\n            i3 += 1\n        if nxt == n5:\n            i5 += 1\n    return ugly[-1]\nprint(nth_ugly_number(10))\n",
    next: Some("py-196-k-closest"),
    show_type_chips: false,
    micro_step: 195,
};

pub const PY196_K_CLOSEST: CodingStep = CodingStep {
    id: "py-196-k-closest",
    title: "DSA K Closest Points",
    objective: "K puntos más cercanos al origen (ordenar por distancia²).",
    prompt_md: "**K Closest Points to Origin**\n\nOrdená por `x²+y²`, tomá k y devolvé **ordenados** para print determinista.\n\n**Micro-reto:**\n1. Definí `k_closest(points, k)`\n2. Imprimí `k_closest([[1, 3], [-2, 2], [2, -2]], 2)` (esperado: `[[-2, 2], [2, -2]]`)",
    starter_code: "# def k_closest(points, k):\n#     ...\n# print(k_closest([[1, 3], [-2, 2], [2, -2]], 2))\n",
    pytest: "def test_k_closest(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('k_closest'))\n    assert ns['k_closest']([[1, 3], [-2, 2], [2, -2]], 2) == [[-2, 2], [2, -2]]\n    assert ns['k_closest']([[0, 1]], 1) == [[0, 1]]\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[[-2, 2], [2, -2]]']\n",
    hint: "def k_closest(points, k):\n    chosen = sorted(points, key=lambda p: p[0] * p[0] + p[1] * p[1])[:k]\n    return sorted(chosen)\nprint(k_closest([[1, 3], [-2, 2], [2, -2]], 2))",
    solution_example: "def k_closest(points, k):\n    chosen = sorted(points, key=lambda p: p[0] * p[0] + p[1] * p[1])[:k]\n    return sorted(chosen)\nprint(k_closest([[1, 3], [-2, 2], [2, -2]], 2))\n",
    next: Some("py-197-coin-change-ii"),
    show_type_chips: false,
    micro_step: 196,
};
pub const PY197_COIN_CHANGE_II: CodingStep = CodingStep {
    id: "py-197-coin-change-ii",
    title: "DSA Coin Change II",
    objective: "Contar combinaciones de monedas (no mínimo: eso es py-129).",
    prompt_md: "**Coin Change II**\n\n`dp[a] += dp[a - coin]` iterando monedas afuera (combinaciones, no permutaciones).\n\n**Micro-reto:**\n1. Definí `coin_change_ways(amount, coins)`\n2. Imprimí `coin_change_ways(5, [1, 2, 5])` (esperado: `4`)",
    starter_code: "# def coin_change_ways(amount, coins):\n#     ...\n# print(coin_change_ways(5, [1, 2, 5]))\n",
    pytest: "def test_coin_change_ways(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('coin_change_ways'))\n    assert ns['coin_change_ways'](5, [1, 2, 5]) == 4\n    assert ns['coin_change_ways'](3, [2]) == 0\n    assert ns['coin_change_ways'](10, [10]) == 1\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['4']\n",
    hint: "def coin_change_ways(amount, coins):\n    dp = [0] * (amount + 1)\n    dp[0] = 1\n    for coin in coins:\n        for a in range(coin, amount + 1):\n            dp[a] += dp[a - coin]\n    return dp[amount]\nprint(coin_change_ways(5, [1, 2, 5]))",
    solution_example: "def coin_change_ways(amount, coins):\n    dp = [0] * (amount + 1)\n    dp[0] = 1\n    for coin in coins:\n        for a in range(coin, amount + 1):\n            dp[a] += dp[a - coin]\n    return dp[amount]\nprint(coin_change_ways(5, [1, 2, 5]))\n",
    next: Some("py-198-house-robber-ii"),
    show_type_chips: false,
    micro_step: 197,
};

pub const PY198_HOUSE_ROBBER_II: CodingStep = CodingStep {
    id: "py-198-house-robber-ii",
    title: "DSA House Robber II",
    objective: "House robber en círculo (excluir primera o última).",
    prompt_md: "**House Robber II**\n\nCasas en círculo: resolvé dos líneas (`nums[:-1]` y `nums[1:]`) y quedate con el máximo.\n\n**Micro-reto:**\n1. Definí `rob_circular(nums)`\n2. Imprimí `rob_circular([2, 3, 2])` (esperado: `3`)",
    starter_code: "# def rob_circular(nums):\n#     ...\n# print(rob_circular([2, 3, 2]))\n",
    pytest: "def test_house_robber_ii(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('rob_circular'))\n    assert ns['rob_circular']([2, 3, 2]) == 3\n    assert ns['rob_circular']([1, 2, 3, 1]) == 4\n    assert ns['rob_circular']([1]) == 1\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['3']\n",
    hint: "def rob_circular(nums):\n    def rob_line(arr):\n        prev = cur = 0\n        for value in arr:\n            prev, cur = cur, max(cur, prev + value)\n        return cur\n    if not nums:\n        return 0\n    if len(nums) == 1:\n        return nums[0]\n    return max(rob_line(nums[:-1]), rob_line(nums[1:]))\nprint(rob_circular([2, 3, 2]))",
    solution_example: "def rob_circular(nums):\n    def rob_line(arr):\n        prev = cur = 0\n        for value in arr:\n            prev, cur = cur, max(cur, prev + value)\n        return cur\n    if not nums:\n        return 0\n    if len(nums) == 1:\n        return nums[0]\n    return max(rob_line(nums[:-1]), rob_line(nums[1:]))\nprint(rob_circular([2, 3, 2]))\n",
    next: Some("py-199-unique-paths-ii"),
    show_type_chips: false,
    micro_step: 198,
};

pub const PY199_UNIQUE_PATHS_II: CodingStep = CodingStep {
    id: "py-199-unique-paths-ii",
    title: "DSA Unique Paths II",
    objective: "Caminos en grilla con obstáculos.",
    prompt_md: "**Unique Paths II**\n\nIgual que unique paths, pero celdas con `1` aportan 0 caminos.\n\n**Micro-reto:**\n1. Definí `unique_paths_with_obstacles(obstacle_grid)`\n2. Imprimí `unique_paths_with_obstacles([[0, 0, 0], [0, 1, 0], [0, 0, 0]])` (esperado: `2`)",
    starter_code: "# def unique_paths_with_obstacles(obstacle_grid):\n#     ...\n# print(unique_paths_with_obstacles([[0, 0, 0], [0, 1, 0], [0, 0, 0]]))\n",
    pytest: "def test_unique_paths_ii(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('unique_paths_with_obstacles'))\n    assert ns['unique_paths_with_obstacles']([[0, 0, 0], [0, 1, 0], [0, 0, 0]]) == 2\n    assert ns['unique_paths_with_obstacles']([[0, 1], [0, 0]]) == 1\n    assert ns['unique_paths_with_obstacles']([[1]]) == 0\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['2']\n",
    hint: "def unique_paths_with_obstacles(obstacle_grid):\n    m, n = len(obstacle_grid), len(obstacle_grid[0])\n    dp = [[0] * n for _ in range(m)]\n    if obstacle_grid[0][0] == 1:\n        return 0\n    dp[0][0] = 1\n    for i in range(m):\n        for j in range(n):\n            if obstacle_grid[i][j] == 1:\n                dp[i][j] = 0\n                continue\n            if i == 0 and j == 0:\n                continue\n            from_up = dp[i - 1][j] if i else 0\n            from_left = dp[i][j - 1] if j else 0\n            dp[i][j] = from_up + from_left\n    return dp[-1][-1]\nprint(unique_paths_with_obstacles([[0, 0, 0], [0, 1, 0], [0, 0, 0]]))",
    solution_example: "def unique_paths_with_obstacles(obstacle_grid):\n    m, n = len(obstacle_grid), len(obstacle_grid[0])\n    dp = [[0] * n for _ in range(m)]\n    if obstacle_grid[0][0] == 1:\n        return 0\n    dp[0][0] = 1\n    for i in range(m):\n        for j in range(n):\n            if obstacle_grid[i][j] == 1:\n                dp[i][j] = 0\n                continue\n            if i == 0 and j == 0:\n                continue\n            from_up = dp[i - 1][j] if i else 0\n            from_left = dp[i][j - 1] if j else 0\n            dp[i][j] = from_up + from_left\n    return dp[-1][-1]\nprint(unique_paths_with_obstacles([[0, 0, 0], [0, 1, 0], [0, 0, 0]]))\n",
    next: Some("py-200-max-product"),
    show_type_chips: false,
    micro_step: 199,
};

pub const PY200_MAX_PRODUCT: CodingStep = CodingStep {
    id: "py-200-max-product",
    title: "DSA Max Product Subarray",
    objective: "Máximo producto de subarray contiguo (track min/max).",
    prompt_md: "**Maximum Product Subarray**\n\nLlevá producto máximo y mínimo vigentes: un negativo puede voltear el mínimo en máximo.\n\n**Micro-reto:**\n1. Definí `max_product(nums)`\n2. Imprimí `max_product([2, 3, -2, 4])` (esperado: `6`)",
    starter_code: "# def max_product(nums):\n#     ...\n# print(max_product([2, 3, -2, 4]))\n",
    pytest: "def test_max_product(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('max_product'))\n    assert ns['max_product']([2, 3, -2, 4]) == 6\n    assert ns['max_product']([-2, 0, -1]) == 0\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['6']\n",
    hint: "def max_product(nums):\n    best = imax = imin = nums[0]\n    for value in nums[1:]:\n        candidates = (value, imax * value, imin * value)\n        imax = max(candidates)\n        imin = min(candidates)\n        best = max(best, imax)\n    return best\nprint(max_product([2, 3, -2, 4]))",
    solution_example: "def max_product(nums):\n    best = imax = imin = nums[0]\n    for value in nums[1:]:\n        candidates = (value, imax * value, imin * value)\n        imax = max(candidates)\n        imin = min(candidates)\n        best = max(best, imax)\n    return best\nprint(max_product([2, 3, -2, 4]))\n",
    next: Some("py-201-partition-subset"),
    show_type_chips: false,
    micro_step: 200,
};

pub const PY201_PARTITION_SUBSET: CodingStep = CodingStep {
    id: "py-201-partition-subset",
    title: "DSA Partition Equal Subset",
    objective: "¿Se puede particionar en dos subsets con la misma suma?",
    prompt_md: "**Partition Equal Subset Sum**\n\nTarget = suma/2; DP de alcanzábilidad (bitset o bool array).\n\n**Micro-reto:**\n1. Definí `can_partition(nums)`\n2. Imprimí `can_partition([1, 5, 11, 5])` (esperado: `True`)",
    starter_code: "# def can_partition(nums):\n#     ...\n# print(can_partition([1, 5, 11, 5]))\n",
    pytest: "def test_partition_subset(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('can_partition'))\n    assert ns['can_partition']([1, 5, 11, 5]) is True\n    assert ns['can_partition']([1, 2, 3, 5]) is False\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['True']\n",
    hint: "def can_partition(nums):\n    total = sum(nums)\n    if total % 2:\n        return False\n    target = total // 2\n    reachable = 1\n    for value in nums:\n        reachable |= reachable << value\n    return bool(reachable & (1 << target))\nprint(can_partition([1, 5, 11, 5]))",
    solution_example: "def can_partition(nums):\n    total = sum(nums)\n    if total % 2:\n        return False\n    target = total // 2\n    reachable = 1\n    for value in nums:\n        reachable |= reachable << value\n    return bool(reachable & (1 << target))\nprint(can_partition([1, 5, 11, 5]))\n",
    next: Some("py-202-perfect-squares"),
    show_type_chips: false,
    micro_step: 201,
};

pub const PY202_PERFECT_SQUARES: CodingStep = CodingStep {
    id: "py-202-perfect-squares",
    title: "DSA Perfect Squares",
    objective: "Mínima cantidad de cuadrados perfectos que suman n.",
    prompt_md: "**Perfect Squares**\n\n`dp[i] = min(dp[i - j²] + 1)` para todo `j² <= i`.\n\n**Micro-reto:**\n1. Definí `num_squares(n)`\n2. Imprimí `num_squares(12)` (esperado: `3`)",
    starter_code: "# def num_squares(n):\n#     ...\n# print(num_squares(12))\n",
    pytest: "def test_perfect_squares(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('num_squares'))\n    assert ns['num_squares'](12) == 3\n    assert ns['num_squares'](13) == 2\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['3']\n",
    hint: "def num_squares(n):\n    dp = [0] + [float('inf')] * n\n    for i in range(1, n + 1):\n        j = 1\n        while j * j <= i:\n            dp[i] = min(dp[i], dp[i - j * j] + 1)\n            j += 1\n    return int(dp[n])\nprint(num_squares(12))",
    solution_example: "def num_squares(n):\n    dp = [0] + [float('inf')] * n\n    for i in range(1, n + 1):\n        j = 1\n        while j * j <= i:\n            dp[i] = min(dp[i], dp[i - j * j] + 1)\n            j += 1\n    return int(dp[n])\nprint(num_squares(12))\n",
    next: Some("py-203-num-islands"),
    show_type_chips: false,
    micro_step: 202,
};
pub const PY203_NUM_ISLANDS: CodingStep = CodingStep {
    id: "py-203-num-islands",
    title: "DSA Number of Islands",
    objective: "Contar islas de unos con DFS/BFS en grilla.",
    prompt_md: "**Number of Islands**\n\nCada isla es un componente 4-conectado de `\"1\"`. Al visitarla, marcála como agua.\n\n**Micro-reto:**\n1. Definí `num_islands(grid)` (podés mutar la grilla)\n2. Imprimí el resultado para la grilla del hint (esperado: `3`)",
    starter_code: "# def num_islands(grid):\n#     ...\n# print(num_islands([[\"1\", \"1\", \"0\", \"0\", \"0\"], [\"1\", \"1\", \"0\", \"0\", \"0\"], [\"0\", \"0\", \"1\", \"0\", \"0\"], [\"0\", \"0\", \"0\", \"1\", \"1\"]]))\n",
    pytest: "def test_num_islands(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('num_islands'))\n    g1 = [[\"1\", \"1\", \"0\", \"0\", \"0\"], [\"1\", \"1\", \"0\", \"0\", \"0\"], [\"0\", \"0\", \"1\", \"0\", \"0\"], [\"0\", \"0\", \"0\", \"1\", \"1\"]]\n    assert ns['num_islands']([row[:] for row in g1]) == 3\n    g2 = [[\"1\", \"1\", \"1\"], [\"0\", \"1\", \"0\"], [\"1\", \"1\", \"1\"]]\n    assert ns['num_islands']([row[:] for row in g2]) == 1\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['3']\n",
    hint: "def num_islands(grid):\n    if not grid:\n        return 0\n    rows, cols = len(grid), len(grid[0])\n    count = 0\n\n    def dfs(i, j):\n        if i < 0 or j < 0 or i >= rows or j >= cols or grid[i][j] != \"1\":\n            return\n        grid[i][j] = \"0\"\n        dfs(i + 1, j)\n        dfs(i - 1, j)\n        dfs(i, j + 1)\n        dfs(i, j - 1)\n\n    for i in range(rows):\n        for j in range(cols):\n            if grid[i][j] == \"1\":\n                count += 1\n                dfs(i, j)\n    return count\n\nprint(num_islands([\n    [\"1\", \"1\", \"0\", \"0\", \"0\"],\n    [\"1\", \"1\", \"0\", \"0\", \"0\"],\n    [\"0\", \"0\", \"1\", \"0\", \"0\"],\n    [\"0\", \"0\", \"0\", \"1\", \"1\"],\n]))",
    solution_example: "def num_islands(grid):\n    if not grid:\n        return 0\n    rows, cols = len(grid), len(grid[0])\n    count = 0\n\n    def dfs(i, j):\n        if i < 0 or j < 0 or i >= rows or j >= cols or grid[i][j] != \"1\":\n            return\n        grid[i][j] = \"0\"\n        dfs(i + 1, j)\n        dfs(i - 1, j)\n        dfs(i, j + 1)\n        dfs(i, j - 1)\n\n    for i in range(rows):\n        for j in range(cols):\n            if grid[i][j] == \"1\":\n                count += 1\n                dfs(i, j)\n    return count\n\nprint(num_islands([\n    [\"1\", \"1\", \"0\", \"0\", \"0\"],\n    [\"1\", \"1\", \"0\", \"0\", \"0\"],\n    [\"0\", \"0\", \"1\", \"0\", \"0\"],\n    [\"0\", \"0\", \"0\", \"1\", \"1\"],\n]))\n",
    next: Some("py-204-clone-graph"),
    show_type_chips: false,
    micro_step: 203,
};

pub const PY204_CLONE_GRAPH: CodingStep = CodingStep {
    id: "py-204-clone-graph",
    title: "DSA Clone Graph",
    objective: "Clonar un grafo no dirigido (nodos con neighbors).",
    prompt_md: "**Clone Graph**\n\nBFS/DFS con un mapa `original → clon` para cablear neighbors sin ciclos infinitos.\n\n**Micro-reto:**\n1. Definí `class Node` con `val` y `neighbors`\n2. Definí `clone_graph(node)`\n3. Construí el cuadrado 1—2—3—4—1 e imprimí `sorted` de vals vecinos del clon de 1 (esperado: `[2, 4]`)",
    starter_code: "# class Node:\n#     ...\n# def clone_graph(node):\n#     ...\n# ...\n# print(sorted(neighbor.val for neighbor in cloned.neighbors))\n",
    pytest: "def test_clone_graph(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('clone_graph'))\n    Node = ns['Node']\n    a, b = Node(1), Node(2)\n    a.neighbors = [b]\n    b.neighbors = [a]\n    cloned = ns['clone_graph'](a)\n    assert cloned is not a\n    assert cloned.val == 1\n    assert [n.val for n in cloned.neighbors] == [2]\n    assert cloned.neighbors[0] is not b\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[2, 4]']\n",
    hint: "from collections import deque\n\nclass Node:\n    def __init__(self, val):\n        self.val = val\n        self.neighbors = []\n\ndef clone_graph(node):\n    if node is None:\n        return None\n    mapping = {node: Node(node.val)}\n    queue = deque([node])\n    while queue:\n        current = queue.popleft()\n        for neighbor in current.neighbors:\n            if neighbor not in mapping:\n                mapping[neighbor] = Node(neighbor.val)\n                queue.append(neighbor)\n            mapping[current].neighbors.append(mapping[neighbor])\n    return mapping[node]\n\nn1, n2, n3, n4 = Node(1), Node(2), Node(3), Node(4)\nn1.neighbors = [n2, n4]\nn2.neighbors = [n1, n3]\nn3.neighbors = [n2, n4]\nn4.neighbors = [n1, n3]\ncloned = clone_graph(n1)\nprint(sorted(neighbor.val for neighbor in cloned.neighbors))",
    solution_example: "from collections import deque\n\nclass Node:\n    def __init__(self, val):\n        self.val = val\n        self.neighbors = []\n\ndef clone_graph(node):\n    if node is None:\n        return None\n    mapping = {node: Node(node.val)}\n    queue = deque([node])\n    while queue:\n        current = queue.popleft()\n        for neighbor in current.neighbors:\n            if neighbor not in mapping:\n                mapping[neighbor] = Node(neighbor.val)\n                queue.append(neighbor)\n            mapping[current].neighbors.append(mapping[neighbor])\n    return mapping[node]\n\nn1, n2, n3, n4 = Node(1), Node(2), Node(3), Node(4)\nn1.neighbors = [n2, n4]\nn2.neighbors = [n1, n3]\nn3.neighbors = [n2, n4]\nn4.neighbors = [n1, n3]\ncloned = clone_graph(n1)\nprint(sorted(neighbor.val for neighbor in cloned.neighbors))\n",
    next: Some("py-205-course-schedule"),
    show_type_chips: false,
    micro_step: 204,
};

pub const PY205_COURSE_SCHEDULE: CodingStep = CodingStep {
    id: "py-205-course-schedule",
    title: "DSA Course Schedule",
    objective: "Detectar si el grafo de prerequisitos es acíclico (Kahn).",
    prompt_md: "**Course Schedule**\n\nArista `prep → course`. Topo-sort por indegree; si procesás todos, no hay ciclo.\n\n**Micro-reto:**\n1. Definí `can_finish(num_courses, prerequisites)`\n2. Imprimí `can_finish(2, [[1, 0]])` (esperado: `True`)",
    starter_code: "# from collections import defaultdict, deque\n# def can_finish(num_courses, prerequisites):\n#     ...\n# print(can_finish(2, [[1, 0]]))\n",
    pytest: "def test_course_schedule(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('can_finish'))\n    assert ns['can_finish'](2, [[1, 0]]) is True\n    assert ns['can_finish'](2, [[1, 0], [0, 1]]) is False\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['True']\n",
    hint: "from collections import defaultdict, deque\n\ndef can_finish(num_courses, prerequisites):\n    adj = defaultdict(list)\n    indeg = [0] * num_courses\n    for course, prep in prerequisites:\n        adj[prep].append(course)\n        indeg[course] += 1\n    queue = deque([i for i in range(num_courses) if indeg[i] == 0])\n    seen = 0\n    while queue:\n        node = queue.popleft()\n        seen += 1\n        for nxt in adj[node]:\n            indeg[nxt] -= 1\n            if indeg[nxt] == 0:\n                queue.append(nxt)\n    return seen == num_courses\nprint(can_finish(2, [[1, 0]]))",
    solution_example: "from collections import defaultdict, deque\n\ndef can_finish(num_courses, prerequisites):\n    adj = defaultdict(list)\n    indeg = [0] * num_courses\n    for course, prep in prerequisites:\n        adj[prep].append(course)\n        indeg[course] += 1\n    queue = deque([i for i in range(num_courses) if indeg[i] == 0])\n    seen = 0\n    while queue:\n        node = queue.popleft()\n        seen += 1\n        for nxt in adj[node]:\n            indeg[nxt] -= 1\n            if indeg[nxt] == 0:\n                queue.append(nxt)\n    return seen == num_courses\nprint(can_finish(2, [[1, 0]]))\n",
    next: Some("py-206-pacific-atlantic"),
    show_type_chips: false,
    micro_step: 205,
};

pub const PY206_PACIFIC_ATLANTIC: CodingStep = CodingStep {
    id: "py-206-pacific-atlantic",
    title: "DSA Pacific Atlantic",
    objective: "Celdas que drenan a Pacífico y Atlántico (BFS inverso).",
    prompt_md: "**Pacific Atlantic Water Flow**\n\nBFS desde ambas costas hacia adentro (solo subir o igual). Intersección ordenada.\n\n**Micro-reto:**\n1. Definí `pacific_atlantic(heights)` → lista ordenada de `[r, c]`\n2. Imprimí el resultado del grid del hint (esperado: `[[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]]`)",
    starter_code: "# from collections import deque\n# def pacific_atlantic(heights):\n#     ...\n# print(pacific_atlantic([[1, 2, 2, 3, 5], [3, 2, 3, 4, 4], [2, 4, 5, 3, 1], [6, 7, 1, 4, 5], [5, 1, 1, 2, 4]]))\n",
    pytest: "def test_pacific_atlantic(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('pacific_atlantic'))\n    heights = [[1, 2, 2, 3, 5], [3, 2, 3, 4, 4], [2, 4, 5, 3, 1], [6, 7, 1, 4, 5], [5, 1, 1, 2, 4]]\n    assert ns['pacific_atlantic'](heights) == [[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]]\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['[[0, 4], [1, 3], [1, 4], [2, 2], [3, 0], [3, 1], [4, 0]]']\n",
    hint: "from collections import deque\n\ndef pacific_atlantic(heights):\n    rows, cols = len(heights), len(heights[0])\n\n    def bfs(starts):\n        seen = set(starts)\n        queue = deque(starts)\n        while queue:\n            i, j = queue.popleft()\n            for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):\n                ni, nj = i + di, j + dj\n                if (\n                    0 <= ni < rows\n                    and 0 <= nj < cols\n                    and (ni, nj) not in seen\n                    and heights[ni][nj] >= heights[i][j]\n                ):\n                    seen.add((ni, nj))\n                    queue.append((ni, nj))\n        return seen\n\n    pacific = [(i, 0) for i in range(rows)] + [(0, j) for j in range(cols)]\n    atlantic = [(i, cols - 1) for i in range(rows)] + [(rows - 1, j) for j in range(cols)]\n    both = sorted(bfs(pacific) & bfs(atlantic))\n    return [[i, j] for i, j in both]\n\nprint(pacific_atlantic([\n    [1, 2, 2, 3, 5],\n    [3, 2, 3, 4, 4],\n    [2, 4, 5, 3, 1],\n    [6, 7, 1, 4, 5],\n    [5, 1, 1, 2, 4],\n]))",
    solution_example: "from collections import deque\n\ndef pacific_atlantic(heights):\n    rows, cols = len(heights), len(heights[0])\n\n    def bfs(starts):\n        seen = set(starts)\n        queue = deque(starts)\n        while queue:\n            i, j = queue.popleft()\n            for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):\n                ni, nj = i + di, j + dj\n                if (\n                    0 <= ni < rows\n                    and 0 <= nj < cols\n                    and (ni, nj) not in seen\n                    and heights[ni][nj] >= heights[i][j]\n                ):\n                    seen.add((ni, nj))\n                    queue.append((ni, nj))\n        return seen\n\n    pacific = [(i, 0) for i in range(rows)] + [(0, j) for j in range(cols)]\n    atlantic = [(i, cols - 1) for i in range(rows)] + [(rows - 1, j) for j in range(cols)]\n    both = sorted(bfs(pacific) & bfs(atlantic))\n    return [[i, j] for i, j in both]\n\nprint(pacific_atlantic([\n    [1, 2, 2, 3, 5],\n    [3, 2, 3, 4, 4],\n    [2, 4, 5, 3, 1],\n    [6, 7, 1, 4, 5],\n    [5, 1, 1, 2, 4],\n]))\n",
    next: Some("py-207-rot-oranges"),
    show_type_chips: false,
    micro_step: 206,
};

pub const PY207_ROT_ORANGES: CodingStep = CodingStep {
    id: "py-207-rot-oranges",
    title: "DSA Rotting Oranges",
    objective: "Minutos hasta podrir todas las naranjas (multi-source BFS).",
    prompt_md: "**Rotting Oranges**\n\nCola multi-fuente con minuto; si quedan frescas al final → `-1`.\n\n**Micro-reto:**\n1. Definí `oranges_rotting(grid)`\n2. Imprimí `oranges_rotting([[2, 1, 1], [1, 1, 0], [0, 1, 1]])` (esperado: `4`)",
    starter_code: "# from collections import deque\n# def oranges_rotting(grid):\n#     ...\n# print(oranges_rotting([[2, 1, 1], [1, 1, 0], [0, 1, 1]]))\n",
    pytest: "def test_rot_oranges(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('oranges_rotting'))\n    assert ns['oranges_rotting']([[2, 1, 1], [1, 1, 0], [0, 1, 1]]) == 4\n    assert ns['oranges_rotting']([[2, 1, 1], [0, 1, 1], [1, 0, 1]]) == -1\n    assert ns['oranges_rotting']([[0, 2]]) == 0\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['4']\n",
    hint: "from collections import deque\n\ndef oranges_rotting(grid):\n    rows, cols = len(grid), len(grid[0])\n    queue = deque()\n    fresh = 0\n    for i in range(rows):\n        for j in range(cols):\n            if grid[i][j] == 2:\n                queue.append((i, j, 0))\n            elif grid[i][j] == 1:\n                fresh += 1\n    minutes = 0\n    while queue:\n        i, j, t = queue.popleft()\n        minutes = t\n        for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):\n            ni, nj = i + di, j + dj\n            if 0 <= ni < rows and 0 <= nj < cols and grid[ni][nj] == 1:\n                grid[ni][nj] = 2\n                fresh -= 1\n                queue.append((ni, nj, t + 1))\n    return minutes if fresh == 0 else -1\nprint(oranges_rotting([[2, 1, 1], [1, 1, 0], [0, 1, 1]]))",
    solution_example: "from collections import deque\n\ndef oranges_rotting(grid):\n    rows, cols = len(grid), len(grid[0])\n    queue = deque()\n    fresh = 0\n    for i in range(rows):\n        for j in range(cols):\n            if grid[i][j] == 2:\n                queue.append((i, j, 0))\n            elif grid[i][j] == 1:\n                fresh += 1\n    minutes = 0\n    while queue:\n        i, j, t = queue.popleft()\n        minutes = t\n        for di, dj in ((1, 0), (-1, 0), (0, 1), (0, -1)):\n            ni, nj = i + di, j + dj\n            if 0 <= ni < rows and 0 <= nj < cols and grid[ni][nj] == 1:\n                grid[ni][nj] = 2\n                fresh -= 1\n                queue.append((ni, nj, t + 1))\n    return minutes if fresh == 0 else -1\nprint(oranges_rotting([[2, 1, 1], [1, 1, 0], [0, 1, 1]]))\n",
    next: Some("py-208-word-ladder"),
    show_type_chips: false,
    micro_step: 207,
};

pub const PY208_WORD_LADDER: CodingStep = CodingStep {
    id: "py-208-word-ladder",
    title: "DSA Word Ladder Length",
    objective: "Longitud del ladder más corto (BFS sobre vecinos 1-edit).",
    prompt_md: "**Word Ladder**\n\nBFS desde `begin_word`; cada arista cambia una letra. Devolvé la longitud (nodos), 0 si imposible.\n\n**Micro-reto:**\n1. Definí `ladder_length(begin_word, end_word, word_list)`\n2. Imprimí `ladder_length('hit', 'cog', ['hot', 'dot', 'dog', 'lot', 'log', 'cog'])` (esperado: `5`)",
    starter_code: "# from collections import deque\n# def ladder_length(begin_word, end_word, word_list):\n#     ...\n# print(ladder_length('hit', 'cog', ['hot', 'dot', 'dog', 'lot', 'log', 'cog']))\n",
    pytest: "def test_word_ladder(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('ladder_length'))\n    assert ns['ladder_length']('hit', 'cog', ['hot', 'dot', 'dog', 'lot', 'log', 'cog']) == 5\n    assert ns['ladder_length']('hit', 'cog', ['hot', 'dot', 'dog', 'lot', 'log']) == 0\n    lines = [ln.strip() for ln in capsys.readouterr().out.splitlines() if ln.strip()]\n    assert lines == ['5']\n",
    hint: "from collections import deque\n\ndef ladder_length(begin_word, end_word, word_list):\n    words = set(word_list)\n    if end_word not in words:\n        return 0\n    queue = deque([(begin_word, 1)])\n    while queue:\n        word, dist = queue.popleft()\n        if word == end_word:\n            return dist\n        for i in range(len(word)):\n            for ord_c in range(ord('a'), ord('z') + 1):\n                nxt = word[:i] + chr(ord_c) + word[i + 1:]\n                if nxt in words:\n                    words.remove(nxt)\n                    queue.append((nxt, dist + 1))\n    return 0\nprint(ladder_length('hit', 'cog', ['hot', 'dot', 'dog', 'lot', 'log', 'cog']))",
    solution_example: "from collections import deque\n\ndef ladder_length(begin_word, end_word, word_list):\n    words = set(word_list)\n    if end_word not in words:\n        return 0\n    queue = deque([(begin_word, 1)])\n    while queue:\n        word, dist = queue.popleft()\n        if word == end_word:\n            return dist\n        for i in range(len(word)):\n            for ord_c in range(ord('a'), ord('z') + 1):\n                nxt = word[:i] + chr(ord_c) + word[i + 1:]\n                if nxt in words:\n                    words.remove(nxt)\n                    queue.append((nxt, dist + 1))\n    return 0\nprint(ladder_length('hit', 'cog', ['hot', 'dot', 'dog', 'lot', 'log', 'cog']))\n",
    next: Some("py-209-lru-cache"),
    show_type_chips: false,
    micro_step: 208,
};

pub const PY209_LRU_CACHE: CodingStep = CodingStep {
    id: "py-209-lru-cache",
    title: "DSA Caché LRU",
    objective: "Implementar una caché de capacidad fija que descarte la clave menos usada recientemente.",
    prompt_md: "**LRU Cache**\n\nDefiní `LRUCache(capacity)` con `get(key)` y `put(key, value)`. Un `get` exitoso también actualiza el uso.\n\n**Micro-reto:** imprimí `[1, -1, -1, 3, 4]` para la secuencia clásica.",
    starter_code: "# class LRUCache:\n#     ...\n",
    pytest: "def test_lru_cache(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    cache = ns['LRUCache'](2)\n    cache.put(1, 1); cache.put(2, 2)\n    assert cache.get(1) == 1\n    cache.put(3, 3)\n    assert cache.get(2) == -1\n    cache.put(4, 4)\n    assert [cache.get(1), cache.get(3), cache.get(4)] == [-1, 3, 4]\n    assert capsys.readouterr().out.strip() == '[1, -1, -1, 3, 4]'\n",
    hint: "from collections import OrderedDict\n\nclass LRUCache:\n    def __init__(self, capacity): self.capacity, self.data = capacity, OrderedDict()\n    def get(self, key):\n        if key not in self.data: return -1\n        self.data.move_to_end(key); return self.data[key]\n    def put(self, key, value):\n        if key in self.data: self.data.move_to_end(key)\n        self.data[key] = value\n        if len(self.data) > self.capacity: self.data.popitem(last=False)",
    solution_example: "from collections import OrderedDict\n\nclass LRUCache:\n    def __init__(self, capacity): self.capacity, self.data = capacity, OrderedDict()\n    def get(self, key):\n        if key not in self.data: return -1\n        self.data.move_to_end(key); return self.data[key]\n    def put(self, key, value):\n        if key in self.data: self.data.move_to_end(key)\n        self.data[key] = value\n        if len(self.data) > self.capacity: self.data.popitem(last=False)\nc = LRUCache(2); c.put(1, 1); c.put(2, 2); a = c.get(1); c.put(3, 3); b = c.get(2); c.put(4, 4)\nprint([a, b, c.get(1), c.get(3), c.get(4)])\n",
    next: Some("py-210-basic-calc"), show_type_chips: false, micro_step: 209,
};

pub const PY210_BASIC_CALC: CodingStep = CodingStep {
    id: "py-210-basic-calc", title: "DSA Calculadora Básica", objective: "Evaluar sumas y restas con espacios.",
    prompt_md: "**Basic Calculator**\n\nDefiní `calculate(s)` para expresiones con enteros, `+`, `-` y espacios.\n\n**Micro-reto:** imprimí `calculate(' 2-1 + 2 ')`.",
    starter_code: "# def calculate(s):\n#     ...\n",
    pytest: "def test_basic_calc(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['calculate']('1 + 1') == 2\n    assert ns['calculate'](' 2-1 + 2 ') == 3\n    assert capsys.readouterr().out.strip() == '3'\n",
    hint: "def calculate(s):\n    total = number = 0; sign = 1\n    for char in s + '+':\n        if char.isdigit(): number = number * 10 + int(char)\n        elif char in '+-': total += sign * number; number = 0; sign = 1 if char == '+' else -1\n    return total",
    solution_example: "def calculate(s):\n    total = number = 0; sign = 1\n    for char in s + '+':\n        if char.isdigit(): number = number * 10 + int(char)\n        elif char in '+-': total += sign * number; number = 0; sign = 1 if char == '+' else -1\n    return total\nprint(calculate(' 2-1 + 2 '))\n",
    next: Some("py-211-encode-decode"), show_type_chips: false, micro_step: 210,
};

pub const PY211_ENCODE_DECODE: CodingStep = CodingStep {
    id: "py-211-encode-decode", title: "DSA Codificar y Decodificar Strings", objective: "Serializar una lista de strings sin ambigüedad.",
    prompt_md: "**Encode / Decode**\n\nDefiní `encode(strs)` y `decode(data)` con prefijos de longitud.\n\n**Micro-reto:** imprimí el roundtrip de `['hello', 'world']`.",
    starter_code: "# def encode(strs): ...\n# def decode(data): ...\n",
    pytest: "def test_encode_decode(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['decode'](ns['encode'](['', 'a#b', 'world'])) == ['', 'a#b', 'world']\n    assert capsys.readouterr().out.strip() == \"['hello', 'world']\"\n",
    hint: "def encode(strs): return ''.join(f'{len(word)}#{word}' for word in strs)\ndef decode(data):\n    out = []; i = 0\n    while i < len(data):\n        j = data.index('#', i); size = int(data[i:j]); i = j + 1\n        out.append(data[i:i + size]); i += size\n    return out",
    solution_example: "def encode(strs): return ''.join(f'{len(word)}#{word}' for word in strs)\ndef decode(data):\n    out = []; i = 0\n    while i < len(data):\n        j = data.index('#', i); size = int(data[i:j]); i = j + 1\n        out.append(data[i:i + size]); i += size\n    return out\nprint(decode(encode(['hello', 'world'])))\n",
    next: Some("py-212-randomized-set"), show_type_chips: false, micro_step: 211,
};

pub const PY212_RANDOMIZED_SET: CodingStep = CodingStep {
    id: "py-212-randomized-set", title: "DSA Conjunto Aleatorio", objective: "Insertar, eliminar y elegir en O(1) promedio.",
    prompt_md: "**Randomized Set**\n\nDefiní `RandomizedSet` con `insert`, `remove` y `get_random`. Usá una lista y un mapa de índices.\n\n**Micro-reto:** la única clave restante hace determinista `get_random()`.",
    starter_code: "# class RandomizedSet:\n#     ...\n",
    pytest: "def test_randomized_set(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    values = ns['RandomizedSet']()\n    assert values.insert(1) and values.insert(2) and not values.insert(1)\n    assert values.remove(1) and values.get_random() == 2\n    assert capsys.readouterr().out.strip() == '[True, True, False, True, 2]'\n",
    hint: "import random\n\nclass RandomizedSet:\n    def __init__(self): self.values, self.positions = [], {}\n    def insert(self, value):\n        if value in self.positions: return False\n        self.positions[value] = len(self.values); self.values.append(value); return True\n    def remove(self, value):\n        if value not in self.positions: return False\n        i = self.positions.pop(value); last = self.values.pop()\n        if i < len(self.values): self.values[i] = last; self.positions[last] = i\n        return True\n    def get_random(self): return random.choice(self.values)",
    solution_example: "import random\n\nclass RandomizedSet:\n    def __init__(self): self.values, self.positions = [], {}\n    def insert(self, value):\n        if value in self.positions: return False\n        self.positions[value] = len(self.values); self.values.append(value); return True\n    def remove(self, value):\n        if value not in self.positions: return False\n        i = self.positions.pop(value); last = self.values.pop()\n        if i < len(self.values): self.values[i] = last; self.positions[last] = i\n        return True\n    def get_random(self): return random.choice(self.values)\nr = RandomizedSet(); print([r.insert(1), r.insert(2), r.insert(1), r.remove(1), r.get_random()])\n",
    next: Some("py-213-time-kv"), show_type_chips: false, micro_step: 212,
};

pub const PY213_TIME_KV: CodingStep = CodingStep {
    id: "py-213-time-kv", title: "DSA Mapa Clave-Valor Temporal", objective: "Buscar el valor más reciente anterior a un timestamp.",
    prompt_md: "**Time Based Key-Value Store**\n\nDefiní `TimeMap` con `set` y `get` usando búsqueda binaria.\n\n**Micro-reto:** imprimí `bar`, `bar` y `bar2` para el ejemplo clásico.",
    starter_code: "# class TimeMap:\n#     ...\n",
    pytest: "def test_time_kv(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    tm = ns['TimeMap'](); tm.set('foo', 'bar', 1); tm.set('foo', 'bar2', 4)\n    assert [tm.get('foo', 1), tm.get('foo', 3), tm.get('foo', 4), tm.get('x', 9)] == ['bar', 'bar', 'bar2', '']\n    assert capsys.readouterr().out.strip() == \"['bar', 'bar', 'bar2']\"\n",
    hint: "from bisect import bisect_right\n\nclass TimeMap:\n    def __init__(self): self.data = {}\n    def set(self, key, value, timestamp): self.data.setdefault(key, []).append((timestamp, value))\n    def get(self, key, timestamp):\n        values = self.data.get(key, []); i = bisect_right(values, (timestamp, chr(0x10ffff))) - 1\n        return values[i][1] if i >= 0 else ''",
    solution_example: "from bisect import bisect_right\n\nclass TimeMap:\n    def __init__(self): self.data = {}\n    def set(self, key, value, timestamp): self.data.setdefault(key, []).append((timestamp, value))\n    def get(self, key, timestamp):\n        values = self.data.get(key, []); i = bisect_right(values, (timestamp, chr(0x10ffff))) - 1\n        return values[i][1] if i >= 0 else ''\nt = TimeMap(); t.set('foo', 'bar', 1); a = t.get('foo', 1); b = t.get('foo', 3); t.set('foo', 'bar2', 4); print([a, b, t.get('foo', 4)])\n",
    next: Some("py-214-snapshot-array"), show_type_chips: false, micro_step: 213,
};

pub const PY214_SNAPSHOT_ARRAY: CodingStep = CodingStep {
    id: "py-214-snapshot-array", title: "DSA Array de Instantáneas", objective: "Consultar valores históricos por identificador de snapshot.",
    prompt_md: "**Snapshot Array**\n\nDefiní `SnapshotArray(length)` con `set`, `snap` y `get`.\n\n**Micro-reto:** imprimí el valor guardado en el primer snapshot.",
    starter_code: "# class SnapshotArray:\n#     ...\n",
    pytest: "def test_snapshot_array(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    array = ns['SnapshotArray'](3); array.set(0, 5); snap = array.snap(); array.set(0, 6)\n    assert snap == 0 and array.get(0, 0) == 5 and array.get(1, 0) == 0\n    assert capsys.readouterr().out.strip() == '5'\n",
    hint: "from bisect import bisect_right\n\nclass SnapshotArray:\n    def __init__(self, length): self.history, self.snap_id = [[(0, 0)] for _ in range(length)], 0\n    def set(self, index, val): self.history[index].append((self.snap_id, val))\n    def snap(self): self.snap_id += 1; return self.snap_id - 1\n    def get(self, index, snap_id):\n        values = self.history[index]; return values[bisect_right(values, (snap_id, float('inf'))) - 1][1]",
    solution_example: "from bisect import bisect_right\n\nclass SnapshotArray:\n    def __init__(self, length): self.history, self.snap_id = [[(0, 0)] for _ in range(length)], 0\n    def set(self, index, val): self.history[index].append((self.snap_id, val))\n    def snap(self): self.snap_id += 1; return self.snap_id - 1\n    def get(self, index, snap_id):\n        values = self.history[index]; return values[bisect_right(values, (snap_id, float('inf'))) - 1][1]\na = SnapshotArray(3); a.set(0, 5); snap = a.snap(); a.set(0, 6); print(a.get(0, snap))\n",
    next: Some("py-215-min-window"), show_type_chips: false, micro_step: 214,
};

pub const PY215_MIN_WINDOW: CodingStep = CodingStep {
    id: "py-215-min-window", title: "DSA Ventana Mínima", objective: "Encontrar el substring mínimo que cubre todos los caracteres requeridos.",
    prompt_md: "**Minimum Window Substring**\n\nDefiní `min_window(s, t)` con ventana deslizante y conteos.\n\n**Micro-reto:** imprimí `BANC` para `ADOBECODEBANC` y `ABC`.",
    starter_code: "# def min_window(s, t):\n#     ...\n",
    pytest: "def test_min_window(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['min_window']('ADOBECODEBANC', 'ABC') == 'BANC'\n    assert ns['min_window']('a', 'aa') == ''\n    assert capsys.readouterr().out.strip() == 'BANC'\n",
    hint: "from collections import Counter\n\ndef min_window(s, t):\n    need = Counter(t); missing = len(t); left = start = end = 0\n    for right, char in enumerate(s, 1):\n        if need[char] > 0: missing -= 1\n        need[char] -= 1\n        if not missing:\n            while left < right and need[s[left]] < 0: need[s[left]] += 1; left += 1\n            if not end or right - left <= end - start: start, end = left, right\n            need[s[left]] += 1; missing += 1; left += 1\n    return s[start:end]",
    solution_example: "from collections import Counter\n\ndef min_window(s, t):\n    need = Counter(t); missing = len(t); left = start = end = 0\n    for right, char in enumerate(s, 1):\n        if need[char] > 0: missing -= 1\n        need[char] -= 1\n        if not missing:\n            while left < right and need[s[left]] < 0: need[s[left]] += 1; left += 1\n            if not end or right - left <= end - start: start, end = left, right\n            need[s[left]] += 1; missing += 1; left += 1\n    return s[start:end]\nprint(min_window('ADOBECODEBANC', 'ABC'))\n",
    next: Some("py-216-char-replace"), show_type_chips: false, micro_step: 215,
};

pub const PY216_CHAR_REPLACE: CodingStep = CodingStep {
    id: "py-216-char-replace", title: "DSA Reemplazo de Caracteres", objective: "Maximizar una ventana uniforme con hasta k reemplazos.",
    prompt_md: "**Longest Repeating Character Replacement**\n\nDefiní `character_replacement(s, k)`.\n\n**Micro-reto:** imprimí `4` para `AABABBA`, `k=1`.",
    starter_code: "# def character_replacement(s, k):\n#     ...\n",
    pytest: "def test_char_replace(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['character_replacement']('ABAB', 2) == 4\n    assert ns['character_replacement']('AABABBA', 1) == 4\n    assert capsys.readouterr().out.strip() == '4'\n",
    hint: "from collections import defaultdict\n\ndef character_replacement(s, k):\n    counts = defaultdict(int); left = best = most = 0\n    for right, char in enumerate(s):\n        counts[char] += 1; most = max(most, counts[char])\n        while right - left + 1 - most > k: counts[s[left]] -= 1; left += 1\n        best = max(best, right - left + 1)\n    return best",
    solution_example: "from collections import defaultdict\n\ndef character_replacement(s, k):\n    counts = defaultdict(int); left = best = most = 0\n    for right, char in enumerate(s):\n        counts[char] += 1; most = max(most, counts[char])\n        while right - left + 1 - most > k: counts[s[left]] -= 1; left += 1\n        best = max(best, right - left + 1)\n    return best\nprint(character_replacement('AABABBA', 1))\n",
    next: Some("py-217-find-anagrams"), show_type_chips: false, micro_step: 216,
};

pub const PY217_FIND_ANAGRAMS: CodingStep = CodingStep {
    id: "py-217-find-anagrams", title: "DSA Encontrar Anagramas", objective: "Detectar todas las posiciones de una permutación mediante ventana deslizante.",
    prompt_md: "**Find All Anagrams**\n\nDefiní `find_anagrams(s, p)` y devolvé índices iniciales.\n\n**Micro-reto:** imprimí `[0, 6]` para `cbaebabacd`, `abc`.",
    starter_code: "# def find_anagrams(s, p):\n#     ...\n",
    pytest: "def test_find_anagrams(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['find_anagrams']('cbaebabacd', 'abc') == [0, 6]\n    assert ns['find_anagrams']('abab', 'ab') == [0, 1, 2]\n    assert capsys.readouterr().out.strip() == '[0, 6]'\n",
    hint: "from collections import Counter\n\ndef find_anagrams(s, p):\n    need = Counter(p); window = Counter(); out = []\n    for i, char in enumerate(s):\n        window[char] += 1\n        if i >= len(p):\n            old = s[i - len(p)]; window[old] -= 1\n            if not window[old]: del window[old]\n        if window == need: out.append(i - len(p) + 1)\n    return out",
    solution_example: "from collections import Counter\n\ndef find_anagrams(s, p):\n    need = Counter(p); window = Counter(); out = []\n    for i, char in enumerate(s):\n        window[char] += 1\n        if i >= len(p):\n            old = s[i - len(p)]; window[old] -= 1\n            if not window[old]: del window[old]\n        if window == need: out.append(i - len(p) + 1)\n    return out\nprint(find_anagrams('cbaebabacd', 'abc'))\n",
    next: Some("py-218-decode-string"), show_type_chips: false, micro_step: 217,
};

pub const PY218_DECODE_STRING: CodingStep = CodingStep {
    id: "py-218-decode-string", title: "DSA Decodificar String", objective: "Expandir repeticiones anidadas con una pila.",
    prompt_md: "**Decode String**\n\nDefiní `decode_string(s)` para expresiones como `3[a2[c]]`.\n\n**Micro-reto:** imprimí `accaccacc`.",
    starter_code: "# def decode_string(s):\n#     ...\n",
    pytest: "def test_decode_string(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['decode_string']('3[a]2[bc]') == 'aaabcbc'\n    assert ns['decode_string']('3[a2[c]]') == 'accaccacc'\n    assert capsys.readouterr().out.strip() == 'accaccacc'\n",
    hint: "def decode_string(s):\n    stack = []; current = ''; number = 0\n    for char in s:\n        if char.isdigit(): number = number * 10 + int(char)\n        elif char == '[': stack.append((current, number)); current = ''; number = 0\n        elif char == ']': previous, repeat = stack.pop(); current = previous + current * repeat\n        else: current += char\n    return current",
    solution_example: "def decode_string(s):\n    stack = []; current = ''; number = 0\n    for char in s:\n        if char.isdigit(): number = number * 10 + int(char)\n        elif char == '[': stack.append((current, number)); current = ''; number = 0\n        elif char == ']': previous, repeat = stack.pop(); current = previous + current * repeat\n        else: current += char\n    return current\nprint(decode_string('3[a2[c]]'))\n",
    next: Some("py-219-str-compress"), show_type_chips: false, micro_step: 218,
};

pub const PY219_STR_COMPRESS: CodingStep = CodingStep {
    id: "py-219-str-compress", title: "DSA Comprimir String", objective: "Comprimir una lista de caracteres in situ.",
    prompt_md: "**String Compression**\n\nDefiní `compress(chars)`: mutá la lista y devolvé la longitud nueva.\n\n**Micro-reto:** imprimí `6` para `aabbccc`.",
    starter_code: "# def compress(chars):\n#     ...\n",
    pytest: "def test_str_compress(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    chars = list('aabbccc'); assert ns['compress'](chars) == 6 and ''.join(chars[:6]) == 'a2b2c3'\n    assert capsys.readouterr().out.strip() == '6'\n",
    hint: "def compress(chars):\n    write = read = 0\n    while read < len(chars):\n        char = chars[read]; start = read\n        while read < len(chars) and chars[read] == char: read += 1\n        chars[write] = char; write += 1\n        for digit in str(read - start): chars[write] = digit; write += 1\n    return write",
    solution_example: "def compress(chars):\n    write = read = 0\n    while read < len(chars):\n        char = chars[read]; start = read\n        while read < len(chars) and chars[read] == char: read += 1\n        chars[write] = char; write += 1\n        for digit in str(read - start): chars[write] = digit; write += 1\n    return write\nchars = list('aabbccc'); print(compress(chars))\n",
    next: Some("py-220-multiply-strings"), show_type_chips: false, micro_step: 219,
};

pub const PY220_MULTIPLY_STRINGS: CodingStep = CodingStep {
    id: "py-220-multiply-strings", title: "DSA Multiplicar Strings", objective: "Multiplicar enteros no negativos representados como strings.",
    prompt_md: "**Multiply Strings**\n\nDefiní `multiply(num1, num2)` sin convertir los operandos completos a enteros.\n\n**Micro-reto:** imprimí `56088` para `123` × `456`.",
    starter_code: "# def multiply(num1, num2):\n#     ...\n",
    pytest: "def test_multiply_strings(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['multiply']('2', '3') == '6'\n    assert ns['multiply']('123', '456') == '56088'\n    assert capsys.readouterr().out.strip() == '56088'\n",
    hint: "def multiply(num1, num2):\n    if num1 == '0' or num2 == '0': return '0'\n    digits = [0] * (len(num1) + len(num2))\n    for i, left in enumerate(reversed(num1)):\n        for j, right in enumerate(reversed(num2)):\n            digits[i + j] += int(left) * int(right)\n            digits[i + j + 1] += digits[i + j] // 10; digits[i + j] %= 10\n    return ''.join(map(str, digits[::-1])).lstrip('0')",
    solution_example: "def multiply(num1, num2):\n    if num1 == '0' or num2 == '0': return '0'\n    digits = [0] * (len(num1) + len(num2))\n    for i, left in enumerate(reversed(num1)):\n        for j, right in enumerate(reversed(num2)):\n            digits[i + j] += int(left) * int(right)\n            digits[i + j + 1] += digits[i + j] // 10; digits[i + j] %= 10\n    return ''.join(map(str, digits[::-1])).lstrip('0')\nprint(multiply('123', '456'))\n",
    next: Some("py-221-insert-interval"), show_type_chips: false, micro_step: 220,
};

pub const PY221_INSERT_INTERVAL: CodingStep = CodingStep {
    id: "py-221-insert-interval", title: "DSA Insertar Intervalo", objective: "Insertar y fusionar un intervalo ordenado.",
    prompt_md: "**Insert Interval**\n\nDefiní `insert(intervals, new_interval)`.\n\n**Micro-reto:** imprimí `[[1, 5], [6, 9]]`.",
    starter_code: "# def insert(intervals, new_interval):\n#     ...\n",
    pytest: "def test_insert_interval(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['insert']([[1, 3], [6, 9]], [2, 5]) == [[1, 5], [6, 9]]\n    assert capsys.readouterr().out.strip() == '[[1, 5], [6, 9]]'\n",
    hint: "def insert(intervals, new_interval):\n    out = []; i = 0\n    while i < len(intervals) and intervals[i][1] < new_interval[0]: out.append(intervals[i]); i += 1\n    while i < len(intervals) and intervals[i][0] <= new_interval[1]:\n        new_interval[0] = min(new_interval[0], intervals[i][0]); new_interval[1] = max(new_interval[1], intervals[i][1]); i += 1\n    return out + [new_interval] + intervals[i:]",
    solution_example: "def insert(intervals, new_interval):\n    out = []; i = 0\n    while i < len(intervals) and intervals[i][1] < new_interval[0]: out.append(intervals[i]); i += 1\n    while i < len(intervals) and intervals[i][0] <= new_interval[1]:\n        new_interval[0] = min(new_interval[0], intervals[i][0]); new_interval[1] = max(new_interval[1], intervals[i][1]); i += 1\n    return out + [new_interval] + intervals[i:]\nprint(insert([[1, 3], [6, 9]], [2, 5]))\n",
    next: Some("py-222-erase-overlap"), show_type_chips: false, micro_step: 221,
};

pub const PY222_ERASE_OVERLAP: CodingStep = CodingStep {
    id: "py-222-erase-overlap", title: "DSA Eliminar Intervalos Solapados", objective: "Conservar el máximo conjunto de intervalos compatibles.",
    prompt_md: "**Non-overlapping Intervals**\n\nDefiní `erase_overlap_intervals(intervals)` y devolvé las eliminaciones mínimas.\n\n**Micro-reto:** imprimí `1`.",
    starter_code: "# def erase_overlap_intervals(intervals):\n#     ...\n",
    pytest: "def test_erase_overlap(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['erase_overlap_intervals']([[1, 2], [2, 3], [3, 4], [1, 3]]) == 1\n    assert ns['erase_overlap_intervals']([[1, 2], [1, 2], [1, 2]]) == 2\n    assert capsys.readouterr().out.strip() == '1'\n",
    hint: "def erase_overlap_intervals(intervals):\n    end = float('-inf'); removed = 0\n    for start, finish in sorted(intervals, key=lambda item: item[1]):\n        if start < end: removed += 1\n        else: end = finish\n    return removed",
    solution_example: "def erase_overlap_intervals(intervals):\n    end = float('-inf'); removed = 0\n    for start, finish in sorted(intervals, key=lambda item: item[1]):\n        if start < end: removed += 1\n        else: end = finish\n    return removed\nprint(erase_overlap_intervals([[1, 2], [2, 3], [3, 4], [1, 3]]))\n",
    next: Some("py-223-meeting-rooms-ii"), show_type_chips: false, micro_step: 222,
};

pub const PY223_MEETING_ROOMS_II: CodingStep = CodingStep {
    id: "py-223-meeting-rooms-ii", title: "DSA Salas de Reunión II", objective: "Calcular salas simultáneas mínimas con un heap.",
    prompt_md: "**Meeting Rooms II**\n\nDefiní `min_meeting_rooms(intervals)`.\n\n**Micro-reto:** imprimí `2`.",
    starter_code: "# def min_meeting_rooms(intervals):\n#     ...\n",
    pytest: "def test_meeting_rooms_ii(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['min_meeting_rooms']([[0, 30], [5, 10], [15, 20]]) == 2\n    assert ns['min_meeting_rooms']([[7, 10], [2, 4]]) == 1\n    assert capsys.readouterr().out.strip() == '2'\n",
    hint: "import heapq\n\ndef min_meeting_rooms(intervals):\n    rooms = []\n    for start, end in sorted(intervals):\n        if rooms and rooms[0] <= start: heapq.heapreplace(rooms, end)\n        else: heapq.heappush(rooms, end)\n    return len(rooms)",
    solution_example: "import heapq\n\ndef min_meeting_rooms(intervals):\n    rooms = []\n    for start, end in sorted(intervals):\n        if rooms and rooms[0] <= start: heapq.heapreplace(rooms, end)\n        else: heapq.heappush(rooms, end)\n    return len(rooms)\nprint(min_meeting_rooms([[0, 30], [5, 10], [15, 20]]))\n",
    next: Some("py-224-single-number-ii"), show_type_chips: false, micro_step: 223,
};

pub const PY224_SINGLE_NUMBER_II: CodingStep = CodingStep {
    id: "py-224-single-number-ii", title: "DSA Número Único II", objective: "Aislar el número que no aparece tres veces.",
    prompt_md: "**Single Number II**\n\nDefiní `single_number(nums)` con la máquina de estados de bits `ones/twos`.\n\n**Micro-reto:** imprimí `3`.",
    starter_code: "# def single_number(nums):\n#     ...\n",
    pytest: "def test_single_number_ii(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['single_number']([2, 2, 3, 2]) == 3\n    assert ns['single_number']([0, 1, 0, 1, 0, 1, 99]) == 99\n    assert capsys.readouterr().out.strip() == '3'\n",
    hint: "def single_number(nums):\n    ones = twos = 0\n    for value in nums:\n        ones = (ones ^ value) & ~twos\n        twos = (twos ^ value) & ~ones\n    return ones",
    solution_example: "def single_number(nums):\n    ones = twos = 0\n    for value in nums:\n        ones = (ones ^ value) & ~twos\n        twos = (twos ^ value) & ~ones\n    return ones\nprint(single_number([2, 2, 3, 2]))\n",
    next: Some("py-225-counting-bits"), show_type_chips: false, micro_step: 224,
};

pub const PY225_COUNTING_BITS: CodingStep = CodingStep {
    id: "py-225-counting-bits", title: "DSA Contar Bits", objective: "Calcular conteos de bits de 0 a n con programación dinámica.",
    prompt_md: "**Counting Bits**\n\nDefiní `count_bits(n)` usando `bits[i] = bits[i >> 1] + (i & 1)`.\n\n**Micro-reto:** imprimí `[0, 1, 1, 2, 1, 2]`.",
    starter_code: "# def count_bits(n):\n#     ...\n",
    pytest: "def test_counting_bits(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['count_bits'](2) == [0, 1, 1]\n    assert ns['count_bits'](5) == [0, 1, 1, 2, 1, 2]\n    assert capsys.readouterr().out.strip() == '[0, 1, 1, 2, 1, 2]'\n",
    hint: "def count_bits(n):\n    bits = [0] * (n + 1)\n    for value in range(1, n + 1): bits[value] = bits[value >> 1] + (value & 1)\n    return bits",
    solution_example: "def count_bits(n):\n    bits = [0] * (n + 1)\n    for value in range(1, n + 1): bits[value] = bits[value >> 1] + (value & 1)\n    return bits\nprint(count_bits(5))\n",
    next: Some("py-226-reverse-bits"), show_type_chips: false, micro_step: 225,
};

pub const PY226_REVERSE_BITS: CodingStep = CodingStep {
    id: "py-226-reverse-bits", title: "DSA Invertir Bits", objective: "Invertir los 32 bits de un entero sin signo.",
    prompt_md: "**Reverse Bits**\n\nDefiní `reverse_bits(n)` para una palabra de 32 bits.\n\n**Micro-reto:** imprimí `964176192` para `43261596`.",
    starter_code: "# def reverse_bits(n):\n#     ...\n",
    pytest: "def test_reverse_bits(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert ns['reverse_bits'](43261596) == 964176192\n    assert ns['reverse_bits'](0) == 0\n    assert capsys.readouterr().out.strip() == '964176192'\n",
    hint: "def reverse_bits(n):\n    out = 0\n    for _ in range(32): out = (out << 1) | (n & 1); n >>= 1\n    return out",
    solution_example: "def reverse_bits(n):\n    out = 0\n    for _ in range(32): out = (out << 1) | (n & 1); n >>= 1\n    return out\nprint(reverse_bits(43261596))\n",
    next: Some("py-227-generate-parens"), show_type_chips: false, micro_step: 226,
};

pub const PY227_GENERATE_PARENS: CodingStep = CodingStep {
    id: "py-227-generate-parens", title: "DSA Generar Paréntesis", objective: "Generar todas las combinaciones válidas de n pares de paréntesis.",
    prompt_md: "**Generate Parentheses**\n\nBacktracking: sumá `(` si quedan abiertos; sumá `)` si cierran menos que abren.\n\n**Micro-reto:**\n1. Definí `generate_parenthesis(n)`\n2. Imprimí `generate_parenthesis(3)`",
    starter_code: "# def generate_parenthesis(n):\n#     ...\n# print(generate_parenthesis(3))\n",
    pytest: "def test_generate_parens(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('generate_parenthesis'))\n    assert ns['generate_parenthesis'](3) == ['((()))', '(()())', '(())()', '()(())', '()()()']\n    assert ns['generate_parenthesis'](1) == ['()']\n    assert capsys.readouterr().out.strip() == \"['((()))', '(()())', '(())()', '()(())', '()()()']\"\n",
    hint: "def generate_parenthesis(n):\n    out = []\n    def bt(s, open_n, close_n):\n        if len(s) == 2 * n:\n            out.append(s); return\n        if open_n < n: bt(s + '(', open_n + 1, close_n)\n        if close_n < open_n: bt(s + ')', open_n, close_n + 1)\n    bt('', 0, 0)\n    return out",
    solution_example: "def generate_parenthesis(n):\n    out = []\n    def bt(s, open_n, close_n):\n        if len(s) == 2 * n:\n            out.append(s); return\n        if open_n < n: bt(s + '(', open_n + 1, close_n)\n        if close_n < open_n: bt(s + ')', open_n, close_n + 1)\n    bt('', 0, 0)\n    return out\nprint(generate_parenthesis(3))\n",
    next: Some("py-228-combination-sum"), show_type_chips: false, micro_step: 227,
};

pub const PY228_COMBINATION_SUM: CodingStep = CodingStep {
    id: "py-228-combination-sum", title: "DSA Combination Sum", objective: "Encontrar combinaciones que sumen el target reusando candidatos.",
    prompt_md: "**Combination Sum**\n\nBacktracking ordenado: reusá el índice actual; cortá si el candidato supera el resto.\n\n**Micro-reto:**\n1. Definí `combination_sum(candidates, target)`\n2. Imprimí `combination_sum([2, 3, 6, 7], 7)`",
    starter_code: "# def combination_sum(candidates, target):\n#     ...\n# print(combination_sum([2, 3, 6, 7], 7))\n",
    pytest: "def test_combination_sum(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('combination_sum'))\n    assert ns['combination_sum']([2, 3, 6, 7], 7) == [[2, 2, 3], [7]]\n    assert ns['combination_sum']([2, 3, 5], 8) == [[2, 2, 2, 2], [2, 3, 3], [3, 5]]\n    assert capsys.readouterr().out.strip() == '[[2, 2, 3], [7]]'\n",
    hint: "def combination_sum(candidates, target):\n    candidates = sorted(candidates)\n    out = []\n    def bt(start, remain, path):\n        if remain == 0:\n            out.append(path[:]); return\n        for i in range(start, len(candidates)):\n            value = candidates[i]\n            if value > remain: break\n            path.append(value); bt(i, remain - value, path); path.pop()\n    bt(0, target, [])\n    return out",
    solution_example: "def combination_sum(candidates, target):\n    candidates = sorted(candidates)\n    out = []\n    def bt(start, remain, path):\n        if remain == 0:\n            out.append(path[:]); return\n        for i in range(start, len(candidates)):\n            value = candidates[i]\n            if value > remain: break\n            path.append(value); bt(i, remain - value, path); path.pop()\n    bt(0, target, [])\n    return out\nprint(combination_sum([2, 3, 6, 7], 7))\n",
    next: Some("py-229-word-search"), show_type_chips: false, micro_step: 228,
};

pub const PY229_WORD_SEARCH: CodingStep = CodingStep {
    id: "py-229-word-search", title: "DSA Word Search", objective: "Buscar una palabra en una grilla con DFS adyacente sin reusar celdas.",
    prompt_md: "**Word Search**\n\nDFS 4-dir: marcá visitado, explorá, restaurá. Distinto de word-ladder (py-208).\n\n**Micro-reto:**\n1. Definí `exist(board, word)`\n2. Imprimí `exist([['A','B','C','E'],['S','F','C','S'],['A','D','E','E']], 'ABCCED')`",
    starter_code: "# def exist(board, word):\n#     ...\n# print(exist([['A','B','C','E'],['S','F','C','S'],['A','D','E','E']], 'ABCCED'))\n",
    pytest: "def test_word_search(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('exist'))\n    board = [['A','B','C','E'],['S','F','C','S'],['A','D','E','E']]\n    assert ns['exist']([row[:] for row in board], 'ABCCED') is True\n    assert ns['exist']([row[:] for row in board], 'SEE') is True\n    assert ns['exist']([row[:] for row in board], 'ABCB') is False\n    assert capsys.readouterr().out.strip() == 'True'\n",
    hint: "def exist(board, word):\n    rows, cols = len(board), len(board[0])\n    def dfs(i, j, k):\n        if k == len(word): return True\n        if i < 0 or j < 0 or i >= rows or j >= cols or board[i][j] != word[k]: return False\n        tmp = board[i][j]; board[i][j] = '#'\n        ok = dfs(i + 1, j, k + 1) or dfs(i - 1, j, k + 1) or dfs(i, j + 1, k + 1) or dfs(i, j - 1, k + 1)\n        board[i][j] = tmp\n        return ok\n    return any(dfs(i, j, 0) for i in range(rows) for j in range(cols))",
    solution_example: "def exist(board, word):\n    rows, cols = len(board), len(board[0])\n    def dfs(i, j, k):\n        if k == len(word): return True\n        if i < 0 or j < 0 or i >= rows or j >= cols or board[i][j] != word[k]: return False\n        tmp = board[i][j]; board[i][j] = '#'\n        ok = dfs(i + 1, j, k + 1) or dfs(i - 1, j, k + 1) or dfs(i, j + 1, k + 1) or dfs(i, j - 1, k + 1)\n        board[i][j] = tmp\n        return ok\n    return any(dfs(i, j, 0) for i in range(rows) for j in range(cols))\nprint(exist([['A','B','C','E'],['S','F','C','S'],['A','D','E','E']], 'ABCCED'))\n",
    next: Some("py-230-letter-combos"), show_type_chips: false, micro_step: 229,
};

pub const PY230_LETTER_COMBOS: CodingStep = CodingStep {
    id: "py-230-letter-combos", title: "DSA Letter Combinations", objective: "Expandir dígitos del teclado a todas las combinaciones de letras.",
    prompt_md: "**Letter Combinations of a Phone Number**\n\nMapa 2–9 → letras; productá cada dígito sobre el prefijo actual.\n\n**Micro-reto:**\n1. Definí `letter_combinations(digits)`\n2. Imprimí `letter_combinations('23')`",
    starter_code: "# def letter_combinations(digits):\n#     ...\n# print(letter_combinations('23'))\n",
    pytest: "def test_letter_combos(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('letter_combinations'))\n    assert ns['letter_combinations']('23') == ['ad', 'ae', 'af', 'bd', 'be', 'bf', 'cd', 'ce', 'cf']\n    assert ns['letter_combinations']('') == []\n    assert capsys.readouterr().out.strip() == \"['ad', 'ae', 'af', 'bd', 'be', 'bf', 'cd', 'ce', 'cf']\"\n",
    hint: "def letter_combinations(digits):\n    if not digits: return []\n    phone = {'2': 'abc', '3': 'def', '4': 'ghi', '5': 'jkl', '6': 'mno', '7': 'pqrs', '8': 'tuv', '9': 'wxyz'}\n    out = ['']\n    for digit in digits:\n        out = [prefix + ch for prefix in out for ch in phone[digit]]\n    return out",
    solution_example: "def letter_combinations(digits):\n    if not digits: return []\n    phone = {'2': 'abc', '3': 'def', '4': 'ghi', '5': 'jkl', '6': 'mno', '7': 'pqrs', '8': 'tuv', '9': 'wxyz'}\n    out = ['']\n    for digit in digits:\n        out = [prefix + ch for prefix in out for ch in phone[digit]]\n    return out\nprint(letter_combinations('23'))\n",
    next: Some("py-231-subsets-ii"), show_type_chips: false, micro_step: 230,
};

pub const PY231_SUBSETS_II: CodingStep = CodingStep {
    id: "py-231-subsets-ii", title: "DSA Subsets II", objective: "Generar todos los subconjuntos únicos a partir de un array con duplicados.",
    prompt_md: "**Subsets II**\n\nOrdená y saltá duplicados en el mismo nivel. Distinto de subsets (py-172).\n\n**Micro-reto:**\n1. Definí `subsets_with_dup(nums)`\n2. Imprimí `subsets_with_dup([1, 2, 2])`",
    starter_code: "# def subsets_with_dup(nums):\n#     ...\n# print(subsets_with_dup([1, 2, 2]))\n",
    pytest: "def test_subsets_ii(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('subsets_with_dup'))\n    assert ns['subsets_with_dup']([1, 2, 2]) == [[], [1], [1, 2], [1, 2, 2], [2], [2, 2]]\n    assert ns['subsets_with_dup']([0]) == [[], [0]]\n    assert capsys.readouterr().out.strip() == '[[], [1], [1, 2], [1, 2, 2], [2], [2, 2]]'\n",
    hint: "def subsets_with_dup(nums):\n    nums = sorted(nums)\n    out = []\n    def bt(start, path):\n        out.append(path[:])\n        for i in range(start, len(nums)):\n            if i > start and nums[i] == nums[i - 1]: continue\n            path.append(nums[i]); bt(i + 1, path); path.pop()\n    bt(0, [])\n    return out",
    solution_example: "def subsets_with_dup(nums):\n    nums = sorted(nums)\n    out = []\n    def bt(start, path):\n        out.append(path[:])\n        for i in range(start, len(nums)):\n            if i > start and nums[i] == nums[i - 1]: continue\n            path.append(nums[i]); bt(i + 1, path); path.pop()\n    bt(0, [])\n    return out\nprint(subsets_with_dup([1, 2, 2]))\n",
    next: Some("py-232-palindrome-partition"), show_type_chips: false, micro_step: 231,
};

pub const PY232_PALINDROME_PARTITION: CodingStep = CodingStep {
    id: "py-232-palindrome-partition", title: "DSA Palindrome Partition", objective: "Particionar un string en todos los cortes donde cada pieza es palíndromo.",
    prompt_md: "**Palindrome Partitioning**\n\nBacktracking: en cada índice, cortá solo si `s[start:end]` es palíndromo.\n\n**Micro-reto:**\n1. Definí `partition(s)`\n2. Imprimí `partition('aab')`",
    starter_code: "# def partition(s):\n#     ...\n# print(partition('aab'))\n",
    pytest: "def test_palindrome_partition(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('partition'))\n    assert ns['partition']('aab') == [['a', 'a', 'b'], ['aa', 'b']]\n    assert ns['partition']('a') == [['a']]\n    assert capsys.readouterr().out.strip() == \"[['a', 'a', 'b'], ['aa', 'b']]\"\n",
    hint: "def partition(s):\n    out = []\n    def is_pal(left, right):\n        while left < right:\n            if s[left] != s[right]: return False\n            left += 1; right -= 1\n        return True\n    def bt(start, path):\n        if start == len(s):\n            out.append(path[:]); return\n        for end in range(start, len(s)):\n            if is_pal(start, end):\n                path.append(s[start:end + 1]); bt(end + 1, path); path.pop()\n    bt(0, [])\n    return out",
    solution_example: "def partition(s):\n    out = []\n    def is_pal(left, right):\n        while left < right:\n            if s[left] != s[right]: return False\n            left += 1; right -= 1\n        return True\n    def bt(start, path):\n        if start == len(s):\n            out.append(path[:]); return\n        for end in range(start, len(s)):\n            if is_pal(start, end):\n                path.append(s[start:end + 1]); bt(end + 1, path); path.pop()\n    bt(0, [])\n    return out\nprint(partition('aab'))\n",
    next: Some("py-233-reverse-integer"), show_type_chips: false, micro_step: 232,
};

pub const PY233_REVERSE_INTEGER: CodingStep = CodingStep {
    id: "py-233-reverse-integer", title: "DSA Reverse Integer", objective: "Invertir los dígitos de un entero con clamp a 32 bits con signo.",
    prompt_md: "**Reverse Integer**\n\nConstruí el reverso dígito a dígito; si sale del rango int32, devolvé `0`.\n\n**Micro-reto:**\n1. Definí `reverse(x)`\n2. Imprimí `reverse(123)`",
    starter_code: "# def reverse(x):\n#     ...\n# print(reverse(123))\n",
    pytest: "def test_reverse_integer(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('reverse'))\n    assert ns['reverse'](123) == 321\n    assert ns['reverse'](-123) == -321\n    assert ns['reverse'](120) == 21\n    assert ns['reverse'](1534236469) == 0\n    assert capsys.readouterr().out.strip() == '321'\n",
    hint: "def reverse(x):\n    sign = -1 if x < 0 else 1\n    x = abs(x)\n    out = 0\n    while x:\n        out = out * 10 + x % 10\n        x //= 10\n    out *= sign\n    if out < -2**31 or out > 2**31 - 1:\n        return 0\n    return out",
    solution_example: "def reverse(x):\n    sign = -1 if x < 0 else 1\n    x = abs(x)\n    out = 0\n    while x:\n        out = out * 10 + x % 10\n        x //= 10\n    out *= sign\n    if out < -2**31 or out > 2**31 - 1:\n        return 0\n    return out\nprint(reverse(123))\n",
    next: Some("py-234-palindrome-number"), show_type_chips: false, micro_step: 233,
};

pub const PY234_PALINDROME_NUMBER: CodingStep = CodingStep {
    id: "py-234-palindrome-number", title: "DSA Palindrome Number", objective: "Decidir si un entero es palíndromo sin convertirlo a string (o con string).",
    prompt_md: "**Palindrome Number**\n\nNegativos no son palíndromo. Compará el número con su reverso.\n\n**Micro-reto:**\n1. Definí `is_palindrome(x)`\n2. Imprimí `is_palindrome(121)`",
    starter_code: "# def is_palindrome(x):\n#     ...\n# print(is_palindrome(121))\n",
    pytest: "def test_palindrome_number(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('is_palindrome'))\n    assert ns['is_palindrome'](121) is True\n    assert ns['is_palindrome'](-121) is False\n    assert ns['is_palindrome'](10) is False\n    assert capsys.readouterr().out.strip() == 'True'\n",
    hint: "def is_palindrome(x):\n    if x < 0: return False\n    original, rev = x, 0\n    while x:\n        rev = rev * 10 + x % 10\n        x //= 10\n    return original == rev",
    solution_example: "def is_palindrome(x):\n    if x < 0: return False\n    original, rev = x, 0\n    while x:\n        rev = rev * 10 + x % 10\n        x //= 10\n    return original == rev\nprint(is_palindrome(121))\n",
    next: Some("py-235-plus-one"), show_type_chips: false, micro_step: 234,
};

pub const PY235_PLUS_ONE: CodingStep = CodingStep {
    id: "py-235-plus-one", title: "DSA Plus One", objective: "Sumar uno a un entero representado como array de dígitos.",
    prompt_md: "**Plus One**\n\nRecorré de derecha a izquierda; propagá el carry si el dígito es 9.\n\n**Micro-reto:**\n1. Definí `plus_one(digits)`\n2. Imprimí `plus_one([1, 2, 3])`",
    starter_code: "# def plus_one(digits):\n#     ...\n# print(plus_one([1, 2, 3]))\n",
    pytest: "def test_plus_one(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('plus_one'))\n    assert ns['plus_one']([1, 2, 3]) == [1, 2, 4]\n    assert ns['plus_one']([9, 9]) == [1, 0, 0]\n    assert ns['plus_one']([9]) == [1, 0]\n    assert capsys.readouterr().out.strip() == '[1, 2, 4]'\n",
    hint: "def plus_one(digits):\n    for i in range(len(digits) - 1, -1, -1):\n        if digits[i] < 9:\n            digits[i] += 1\n            return digits\n        digits[i] = 0\n    return [1] + digits",
    solution_example: "def plus_one(digits):\n    for i in range(len(digits) - 1, -1, -1):\n        if digits[i] < 9:\n            digits[i] += 1\n            return digits\n        digits[i] = 0\n    return [1] + digits\nprint(plus_one([1, 2, 3]))\n",
    next: Some("py-236-add-binary"), show_type_chips: false, micro_step: 235,
};

pub const PY236_ADD_BINARY: CodingStep = CodingStep {
    id: "py-236-add-binary", title: "DSA Add Binary", objective: "Sumar dos strings binarios y devolver el resultado como string.",
    prompt_md: "**Add Binary**\n\nSumá bit a bit desde la derecha con carry.\n\n**Micro-reto:**\n1. Definí `add_binary(a, b)`\n2. Imprimí `add_binary('11', '1')`",
    starter_code: "# def add_binary(a, b):\n#     ...\n# print(add_binary('11', '1'))\n",
    pytest: "def test_add_binary(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('add_binary'))\n    assert ns['add_binary']('11', '1') == '100'\n    assert ns['add_binary']('1010', '1011') == '10101'\n    assert capsys.readouterr().out.strip() == '100'\n",
    hint: "def add_binary(a, b):\n    i, j, carry = len(a) - 1, len(b) - 1, 0\n    out = []\n    while i >= 0 or j >= 0 or carry:\n        total = carry\n        if i >= 0:\n            total += int(a[i]); i -= 1\n        if j >= 0:\n            total += int(b[j]); j -= 1\n        out.append(str(total % 2))\n        carry = total // 2\n    return ''.join(reversed(out))",
    solution_example: "def add_binary(a, b):\n    i, j, carry = len(a) - 1, len(b) - 1, 0\n    out = []\n    while i >= 0 or j >= 0 or carry:\n        total = carry\n        if i >= 0:\n            total += int(a[i]); i -= 1\n        if j >= 0:\n            total += int(b[j]); j -= 1\n        out.append(str(total % 2))\n        carry = total // 2\n    return ''.join(reversed(out))\nprint(add_binary('11', '1'))\n",
    next: Some("py-237-my-pow"), show_type_chips: false, micro_step: 236,
};

pub const PY237_MY_POW: CodingStep = CodingStep {
    id: "py-237-my-pow", title: "DSA Pow(x, n)", objective: "Calcular x elevado a n con exponentiation by squaring.",
    prompt_md: "**Pow(x, n)**\n\nSi `n` es negativo, invertí la base. Cuadrá y desplazá el exponente.\n\n**Micro-reto:**\n1. Definí `my_pow(x, n)`\n2. Imprimí `my_pow(2.0, 10)`",
    starter_code: "# def my_pow(x, n):\n#     ...\n# print(my_pow(2.0, 10))\n",
    pytest: "def test_my_pow(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('my_pow'))\n    assert abs(ns['my_pow'](2.0, 10) - 1024.0) < 1e-9\n    assert abs(ns['my_pow'](2.1, 3) - 9.261) < 1e-9\n    assert abs(ns['my_pow'](2.0, -2) - 0.25) < 1e-9\n    assert capsys.readouterr().out.strip() == '1024.0'\n",
    hint: "def my_pow(x, n):\n    if n < 0:\n        x = 1 / x\n        n = -n\n    out = 1.0\n    while n:\n        if n & 1:\n            out *= x\n        x *= x\n        n >>= 1\n    return out",
    solution_example: "def my_pow(x, n):\n    if n < 0:\n        x = 1 / x\n        n = -n\n    out = 1.0\n    while n:\n        if n & 1:\n            out *= x\n        x *= x\n        n >>= 1\n    return out\nprint(my_pow(2.0, 10))\n",
    next: Some("py-238-trailing-zeroes"), show_type_chips: false, micro_step: 237,
};

pub const PY238_TRAILING_ZEROES: CodingStep = CodingStep {
    id: "py-238-trailing-zeroes", title: "DSA Trailing Zeroes", objective: "Contar ceros finales de n! contando factores de 5.",
    prompt_md: "**Factorial Trailing Zeroes**\n\nSumá `n//5 + n//25 + …` hasta agotar potencias de 5.\n\n**Micro-reto:**\n1. Definí `trailing_zeroes(n)`\n2. Imprimí `trailing_zeroes(25)`",
    starter_code: "# def trailing_zeroes(n):\n#     ...\n# print(trailing_zeroes(25))\n",
    pytest: "def test_trailing_zeroes(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('trailing_zeroes'))\n    assert ns['trailing_zeroes'](3) == 0\n    assert ns['trailing_zeroes'](5) == 1\n    assert ns['trailing_zeroes'](25) == 6\n    assert capsys.readouterr().out.strip() == '6'\n",
    hint: "def trailing_zeroes(n):\n    zeros = 0\n    while n:\n        n //= 5\n        zeros += n\n    return zeros",
    solution_example: "def trailing_zeroes(n):\n    zeros = 0\n    while n:\n        n //= 5\n        zeros += n\n    return zeros\nprint(trailing_zeroes(25))\n",
    next: Some("py-239-tree-diameter"), show_type_chips: false, micro_step: 238,
};

pub const PY239_TREE_DIAMETER: CodingStep = CodingStep {
    id: "py-239-tree-diameter", title: "DSA Tree Diameter", objective: "Calcular el diámetro de un árbol binario (aristas del camino más largo).",
    prompt_md: "**Diameter of Binary Tree**\n\nEn cada nodo, el mejor camino local es `left_depth + right_depth`; trackeá el máximo.\n\n**Micro-reto:**\n1. Definí `class TreeNode` y `diameter_of_binary_tree(root)`\n2. Construí `1` → left `2` (4, 5) / right `3`\n3. Imprimí el diámetro (esperado: `3`)",
    starter_code: "# class TreeNode:\n#     ...\n# def diameter_of_binary_tree(root):\n#     ...\n# root = ...\n# print(diameter_of_binary_tree(root))\n",
    pytest: "def test_tree_diameter(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('diameter_of_binary_tree'))\n    TreeNode = ns['TreeNode']\n    root = TreeNode(1)\n    root.left = TreeNode(2)\n    root.right = TreeNode(3)\n    root.left.left = TreeNode(4)\n    root.left.right = TreeNode(5)\n    assert ns['diameter_of_binary_tree'](root) == 3\n    assert ns['diameter_of_binary_tree'](None) == 0\n    assert capsys.readouterr().out.strip() == '3'\n",
    hint: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef diameter_of_binary_tree(root):\n    best = [0]\n    def depth(node):\n        if node is None: return 0\n        left = depth(node.left); right = depth(node.right)\n        best[0] = max(best[0], left + right)\n        return 1 + max(left, right)\n    depth(root)\n    return best[0]",
    solution_example: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef diameter_of_binary_tree(root):\n    best = [0]\n    def depth(node):\n        if node is None: return 0\n        left = depth(node.left); right = depth(node.right)\n        best[0] = max(best[0], left + right)\n        return 1 + max(left, right)\n    depth(root)\n    return best[0]\n\nroot = TreeNode(1)\nroot.left = TreeNode(2)\nroot.right = TreeNode(3)\nroot.left.left = TreeNode(4)\nroot.left.right = TreeNode(5)\nprint(diameter_of_binary_tree(root))\n",
    next: Some("py-240-lca"), show_type_chips: false, micro_step: 239,
};

pub const PY240_LCA: CodingStep = CodingStep {
    id: "py-240-lca", title: "DSA Lowest Common Ancestor", objective: "Encontrar el ancestro común más bajo de dos nodos en un árbol binario.",
    prompt_md: "**Lowest Common Ancestor**\n\nSi `p` y `q` caen en subárboles distintos, el nodo actual es el LCA.\n\n**Micro-reto:**\n1. Definí `lowest_common_ancestor(root, p, q)`\n2. Árbol `3` → left `5` / right `1`; imprimí `.data` del LCA de `5` y `1` (esperado: `3`)",
    starter_code: "# class TreeNode:\n#     ...\n# def lowest_common_ancestor(root, p, q):\n#     ...\n# root = ...\n# print(lowest_common_ancestor(root, root.left, root.right).data)\n",
    pytest: "def test_lca(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('lowest_common_ancestor'))\n    TreeNode = ns['TreeNode']\n    root = TreeNode(3)\n    root.left = TreeNode(5)\n    root.right = TreeNode(1)\n    assert ns['lowest_common_ancestor'](root, root.left, root.right).data == 3\n    assert ns['lowest_common_ancestor'](root, root.left, root.left).data == 5\n    assert capsys.readouterr().out.strip() == '3'\n",
    hint: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef lowest_common_ancestor(root, p, q):\n    if root is None or root is p or root is q:\n        return root\n    left = lowest_common_ancestor(root.left, p, q)\n    right = lowest_common_ancestor(root.right, p, q)\n    if left and right:\n        return root\n    return left or right",
    solution_example: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef lowest_common_ancestor(root, p, q):\n    if root is None or root is p or root is q:\n        return root\n    left = lowest_common_ancestor(root.left, p, q)\n    right = lowest_common_ancestor(root.right, p, q)\n    if left and right:\n        return root\n    return left or right\n\nroot = TreeNode(3)\nroot.left = TreeNode(5)\nroot.right = TreeNode(1)\nprint(lowest_common_ancestor(root, root.left, root.right).data)\n",
    next: Some("py-241-path-sum"), show_type_chips: false, micro_step: 240,
};

pub const PY241_PATH_SUM: CodingStep = CodingStep {
    id: "py-241-path-sum", title: "DSA Path Sum", objective: "Decidir si existe un camino raíz→hoja con suma exacta.",
    prompt_md: "**Path Sum**\n\nRestá `root.data` al target y preguntá a left/right; en hoja compará igualdad.\n\n**Micro-reto:**\n1. Definí `has_path_sum(root, target)`\n2. Árbol clásico de LeetCode 112; imprimí `has_path_sum(root, 22)` (esperado: `True`)",
    starter_code: "# class TreeNode:\n#     ...\n# def has_path_sum(root, target):\n#     ...\n# root = ...\n# print(has_path_sum(root, 22))\n",
    pytest: "def test_path_sum(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('has_path_sum'))\n    TreeNode = ns['TreeNode']\n    root = TreeNode(5)\n    root.left = TreeNode(4)\n    root.right = TreeNode(8)\n    root.left.left = TreeNode(11)\n    root.left.left.left = TreeNode(7)\n    root.left.left.right = TreeNode(2)\n    root.right.left = TreeNode(13)\n    root.right.right = TreeNode(4)\n    root.right.right.right = TreeNode(1)\n    assert ns['has_path_sum'](root, 22) is True\n    assert ns['has_path_sum'](root, 100) is False\n    assert capsys.readouterr().out.strip() == 'True'\n",
    hint: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef has_path_sum(root, target):\n    if root is None: return False\n    if root.left is None and root.right is None:\n        return root.data == target\n    return has_path_sum(root.left, target - root.data) or has_path_sum(root.right, target - root.data)",
    solution_example: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef has_path_sum(root, target):\n    if root is None: return False\n    if root.left is None and root.right is None:\n        return root.data == target\n    return has_path_sum(root.left, target - root.data) or has_path_sum(root.right, target - root.data)\n\nroot = TreeNode(5)\nroot.left = TreeNode(4)\nroot.right = TreeNode(8)\nroot.left.left = TreeNode(11)\nroot.left.left.left = TreeNode(7)\nroot.left.left.right = TreeNode(2)\nroot.right.left = TreeNode(13)\nroot.right.right = TreeNode(4)\nroot.right.right.right = TreeNode(1)\nprint(has_path_sum(root, 22))\n",
    next: Some("py-242-right-side"), show_type_chips: false, micro_step: 241,
};

pub const PY242_RIGHT_SIDE: CodingStep = CodingStep {
    id: "py-242-right-side", title: "DSA Right Side View", objective: "Devolver los valores visibles desde la derecha nivel a nivel.",
    prompt_md: "**Binary Tree Right Side View**\n\nBFS por niveles; guardá el último nodo de cada nivel.\n\n**Micro-reto:**\n1. Definí `right_side_view(root)`\n2. Árbol `1` → left `2` (right 5) / right `3` (right 4); imprimí la vista (esperado: `[1, 3, 4]`)",
    starter_code: "# from collections import deque\n# class TreeNode:\n#     ...\n# def right_side_view(root):\n#     ...\n# root = ...\n# print(right_side_view(root))\n",
    pytest: "def test_right_side(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('right_side_view'))\n    TreeNode = ns['TreeNode']\n    root = TreeNode(1)\n    root.left = TreeNode(2)\n    root.right = TreeNode(3)\n    root.left.right = TreeNode(5)\n    root.right.right = TreeNode(4)\n    assert ns['right_side_view'](root) == [1, 3, 4]\n    assert ns['right_side_view'](None) == []\n    assert capsys.readouterr().out.strip() == '[1, 3, 4]'\n",
    hint: "from collections import deque\n\nclass TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef right_side_view(root):\n    if root is None: return []\n    out = []\n    queue = deque([root])\n    while queue:\n        last = None\n        for _ in range(len(queue)):\n            node = queue.popleft()\n            last = node.data\n            if node.left: queue.append(node.left)\n            if node.right: queue.append(node.right)\n        out.append(last)\n    return out",
    solution_example: "from collections import deque\n\nclass TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef right_side_view(root):\n    if root is None: return []\n    out = []\n    queue = deque([root])\n    while queue:\n        last = None\n        for _ in range(len(queue)):\n            node = queue.popleft()\n            last = node.data\n            if node.left: queue.append(node.left)\n            if node.right: queue.append(node.right)\n        out.append(last)\n    return out\n\nroot = TreeNode(1)\nroot.left = TreeNode(2)\nroot.right = TreeNode(3)\nroot.left.right = TreeNode(5)\nroot.right.right = TreeNode(4)\nprint(right_side_view(root))\n",
    next: Some("py-243-flatten-tree"), show_type_chips: false, micro_step: 242,
};

pub const PY243_FLATTEN_TREE: CodingStep = CodingStep {
    id: "py-243-flatten-tree", title: "DSA Flatten Tree", objective: "Aplanar un árbol binario a una lista enlazada preorder usando right.",
    prompt_md: "**Flatten Binary Tree to Linked List**\n\nSi hay left, colgá el right al predecesor derecho del left y mové left a right.\n\n**Micro-reto:**\n1. Definí `flatten(root)` (in-place)\n2. Árbol `1` → left `2` (3, 4) / right `5` (6); imprimí valores por `right` (esperado: `[1, 2, 3, 4, 5, 6]`)",
    starter_code: "# class TreeNode:\n#     ...\n# def flatten(root):\n#     ...\n# root = ...\n# flatten(root)\n# ...\n# print(values)\n",
    pytest: "def test_flatten_tree(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('flatten'))\n    TreeNode = ns['TreeNode']\n    root = TreeNode(1)\n    root.left = TreeNode(2)\n    root.right = TreeNode(5)\n    root.left.left = TreeNode(3)\n    root.left.right = TreeNode(4)\n    root.right.right = TreeNode(6)\n    ns['flatten'](root)\n    values = []\n    cur = root\n    while cur:\n        assert cur.left is None\n        values.append(cur.data)\n        cur = cur.right\n    assert values == [1, 2, 3, 4, 5, 6]\n    assert capsys.readouterr().out.strip() == '[1, 2, 3, 4, 5, 6]'\n",
    hint: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef flatten(root):\n    while root:\n        if root.left:\n            pred = root.left\n            while pred.right:\n                pred = pred.right\n            pred.right = root.right\n            root.right = root.left\n            root.left = None\n        root = root.right",
    solution_example: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef flatten(root):\n    while root:\n        if root.left:\n            pred = root.left\n            while pred.right:\n                pred = pred.right\n            pred.right = root.right\n            root.right = root.left\n            root.left = None\n        root = root.right\n\nroot = TreeNode(1)\nroot.left = TreeNode(2)\nroot.right = TreeNode(5)\nroot.left.left = TreeNode(3)\nroot.left.right = TreeNode(4)\nroot.right.right = TreeNode(6)\nflatten(root)\nvalues = []\ncur = root\nwhile cur:\n    values.append(cur.data)\n    cur = cur.right\nprint(values)\n",
    next: Some("py-244-validate-bst"), show_type_chips: false, micro_step: 243,
};

pub const PY244_VALIDATE_BST: CodingStep = CodingStep {
    id: "py-244-validate-bst", title: "DSA Validate BST", objective: "Validar si un árbol binario es un BST con rangos (lo, hi).",
    prompt_md: "**Validate Binary Search Tree**\n\nCada nodo debe vivir en `(lo, hi)` estricto; propagá el rango a left/right.\n\n**Micro-reto:**\n1. Definí `is_valid_bst(root)`\n2. Árbol `2` → left `1` / right `3`; imprimí `True`",
    starter_code: "# class TreeNode:\n#     ...\n# def is_valid_bst(root):\n#     ...\n# root = ...\n# print(is_valid_bst(root))\n",
    pytest: "def test_validate_bst(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('is_valid_bst'))\n    TreeNode = ns['TreeNode']\n    root = TreeNode(2)\n    root.left = TreeNode(1)\n    root.right = TreeNode(3)\n    assert ns['is_valid_bst'](root) is True\n    bad = TreeNode(5)\n    bad.left = TreeNode(1)\n    bad.right = TreeNode(4)\n    bad.right.left = TreeNode(3)\n    bad.right.right = TreeNode(6)\n    assert ns['is_valid_bst'](bad) is False\n    assert capsys.readouterr().out.strip() == 'True'\n",
    hint: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef is_valid_bst(root):\n    def valid(node, lo, hi):\n        if node is None: return True\n        if not (lo < node.data < hi): return False\n        return valid(node.left, lo, node.data) and valid(node.right, node.data, hi)\n    return valid(root, float('-inf'), float('inf'))",
    solution_example: "class TreeNode:\n    def __init__(self, data):\n        self.data = data\n        self.left = None\n        self.right = None\n\ndef is_valid_bst(root):\n    def valid(node, lo, hi):\n        if node is None: return True\n        if not (lo < node.data < hi): return False\n        return valid(node.left, lo, node.data) and valid(node.right, node.data, hi)\n    return valid(root, float('-inf'), float('inf'))\n\nroot = TreeNode(2)\nroot.left = TreeNode(1)\nroot.right = TreeNode(3)\nprint(is_valid_bst(root))\n",
    next: Some("py-245-remove-nth"), show_type_chips: false, micro_step: 244,
};

pub const PY245_REMOVE_NTH: CodingStep = CodingStep {
    id: "py-245-remove-nth", title: "DSA Remove Nth Node", objective: "Eliminar el n-ésimo nodo desde el final con dos punteros.",
    prompt_md: "**Remove Nth Node From End**\n\nDummy + gap de n: cuando `fast` llega al final, `slow.next` es el objetivo. Distinto de reverse-list (py-161, arrays).\n\n**Micro-reto:**\n1. Definí `class ListNode` (`data`, `next`) y `remove_nth_from_end(head, n)`\n2. Definí `to_list(head)`\n3. Lista `1→2→3→4→5`, n=2; imprimí `to_list(...)` (esperado: `[1, 2, 3, 5]`)",
    starter_code: "# class ListNode:\n#     ...\n# def remove_nth_from_end(head, n):\n#     ...\n# def to_list(head):\n#     ...\n# head = ...\n# print(to_list(remove_nth_from_end(head, 2)))\n",
    pytest: "def test_remove_nth(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('remove_nth_from_end'))\n    ListNode = ns['ListNode']\n    def build(vals):\n        dummy = ListNode(0); cur = dummy\n        for v in vals:\n            cur.next = ListNode(v); cur = cur.next\n        return dummy.next\n    def walk(head):\n        out = []\n        while head:\n            out.append(head.data); head = head.next\n        return out\n    assert walk(ns['remove_nth_from_end'](build([1, 2, 3, 4, 5]), 2)) == [1, 2, 3, 5]\n    assert walk(ns['remove_nth_from_end'](build([1]), 1)) == []\n    assert capsys.readouterr().out.strip() == '[1, 2, 3, 5]'\n",
    hint: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data\n        self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.data); head = head.next\n    return out\n\ndef remove_nth_from_end(head, n):\n    dummy = ListNode(0, head)\n    fast = slow = dummy\n    for _ in range(n):\n        fast = fast.next\n    while fast.next:\n        fast = fast.next; slow = slow.next\n    slow.next = slow.next.next\n    return dummy.next",
    solution_example: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data\n        self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.data); head = head.next\n    return out\n\ndef remove_nth_from_end(head, n):\n    dummy = ListNode(0, head)\n    fast = slow = dummy\n    for _ in range(n):\n        fast = fast.next\n    while fast.next:\n        fast = fast.next; slow = slow.next\n    slow.next = slow.next.next\n    return dummy.next\n\nhead = ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5)))))\nprint(to_list(remove_nth_from_end(head, 2)))\n",
    next: Some("py-246-reorder-list"), show_type_chips: false, micro_step: 245,
};

pub const PY246_REORDER_LIST: CodingStep = CodingStep {
    id: "py-246-reorder-list", title: "DSA Reorder List", objective: "Reordenar L0→Ln→L1→Ln-1 in-place partiendo y revirtiendo la segunda mitad.",
    prompt_md: "**Reorder List**\n\nMitad (slow/fast), invertí la segunda, intercalá. Distinto de flatten-tree (py-243).\n\n**Micro-reto:**\n1. Definí `reorder_list(head)` (in-place, no hace falta devolver)\n2. Lista `1→2→3→4→5`; imprimí `to_list(head)` (esperado: `[1, 5, 2, 4, 3]`)",
    starter_code: "# class ListNode:\n#     ...\n# def reorder_list(head):\n#     ...\n# def to_list(head):\n#     ...\n# head = ...\n# reorder_list(head)\n# print(to_list(head))\n",
    pytest: "def test_reorder_list(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('reorder_list'))\n    ListNode = ns['ListNode']\n    def build(vals):\n        dummy = ListNode(0); cur = dummy\n        for v in vals:\n            cur.next = ListNode(v); cur = cur.next\n        return dummy.next\n    def walk(head):\n        out = []\n        while head:\n            out.append(head.data); head = head.next\n        return out\n    head = build([1, 2, 3, 4, 5])\n    ns['reorder_list'](head)\n    assert walk(head) == [1, 5, 2, 4, 3]\n    short = build([1, 2])\n    ns['reorder_list'](short)\n    assert walk(short) == [1, 2]\n    assert capsys.readouterr().out.strip() == '[1, 5, 2, 4, 3]'\n",
    hint: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data\n        self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.data); head = head.next\n    return out\n\ndef reorder_list(head):\n    if not head or not head.next: return\n    slow = fast = head\n    while fast.next and fast.next.next:\n        slow = slow.next; fast = fast.next.next\n    second = slow.next; slow.next = None\n    prev = None\n    while second:\n        nxt = second.next; second.next = prev; prev = second; second = nxt\n    first, second = head, prev\n    while second:\n        t1, t2 = first.next, second.next\n        first.next = second; second.next = t1\n        first, second = t1, t2",
    solution_example: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data\n        self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.data); head = head.next\n    return out\n\ndef reorder_list(head):\n    if not head or not head.next: return\n    slow = fast = head\n    while fast.next and fast.next.next:\n        slow = slow.next; fast = fast.next.next\n    second = slow.next; slow.next = None\n    prev = None\n    while second:\n        nxt = second.next; second.next = prev; prev = second; second = nxt\n    first, second = head, prev\n    while second:\n        t1, t2 = first.next, second.next\n        first.next = second; second.next = t1\n        first, second = t1, t2\n\nhead = ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5)))))\nreorder_list(head)\nprint(to_list(head))\n",
    next: Some("py-247-add-two-lists"), show_type_chips: false, micro_step: 246,
};

pub const PY247_ADD_TWO_LISTS: CodingStep = CodingStep {
    id: "py-247-add-two-lists", title: "DSA Add Two Lists", objective: "Sumar dos enteros representados como listas enlazadas (dígito menos significativo primero).",
    prompt_md: "**Add Two Numbers**\n\nCarry dígito a dígito. Distinto de py-78 (suma de variables) y py-236 (strings binarios).\n\n**Micro-reto:**\n1. Definí `add_two_numbers(l1, l2)`\n2. `2→4→3` + `5→6→4`; imprimí `to_list(...)` (esperado: `[7, 0, 8]`)",
    starter_code: "# class ListNode:\n#     ...\n# def add_two_numbers(l1, l2):\n#     ...\n# def to_list(head):\n#     ...\n# print(to_list(add_two_numbers(l1, l2)))\n",
    pytest: "def test_add_two_lists(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('add_two_numbers'))\n    ListNode = ns['ListNode']\n    def build(vals):\n        dummy = ListNode(0); cur = dummy\n        for v in vals:\n            cur.next = ListNode(v); cur = cur.next\n        return dummy.next\n    def walk(head):\n        out = []\n        while head:\n            out.append(head.data); head = head.next\n        return out\n    assert walk(ns['add_two_numbers'](build([2, 4, 3]), build([5, 6, 4]))) == [7, 0, 8]\n    assert walk(ns['add_two_numbers'](build([0]), build([0]))) == [0]\n    assert capsys.readouterr().out.strip() == '[7, 0, 8]'\n",
    hint: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data\n        self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.data); head = head.next\n    return out\n\ndef add_two_numbers(l1, l2):\n    dummy = ListNode(0); cur = dummy; carry = 0\n    while l1 or l2 or carry:\n        total = carry\n        if l1: total += l1.data; l1 = l1.next\n        if l2: total += l2.data; l2 = l2.next\n        cur.next = ListNode(total % 10); cur = cur.next; carry = total // 10\n    return dummy.next",
    solution_example: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data\n        self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.data); head = head.next\n    return out\n\ndef add_two_numbers(l1, l2):\n    dummy = ListNode(0); cur = dummy; carry = 0\n    while l1 or l2 or carry:\n        total = carry\n        if l1: total += l1.data; l1 = l1.next\n        if l2: total += l2.data; l2 = l2.next\n        cur.next = ListNode(total % 10); cur = cur.next; carry = total // 10\n    return dummy.next\n\nl1 = ListNode(2, ListNode(4, ListNode(3)))\nl2 = ListNode(5, ListNode(6, ListNode(4)))\nprint(to_list(add_two_numbers(l1, l2)))\n",
    next: Some("py-248-swap-pairs"), show_type_chips: false, micro_step: 247,
};

pub const PY248_SWAP_PAIRS: CodingStep = CodingStep {
    id: "py-248-swap-pairs", title: "DSA Swap Pairs", objective: "Intercambiar nodos adyacentes de a pares sin mutar los valores.",
    prompt_md: "**Swap Nodes in Pairs**\n\nDummy: re-enlazá `prev → b → a` y avanzá `prev` a `a`.\n\n**Micro-reto:**\n1. Definí `swap_pairs(head)`\n2. Lista `1→2→3→4`; imprimí `to_list(...)` (esperado: `[2, 1, 4, 3]`)",
    starter_code: "# class ListNode:\n#     ...\n# def swap_pairs(head):\n#     ...\n# def to_list(head):\n#     ...\n# print(to_list(swap_pairs(head)))\n",
    pytest: "def test_swap_pairs(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('swap_pairs'))\n    ListNode = ns['ListNode']\n    def build(vals):\n        dummy = ListNode(0); cur = dummy\n        for v in vals:\n            cur.next = ListNode(v); cur = cur.next\n        return dummy.next\n    def walk(head):\n        out = []\n        while head:\n            out.append(head.data); head = head.next\n        return out\n    assert walk(ns['swap_pairs'](build([1, 2, 3, 4]))) == [2, 1, 4, 3]\n    assert walk(ns['swap_pairs'](build([1]))) == [1]\n    assert capsys.readouterr().out.strip() == '[2, 1, 4, 3]'\n",
    hint: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data\n        self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.data); head = head.next\n    return out\n\ndef swap_pairs(head):\n    dummy = ListNode(0, head); prev = dummy\n    while prev.next and prev.next.next:\n        a = prev.next; b = a.next\n        prev.next, a.next, b.next = b, b.next, a\n        prev = a\n    return dummy.next",
    solution_example: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data\n        self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.data); head = head.next\n    return out\n\ndef swap_pairs(head):\n    dummy = ListNode(0, head); prev = dummy\n    while prev.next and prev.next.next:\n        a = prev.next; b = a.next\n        prev.next, a.next, b.next = b, b.next, a\n        prev = a\n    return dummy.next\n\nhead = ListNode(1, ListNode(2, ListNode(3, ListNode(4))))\nprint(to_list(swap_pairs(head)))\n",
    next: Some("py-249-rotate-list"), show_type_chips: false, micro_step: 248,
};

pub const PY249_ROTATE_LIST: CodingStep = CodingStep {
    id: "py-249-rotate-list", title: "DSA Rotate List", objective: "Rotar una lista enlazada k lugares a la derecha.",
    prompt_md: "**Rotate List**\n\nCerrá el anillo, cortá en `n - k%n`. Distinto de rotate-matrix (py-140).\n\n**Micro-reto:**\n1. Definí `rotate_right(head, k)`\n2. Lista `1→2→3→4→5`, k=2; imprimí `to_list(...)` (esperado: `[4, 5, 1, 2, 3]`)",
    starter_code: "# class ListNode:\n#     ...\n# def rotate_right(head, k):\n#     ...\n# def to_list(head):\n#     ...\n# print(to_list(rotate_right(head, 2)))\n",
    pytest: "def test_rotate_list(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('rotate_right'))\n    ListNode = ns['ListNode']\n    def build(vals):\n        dummy = ListNode(0); cur = dummy\n        for v in vals:\n            cur.next = ListNode(v); cur = cur.next\n        return dummy.next\n    def walk(head):\n        out = []\n        while head:\n            out.append(head.data); head = head.next\n        return out\n    assert walk(ns['rotate_right'](build([1, 2, 3, 4, 5]), 2)) == [4, 5, 1, 2, 3]\n    assert walk(ns['rotate_right'](build([0, 1, 2]), 4)) == [2, 0, 1]\n    assert capsys.readouterr().out.strip() == '[4, 5, 1, 2, 3]'\n",
    hint: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data\n        self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.data); head = head.next\n    return out\n\ndef rotate_right(head, k):\n    if not head or not head.next or k == 0: return head\n    n = 1; tail = head\n    while tail.next:\n        tail = tail.next; n += 1\n    k %= n\n    if k == 0: return head\n    tail.next = head\n    new_tail = head\n    for _ in range(n - k - 1):\n        new_tail = new_tail.next\n    new_head = new_tail.next; new_tail.next = None\n    return new_head",
    solution_example: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data\n        self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.data); head = head.next\n    return out\n\ndef rotate_right(head, k):\n    if not head or not head.next or k == 0: return head\n    n = 1; tail = head\n    while tail.next:\n        tail = tail.next; n += 1\n    k %= n\n    if k == 0: return head\n    tail.next = head\n    new_tail = head\n    for _ in range(n - k - 1):\n        new_tail = new_tail.next\n    new_head = new_tail.next; new_tail.next = None\n    return new_head\n\nhead = ListNode(1, ListNode(2, ListNode(3, ListNode(4, ListNode(5)))))\nprint(to_list(rotate_right(head, 2)))\n",
    next: Some("py-250-palindrome-list"), show_type_chips: false, micro_step: 249,
};

pub const PY250_PALINDROME_LIST: CodingStep = CodingStep {
    id: "py-250-palindrome-list", title: "DSA Palindrome List", objective: "Decidir si una lista enlazada es palíndromo revirtiendo la segunda mitad.",
    prompt_md: "**Palindrome Linked List**\n\nSlow/fast hasta la mitad, invertí y compará. Distinto de py-164 (string) y py-234 (entero).\n\n**Micro-reto:**\n1. Definí `is_palindrome_list(head)`\n2. Lista `1→2→2→1`; imprimí `True`",
    starter_code: "# class ListNode:\n#     ...\n# def is_palindrome_list(head):\n#     ...\n# head = ...\n# print(is_palindrome_list(head))\n",
    pytest: "def test_palindrome_list(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('is_palindrome_list'))\n    ListNode = ns['ListNode']\n    def build(vals):\n        dummy = ListNode(0); cur = dummy\n        for v in vals:\n            cur.next = ListNode(v); cur = cur.next\n        return dummy.next\n    assert ns['is_palindrome_list'](build([1, 2, 2, 1])) is True\n    assert ns['is_palindrome_list'](build([1, 2])) is False\n    assert capsys.readouterr().out.strip() == 'True'\n",
    hint: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data\n        self.next = next\n\ndef is_palindrome_list(head):\n    slow = fast = head\n    while fast and fast.next:\n        slow = slow.next; fast = fast.next.next\n    prev = None\n    while slow:\n        nxt = slow.next; slow.next = prev; prev = slow; slow = nxt\n    while prev:\n        if prev.data != head.data: return False\n        prev = prev.next; head = head.next\n    return True",
    solution_example: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data\n        self.next = next\n\ndef is_palindrome_list(head):\n    slow = fast = head\n    while fast and fast.next:\n        slow = slow.next; fast = fast.next.next\n    prev = None\n    while slow:\n        nxt = slow.next; slow.next = prev; prev = slow; slow = nxt\n    while prev:\n        if prev.data != head.data: return False\n        prev = prev.next; head = head.next\n    return True\n\nhead = ListNode(1, ListNode(2, ListNode(2, ListNode(1))))\nprint(is_palindrome_list(head))\n",
    next: Some("py-251-copy-random"), show_type_chips: false, micro_step: 250,
};

pub const PY251_COPY_RANDOM: CodingStep = CodingStep {
    id: "py-251-copy-random", title: "DSA Copy Random List", objective: "Clonar una lista con puntero random en O(n) extra.",
    prompt_md: "**Copy List with Random Pointer**\n\nDos pasadas con hash `original → clone`. Distinto de py-26 list-copy (arrays).\n\n**Micro-reto:**\n1. Definí `ListNode(data, next=None, random=None)` y `copy_random_list(head)`\n2. Lista `1→2` con randoms `[2, 1]`; imprimí `walk(copy)[1]` (esperado: `[1, 2]`)",
    starter_code: "# class ListNode:\n#     ...\n# def copy_random_list(head):\n#     ...\n# def walk(head):\n#     ...\n# head = ...\n# print(walk(copy_random_list(head))[0])\n",
    pytest: "def test_copy_random(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('copy_random_list'))\n    ListNode = ns['ListNode']\n    a = ListNode(1); b = ListNode(2)\n    a.next = b; a.random = b; b.random = a\n    copied = ns['copy_random_list'](a)\n    assert copied is not a and copied.next is not b\n    assert copied.data == 1 and copied.next.data == 2\n    assert copied.random.data == 2 and copied.next.random.data == 1\n    vals, rands = ns['walk'](copied)\n    assert vals == [1, 2] and rands == [2, 1]\n    assert capsys.readouterr().out.strip() == '[1, 2]'\n",
    hint: "class ListNode:\n    def __init__(self, data=0, next=None, random=None):\n        self.data = data; self.next = next; self.random = random\n\ndef walk(head):\n    vals, rands = [], []\n    while head:\n        vals.append(head.data)\n        rands.append(head.random.data if head.random else None)\n        head = head.next\n    return vals, rands\n\ndef copy_random_list(head):\n    if not head: return None\n    mapping = {}\n    cur = head\n    while cur:\n        mapping[cur] = ListNode(cur.data); cur = cur.next\n    cur = head\n    while cur:\n        mapping[cur].next = mapping.get(cur.next)\n        mapping[cur].random = mapping.get(cur.random)\n        cur = cur.next\n    return mapping[head]",
    solution_example: "class ListNode:\n    def __init__(self, data=0, next=None, random=None):\n        self.data = data; self.next = next; self.random = random\n\ndef walk(head):\n    vals, rands = [], []\n    while head:\n        vals.append(head.data)\n        rands.append(head.random.data if head.random else None)\n        head = head.next\n    return vals, rands\n\ndef copy_random_list(head):\n    if not head: return None\n    mapping = {}\n    cur = head\n    while cur:\n        mapping[cur] = ListNode(cur.data); cur = cur.next\n    cur = head\n    while cur:\n        mapping[cur].next = mapping.get(cur.next)\n        mapping[cur].random = mapping.get(cur.random)\n        cur = cur.next\n    return mapping[head]\n\na = ListNode(1); b = ListNode(2)\na.next = b; a.random = b; b.random = a\ncopy = copy_random_list(a)\nprint(walk(copy)[0])\n",
    next: Some("py-252-sort-list"), show_type_chips: false, micro_step: 251,
};

pub const PY252_SORT_LIST: CodingStep = CodingStep {
    id: "py-252-sort-list", title: "DSA Sort List", objective: "Ordenar una lista enlazada con merge sort O(n log n).",
    prompt_md: "**Sort List**\n\nPartí slow/fast, ordená mitades, merge. Distinto de py-25 list-sort (arrays in-place).\n\n**Micro-reto:**\n1. Definí `sort_list(head)`\n2. Lista `4→2→1→3`; imprimí `to_list(sort_list(head))` (esperado: `[1, 2, 3, 4]`)",
    starter_code: "# class ListNode:\n#     ...\n# def sort_list(head):\n#     ...\n# def to_list(head):\n#     ...\n# print(to_list(sort_list(head)))\n",
    pytest: "def test_sort_list(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('sort_list'))\n    ListNode = ns['ListNode']\n    def build(vals):\n        dummy = ListNode(0); cur = dummy\n        for v in vals:\n            cur.next = ListNode(v); cur = cur.next\n        return dummy.next\n    def walk(head):\n        out = []\n        while head:\n            out.append(head.data); head = head.next\n        return out\n    head = build([4, 2, 1, 3])\n    assert walk(ns['sort_list'](head)) == [1, 2, 3, 4]\n    assert walk(ns['sort_list'](build([-1, 5, 3, 4, 0]))) == [-1, 0, 3, 4, 5]\n    assert capsys.readouterr().out.strip() == '[1, 2, 3, 4]'\n",
    hint: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data; self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.data); head = head.next\n    return out\n\ndef sort_list(head):\n    if not head or not head.next: return head\n    slow, fast = head, head.next\n    while fast and fast.next:\n        slow = slow.next; fast = fast.next.next\n    mid = slow.next; slow.next = None\n    return merge(sort_list(head), sort_list(mid))\n\ndef merge(a, b):\n    dummy = ListNode(0); cur = dummy\n    while a and b:\n        if a.data <= b.data: cur.next = a; a = a.next\n        else: cur.next = b; b = b.next\n        cur = cur.next\n    cur.next = a or b\n    return dummy.next",
    solution_example: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data; self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.data); head = head.next\n    return out\n\ndef merge(a, b):\n    dummy = ListNode(0); cur = dummy\n    while a and b:\n        if a.data <= b.data: cur.next = a; a = a.next\n        else: cur.next = b; b = b.next\n        cur = cur.next\n    cur.next = a or b\n    return dummy.next\n\ndef sort_list(head):\n    if not head or not head.next: return head\n    slow, fast = head, head.next\n    while fast and fast.next:\n        slow = slow.next; fast = fast.next.next\n    mid = slow.next; slow.next = None\n    return merge(sort_list(head), sort_list(mid))\n\nhead = ListNode(4, ListNode(2, ListNode(1, ListNode(3))))\nprint(to_list(sort_list(head)))\n",
    next: Some("py-253-merge-two-lists"), show_type_chips: false, micro_step: 252,
};

pub const PY253_MERGE_TWO_LISTS: CodingStep = CodingStep {
    id: "py-253-merge-two-lists", title: "DSA Merge Two Lists", objective: "Fusionar dos listas enlazadas ordenadas en una sola cadena.",
    prompt_md: "**Merge Two Sorted Lists**\n\nDummy + elegir el menor `next`. Distinto de py-162 (arrays) y py-193 (k listas con heap).\n\n**Micro-reto:**\n1. Definí `merge_two_lists(l1, l2)`\n2. `1→2→4` + `1→3→4`; imprimí `to_list(...)` (esperado: `[1, 1, 2, 3, 4, 4]`)",
    starter_code: "# class ListNode:\n#     ...\n# def merge_two_lists(l1, l2):\n#     ...\n# def to_list(head):\n#     ...\n# print(to_list(merge_two_lists(l1, l2)))\n",
    pytest: "def test_merge_two_lists(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('merge_two_lists'))\n    ListNode = ns['ListNode']\n    def build(vals):\n        dummy = ListNode(0); cur = dummy\n        for v in vals:\n            cur.next = ListNode(v); cur = cur.next\n        return dummy.next\n    def walk(head):\n        out = []\n        while head:\n            out.append(head.data); head = head.next\n        return out\n    l1 = build([1, 2, 4]); l2 = build([1, 3, 4])\n    assert walk(ns['merge_two_lists'](l1, l2)) == [1, 1, 2, 3, 4, 4]\n    assert walk(ns['merge_two_lists'](None, build([0]))) == [0]\n    assert capsys.readouterr().out.strip() == '[1, 1, 2, 3, 4, 4]'\n",
    hint: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data; self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.data); head = head.next\n    return out\n\ndef merge_two_lists(l1, l2):\n    dummy = ListNode(0); cur = dummy\n    while l1 and l2:\n        if l1.data <= l2.data: cur.next = l1; l1 = l1.next\n        else: cur.next = l2; l2 = l2.next\n        cur = cur.next\n    cur.next = l1 or l2\n    return dummy.next",
    solution_example: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data; self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.data); head = head.next\n    return out\n\ndef merge_two_lists(l1, l2):\n    dummy = ListNode(0); cur = dummy\n    while l1 and l2:\n        if l1.data <= l2.data: cur.next = l1; l1 = l1.next\n        else: cur.next = l2; l2 = l2.next\n        cur = cur.next\n    cur.next = l1 or l2\n    return dummy.next\n\nl1 = ListNode(1, ListNode(2, ListNode(4)))\nl2 = ListNode(1, ListNode(3, ListNode(4)))\nprint(to_list(merge_two_lists(l1, l2)))\n",
    next: Some("py-254-intersection"), show_type_chips: false, micro_step: 253,
};

pub const PY254_INTERSECTION: CodingStep = CodingStep {
    id: "py-254-intersection", title: "DSA List Intersection", objective: "Encontrar el nodo donde dos listas enlazadas convergen.",
    prompt_md: "**Intersection of Two Linked Lists**\n\nTruco A/B: cuando un puntero llega al final, saltá a la otra cabeza.\n\n**Micro-reto:**\n1. Definí `get_intersection_node(head_a, head_b)`\n2. Compartí cola `3→4`; imprimí `.data` del nodo común (esperado: `3`)",
    starter_code: "# class ListNode:\n#     ...\n# def get_intersection_node(head_a, head_b):\n#     ...\n# head_a = ...\n# head_b = ...\n# print(get_intersection_node(head_a, head_b).data)\n",
    pytest: "def test_intersection(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('get_intersection_node'))\n    ListNode = ns['ListNode']\n    shared = ListNode(3, ListNode(4))\n    a = ListNode(1, ListNode(2, shared))\n    b = ListNode(5, shared)\n    node = ns['get_intersection_node'](a, b)\n    assert node is shared and node.data == 3\n    c = ListNode(9)\n    assert ns['get_intersection_node'](a, c) is None\n    assert capsys.readouterr().out.strip() == '3'\n",
    hint: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data; self.next = next\n\ndef get_intersection_node(head_a, head_b):\n    a, b = head_a, head_b\n    while a is not b:\n        a = a.next if a else head_b\n        b = b.next if b else head_a\n    return a",
    solution_example: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data; self.next = next\n\ndef get_intersection_node(head_a, head_b):\n    a, b = head_a, head_b\n    while a is not b:\n        a = a.next if a else head_b\n        b = b.next if b else head_a\n    return a\n\nshared = ListNode(3, ListNode(4))\nhead_a = ListNode(1, ListNode(2, shared))\nhead_b = ListNode(5, shared)\nprint(get_intersection_node(head_a, head_b).data)\n",
    next: Some("py-255-cycle-start"), show_type_chips: false, micro_step: 254,
};

pub const PY255_CYCLE_START: CodingStep = CodingStep {
    id: "py-255-cycle-start", title: "DSA Cycle Start", objective: "Devolver el nodo donde comienza un ciclo (Floyd fase II).",
    prompt_md: "**Linked List Cycle II**\n\nTras encontrar slow==fast, reiniciá un puntero al head. Distinto de py-163 (solo bool).\n\n**Micro-reto:**\n1. Definí `detect_cycle_start(head)`\n2. Ciclo en nodo `2`; imprimí `.data` (esperado: `2`)",
    starter_code: "# class ListNode:\n#     ...\n# def detect_cycle_start(head):\n#     ...\n# head = ...\n# print(detect_cycle_start(head).data)\n",
    pytest: "def test_cycle_start(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('detect_cycle_start'))\n    ListNode = ns['ListNode']\n    n1 = ListNode(1); n2 = ListNode(2); n3 = ListNode(3)\n    n1.next = n2; n2.next = n3; n3.next = n2\n    start = ns['detect_cycle_start'](n1)\n    assert start is n2 and start.data == 2\n    assert ns['detect_cycle_start'](ListNode(1)) is None\n    assert capsys.readouterr().out.strip() == '2'\n",
    hint: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data; self.next = next\n\ndef detect_cycle_start(head):\n    slow = fast = head\n    while fast and fast.next:\n        slow = slow.next; fast = fast.next.next\n        if slow is fast:\n            slow = head\n            while slow is not fast:\n                slow = slow.next; fast = fast.next\n            return slow\n    return None",
    solution_example: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data; self.next = next\n\ndef detect_cycle_start(head):\n    slow = fast = head\n    while fast and fast.next:\n        slow = slow.next; fast = fast.next.next\n        if slow is fast:\n            slow = head\n            while slow is not fast:\n                slow = slow.next; fast = fast.next\n            return slow\n    return None\n\nn1 = ListNode(1); n2 = ListNode(2); n3 = ListNode(3)\nn1.next = n2; n2.next = n3; n3.next = n2\nprint(detect_cycle_start(n1).data)\n",
    next: Some("py-256-remove-dupes-ii"), show_type_chips: false, micro_step: 255,
};

pub const PY256_REMOVE_DUPES_II: CodingStep = CodingStep {
    id: "py-256-remove-dupes-ii", title: "DSA Remove Dupes II", objective: "Eliminar todos los nodos cuyo valor aparece más de una vez en lista ordenada.",
    prompt_md: "**Remove Duplicates from Sorted List II**\n\nDummy: si `prev.next` repite, saltá todo el bloque. Distinto de dedupe simple (keep one).\n\n**Micro-reto:**\n1. Definí `delete_duplicates(head)`\n2. `1→1→1→2→3→3`; imprimí `to_list(...)` (esperado: `[2]`)",
    starter_code: "# class ListNode:\n#     ...\n# def delete_duplicates(head):\n#     ...\n# def to_list(head):\n#     ...\n# print(to_list(delete_duplicates(head)))\n",
    pytest: "def test_remove_dupes_ii(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('delete_duplicates'))\n    ListNode = ns['ListNode']\n    def build(vals):\n        dummy = ListNode(0); cur = dummy\n        for v in vals:\n            cur.next = ListNode(v); cur = cur.next\n        return dummy.next\n    def walk(head):\n        out = []\n        while head:\n            out.append(head.data); head = head.next\n        return out\n    assert walk(ns['delete_duplicates'](build([1, 1, 1, 2, 3, 3]))) == [2]\n    assert walk(ns['delete_duplicates'](build([1, 1, 2, 2]))) == []\n    assert capsys.readouterr().out.strip() == '[2]'\n",
    hint: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data; self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.data); head = head.next\n    return out\n\ndef delete_duplicates(head):\n    dummy = ListNode(0, head); prev = dummy\n    while prev.next:\n        if prev.next.next and prev.next.data == prev.next.next.data:\n            val = prev.next.data\n            while prev.next and prev.next.data == val:\n                prev.next = prev.next.next\n        else:\n            prev = prev.next\n    return dummy.next",
    solution_example: "class ListNode:\n    def __init__(self, data=0, next=None):\n        self.data = data; self.next = next\n\ndef to_list(head):\n    out = []\n    while head:\n        out.append(head.data); head = head.next\n    return out\n\ndef delete_duplicates(head):\n    dummy = ListNode(0, head); prev = dummy\n    while prev.next:\n        if prev.next.next and prev.next.data == prev.next.next.data:\n            val = prev.next.data\n            while prev.next and prev.next.data == val:\n                prev.next = prev.next.next\n        else:\n            prev = prev.next\n    return dummy.next\n\nhead = ListNode(1, ListNode(1, ListNode(1, ListNode(2, ListNode(3, ListNode(3))))))\nprint(to_list(delete_duplicates(head)))\n",
    next: Some("py-257-remove-k-digits"), show_type_chips: false, micro_step: 256,
};

pub const PY257_REMOVE_K_DIGITS: CodingStep = CodingStep {
    id: "py-257-remove-k-digits", title: "DSA Remove K Digits", objective: "Quitar k dígitos para obtener el entero mínimo (stack monotónico).",
    prompt_md: "**Remove K Digits**\n\nStack creciente: pop mientras el tope sea mayor que el dígito actual. Distinto de py-179 (índices) y py-180 (next greater).\n\n**Micro-reto:**\n1. Definí `remove_k_digits(num, k)`\n2. `num=\"1432219\"`, `k=3`; imprimí el resultado (esperado: `\"1219\"`)",
    starter_code: "# def remove_k_digits(num, k):\n#     ...\n# print(remove_k_digits(\"1432219\", 3))\n",
    pytest: "def test_remove_k_digits(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('remove_k_digits'))\n    assert ns['remove_k_digits']('1432219', 3) == '1219'\n    assert ns['remove_k_digits']('10200', 1) == '200'\n    assert ns['remove_k_digits']('10', 2) == '0'\n    assert capsys.readouterr().out.strip() == '1219'\n",
    hint: "def remove_k_digits(num, k):\n    stack = []\n    for digit in num:\n        while k and stack and stack[-1] > digit:\n            stack.pop(); k -= 1\n        stack.append(digit)\n    return ''.join(stack[:len(stack) - k]).lstrip('0') or '0'\nprint(remove_k_digits('1432219', 3))",
    solution_example: "def remove_k_digits(num, k):\n    stack = []\n    for digit in num:\n        while k and stack and stack[-1] > digit:\n            stack.pop(); k -= 1\n        stack.append(digit)\n    return ''.join(stack[:len(stack) - k]).lstrip('0') or '0'\nprint(remove_k_digits('1432219', 3))\n",
    next: Some("py-258-asteroid-collision"), show_type_chips: false, micro_step: 257,
};

pub const PY258_ASTEROID_COLLISION: CodingStep = CodingStep {
    id: "py-258-asteroid-collision", title: "DSA Asteroid Collision", objective: "Simular colisiones entre asteroides con stack.",
    prompt_md: "**Asteroid Collision**\n\nPositivos van derecha, negativos izquierda; colisionan en el tope. Distinto de py-181 (RPN).\n\n**Micro-reto:**\n1. Definí `asteroid_collision(asteroids)`\n2. `[5, 10, -5]`; imprimí el resultado (esperado: `[5, 10]`)",
    starter_code: "# def asteroid_collision(asteroids):\n#     ...\n# print(asteroid_collision([5, 10, -5]))\n",
    pytest: "def test_asteroid_collision(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('asteroid_collision'))\n    assert ns['asteroid_collision']([5, 10, -5]) == [5, 10]\n    assert ns['asteroid_collision']([8, -8]) == []\n    assert ns['asteroid_collision']([10, 2, -5]) == [10]\n    assert capsys.readouterr().out.strip() == '[5, 10]'\n",
    hint: "def asteroid_collision(asteroids):\n    stack = []\n    for a in asteroids:\n        alive = True\n        while alive and a < 0 and stack and stack[-1] > 0:\n            if stack[-1] < -a: stack.pop(); continue\n            elif stack[-1] == -a: stack.pop()\n            alive = False; break\n        if alive: stack.append(a)\n    return stack\nprint(asteroid_collision([5, 10, -5]))",
    solution_example: "def asteroid_collision(asteroids):\n    stack = []\n    for a in asteroids:\n        alive = True\n        while alive and a < 0 and stack and stack[-1] > 0:\n            if stack[-1] < -a: stack.pop(); continue\n            elif stack[-1] == -a: stack.pop()\n            alive = False; break\n        if alive: stack.append(a)\n    return stack\nprint(asteroid_collision([5, 10, -5]))\n",
    next: Some("py-259-simplify-path"), show_type_chips: false, micro_step: 258,
};

pub const PY259_SIMPLIFY_PATH: CodingStep = CodingStep {
    id: "py-259-simplify-path", title: "DSA Simplify Path", objective: "Normalizar una ruta Unix con `.`, `..` y barras duplicadas.",
    prompt_md: "**Simplify Path**\n\nStack de carpetas: `..` hace pop, `.` se ignora. Distinto de py-141 (paréntesis).\n\n**Micro-reto:**\n1. Definí `simplify_path(path)`\n2. `\"/home//foo/\"`; imprimí el resultado (esperado: `\"/home/foo\"`)",
    starter_code: "# def simplify_path(path):\n#     ...\n# print(simplify_path(\"/home//foo/\"))\n",
    pytest: "def test_simplify_path(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('simplify_path'))\n    assert ns['simplify_path']('/home/') == '/home'\n    assert ns['simplify_path']('/home//foo/') == '/home/foo'\n    assert ns['simplify_path']('/a/./b/../../c/') == '/c'\n    assert capsys.readouterr().out.strip() == '/home/foo'\n",
    hint: "def simplify_path(path):\n    stack = []\n    for part in path.split('/'):\n        if part == '' or part == '.': continue\n        elif part == '..':\n            if stack: stack.pop()\n        else: stack.append(part)\n    return '/' + '/'.join(stack)\nprint(simplify_path('/home//foo/'))",
    solution_example: "def simplify_path(path):\n    stack = []\n    for part in path.split('/'):\n        if part == '' or part == '.': continue\n        elif part == '..':\n            if stack: stack.pop()\n        else: stack.append(part)\n    return '/' + '/'.join(stack)\nprint(simplify_path('/home//foo/'))\n",
    next: Some("py-260-calc-ii"), show_type_chips: false, micro_step: 259,
};

pub const PY260_CALC_II: CodingStep = CodingStep {
    id: "py-260-calc-ii", title: "DSA Calculator II", objective: "Evaluar expresiones con +, −, × y ÷ sin paréntesis.",
    prompt_md: "**Basic Calculator II**\n\nStack de operandos con precedencia `*`/`/` sobre `+`/`-`. Distinto de py-210 (solo suma/resta).\n\n**Micro-reto:**\n1. Definí `calculate_ii(s)`\n2. `\"3+2*2\"`; imprimí el resultado (esperado: `7`)",
    starter_code: "# def calculate_ii(s):\n#     ...\n# print(calculate_ii(\"3+2*2\"))\n",
    pytest: "def test_calc_ii(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('calculate_ii'))\n    assert ns['calculate_ii']('3+2*2') == 7\n    assert ns['calculate_ii'](' 3/2 ') == 1\n    assert ns['calculate_ii'](' 3+5 / 2 ') == 5\n    assert capsys.readouterr().out.strip() == '7'\n",
    hint: "def calculate_ii(s):\n    stack = []; num = 0; op = '+'\n    for i, ch in enumerate(s):\n        if ch.isdigit(): num = num * 10 + int(ch)\n        if (not ch.isdigit() and ch != ' ') or i == len(s) - 1:\n            if op == '+': stack.append(num)\n            elif op == '-': stack.append(-num)\n            elif op == '*': stack[-1] *= num\n            elif op == '/': stack[-1] = int(stack[-1] / num)\n            num = 0; op = ch\n    return sum(stack)\nprint(calculate_ii('3+2*2'))",
    solution_example: "def calculate_ii(s):\n    stack = []; num = 0; op = '+'\n    for i, ch in enumerate(s):\n        if ch.isdigit(): num = num * 10 + int(ch)\n        if (not ch.isdigit() and ch != ' ') or i == len(s) - 1:\n            if op == '+': stack.append(num)\n            elif op == '-': stack.append(-num)\n            elif op == '*': stack.append(num * stack.pop())\n            elif op == '/': stack.append(int(stack.pop() / num))\n            num = 0; op = ch\n    return sum(stack)\nprint(calculate_ii('3+2*2'))\n",
    next: Some("py-261-car-fleet"), show_type_chips: false, micro_step: 260,
};

pub const PY261_CAR_FLEET: CodingStep = CodingStep {
    id: "py-261-car-fleet", title: "DSA Car Fleet", objective: "Contar flotas que llegan juntas al destino (orden + stack lógico).",
    prompt_md: "**Car Fleet**\n\nOrdená por posición descendente; cada flota avanza al tiempo del líder más lento adelante. Distinto de py-183 (sliding max).\n\n**Micro-reto:**\n1. Definí `car_fleet(target, position, speed)`\n2. `target=12`, pos `[10,8,0,5,3]`, vel `[2,4,1,1,3]`; imprimí flotas (esperado: `3`)",
    starter_code: "# def car_fleet(target, position, speed):\n#     ...\n# print(car_fleet(12, [10, 8, 0, 5, 3], [2, 4, 1, 1, 3]))\n",
    pytest: "def test_car_fleet(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('car_fleet'))\n    assert ns['car_fleet'](12, [10, 8, 0, 5, 3], [2, 4, 1, 1, 3]) == 3\n    assert ns['car_fleet'](10, [3], [3]) == 1\n    assert ns['car_fleet'](100, [0, 2, 4], [4, 2, 1]) == 1\n    assert capsys.readouterr().out.strip() == '3'\n",
    hint: "def car_fleet(target, position, speed):\n    pairs = sorted(zip(position, speed), reverse=True)\n    fleets = 0; curr_time = 0\n    for pos, spd in pairs:\n        time = (target - pos) / spd\n        if time > curr_time:\n            fleets += 1; curr_time = time\n    return fleets\nprint(car_fleet(12, [10, 8, 0, 5, 3], [2, 4, 1, 1, 3]))",
    solution_example: "def car_fleet(target, position, speed):\n    pairs = sorted(zip(position, speed), reverse=True)\n    fleets = 0; curr_time = 0\n    for pos, spd in pairs:\n        time = (target - pos) / spd\n        if time > curr_time:\n            fleets += 1; curr_time = time\n    return fleets\nprint(car_fleet(12, [10, 8, 0, 5, 3], [2, 4, 1, 1, 3]))\n",
    next: Some("py-262-largest-rect"), show_type_chips: false, micro_step: 261,
};

pub const PY262_LARGEST_RECT: CodingStep = CodingStep {
    id: "py-262-largest-rect", title: "DSA Largest Rectangle", objective: "Área máxima en histograma con stack monotónico.",
    prompt_md: "**Largest Rectangle in Histogram**\n\nSentinel `0` al final; pop cuando la altura baja. Distinto de py-177 (trapping rain).\n\n**Micro-reto:**\n1. Definí `largest_rectangle(heights)`\n2. `[2, 1, 5, 6, 2, 3]`; imprimí el área (esperado: `10`)",
    starter_code: "# def largest_rectangle(heights):\n#     ...\n# print(largest_rectangle([2, 1, 5, 6, 2, 3]))\n",
    pytest: "def test_largest_rect(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('largest_rectangle'))\n    assert ns['largest_rectangle']([2, 1, 5, 6, 2, 3]) == 10\n    assert ns['largest_rectangle']([2, 4]) == 4\n    assert ns['largest_rectangle']([1]) == 1\n    assert capsys.readouterr().out.strip() == '10'\n",
    hint: "def largest_rectangle(heights):\n    stack = []; best = 0\n    for i, h in enumerate(heights + [0]):\n        while stack and heights[stack[-1]] > h:\n            height = heights[stack.pop()]\n            width = i if not stack else i - stack[-1] - 1\n            best = max(best, height * width)\n        stack.append(i)\n    return best\nprint(largest_rectangle([2, 1, 5, 6, 2, 3]))",
    solution_example: "def largest_rectangle(heights):\n    stack = []; best = 0\n    for i, h in enumerate(heights + [0]):\n        while stack and heights[stack[-1]] > h:\n            height = heights[stack.pop()]\n            width = i if not stack else i - stack[-1] - 1\n            best = max(best, height * width)\n        stack.append(i)\n    return best\nprint(largest_rectangle([2, 1, 5, 6, 2, 3]))\n",
    next: Some("py-263-open-lock"), show_type_chips: false, micro_step: 262,
};

pub const PY263_OPEN_LOCK: CodingStep = CodingStep {
    id: "py-263-open-lock", title: "DSA Open the Lock", objective: "Abrir un candado de 4 dígitos con BFS evitando deadends.",
    prompt_md: "**Open the Lock**\n\nBFS desde `\"0000\"`; cada giro ±1 en un dígito. Distinto de py-208 (word ladder sobre diccionario).\n\n**Micro-reto:**\n1. Definí `open_lock(deadends, target)`\n2. deadends `['0201','0101','0102','1212','2002']`, target `'0202'`; imprimí pasos (esperado: `6`)",
    starter_code: "# from collections import deque\n# def open_lock(deadends, target):\n#     ...\n# print(open_lock(['0201', '0101', '0102', '1212', '2002'], '0202'))\n",
    pytest: "def test_open_lock(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('open_lock'))\n    assert ns['open_lock'](['0201', '0101', '0102', '1212', '2002'], '0202') == 6\n    assert ns['open_lock'](['8888'], '0009') == 1\n    assert ns['open_lock'](['8887', '8889', '8878', '8898', '8788', '8988', '7888', '9888'], '8888') == -1\n    assert capsys.readouterr().out.strip() == '6'\n",
    hint: "from collections import deque\n\ndef open_lock(deadends, target):\n    dead = set(deadends)\n    if '0000' in dead: return -1\n    q = deque([('0000', 0)]); seen = {'0000'}\n    while q:\n        cur, dist = q.popleft()\n        if cur == target: return dist\n        for i in range(4):\n            for d in (-1, 1):\n                nxt = cur[:i] + str((int(cur[i]) + d) % 10) + cur[i+1:]\n                if nxt not in seen and nxt not in dead:\n                    seen.add(nxt); q.append((nxt, dist + 1))\n    return -1\nprint(open_lock(['0201', '0101', '0102', '1212', '2002'], '0202'))",
    solution_example: "from collections import deque\n\ndef open_lock(deadends, target):\n    dead = set(deadends)\n    if '0000' in dead: return -1\n    q = deque([('0000', 0)]); seen = {'0000'}\n    while q:\n        cur, dist = q.popleft()\n        if cur == target: return dist\n        for i in range(4):\n            for d in (-1, 1):\n                nxt = cur[:i] + str((int(cur[i]) + d) % 10) + cur[i+1:]\n                if nxt not in seen and nxt not in dead:\n                    seen.add(nxt); q.append((nxt, dist + 1))\n    return -1\nprint(open_lock(['0201', '0101', '0102', '1212', '2002'], '0202'))\n",
    next: Some("py-264-shortest-binary"), show_type_chips: false, micro_step: 263,
};

pub const PY264_SHORTEST_BINARY: CodingStep = CodingStep {
    id: "py-264-shortest-binary", title: "DSA Shortest Binary Path", objective: "Camino más corto en grilla binaria (8 direcciones) con BFS.",
    prompt_md: "**Shortest Path in Binary Matrix**\n\nBFS 8-dir desde `(0,0)` hasta `(n-1,n-1)` solo por celdas `0`. Distinto de py-207 (rotting multi-fuente).\n\n**Micro-reto:**\n1. Definí `shortest_path_binary(grid)`\n2. `[[0,1],[1,0]]`; imprimí longitud (esperado: `2`)",
    starter_code: "# from collections import deque\n# def shortest_path_binary(grid):\n#     ...\n# print(shortest_path_binary([[0, 1], [1, 0]]))\n",
    pytest: "def test_shortest_binary(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('shortest_path_binary'))\n    assert ns['shortest_path_binary']([[0, 1], [1, 0]]) == 2\n    assert ns['shortest_path_binary']([[0, 0, 0], [1, 1, 0], [1, 1, 0]]) == 4\n    assert ns['shortest_path_binary']([[1, 0], [1, 0]]) == -1\n    assert capsys.readouterr().out.strip() == '2'\n",
    hint: "from collections import deque\n\ndef shortest_path_binary(grid):\n    n = len(grid)\n    if grid[0][0] or grid[n-1][n-1]: return -1\n    q = deque([(0, 0, 1)]); grid[0][0] = 1\n    dirs = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)]\n    while q:\n        r, c, dist = q.popleft()\n        if r == n-1 and c == n-1: return dist\n        for dr, dc in dirs:\n            nr, nc = r + dr, c + dc\n            if 0 <= nr < n and 0 <= nc < n and grid[nr][nc] == 0:\n                grid[nr][nc] = 1; q.append((nr, nc, dist + 1))\n    return -1\nprint(shortest_path_binary([[0, 1], [1, 0]]))",
    solution_example: "from collections import deque\n\ndef shortest_path_binary(grid):\n    n = len(grid)\n    if grid[0][0] or grid[n-1][n-1]: return -1\n    q = deque([(0, 0, 1)]); grid[0][0] = 1\n    dirs = [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)]\n    while q:\n        r, c, dist = q.popleft()\n        if r == n-1 and c == n-1: return dist\n        for dr, dc in dirs:\n            nr, nc = r + dr, c + dc\n            if 0 <= nr < n and 0 <= nc < n and grid[nr][nc] == 0:\n                grid[nr][nc] = 1; q.append((nr, nc, dist + 1))\n    return -1\nprint(shortest_path_binary([[0, 1], [1, 0]]))\n",
    next: Some("py-265-walls-gates"), show_type_chips: false, micro_step: 264,
};

pub const PY265_WALLS_GATES: CodingStep = CodingStep {
    id: "py-265-walls-gates", title: "DSA Walls and Gates", objective: "Rellenar distancias a la puerta más cercana con BFS multi-fuente.",
    prompt_md: "**Walls and Gates**\n\n`INF=2147483647` = habitación vacía, `0` = puerta, `-1` = muro. Distinto de py-206 (pacific atlantic DFS).\n\n**Micro-reto:**\n1. Definí `walls_and_gates(rooms)` in-place\n2. Grilla clásica 4×4; imprimí `rooms` tras el fill",
    starter_code: "# from collections import deque\n# INF = 2147483647\n# def walls_and_gates(rooms):\n#     ...\n# rooms = [[INF, -1, 0, INF], [INF, INF, INF, -1], [INF, -1, INF, -1], [0, -1, INF, INF]]\n# walls_and_gates(rooms)\n# print(rooms)\n",
    pytest: "def test_walls_gates(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('walls_and_gates'))\n    INF = 2147483647\n    rooms = [[INF, -1, 0, INF], [INF, INF, INF, -1], [INF, -1, INF, -1], [0, -1, INF, INF]]\n    ns['walls_and_gates'](rooms)\n    assert rooms == [[3, -1, 0, 1], [2, 2, 1, -1], [1, -1, 2, -1], [0, -1, 3, 4]]\n    assert capsys.readouterr().out.strip() == '[[3, -1, 0, 1], [2, 2, 1, -1], [1, -1, 2, -1], [0, -1, 3, 4]]'\n",
    hint: "from collections import deque\n\nINF = 2147483647\n\ndef walls_and_gates(rooms):\n    if not rooms: return\n    rows, cols = len(rooms), len(rooms[0])\n    q = deque()\n    for i in range(rows):\n        for j in range(cols):\n            if rooms[i][j] == 0: q.append((i, j))\n    while q:\n        r, c = q.popleft()\n        for dr, dc in ((1,0),(-1,0),(0,1),(0,-1)):\n            nr, nc = r + dr, c + dc\n            if 0 <= nr < rows and 0 <= nc < cols and rooms[nr][nc] == INF:\n                rooms[nr][nc] = rooms[r][c] + 1; q.append((nr, nc))\nrooms = [[INF, -1, 0, INF], [INF, INF, INF, -1], [INF, -1, INF, -1], [0, -1, INF, INF]]\nwalls_and_gates(rooms)\nprint(rooms)",
    solution_example: "from collections import deque\n\nINF = 2147483647\n\ndef walls_and_gates(rooms):\n    if not rooms: return\n    rows, cols = len(rooms), len(rooms[0])\n    q = deque()\n    for i in range(rows):\n        for j in range(cols):\n            if rooms[i][j] == 0: q.append((i, j))\n    while q:\n        r, c = q.popleft()\n        for dr, dc in ((1,0),(-1,0),(0,1),(0,-1)):\n            nr, nc = r + dr, c + dc\n            if 0 <= nr < rows and 0 <= nc < cols and rooms[nr][nc] == INF:\n                rooms[nr][nc] = rooms[r][c] + 1; q.append((nr, nc))\nrooms = [[INF, -1, 0, INF], [INF, INF, INF, -1], [INF, -1, INF, -1], [0, -1, INF, INF]]\nwalls_and_gates(rooms)\nprint(rooms)\n",
    next: Some("py-266-circular-queue"), show_type_chips: false, micro_step: 265,
};

pub const PY266_CIRCULAR_QUEUE: CodingStep = CodingStep {
    id: "py-266-circular-queue", title: "DSA Circular Queue", objective: "Implementar una cola circular de capacidad fija.",
    prompt_md: "**Design Circular Queue**\n\nClase `MyCircularQueue(k)` con `en_queue`, `de_queue`, `front`, `rear`, `is_empty`, `is_full`. Distinto de py-182 (cola con dos stacks).\n\n**Micro-reto:**\n1. Capacidad `3`; secuencia clásica; imprimí `[True, True, True, False, 3, True, True, True, 4]`",
    starter_code: "# class MyCircularQueue:\n#     ...\n# q = MyCircularQueue(3)\n# print([q.en_queue(1), q.en_queue(2), q.en_queue(3), q.en_queue(4), q.rear(), q.is_full(), q.de_queue(), q.en_queue(4), q.rear()])\n",
    pytest: "def test_circular_queue(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    q = ns['MyCircularQueue'](3)\n    assert q.en_queue(1) is True\n    assert q.en_queue(2) is True\n    assert q.en_queue(3) is True\n    assert q.en_queue(4) is False\n    assert q.rear() == 3\n    assert q.is_full() is True\n    assert q.de_queue() is True\n    assert q.en_queue(4) is True\n    assert q.rear() == 4\n    assert capsys.readouterr().out.strip() == '[True, True, True, False, 3, True, True, True, 4]'\n",
    hint: "class MyCircularQueue:\n    def __init__(self, k):\n        self.data = [0] * k; self.k = k; self.head = 0; self.size = 0\n    def en_queue(self, value):\n        if self.is_full(): return False\n        self.data[(self.head + self.size) % self.k] = value; self.size += 1; return True\n    def de_queue(self):\n        if self.is_empty(): return False\n        self.head = (self.head + 1) % self.k; self.size -= 1; return True\n    def front(self):\n        return -1 if self.is_empty() else self.data[self.head]\n    def rear(self):\n        return -1 if self.is_empty() else self.data[(self.head + self.size - 1) % self.k]\n    def is_empty(self): return self.size == 0\n    def is_full(self): return self.size == self.k\nq = MyCircularQueue(3)\nprint([q.en_queue(1), q.en_queue(2), q.en_queue(3), q.en_queue(4), q.rear(), q.is_full(), q.de_queue(), q.en_queue(4), q.rear()])",
    solution_example: "class MyCircularQueue:\n    def __init__(self, k):\n        self.data = [0] * k; self.k = k; self.head = 0; self.size = 0\n    def en_queue(self, value):\n        if self.is_full(): return False\n        self.data[(self.head + self.size) % self.k] = value; self.size += 1; return True\n    def de_queue(self):\n        if self.is_empty(): return False\n        self.head = (self.head + 1) % self.k; self.size -= 1; return True\n    def front(self):\n        return -1 if self.is_empty() else self.data[self.head]\n    def rear(self):\n        return -1 if self.is_empty() else self.data[(self.head + self.size - 1) % self.k]\n    def is_empty(self): return self.size == 0\n    def is_full(self): return self.size == self.k\nq = MyCircularQueue(3)\nprint([q.en_queue(1), q.en_queue(2), q.en_queue(3), q.en_queue(4), q.rear(), q.is_full(), q.de_queue(), q.en_queue(4), q.rear()])\n",
    next: Some("py-267-recent-counter"), show_type_chips: false, micro_step: 266,
};

pub const PY267_RECENT_COUNTER: CodingStep = CodingStep {
    id: "py-267-recent-counter", title: "DSA Recent Counter", objective: "Contar pings en la ventana deslizante de los últimos 3000 ms.",
    prompt_md: "**Number of Recent Calls**\n\nCola de timestamps; descartá los fuera de `[t-3000, t]`. Distinto de py-183 (sliding max).\n\n**Micro-reto:**\n1. Clase `RecentCounter` con `ping(t)`\n2. Pings `1, 100, 3001, 3002`; imprimí `[1, 2, 3, 3]`",
    starter_code: "# from collections import deque\n# class RecentCounter:\n#     ...\n",
    pytest: "def test_recent_counter(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    c = ns['RecentCounter']()\n    assert [c.ping(1), c.ping(100), c.ping(3001), c.ping(3002)] == [1, 2, 3, 3]\n    assert capsys.readouterr().out.strip() == '[1, 2, 3, 3]'\n",
    hint: "from collections import deque\n\nclass RecentCounter:\n    def __init__(self):\n        self.q = deque()\n    def ping(self, t):\n        self.q.append(t)\n        while self.q[0] < t - 3000:\n            self.q.popleft()\n        return len(self.q)\nc = RecentCounter()\nprint([c.ping(1), c.ping(100), c.ping(3001), c.ping(3002)])",
    solution_example: "from collections import deque\n\nclass RecentCounter:\n    def __init__(self):\n        self.q = deque()\n    def ping(self, t):\n        self.q.append(t)\n        while self.q[0] < t - 3000:\n            self.q.popleft()\n        return len(self.q)\nc = RecentCounter()\nprint([c.ping(1), c.ping(100), c.ping(3001), c.ping(3002)])\n",
    next: Some("py-268-time-tickets"), show_type_chips: false, micro_step: 267,
};

pub const PY268_TIME_TICKETS: CodingStep = CodingStep {
    id: "py-268-time-tickets", title: "DSA Time Needed Tickets", objective: "Simular la cola de tickets hasta que la persona k termine.",
    prompt_md: "**Time Needed to Buy Tickets**\n\nCada turno compra 1 ticket; quien llega a 0 sale. Distinto de py-84 (queue list básica).\n\n**Micro-reto:**\n1. Definí `time_required(tickets, k)`\n2. `tickets=[2,3,2]`, `k=2`; imprimí tiempo (esperado: `6`)",
    starter_code: "# def time_required(tickets, k):\n#     ...\n# print(time_required([2, 3, 2], 2))\n",
    pytest: "def test_time_tickets(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('time_required'))\n    assert ns['time_required']([2, 3, 2], 2) == 6\n    assert ns['time_required']([5, 1, 1, 1], 0) == 8\n    assert ns['time_required']([1], 0) == 1\n    assert capsys.readouterr().out.strip() == '6'\n",
    hint: "def time_required(tickets, k):\n    time = 0\n    for i, t in enumerate(tickets):\n        if i <= k: time += min(t, tickets[k])\n        else: time += min(t, tickets[k] - 1)\n    return time\nprint(time_required([2, 3, 2], 2))",
    solution_example: "def time_required(tickets, k):\n    time = 0\n    for i, t in enumerate(tickets):\n        if i <= k: time += min(t, tickets[k])\n        else: time += min(t, tickets[k] - 1)\n    return time\nprint(time_required([2, 3, 2], 2))\n",
    next: Some("py-269-last-stone"), show_type_chips: false, micro_step: 268,
};

pub const PY269_LAST_STONE: CodingStep = CodingStep {
    id: "py-269-last-stone", title: "DSA Last Stone Weight", objective: "Simular choques de piedras con un max-heap hasta quedar ≤1.",
    prompt_md: "**Last Stone Weight**\n\nSiempre chocá las dos más pesadas; si difieren, reinsertá la diferencia. Distinto de py-113 (heap intro).\n\n**Micro-reto:**\n1. Definí `last_stone_weight(stones)`\n2. `[2,7,4,1,8,1]`; imprimí el peso final (esperado: `1`)",
    starter_code: "# import heapq\n# def last_stone_weight(stones):\n#     ...\n# print(last_stone_weight([2, 7, 4, 1, 8, 1]))\n",
    pytest: "def test_last_stone(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('last_stone_weight'))\n    assert ns['last_stone_weight']([2, 7, 4, 1, 8, 1]) == 1\n    assert ns['last_stone_weight']([1]) == 1\n    assert ns['last_stone_weight']([2, 2]) == 0\n    assert capsys.readouterr().out.strip() == '1'\n",
    hint: "import heapq\n\ndef last_stone_weight(stones):\n    heap = [-s for s in stones]\n    heapq.heapify(heap)\n    while len(heap) > 1:\n        a = -heapq.heappop(heap); b = -heapq.heappop(heap)\n        if a != b: heapq.heappush(heap, -(a - b))\n    return -heap[0] if heap else 0\nprint(last_stone_weight([2, 7, 4, 1, 8, 1]))",
    solution_example: "import heapq\n\ndef last_stone_weight(stones):\n    heap = [-s for s in stones]\n    heapq.heapify(heap)\n    while len(heap) > 1:\n        a = -heapq.heappop(heap); b = -heapq.heappop(heap)\n        if a != b: heapq.heappush(heap, -(a - b))\n    return -heap[0] if heap else 0\nprint(last_stone_weight([2, 7, 4, 1, 8, 1]))\n",
    next: Some("py-270-task-scheduler"), show_type_chips: false, micro_step: 269,
};

pub const PY270_TASK_SCHEDULER: CodingStep = CodingStep {
    id: "py-270-task-scheduler", title: "DSA Task Scheduler", objective: "Calcular el tiempo mínimo con cooldown n entre tareas iguales.",
    prompt_md: "**Task Scheduler**\n\nFórmula greedy: `max(len(tasks), (max_freq-1)*(n+1) + count_of_max)`. Distinto de py-192 (top-k frequent).\n\n**Micro-reto:**\n1. Definí `least_interval(tasks, n)`\n2. `tasks=['A','A','A','B','B','B']`, `n=2`; imprimí (esperado: `8`)",
    starter_code: "# from collections import Counter\n# def least_interval(tasks, n):\n#     ...\n# print(least_interval(['A', 'A', 'A', 'B', 'B', 'B'], 2))\n",
    pytest: "def test_task_scheduler(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('least_interval'))\n    assert ns['least_interval'](['A', 'A', 'A', 'B', 'B', 'B'], 2) == 8\n    assert ns['least_interval'](['A', 'A', 'A', 'B', 'B', 'B'], 0) == 6\n    assert ns['least_interval'](['A', 'B', 'C', 'D'], 2) == 4\n    assert capsys.readouterr().out.strip() == '8'\n",
    hint: "from collections import Counter\n\ndef least_interval(tasks, n):\n    freqs = list(Counter(tasks).values())\n    max_f = max(freqs)\n    count_max = freqs.count(max_f)\n    return max(len(tasks), (max_f - 1) * (n + 1) + count_max)\nprint(least_interval(['A', 'A', 'A', 'B', 'B', 'B'], 2))",
    solution_example: "from collections import Counter\n\ndef least_interval(tasks, n):\n    freqs = list(Counter(tasks).values())\n    max_f = max(freqs)\n    count_max = freqs.count(max_f)\n    return max(len(tasks), (max_f - 1) * (n + 1) + count_max)\nprint(least_interval(['A', 'A', 'A', 'B', 'B', 'B'], 2))\n",
    next: Some("py-271-reorganize-string"), show_type_chips: false, micro_step: 270,
};

pub const PY271_REORGANIZE_STRING: CodingStep = CodingStep {
    id: "py-271-reorganize-string", title: "DSA Reorganize String", objective: "Rearreglar un string para que no haya dos caracteres iguales adyacentes.",
    prompt_md: "**Reorganize String**\n\nMax-heap de frecuencias; si es imposible devolvé `\"\"`. Distinto de py-178 (group anagrams).\n\n**Micro-reto:**\n1. Definí `reorganize_string(s)`\n2. `\"aab\"`; imprimí un reorden válido (esperado: `\"aba\"`)",
    starter_code: "# import heapq\n# from collections import Counter\n# def reorganize_string(s):\n#     ...\n# print(reorganize_string(\"aab\"))\n",
    pytest: "def test_reorganize_string(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('reorganize_string'))\n    out = ns['reorganize_string']('aab')\n    assert out == 'aba' or (len(out) == 3 and all(out[i] != out[i+1] for i in range(2)) and sorted(out) == ['a', 'a', 'b'])\n    assert ns['reorganize_string']('aaab') == ''\n    printed = capsys.readouterr().out.strip()\n    assert printed == 'aba' or (len(printed) == 3 and all(printed[i] != printed[i+1] for i in range(2)))\n",
    hint: "import heapq\nfrom collections import Counter\n\ndef reorganize_string(s):\n    heap = [(-c, ch) for ch, c in Counter(s).items()]\n    heapq.heapify(heap)\n    out = []\n    prev = (0, '')\n    while heap:\n        count, ch = heapq.heappop(heap)\n        out.append(ch)\n        if prev[0] < 0: heapq.heappush(heap, prev)\n        prev = (count + 1, ch)\n    ans = ''.join(out)\n    return ans if len(ans) == len(s) else ''\nprint(reorganize_string('aab'))",
    solution_example: "import heapq\nfrom collections import Counter\n\ndef reorganize_string(s):\n    heap = [(-c, ch) for ch, c in Counter(s).items()]\n    heapq.heapify(heap)\n    out = []\n    prev = (0, '')\n    while heap:\n        count, ch = heapq.heappop(heap)\n        out.append(ch)\n        if prev[0] < 0: heapq.heappush(heap, prev)\n        prev = (count + 1, ch)\n    ans = ''.join(out)\n    return ans if len(ans) == len(s) else ''\nprint(reorganize_string('aab'))\n",
    next: Some("py-272-find-median"), show_type_chips: false, micro_step: 271,
};

pub const PY272_FIND_MEDIAN: CodingStep = CodingStep {
    id: "py-272-find-median", title: "DSA Find Median Stream", objective: "Mantener la mediana online con dos heaps (max + min).",
    prompt_md: "**Find Median from Data Stream**\n\nClase `MedianFinder` con `add_num` y `find_median`. Distinto de py-191 (kth largest estático).\n\n**Micro-reto:**\n1. add `1`, `2`; mediana; add `3`; mediana\n2. Imprimí `[1.5, 2.0]`",
    starter_code: "# import heapq\n# class MedianFinder:\n#     ...\n",
    pytest: "def test_find_median(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    mf = ns['MedianFinder']()\n    mf.add_num(1); mf.add_num(2)\n    a = mf.find_median()\n    mf.add_num(3)\n    b = mf.find_median()\n    assert [a, b] == [1.5, 2.0]\n    assert capsys.readouterr().out.strip() == '[1.5, 2.0]'\n",
    hint: "import heapq\n\nclass MedianFinder:\n    def __init__(self):\n        self.lo = []; self.hi = []\n    def add_num(self, num):\n        heapq.heappush(self.lo, -num)\n        heapq.heappush(self.hi, -heapq.heappop(self.lo))\n        if len(self.hi) > len(self.lo):\n            heapq.heappush(self.lo, -heapq.heappop(self.hi))\n    def find_median(self):\n        if len(self.lo) > len(self.hi): return float(-self.lo[0])\n        return (-self.lo[0] + self.hi[0]) / 2.0\nmf = MedianFinder(); mf.add_num(1); mf.add_num(2); a = mf.find_median(); mf.add_num(3); b = mf.find_median()\nprint([a, b])",
    solution_example: "import heapq\n\nclass MedianFinder:\n    def __init__(self):\n        self.lo = []; self.hi = []\n    def add_num(self, num):\n        heapq.heappush(self.lo, -num)\n        heapq.heappush(self.hi, -heapq.heappop(self.lo))\n        if len(self.hi) > len(self.lo):\n            heapq.heappush(self.lo, -heapq.heappop(self.hi))\n    def find_median(self):\n        if len(self.lo) > len(self.hi): return float(-self.lo[0])\n        return (-self.lo[0] + self.hi[0]) / 2.0\nmf = MedianFinder(); mf.add_num(1); mf.add_num(2); a = mf.find_median(); mf.add_num(3); b = mf.find_median()\nprint([a, b])\n",
    next: Some("py-273-kth-matrix"), show_type_chips: false, micro_step: 272,
};

pub const PY273_KTH_MATRIX: CodingStep = CodingStep {
    id: "py-273-kth-matrix", title: "DSA Kth Matrix Element", objective: "Encontrar el k-ésimo menor en una matriz ordenada por filas/columnas.",
    prompt_md: "**Kth Smallest Element in a Sorted Matrix**\n\nMin-heap de candidatos por fila (o binary search). Distinto de py-196 (k closest points).\n\n**Micro-reto:**\n1. Definí `kth_smallest(matrix, k)`\n2. matrix `[[1,5,9],[10,11,13],[12,13,15]]`, `k=8`; imprimí (esperado: `13`)",
    starter_code: "# import heapq\n# def kth_smallest(matrix, k):\n#     ...\n# print(kth_smallest([[1, 5, 9], [10, 11, 13], [12, 13, 15]], 8))\n",
    pytest: "def test_kth_matrix(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('kth_smallest'))\n    assert ns['kth_smallest']([[1, 5, 9], [10, 11, 13], [12, 13, 15]], 8) == 13\n    assert ns['kth_smallest']([[1, 2], [1, 3]], 2) == 1\n    assert ns['kth_smallest']([[-5]], 1) == -5\n    assert capsys.readouterr().out.strip() == '13'\n",
    hint: "import heapq\n\ndef kth_smallest(matrix, k):\n    n = len(matrix)\n    heap = [(matrix[i][0], i, 0) for i in range(n)]\n    heapq.heapify(heap)\n    for _ in range(k):\n        val, r, c = heapq.heappop(heap)\n        if c + 1 < n: heapq.heappush(heap, (matrix[r][c + 1], r, c + 1))\n    return val\nprint(kth_smallest([[1, 5, 9], [10, 11, 13], [12, 13, 15]], 8))",
    solution_example: "import heapq\n\ndef kth_smallest(matrix, k):\n    n = len(matrix)\n    heap = [(matrix[i][0], i, 0) for i in range(n)]\n    heapq.heapify(heap)\n    for _ in range(k):\n        val, r, c = heapq.heappop(heap)\n        if c + 1 < n: heapq.heappush(heap, (matrix[r][c + 1], r, c + 1))\n    return val\nprint(kth_smallest([[1, 5, 9], [10, 11, 13], [12, 13, 15]], 8))\n",
    next: Some("py-274-network-delay"), show_type_chips: false, micro_step: 273,
};

pub const PY274_NETWORK_DELAY: CodingStep = CodingStep {
    id: "py-274-network-delay", title: "DSA Network Delay Time", objective: "Tiempo para que una señal llegue a todos los nodos (Dijkstra).",
    prompt_md: "**Network Delay Time**\n\nDijkstra desde `k`; si algún nodo es inalcanzable → `-1`. Distinto de py-112 (dijkstra intro genérico).\n\n**Micro-reto:**\n1. Definí `network_delay_time(times, n, k)`\n2. times `[[2,1,1],[2,3,1],[3,4,1]]`, `n=4`, `k=2`; imprimí (esperado: `2`)",
    starter_code: "# import heapq\n# from collections import defaultdict\n# def network_delay_time(times, n, k):\n#     ...\n# print(network_delay_time([[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2))\n",
    pytest: "def test_network_delay(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('network_delay_time'))\n    assert ns['network_delay_time']([[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2) == 2\n    assert ns['network_delay_time']([[1, 2, 1]], 2, 1) == 1\n    assert ns['network_delay_time']([[1, 2, 1]], 2, 2) == -1\n    assert capsys.readouterr().out.strip() == '2'\n",
    hint: "import heapq\nfrom collections import defaultdict\n\ndef network_delay_time(times, n, k):\n    graph = defaultdict(list)\n    for u, v, w in times: graph[u].append((v, w))\n    dist = {k: 0}; heap = [(0, k)]\n    while heap:\n        d, node = heapq.heappop(heap)\n        if d > dist.get(node, float('inf')): continue\n        for nei, w in graph[node]:\n            nd = d + w\n            if nd < dist.get(nei, float('inf')):\n                dist[nei] = nd; heapq.heappush(heap, (nd, nei))\n    return max(dist.values()) if len(dist) == n else -1\nprint(network_delay_time([[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2))",
    solution_example: "import heapq\nfrom collections import defaultdict\n\ndef network_delay_time(times, n, k):\n    graph = defaultdict(list)\n    for u, v, w in times: graph[u].append((v, w))\n    dist = {k: 0}; heap = [(0, k)]\n    while heap:\n        d, node = heapq.heappop(heap)\n        if d > dist.get(node, float('inf')): continue\n        for nei, w in graph[node]:\n            nd = d + w\n            if nd < dist.get(nei, float('inf')):\n                dist[nei] = nd; heapq.heappush(heap, (nd, nei))\n    return max(dist.values()) if len(dist) == n else -1\nprint(network_delay_time([[2, 1, 1], [2, 3, 1], [3, 4, 1]], 4, 2))\n",
    next: Some("py-275-course-order"), show_type_chips: false, micro_step: 274,
};

pub const PY275_COURSE_ORDER: CodingStep = CodingStep {
    id: "py-275-course-order", title: "DSA Course Schedule II", objective: "Devolver un orden topológico válido de cursos (o vacío si hay ciclo).",
    prompt_md: "**Course Schedule II**\n\nKahn (BFS indegree) o DFS postorder. Distinto de py-205 (solo bool).\n\n**Micro-reto:**\n1. Definí `find_order(num_courses, prerequisites)`\n2. `num_courses=4`, prereqs `[[1,0],[2,0],[3,1],[3,2]]`; imprimí un orden válido que empiece por `0` (esperado: `[0, 1, 2, 3]` o `[0, 2, 1, 3]` — usá Kahn estable por cola FIFO y append en orden de descubrimiento desde 0..n-1)",
    starter_code: "# from collections import deque, defaultdict\n# def find_order(num_courses, prerequisites):\n#     ...\n# print(find_order(4, [[1, 0], [2, 0], [3, 1], [3, 2]]))\n",
    pytest: "def test_course_order(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('find_order'))\n    order = ns['find_order'](4, [[1, 0], [2, 0], [3, 1], [3, 2]])\n    assert order in ([0, 1, 2, 3], [0, 2, 1, 3])\n    assert ns['find_order'](2, [[1, 0], [0, 1]]) == []\n    assert ns['find_order'](1, []) == [0]\n    printed = capsys.readouterr().out.strip()\n    assert printed in ('[0, 1, 2, 3]', '[0, 2, 1, 3]')\n",
    hint: "from collections import deque, defaultdict\n\ndef find_order(num_courses, prerequisites):\n    graph = defaultdict(list); indeg = [0] * num_courses\n    for a, b in prerequisites:\n        graph[b].append(a); indeg[a] += 1\n    q = deque([i for i in range(num_courses) if indeg[i] == 0])\n    order = []\n    while q:\n        u = q.popleft(); order.append(u)\n        for v in graph[u]:\n            indeg[v] -= 1\n            if indeg[v] == 0: q.append(v)\n    return order if len(order) == num_courses else []\nprint(find_order(4, [[1, 0], [2, 0], [3, 1], [3, 2]]))",
    solution_example: "from collections import deque, defaultdict\n\ndef find_order(num_courses, prerequisites):\n    graph = defaultdict(list); indeg = [0] * num_courses\n    for a, b in prerequisites:\n        graph[b].append(a); indeg[a] += 1\n    q = deque([i for i in range(num_courses) if indeg[i] == 0])\n    order = []\n    while q:\n        u = q.popleft(); order.append(u)\n        for v in graph[u]:\n            indeg[v] -= 1\n            if indeg[v] == 0: q.append(v)\n    return order if len(order) == num_courses else []\nprint(find_order(4, [[1, 0], [2, 0], [3, 1], [3, 2]]))\n",
    next: Some("py-276-cheapest-flights"), show_type_chips: false, micro_step: 275,
};

pub const PY276_CHEAPEST_FLIGHTS: CodingStep = CodingStep {
    id: "py-276-cheapest-flights", title: "DSA Cheapest Flights", objective: "Vuelo más barato con a lo sumo k escalas (Bellman-Ford acotado).",
    prompt_md: "**Cheapest Flights Within K Stops**\n\nRelajá aristas hasta `k+1` veces. Distinto de py-274 (Dijkstra sin límite de hops).\n\n**Micro-reto:**\n1. Definí `find_cheapest_price(n, flights, src, dst, k)`\n2. n=4, flights `[[0,1,100],[1,2,100],[2,0,100],[1,3,600],[2,3,200]]`, src=0, dst=3, k=1; imprimí (esperado: `700`)",
    starter_code: "# def find_cheapest_price(n, flights, src, dst, k):\n#     ...\n# print(find_cheapest_price(4, [[0, 1, 100], [1, 2, 100], [2, 0, 100], [1, 3, 600], [2, 3, 200]], 0, 3, 1))\n",
    pytest: "def test_cheapest_flights(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('find_cheapest_price'))\n    flights = [[0, 1, 100], [1, 2, 100], [2, 0, 100], [1, 3, 600], [2, 3, 200]]\n    assert ns['find_cheapest_price'](4, flights, 0, 3, 1) == 700\n    assert ns['find_cheapest_price'](3, [[0, 1, 100], [1, 2, 100], [0, 2, 500]], 0, 2, 1) == 200\n    assert ns['find_cheapest_price'](3, [[0, 1, 100], [1, 2, 100], [0, 2, 500]], 0, 2, 0) == 500\n    assert capsys.readouterr().out.strip() == '700'\n",
    hint: "def find_cheapest_price(n, flights, src, dst, k):\n    prices = [float('inf')] * n; prices[src] = 0\n    for _ in range(k + 1):\n        nxt = prices[:]\n        for u, v, w in flights:\n            if prices[u] + w < nxt[v]: nxt[v] = prices[u] + w\n        prices = nxt\n    return -1 if prices[dst] == float('inf') else prices[dst]\nprint(find_cheapest_price(4, [[0, 1, 100], [1, 2, 100], [2, 0, 100], [1, 3, 600], [2, 3, 200]], 0, 3, 1))",
    solution_example: "def find_cheapest_price(n, flights, src, dst, k):\n    prices = [float('inf')] * n; prices[src] = 0\n    for _ in range(k + 1):\n        nxt = prices[:]\n        for u, v, w in flights:\n            if prices[u] + w < nxt[v]: nxt[v] = prices[u] + w\n        prices = nxt\n    return -1 if prices[dst] == float('inf') else prices[dst]\nprint(find_cheapest_price(4, [[0, 1, 100], [1, 2, 100], [2, 0, 100], [1, 3, 600], [2, 3, 200]], 0, 3, 1))\n",
    next: Some("py-277-redundant-edge"), show_type_chips: false, micro_step: 276,
};

pub const PY277_REDUNDANT_EDGE: CodingStep = CodingStep {
    id: "py-277-redundant-edge", title: "DSA Redundant Connection", objective: "Encontrar la arista que forma un ciclo en un grafo casi-árbol.",
    prompt_md: "**Redundant Connection**\n\nUnion-Find: la primera arista cuyos extremos ya están unidos es la redundante. Distinto de py-115 (UF intro).\n\n**Micro-reto:**\n1. Definí `find_redundant_connection(edges)`\n2. `[[1,2],[1,3],[2,3]]`; imprimí (esperado: `[2, 3]`)",
    starter_code: "# def find_redundant_connection(edges):\n#     ...\n# print(find_redundant_connection([[1, 2], [1, 3], [2, 3]]))\n",
    pytest: "def test_redundant_edge(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('find_redundant_connection'))\n    assert ns['find_redundant_connection']([[1, 2], [1, 3], [2, 3]]) == [2, 3]\n    assert ns['find_redundant_connection']([[1, 2], [2, 3], [3, 4], [1, 4], [1, 5]]) == [1, 4]\n    assert capsys.readouterr().out.strip() == '[2, 3]'\n",
    hint: "def find_redundant_connection(edges):\n    parent = list(range(len(edges) + 1))\n    def find(x):\n        while parent[x] != x:\n            parent[x] = parent[parent[x]]; x = parent[x]\n        return x\n    for a, b in edges:\n        ra, rb = find(a), find(b)\n        if ra == rb: return [a, b]\n        parent[rb] = ra\n    return []\nprint(find_redundant_connection([[1, 2], [1, 3], [2, 3]]))",
    solution_example: "def find_redundant_connection(edges):\n    parent = list(range(len(edges) + 1))\n    def find(x):\n        while parent[x] != x:\n            parent[x] = parent[parent[x]]; x = parent[x]\n        return x\n    for a, b in edges:\n        ra, rb = find(a), find(b)\n        if ra == rb: return [a, b]\n        parent[rb] = ra\n    return []\nprint(find_redundant_connection([[1, 2], [1, 3], [2, 3]]))\n",
    next: Some("py-278-accounts-merge"), show_type_chips: false, micro_step: 277,
};

pub const PY278_ACCOUNTS_MERGE: CodingStep = CodingStep {
    id: "py-278-accounts-merge", title: "DSA Accounts Merge", objective: "Fusionar cuentas que comparten emails con Union-Find.",
    prompt_md: "**Accounts Merge**\n\nUF sobre índices de cuenta; devolvés `[name, ...emails ordenados]`. Distinto de py-178 (anagramas).\n\n**Micro-reto:**\n1. Definí `accounts_merge(accounts)`\n2. Input clásico John; imprimí el resultado ordenado por nombre+primer email (esperado una lista de 2 cuentas)",
    starter_code: "# from collections import defaultdict\n# def accounts_merge(accounts):\n#     ...\n# accounts = [['John', 'j1@mail.com', 'j2@mail.com'], ['John', 'j3@mail.com'], ['John', 'j1@mail.com', 'j4@mail.com'], ['Mary', 'm@mail.com']]\n# print(accounts_merge(accounts))\n",
    pytest: "def test_accounts_merge(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('accounts_merge'))\n    accounts = [['John', 'j1@mail.com', 'j2@mail.com'], ['John', 'j3@mail.com'], ['John', 'j1@mail.com', 'j4@mail.com'], ['Mary', 'm@mail.com']]\n    expected = [['John', 'j1@mail.com', 'j2@mail.com', 'j4@mail.com'], ['John', 'j3@mail.com'], ['Mary', 'm@mail.com']]\n    merged = sorted([[a[0]] + sorted(a[1:]) for a in ns['accounts_merge'](accounts)])\n    assert merged == expected\n    printed = eval(capsys.readouterr().out.strip())\n    assert sorted([[a[0]] + sorted(a[1:]) for a in printed]) == expected\n",
    hint: "from collections import defaultdict\n\ndef accounts_merge(accounts):\n    n = len(accounts); parent = list(range(n))\n    def find(x):\n        while parent[x] != x:\n            parent[x] = parent[parent[x]]; x = parent[x]\n        return x\n    email_to_id = {}\n    for i, acc in enumerate(accounts):\n        for email in acc[1:]:\n            if email in email_to_id: parent[find(i)] = find(email_to_id[email])\n            else: email_to_id[email] = i\n    groups = defaultdict(set)\n    for email, i in email_to_id.items():\n        groups[find(i)].add(email)\n    out = [[accounts[i][0]] + sorted(emails) for i, emails in groups.items()]\n    return sorted(out, key=lambda a: (a[0], a[1]))\naccounts = [['John', 'j1@mail.com', 'j2@mail.com'], ['John', 'j3@mail.com'], ['John', 'j1@mail.com', 'j4@mail.com'], ['Mary', 'm@mail.com']]\nprint(accounts_merge(accounts))",
    solution_example: "from collections import defaultdict\n\ndef accounts_merge(accounts):\n    n = len(accounts); parent = list(range(n))\n    def find(x):\n        while parent[x] != x:\n            parent[x] = parent[parent[x]]; x = parent[x]\n        return x\n    email_to_id = {}\n    for i, acc in enumerate(accounts):\n        for email in acc[1:]:\n            if email in email_to_id: parent[find(i)] = find(email_to_id[email])\n            else: email_to_id[email] = i\n    groups = defaultdict(set)\n    for email, i in email_to_id.items():\n        groups[find(i)].add(email)\n    out = [[accounts[i][0]] + sorted(emails) for i, emails in groups.items()]\n    return sorted(out, key=lambda a: (a[0], a[1]))\naccounts = [['John', 'j1@mail.com', 'j2@mail.com'], ['John', 'j3@mail.com'], ['John', 'j1@mail.com', 'j4@mail.com'], ['Mary', 'm@mail.com']]\nprint(accounts_merge(accounts))\n",
    next: Some("py-279-alien-dict"), show_type_chips: false, micro_step: 278,
};

pub const PY279_ALIEN_DICT: CodingStep = CodingStep {
    id: "py-279-alien-dict", title: "DSA Alien Dictionary", objective: "Reconstruir el orden de letras de un alfabeto alienígena.",
    prompt_md: "**Alien Dictionary**\n\nCompará palabras consecutivas → aristas de precedencia; Kahn. Distinto de py-118 (topo genérico).\n\n**Micro-reto:**\n1. Definí `alien_order(words)`\n2. `['wrt','wrf','er','ett','rftt']`; imprimí (esperado: `\"wertf\"`)",
    starter_code: "# from collections import defaultdict, deque\n# def alien_order(words):\n#     ...\n# print(alien_order(['wrt', 'wrf', 'er', 'ett', 'rftt']))\n",
    pytest: "def test_alien_dict(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('alien_order'))\n    assert ns['alien_order'](['wrt', 'wrf', 'er', 'ett', 'rftt']) == 'wertf'\n    assert ns['alien_order'](['z', 'x']) == 'zx'\n    assert ns['alien_order'](['z', 'x', 'z']) == ''\n    assert capsys.readouterr().out.strip() == 'wertf'\n",
    hint: "from collections import defaultdict, deque\n\ndef alien_order(words):\n    graph = defaultdict(set); indeg = {c: 0 for w in words for c in w}\n    for w1, w2 in zip(words, words[1:]):\n        if len(w1) > len(w2) and w1.startswith(w2): return ''\n        for a, b in zip(w1, w2):\n            if a != b:\n                if b not in graph[a]:\n                    graph[a].add(b); indeg[b] += 1\n                break\n    q = deque(sorted([c for c in indeg if indeg[c] == 0]))\n    out = []\n    while q:\n        u = q.popleft(); out.append(u)\n        for v in sorted(graph[u]):\n            indeg[v] -= 1\n            if indeg[v] == 0: q.append(v)\n    return ''.join(out) if len(out) == len(indeg) else ''\nprint(alien_order(['wrt', 'wrf', 'er', 'ett', 'rftt']))",
    solution_example: "from collections import defaultdict, deque\n\ndef alien_order(words):\n    graph = defaultdict(set); indeg = {c: 0 for w in words for c in w}\n    for w1, w2 in zip(words, words[1:]):\n        if len(w1) > len(w2) and w1.startswith(w2): return ''\n        for a, b in zip(w1, w2):\n            if a != b:\n                if b not in graph[a]:\n                    graph[a].add(b); indeg[b] += 1\n                break\n    q = deque(sorted([c for c in indeg if indeg[c] == 0]))\n    out = []\n    while q:\n        u = q.popleft(); out.append(u)\n        for v in sorted(graph[u]):\n            indeg[v] -= 1\n            if indeg[v] == 0: q.append(v)\n    return ''.join(out) if len(out) == len(indeg) else ''\nprint(alien_order(['wrt', 'wrf', 'er', 'ett', 'rftt']))\n",
    next: Some("py-280-min-cost-points"), show_type_chips: false, micro_step: 279,
};

pub const PY280_MIN_COST_POINTS: CodingStep = CodingStep {
    id: "py-280-min-cost-points", title: "DSA Min Cost Points", objective: "Conectar todos los puntos con costo Manhattan mínimo (MST).",
    prompt_md: "**Min Cost to Connect All Points**\n\nPrim desde 0 o Kruskal+UF. Distinto de py-116/117 (MST intro).\n\n**Micro-reto:**\n1. Definí `min_cost_connect(points)`\n2. `[[0,0],[2,2],[3,10],[5,2],[7,0]]`; imprimí (esperado: `20`)",
    starter_code: "# import heapq\n# def min_cost_connect(points):\n#     ...\n# print(min_cost_connect([[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]]))\n",
    pytest: "def test_min_cost_points(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('min_cost_connect'))\n    assert ns['min_cost_connect']([[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]]) == 20\n    assert ns['min_cost_connect']([[3, 12], [-2, 5], [-4, 1]]) == 18\n    assert ns['min_cost_connect']([[0, 0]]) == 0\n    assert capsys.readouterr().out.strip() == '20'\n",
    hint: "import heapq\n\ndef min_cost_connect(points):\n    n = len(points)\n    if n <= 1: return 0\n    in_mst = [False] * n; heap = [(0, 0)]; cost = 0; used = 0\n    while heap and used < n:\n        d, i = heapq.heappop(heap)\n        if in_mst[i]: continue\n        in_mst[i] = True; cost += d; used += 1\n        xi, yi = points[i]\n        for j in range(n):\n            if not in_mst[j]:\n                xj, yj = points[j]\n                heapq.heappush(heap, (abs(xi - xj) + abs(yi - yj), j))\n    return cost\nprint(min_cost_connect([[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]]))",
    solution_example: "import heapq\n\ndef min_cost_connect(points):\n    n = len(points)\n    if n <= 1: return 0\n    in_mst = [False] * n; heap = [(0, 0)]; cost = 0; used = 0\n    while heap and used < n:\n        d, i = heapq.heappop(heap)\n        if in_mst[i]: continue\n        in_mst[i] = True; cost += d; used += 1\n        xi, yi = points[i]\n        for j in range(n):\n            if not in_mst[j]:\n                xj, yj = points[j]\n                heapq.heappush(heap, (abs(xi - xj) + abs(yi - yj), j))\n    return cost\nprint(min_cost_connect([[0, 0], [2, 2], [3, 10], [5, 2], [7, 0]]))\n",
    next: Some("py-281-jump-game-ii"), show_type_chips: false, micro_step: 280,
};

pub const PY281_JUMP_GAME_II: CodingStep = CodingStep {
    id: "py-281-jump-game-ii", title: "DSA Jump Game II", objective: "Mínimo de saltos para llegar al final (greedy por alcance).",
    prompt_md: "**Jump Game II**\n\nVentana `[start, end]`; cada salto extiende el alcance máximo. Distinto de py-173 (solo alcanzabilidad).\n\n**Micro-reto:**\n1. Definí `jump(nums)`\n2. `[2,3,1,1,4]`; imprimí saltos (esperado: `2`)",
    starter_code: "# def jump(nums):\n#     ...\n# print(jump([2, 3, 1, 1, 4]))\n",
    pytest: "def test_jump_game_ii(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('jump'))\n    assert ns['jump']([2, 3, 1, 1, 4]) == 2\n    assert ns['jump']([2, 3, 0, 1, 4]) == 2\n    assert ns['jump']([1]) == 0\n    assert capsys.readouterr().out.strip() == '2'\n",
    hint: "def jump(nums):\n    jumps = end = farthest = 0\n    for i in range(len(nums) - 1):\n        farthest = max(farthest, i + nums[i])\n        if i == end:\n            jumps += 1; end = farthest\n    return jumps\nprint(jump([2, 3, 1, 1, 4]))",
    solution_example: "def jump(nums):\n    jumps = end = farthest = 0\n    for i in range(len(nums) - 1):\n        farthest = max(farthest, i + nums[i])\n        if i == end:\n            jumps += 1; end = farthest\n    return jumps\nprint(jump([2, 3, 1, 1, 4]))\n",
    next: Some("py-282-target-sum"), show_type_chips: false, micro_step: 281,
};

pub const PY282_TARGET_SUM: CodingStep = CodingStep {
    id: "py-282-target-sum", title: "DSA Target Sum", objective: "Contar formas de asignar +/− a cada número para llegar al target.",
    prompt_md: "**Target Sum**\n\nEquivalente a subset-sum hacia `(sum+target)/2`. Distinto de py-201 (partition equal subset).\n\n**Micro-reto:**\n1. Definí `find_target_sum_ways(nums, target)`\n2. nums=`[1,1,1,1,1]`, target=`3`; imprimí (esperado: `5`)",
    starter_code: "# def find_target_sum_ways(nums, target):\n#     ...\n# print(find_target_sum_ways([1, 1, 1, 1, 1], 3))\n",
    pytest: "def test_target_sum(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('find_target_sum_ways'))\n    assert ns['find_target_sum_ways']([1, 1, 1, 1, 1], 3) == 5\n    assert ns['find_target_sum_ways']([1], 1) == 1\n    assert ns['find_target_sum_ways']([1], 2) == 0\n    assert capsys.readouterr().out.strip() == '5'\n",
    hint: "def find_target_sum_ways(nums, target):\n    total = sum(nums)\n    if (total + target) % 2 or abs(target) > total: return 0\n    subset = (total + target) // 2\n    dp = [0] * (subset + 1); dp[0] = 1\n    for num in nums:\n        for s in range(subset, num - 1, -1):\n            dp[s] += dp[s - num]\n    return dp[subset]\nprint(find_target_sum_ways([1, 1, 1, 1, 1], 3))",
    solution_example: "def find_target_sum_ways(nums, target):\n    total = sum(nums)\n    if (total + target) % 2 or abs(target) > total: return 0\n    subset = (total + target) // 2\n    dp = [0] * (subset + 1); dp[0] = 1\n    for num in nums:\n        for s in range(subset, num - 1, -1):\n            dp[s] += dp[s - num]\n    return dp[subset]\nprint(find_target_sum_ways([1, 1, 1, 1, 1], 3))\n",
    next: Some("py-283-maximal-square"), show_type_chips: false, micro_step: 282,
};

pub const PY283_MAXIMAL_SQUARE: CodingStep = CodingStep {
    id: "py-283-maximal-square", title: "DSA Maximal Square", objective: "Área del cuadrado máximo de 1s en una matriz binaria.",
    prompt_md: "**Maximal Square**\n\n`dp[i][j] = min(arriba, izq, diag) + 1` si celda es `'1'`. Distinto de py-203 (num islands).\n\n**Micro-reto:**\n1. Definí `maximal_square(matrix)`\n2. Matriz clásica 4×5; imprimí área (esperado: `4`)",
    starter_code: "# def maximal_square(matrix):\n#     ...\n# print(maximal_square([['1','0','1','0','0'],['1','0','1','1','1'],['1','1','1','1','1'],['1','0','0','1','0']]))\n",
    pytest: "def test_maximal_square(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('maximal_square'))\n    m = [['1','0','1','0','0'],['1','0','1','1','1'],['1','1','1','1','1'],['1','0','0','1','0']]\n    assert ns['maximal_square'](m) == 4\n    assert ns['maximal_square']([['0','1'],['1','0']]) == 1\n    assert ns['maximal_square']([['0']]) == 0\n    assert capsys.readouterr().out.strip() == '4'\n",
    hint: "def maximal_square(matrix):\n    if not matrix: return 0\n    rows, cols = len(matrix), len(matrix[0])\n    dp = [[0] * (cols + 1) for _ in range(rows + 1)]\n    best = 0\n    for i in range(1, rows + 1):\n        for j in range(1, cols + 1):\n            if matrix[i-1][j-1] == '1':\n                dp[i][j] = min(dp[i-1][j], dp[i][j-1], dp[i-1][j-1]) + 1\n                best = max(best, dp[i][j])\n    return best * best\nprint(maximal_square([['1','0','1','0','0'],['1','0','1','1','1'],['1','1','1','1','1'],['1','0','0','1','0']]))",
    solution_example: "def maximal_square(matrix):\n    if not matrix: return 0\n    rows, cols = len(matrix), len(matrix[0])\n    dp = [[0] * (cols + 1) for _ in range(rows + 1)]\n    best = 0\n    for i in range(1, rows + 1):\n        for j in range(1, cols + 1):\n            if matrix[i-1][j-1] == '1':\n                dp[i][j] = min(dp[i-1][j], dp[i][j-1], dp[i-1][j-1]) + 1\n                best = max(best, dp[i][j])\n    return best * best\nprint(maximal_square([['1','0','1','0','0'],['1','0','1','1','1'],['1','1','1','1','1'],['1','0','0','1','0']]))\n",
    next: Some("py-284-stock-cooldown"), show_type_chips: false, micro_step: 283,
};

pub const PY284_STOCK_COOLDOWN: CodingStep = CodingStep {
    id: "py-284-stock-cooldown", title: "DSA Stock Cooldown", objective: "Máximo profit con cooldown de un día tras vender.",
    prompt_md: "**Best Time to Buy and Sell Stock with Cooldown**\n\nEstados hold / sold / rest. Distinto de py-156 (una sola transacción).\n\n**Micro-reto:**\n1. Definí `max_profit_cooldown(prices)`\n2. `[1,2,3,0,2]`; imprimí (esperado: `3`)",
    starter_code: "# def max_profit_cooldown(prices):\n#     ...\n# print(max_profit_cooldown([1, 2, 3, 0, 2]))\n",
    pytest: "def test_stock_cooldown(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('max_profit_cooldown'))\n    assert ns['max_profit_cooldown']([1, 2, 3, 0, 2]) == 3\n    assert ns['max_profit_cooldown']([1]) == 0\n    assert ns['max_profit_cooldown']([1, 2]) == 1\n    assert capsys.readouterr().out.strip() == '3'\n",
    hint: "def max_profit_cooldown(prices):\n    hold = float('-inf'); sold = 0; rest = 0\n    for p in prices:\n        prev_sold = sold\n        sold = hold + p\n        hold = max(hold, rest - p)\n        rest = max(rest, prev_sold)\n    return max(sold, rest)\nprint(max_profit_cooldown([1, 2, 3, 0, 2]))",
    solution_example: "def max_profit_cooldown(prices):\n    hold = float('-inf'); sold = 0; rest = 0\n    for p in prices:\n        prev_sold = sold\n        sold = hold + p\n        hold = max(hold, rest - p)\n        rest = max(rest, prev_sold)\n    return max(sold, rest)\nprint(max_profit_cooldown([1, 2, 3, 0, 2]))\n",
    next: Some("py-285-interleaving"), show_type_chips: false, micro_step: 284,
};

pub const PY285_INTERLEAVING: CodingStep = CodingStep {
    id: "py-285-interleaving", title: "DSA Interleaving String", objective: "Decidir si s3 es entrelazado de s1 y s2 preservando orden.",
    prompt_md: "**Interleaving String**\n\nDP booleana 2D (o 1D). Distinto de py-128 (LCS longitud).\n\n**Micro-reto:**\n1. Definí `is_interleave(s1, s2, s3)`\n2. `s1=\"aabcc\"`, `s2=\"dbbca\"`, `s3=\"aadbbcbcac\"`; imprimí `True`",
    starter_code: "# def is_interleave(s1, s2, s3):\n#     ...\n# print(is_interleave(\"aabcc\", \"dbbca\", \"aadbbcbcac\"))\n",
    pytest: "def test_interleaving(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('is_interleave'))\n    assert ns['is_interleave']('aabcc', 'dbbca', 'aadbbcbcac') is True\n    assert ns['is_interleave']('aabcc', 'dbbca', 'aadbbbaccc') is False\n    assert ns['is_interleave']('', '', '') is True\n    assert capsys.readouterr().out.strip() == 'True'\n",
    hint: "def is_interleave(s1, s2, s3):\n    m, n = len(s1), len(s2)\n    if m + n != len(s3): return False\n    dp = [False] * (n + 1); dp[0] = True\n    for j in range(1, n + 1):\n        dp[j] = dp[j-1] and s2[j-1] == s3[j-1]\n    for i in range(1, m + 1):\n        dp[0] = dp[0] and s1[i-1] == s3[i-1]\n        for j in range(1, n + 1):\n            dp[j] = (dp[j] and s1[i-1] == s3[i+j-1]) or (dp[j-1] and s2[j-1] == s3[i+j-1])\n    return dp[n]\nprint(is_interleave('aabcc', 'dbbca', 'aadbbcbcac'))",
    solution_example: "def is_interleave(s1, s2, s3):\n    m, n = len(s1), len(s2)\n    if m + n != len(s3): return False\n    dp = [False] * (n + 1); dp[0] = True\n    for j in range(1, n + 1):\n        dp[j] = dp[j-1] and s2[j-1] == s3[j-1]\n    for i in range(1, m + 1):\n        dp[0] = dp[0] and s1[i-1] == s3[i-1]\n        for j in range(1, n + 1):\n            dp[j] = (dp[j] and s1[i-1] == s3[i+j-1]) or (dp[j-1] and s2[j-1] == s3[i+j-1])\n    return dp[n]\nprint(is_interleave('aabcc', 'dbbca', 'aadbbcbcac'))\n",
    next: Some("py-286-palindrome-subseq"), show_type_chips: false, micro_step: 285,
};

pub const PY286_PALINDROME_SUBSEQ: CodingStep = CodingStep {
    id: "py-286-palindrome-subseq", title: "DSA Palindrome Subseq", objective: "Longitud de la subsecuencia palindrómica más larga.",
    prompt_md: "**Longest Palindromic Subsequence**\n\nDP intervalo: si extremos iguales, `2 + dp[i+1][j-1]`. Distinto de py-154 (substring contigua) y py-250 (lista).\n\n**Micro-reto:**\n1. Definí `longest_palindrome_subseq(s)`\n2. `\"bbbab\"`; imprimí (esperado: `4`)",
    starter_code: "# def longest_palindrome_subseq(s):\n#     ...\n# print(longest_palindrome_subseq(\"bbbab\"))\n",
    pytest: "def test_palindrome_subseq(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('longest_palindrome_subseq'))\n    assert ns['longest_palindrome_subseq']('bbbab') == 4\n    assert ns['longest_palindrome_subseq']('cbbd') == 2\n    assert ns['longest_palindrome_subseq']('a') == 1\n    assert capsys.readouterr().out.strip() == '4'\n",
    hint: "def longest_palindrome_subseq(s):\n    n = len(s)\n    dp = [[0] * n for _ in range(n)]\n    for i in range(n): dp[i][i] = 1\n    for length in range(2, n + 1):\n        for i in range(n - length + 1):\n            j = i + length - 1\n            if s[i] == s[j]:\n                dp[i][j] = 2 if length == 2 else 2 + dp[i+1][j-1]\n            else:\n                dp[i][j] = max(dp[i+1][j], dp[i][j-1])\n    return dp[0][n-1]\nprint(longest_palindrome_subseq('bbbab'))",
    solution_example: "def longest_palindrome_subseq(s):\n    n = len(s)\n    dp = [[0] * n for _ in range(n)]\n    for i in range(n): dp[i][i] = 1\n    for length in range(2, n + 1):\n        for i in range(n - length + 1):\n            j = i + length - 1\n            if s[i] == s[j]:\n                dp[i][j] = 2 if length == 2 else 2 + dp[i+1][j-1]\n            else:\n                dp[i][j] = max(dp[i+1][j], dp[i][j-1])\n    return dp[0][n-1]\nprint(longest_palindrome_subseq('bbbab'))\n",
    next: Some("py-287-koko-bananas"), show_type_chips: false, micro_step: 286,
};

pub const PY287_KOKO_BANANAS: CodingStep = CodingStep {
    id: "py-287-koko-bananas", title: "DSA Koko Bananas", objective: "Velocidad mínima para comer todos los pilones en h horas.",
    prompt_md: "**Koko Eating Bananas**\n\nBinary search sobre la velocidad `k`. Distinto de py-189 (ship capacity).\n\n**Micro-reto:**\n1. Definí `min_eating_speed(piles, h)`\n2. piles=`[3,6,7,11]`, h=`8`; imprimí (esperado: `4`)",
    starter_code: "# def min_eating_speed(piles, h):\n#     ...\n# print(min_eating_speed([3, 6, 7, 11], 8))\n",
    pytest: "def test_koko_bananas(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('min_eating_speed'))\n    assert ns['min_eating_speed']([3, 6, 7, 11], 8) == 4\n    assert ns['min_eating_speed']([30, 11, 23, 4, 20], 5) == 30\n    assert ns['min_eating_speed']([30, 11, 23, 4, 20], 6) == 23\n    assert capsys.readouterr().out.strip() == '4'\n",
    hint: "def min_eating_speed(piles, h):\n    def hours(k):\n        return sum((p + k - 1) // k for p in piles)\n    lo, hi = 1, max(piles)\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if hours(mid) <= h: hi = mid\n        else: lo = mid + 1\n    return lo\nprint(min_eating_speed([3, 6, 7, 11], 8))",
    solution_example: "def min_eating_speed(piles, h):\n    def hours(k):\n        return sum((p + k - 1) // k for p in piles)\n    lo, hi = 1, max(piles)\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if hours(mid) <= h: hi = mid\n        else: lo = mid + 1\n    return lo\nprint(min_eating_speed([3, 6, 7, 11], 8))\n",
    next: Some("py-288-split-array"), show_type_chips: false, micro_step: 287,
};

pub const PY288_SPLIT_ARRAY: CodingStep = CodingStep {
    id: "py-288-split-array", title: "DSA Split Array Largest", objective: "Minimizar la suma máxima al partir el array en m subarrays contiguos.",
    prompt_md: "**Split Array Largest Sum**\n\nBinary search sobre el bound de suma. Distinto de py-189 (días de barco).\n\n**Micro-reto:**\n1. Definí `split_array(nums, m)`\n2. nums=`[7,2,5,10,8]`, m=`2`; imprimí (esperado: `18`)",
    starter_code: "# def split_array(nums, m):\n#     ...\n# print(split_array([7, 2, 5, 10, 8], 2))\n",
    pytest: "def test_split_array(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('split_array'))\n    assert ns['split_array']([7, 2, 5, 10, 8], 2) == 18\n    assert ns['split_array']([1, 2, 3, 4, 5], 2) == 9\n    assert ns['split_array']([1, 4, 4], 3) == 4\n    assert capsys.readouterr().out.strip() == '18'\n",
    hint: "def split_array(nums, m):\n    def needed(limit):\n        parts = 1; cur = 0\n        for x in nums:\n            if cur + x > limit:\n                parts += 1; cur = x\n            else: cur += x\n        return parts\n    lo, hi = max(nums), sum(nums)\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if needed(mid) <= m: hi = mid\n        else: lo = mid + 1\n    return lo\nprint(split_array([7, 2, 5, 10, 8], 2))",
    solution_example: "def split_array(nums, m):\n    def needed(limit):\n        parts = 1; cur = 0\n        for x in nums:\n            if cur + x > limit:\n                parts += 1; cur = x\n            else: cur += x\n        return parts\n    lo, hi = max(nums), sum(nums)\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if needed(mid) <= m: hi = mid\n        else: lo = mid + 1\n    return lo\nprint(split_array([7, 2, 5, 10, 8], 2))\n",
    next: Some("py-289-median-two"), show_type_chips: false, micro_step: 288,
};

pub const PY289_MEDIAN_TWO: CodingStep = CodingStep {
    id: "py-289-median-two", title: "DSA Median Two Arrays", objective: "Mediana de dos arrays ordenados en O(log(m+n)).",
    prompt_md: "**Median of Two Sorted Arrays**\n\nBinary search sobre el corte del array más corto. Distinto de py-272 (stream con heaps).\n\n**Micro-reto:**\n1. Definí `find_median_sorted(nums1, nums2)`\n2. `[1,3]` + `[2]`; imprimí `2.0`",
    starter_code: "# def find_median_sorted(nums1, nums2):\n#     ...\n# print(find_median_sorted([1, 3], [2]))\n",
    pytest: "def test_median_two(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('find_median_sorted'))\n    assert ns['find_median_sorted']([1, 3], [2]) == 2.0\n    assert ns['find_median_sorted']([1, 2], [3, 4]) == 2.5\n    assert ns['find_median_sorted']([], [1]) == 1.0\n    assert capsys.readouterr().out.strip() == '2.0'\n",
    hint: "def find_median_sorted(nums1, nums2):\n    if len(nums1) > len(nums2): return find_median_sorted(nums2, nums1)\n    m, n = len(nums1), len(nums2)\n    lo, hi = 0, m\n    while lo <= hi:\n        i = (lo + hi) // 2; j = (m + n + 1) // 2 - i\n        left1 = float('-inf') if i == 0 else nums1[i-1]\n        right1 = float('inf') if i == m else nums1[i]\n        left2 = float('-inf') if j == 0 else nums2[j-1]\n        right2 = float('inf') if j == n else nums2[j]\n        if left1 <= right2 and left2 <= right1:\n            if (m + n) % 2: return float(max(left1, left2))\n            return (max(left1, left2) + min(right1, right2)) / 2.0\n        elif left1 > right2: hi = i - 1\n        else: lo = i + 1\n    return 0.0\nprint(find_median_sorted([1, 3], [2]))",
    solution_example: "def find_median_sorted(nums1, nums2):\n    if len(nums1) > len(nums2): return find_median_sorted(nums2, nums1)\n    m, n = len(nums1), len(nums2)\n    lo, hi = 0, m\n    while lo <= hi:\n        i = (lo + hi) // 2; j = (m + n + 1) // 2 - i\n        left1 = float('-inf') if i == 0 else nums1[i-1]\n        right1 = float('inf') if i == m else nums1[i]\n        left2 = float('-inf') if j == 0 else nums2[j-1]\n        right2 = float('inf') if j == n else nums2[j]\n        if left1 <= right2 and left2 <= right1:\n            if (m + n) % 2: return float(max(left1, left2))\n            return (max(left1, left2) + min(right1, right2)) / 2.0\n        elif left1 > right2: hi = i - 1\n        else: lo = i + 1\n    return 0.0\nprint(find_median_sorted([1, 3], [2]))\n",
    next: Some("py-290-search-2d-ii"), show_type_chips: false, micro_step: 289,
};

pub const PY290_SEARCH_2D_II: CodingStep = CodingStep {
    id: "py-290-search-2d-ii", title: "DSA Search 2D II", objective: "Buscar un target en matriz ordenada por filas y columnas.",
    prompt_md: "**Search a 2D Matrix II**\n\nPartí desde la esquina top-right (o bottom-left). Distinto de py-140 (rotate matrix).\n\n**Micro-reto:**\n1. Definí `search_matrix(matrix, target)`\n2. Matriz clásica, target=`5`; imprimí `True`",
    starter_code: "# def search_matrix(matrix, target):\n#     ...\n# matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]]\n# print(search_matrix(matrix, 5))\n",
    pytest: "def test_search_2d_ii(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('search_matrix'))\n    matrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]]\n    assert ns['search_matrix'](matrix, 5) is True\n    assert ns['search_matrix'](matrix, 20) is False\n    assert ns['search_matrix']([[-1, 3]], 3) is True\n    assert capsys.readouterr().out.strip() == 'True'\n",
    hint: "def search_matrix(matrix, target):\n    if not matrix or not matrix[0]: return False\n    r, c = 0, len(matrix[0]) - 1\n    while r < len(matrix) and c >= 0:\n        if matrix[r][c] == target: return True\n        if matrix[r][c] > target: c -= 1\n        else: r += 1\n    return False\nmatrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]]\nprint(search_matrix(matrix, 5))",
    solution_example: "def search_matrix(matrix, target):\n    if not matrix or not matrix[0]: return False\n    r, c = 0, len(matrix[0]) - 1\n    while r < len(matrix) and c >= 0:\n        if matrix[r][c] == target: return True\n        if matrix[r][c] > target: c -= 1\n        else: r += 1\n    return False\nmatrix = [[1,4,7,11,15],[2,5,8,12,19],[3,6,9,16,22],[10,13,14,17,24],[18,21,23,26,30]]\nprint(search_matrix(matrix, 5))\n",
    next: Some("py-291-find-duplicate"), show_type_chips: false, micro_step: 290,
};

pub const PY291_FIND_DUPLICATE: CodingStep = CodingStep {
    id: "py-291-find-duplicate", title: "DSA Find Duplicate", objective: "Encontrar el entero duplicado en [1..n] sin modificar el array (Floyd).",
    prompt_md: "**Find the Duplicate Number**\n\nTratá índices como lista enlazada; ciclo → entrada. Distinto de py-155 (contains duplicate bool).\n\n**Micro-reto:**\n1. Definí `find_duplicate(nums)`\n2. `[1,3,4,2,2]`; imprimí (esperado: `2`)",
    starter_code: "# def find_duplicate(nums):\n#     ...\n# print(find_duplicate([1, 3, 4, 2, 2]))\n",
    pytest: "def test_find_duplicate(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('find_duplicate'))\n    assert ns['find_duplicate']([1, 3, 4, 2, 2]) == 2\n    assert ns['find_duplicate']([3, 1, 3, 4, 2]) == 3\n    assert ns['find_duplicate']([1, 1]) == 1\n    assert capsys.readouterr().out.strip() == '2'\n",
    hint: "def find_duplicate(nums):\n    slow = fast = nums[0]\n    while True:\n        slow = nums[slow]; fast = nums[nums[fast]]\n        if slow == fast: break\n    slow = nums[0]\n    while slow != fast:\n        slow = nums[slow]; fast = nums[fast]\n    return slow\nprint(find_duplicate([1, 3, 4, 2, 2]))",
    solution_example: "def find_duplicate(nums):\n    slow = fast = nums[0]\n    while True:\n        slow = nums[slow]; fast = nums[nums[fast]]\n        if slow == fast: break\n    slow = nums[0]\n    while slow != fast:\n        slow = nums[slow]; fast = nums[fast]\n    return slow\nprint(find_duplicate([1, 3, 4, 2, 2]))\n",
    next: Some("py-292-first-bad"), show_type_chips: false, micro_step: 291,
};

pub const PY292_FIRST_BAD: CodingStep = CodingStep {
    id: "py-292-first-bad", title: "DSA First Bad Version", objective: "Primera versión mala con el mínimo de llamadas a isBadVersion.",
    prompt_md: "**First Bad Version**\n\nBinary search clásico sobre versiones 1..n. Distinto de py-188 (sqrt).\n\n**Micro-reto:**\n1. Definí `first_bad_version(n, is_bad)` donde `is_bad(v)` es callable\n2. n=`5`, mala desde `4`; imprimí (esperado: `4`)",
    starter_code: "# def first_bad_version(n, is_bad):\n#     ...\n# print(first_bad_version(5, lambda v: v >= 4))\n",
    pytest: "def test_first_bad(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('first_bad_version'))\n    assert ns['first_bad_version'](5, lambda v: v >= 4) == 4\n    assert ns['first_bad_version'](1, lambda v: v >= 1) == 1\n    assert ns['first_bad_version'](3, lambda v: v >= 2) == 2\n    assert capsys.readouterr().out.strip() == '4'\n",
    hint: "def first_bad_version(n, is_bad):\n    lo, hi = 1, n\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if is_bad(mid): hi = mid\n        else: lo = mid + 1\n    return lo\nprint(first_bad_version(5, lambda v: v >= 4))",
    solution_example: "def first_bad_version(n, is_bad):\n    lo, hi = 1, n\n    while lo < hi:\n        mid = (lo + hi) // 2\n        if is_bad(mid): hi = mid\n        else: lo = mid + 1\n    return lo\nprint(first_bad_version(5, lambda v: v >= 4))\n",
    next: Some("py-293-fruit-baskets"), show_type_chips: false, micro_step: 292,
};

pub const PY293_FRUIT_BASKETS: CodingStep = CodingStep {
    id: "py-293-fruit-baskets", title: "DSA Fruit Baskets", objective: "Máxima ventana con a lo sumo 2 tipos de fruta.",
    prompt_md: "**Fruit Into Baskets**\n\nSliding window + mapa de tipos. Distinto de py-216 (k reemplazos).\n\n**Micro-reto:**\n1. Definí `total_fruit(fruits)`\n2. `[1,2,1]`; imprimí (esperado: `3`)",
    starter_code: "# def total_fruit(fruits):\n#     ...\n# print(total_fruit([1, 2, 1]))\n",
    pytest: "def test_fruit_baskets(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('total_fruit'))\n    assert ns['total_fruit']([1, 2, 1]) == 3\n    assert ns['total_fruit']([0, 1, 2, 2]) == 3\n    assert ns['total_fruit']([1, 2, 3, 2, 2]) == 4\n    assert capsys.readouterr().out.strip() == '3'\n",
    hint: "from collections import defaultdict\n\ndef total_fruit(fruits):\n    count = defaultdict(int); left = best = 0\n    for right, f in enumerate(fruits):\n        count[f] += 1\n        while len(count) > 2:\n            count[fruits[left]] -= 1\n            if count[fruits[left]] == 0: del count[fruits[left]]\n            left += 1\n        best = max(best, right - left + 1)\n    return best\nprint(total_fruit([1, 2, 1]))",
    solution_example: "from collections import defaultdict\n\ndef total_fruit(fruits):\n    count = defaultdict(int); left = best = 0\n    for right, f in enumerate(fruits):\n        count[f] += 1\n        while len(count) > 2:\n            count[fruits[left]] -= 1\n            if count[fruits[left]] == 0: del count[fruits[left]]\n            left += 1\n        best = max(best, right - left + 1)\n    return best\nprint(total_fruit([1, 2, 1]))\n",
    next: Some("py-294-product-less-k"), show_type_chips: false, micro_step: 293,
};

pub const PY294_PRODUCT_LESS_K: CodingStep = CodingStep {
    id: "py-294-product-less-k", title: "DSA Product Less Than K", objective: "Contar subarrays contiguos cuyo producto es estrictamente < k.",
    prompt_md: "**Subarray Product Less Than K**\n\nVentana con producto acumulado. Distinto de py-132 (suma máxima fija).\n\n**Micro-reto:**\n1. Definí `num_subarray_product_less_than_k(nums, k)`\n2. nums=`[10,5,2,6]`, k=`100`; imprimí (esperado: `8`)",
    starter_code: "# def num_subarray_product_less_than_k(nums, k):\n#     ...\n# print(num_subarray_product_less_than_k([10, 5, 2, 6], 100))\n",
    pytest: "def test_product_less_k(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('num_subarray_product_less_than_k'))\n    assert ns['num_subarray_product_less_than_k']([10, 5, 2, 6], 100) == 8\n    assert ns['num_subarray_product_less_than_k']([1, 2, 3], 0) == 0\n    assert ns['num_subarray_product_less_than_k']([1, 1, 1], 1) == 0\n    assert capsys.readouterr().out.strip() == '8'\n",
    hint: "def num_subarray_product_less_than_k(nums, k):\n    if k <= 1: return 0\n    prod = 1; left = ans = 0\n    for right, x in enumerate(nums):\n        prod *= x\n        while prod >= k:\n            prod //= nums[left]; left += 1\n        ans += right - left + 1\n    return ans\nprint(num_subarray_product_less_than_k([10, 5, 2, 6], 100))",
    solution_example: "def num_subarray_product_less_than_k(nums, k):\n    if k <= 1: return 0\n    prod = 1; left = ans = 0\n    for right, x in enumerate(nums):\n        prod *= x\n        while prod >= k:\n            prod //= nums[left]; left += 1\n        ans += right - left + 1\n    return ans\nprint(num_subarray_product_less_than_k([10, 5, 2, 6], 100))\n",
    next: Some("py-295-ones-iii"), show_type_chips: false, micro_step: 294,
};

pub const PY295_ONES_III: CodingStep = CodingStep {
    id: "py-295-ones-iii", title: "DSA Max Consecutive Ones III", objective: "Máxima racha de 1s permitiendo voltear hasta k ceros.",
    prompt_md: "**Max Consecutive Ones III**\n\nVentana donde `zeros <= k`. Distinto de py-216 (letras con reemplazo).\n\n**Micro-reto:**\n1. Definí `longest_ones(nums, k)`\n2. nums=`[1,1,1,0,0,0,1,1,1,1,0]`, k=`2`; imprimí (esperado: `6`)",
    starter_code: "# def longest_ones(nums, k):\n#     ...\n# print(longest_ones([1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2))\n",
    pytest: "def test_ones_iii(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('longest_ones'))\n    assert ns['longest_ones']([1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2) == 6\n    assert ns['longest_ones']([0, 0, 1, 1, 0, 0, 1, 1, 1, 0, 1, 1, 0, 0, 0, 1, 1, 1, 1], 3) == 10\n    assert ns['longest_ones']([1, 1, 1], 0) == 3\n    assert capsys.readouterr().out.strip() == '6'\n",
    hint: "def longest_ones(nums, k):\n    left = zeros = best = 0\n    for right, x in enumerate(nums):\n        if x == 0: zeros += 1\n        while zeros > k:\n            if nums[left] == 0: zeros -= 1\n            left += 1\n        best = max(best, right - left + 1)\n    return best\nprint(longest_ones([1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2))",
    solution_example: "def longest_ones(nums, k):\n    left = zeros = best = 0\n    for right, x in enumerate(nums):\n        if x == 0: zeros += 1\n        while zeros > k:\n            if nums[left] == 0: zeros -= 1\n            left += 1\n        best = max(best, right - left + 1)\n    return best\nprint(longest_ones([1, 1, 1, 0, 0, 0, 1, 1, 1, 1, 0], 2))\n",
    next: Some("py-296-k-distinct"), show_type_chips: false, micro_step: 295,
};

pub const PY296_K_DISTINCT: CodingStep = CodingStep {
    id: "py-296-k-distinct", title: "DSA Longest K Distinct", objective: "Longitud de la substring más larga con exactamente k caracteres distintos.",
    prompt_md: "**Longest Substring with At Most K Distinct**\n\nUsamos *at most* k (variante clásica de entrevista). Distinto de py-293 (fijo k=2).\n\n**Micro-reto:**\n1. Definí `length_of_longest_substring_k_distinct(s, k)`\n2. s=`\"eceba\"`, k=`2`; imprimí (esperado: `3`)",
    starter_code: "# def length_of_longest_substring_k_distinct(s, k):\n#     ...\n# print(length_of_longest_substring_k_distinct(\"eceba\", 2))\n",
    pytest: "def test_k_distinct(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('length_of_longest_substring_k_distinct'))\n    assert ns['length_of_longest_substring_k_distinct']('eceba', 2) == 3\n    assert ns['length_of_longest_substring_k_distinct']('aa', 1) == 2\n    assert ns['length_of_longest_substring_k_distinct']('a', 0) == 0\n    assert capsys.readouterr().out.strip() == '3'\n",
    hint: "from collections import defaultdict\n\ndef length_of_longest_substring_k_distinct(s, k):\n    if k == 0: return 0\n    count = defaultdict(int); left = best = 0\n    for right, ch in enumerate(s):\n        count[ch] += 1\n        while len(count) > k:\n            count[s[left]] -= 1\n            if count[s[left]] == 0: del count[s[left]]\n            left += 1\n        best = max(best, right - left + 1)\n    return best\nprint(length_of_longest_substring_k_distinct('eceba', 2))",
    solution_example: "from collections import defaultdict\n\ndef length_of_longest_substring_k_distinct(s, k):\n    if k == 0: return 0\n    count = defaultdict(int); left = best = 0\n    for right, ch in enumerate(s):\n        count[ch] += 1\n        while len(count) > k:\n            count[s[left]] -= 1\n            if count[s[left]] == 0: del count[s[left]]\n            left += 1\n        best = max(best, right - left + 1)\n    return best\nprint(length_of_longest_substring_k_distinct('eceba', 2))\n",
    next: Some("py-297-check-inclusion"), show_type_chips: false, micro_step: 296,
};

pub const PY297_CHECK_INCLUSION: CodingStep = CodingStep {
    id: "py-297-check-inclusion", title: "DSA Check Inclusion", objective: "Decidir si s2 contiene alguna permutación de s1.",
    prompt_md: "**Permutation in String**\n\nVentana fija del tamaño de s1 + contadores. Distinto de py-217 (listar índices de anagramas).\n\n**Micro-reto:**\n1. Definí `check_inclusion(s1, s2)`\n2. s1=`\"ab\"`, s2=`\"eidbaooo\"`; imprimí `True`",
    starter_code: "# def check_inclusion(s1, s2):\n#     ...\n# print(check_inclusion(\"ab\", \"eidbaooo\"))\n",
    pytest: "def test_check_inclusion(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('check_inclusion'))\n    assert ns['check_inclusion']('ab', 'eidbaooo') is True\n    assert ns['check_inclusion']('ab', 'eidboaoo') is False\n    assert ns['check_inclusion']('adc', 'dcda') is True\n    assert capsys.readouterr().out.strip() == 'True'\n",
    hint: "from collections import Counter\n\ndef check_inclusion(s1, s2):\n    need = Counter(s1); n = len(s1)\n    window = Counter()\n    for i, ch in enumerate(s2):\n        window[ch] += 1\n        if i >= n:\n            old = s2[i - n]; window[old] -= 1\n            if not window[old]: del window[old]\n        if window == need: return True\n    return False\nprint(check_inclusion('ab', 'eidbaooo'))",
    solution_example: "from collections import Counter\n\ndef check_inclusion(s1, s2):\n    need = Counter(s1); n = len(s1)\n    window = Counter()\n    for i, ch in enumerate(s2):\n        window[ch] += 1\n        if i >= n:\n            old = s2[i - n]; window[old] -= 1\n            if not window[old]: del window[old]\n        if window == need: return True\n    return False\nprint(check_inclusion('ab', 'eidbaooo'))\n",
    next: Some("py-298-sort-colors"), show_type_chips: false, micro_step: 297,
};

pub const PY298_SORT_COLORS: CodingStep = CodingStep {
    id: "py-298-sort-colors", title: "DSA Sort Colors", objective: "Ordenar in-place un array de 0/1/2 con tres punteros (Dutch flag).",
    prompt_md: "**Sort Colors**\n\nPunteros `lo/mid/hi`. Distinto de py-25 (sort genérico de listas).\n\n**Micro-reto:**\n1. Definí `sort_colors(nums)` in-place\n2. `[2,0,2,1,1,0]`; imprimí el array (esperado: `[0, 0, 1, 1, 2, 2]`)",
    starter_code: "# def sort_colors(nums):\n#     ...\n# nums = [2, 0, 2, 1, 1, 0]\n# sort_colors(nums)\n# print(nums)\n",
    pytest: "def test_sort_colors(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert callable(ns.get('sort_colors'))\n    a = [2, 0, 2, 1, 1, 0]; ns['sort_colors'](a); assert a == [0, 0, 1, 1, 2, 2]\n    b = [2, 0, 1]; ns['sort_colors'](b); assert b == [0, 1, 2]\n    assert capsys.readouterr().out.strip() == '[0, 0, 1, 1, 2, 2]'\n",
    hint: "def sort_colors(nums):\n    lo = mid = 0; hi = len(nums) - 1\n    while mid <= hi:\n        if nums[mid] == 0:\n            nums[lo], nums[mid] = nums[mid], nums[lo]; lo += 1; mid += 1\n        elif nums[mid] == 1: mid += 1\n        else:\n            nums[mid], nums[hi] = nums[hi], nums[mid]; hi -= 1\nnums = [2, 0, 2, 1, 1, 0]\nsort_colors(nums)\nprint(nums)",
    solution_example: "def sort_colors(nums):\n    lo = mid = 0; hi = len(nums) - 1\n    while mid <= hi:\n        if nums[mid] == 0:\n            nums[lo], nums[mid] = nums[mid], nums[lo]; lo += 1; mid += 1\n        elif nums[mid] == 1: mid += 1\n        else:\n            nums[mid], nums[hi] = nums[hi], nums[mid]; hi -= 1\nnums = [2, 0, 2, 1, 1, 0]\nsort_colors(nums)\nprint(nums)\n",
    next: None, show_type_chips: false, micro_step: 298,
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
    &PY88_LINEAR_IN,
    &PY89_LINEAR_SEARCH,
    &PY90_BUBBLE_SORT,
    &PY91_BINARY_SEARCH,
    &PY92_SELECTION_SORT,
    &PY93_INSERTION_SORT,
    &PY94_LINKED_NODE,
    &PY95_LINKED_TRAVERSE,
    &PY96_LINKED_LOWEST,
    &PY97_RECURSION,
    &PY98_FIBONACCI,
    &PY99_QUICKSORT,
    &PY100_HASH_COUNT,
    &PY101_LINKED_DELETE,
    &PY102_LINKED_INSERT,
    &PY103_MERGE_SORT,
    &PY104_COUNTING_SORT,
    &PY105_TREE_NODE,
    &PY106_TREE_PREORDER,
    &PY107_TREE_INORDER,
    &PY108_TREE_POSTORDER,
    &PY109_GRAPH_DFS,
    &PY110_GRAPH_BFS,
    &PY111_TREE_HEIGHT,
    &PY112_DIJKSTRA,
    &PY113_HEAP,
    &PY114_PRIORITY_QUEUE,
    &PY115_UNION_FIND,
    &PY116_KRUSKAL,
    &PY117_PRIM,
    &PY118_TOPO_SORT,
    &PY119_BELLMAN_FORD,
    &PY120_MEMO_FIB,
    &PY121_TAB_FIB,
    &PY122_KNAPSACK,
    &PY123_EUCLIDEAN,
    &PY124_HUFFMAN_COST,
    &PY125_GREEDY_COIN,
    &PY126_ACTIVITY_SELECT,
    &PY127_TSP_NEAREST,
    &PY128_LCS,
    &PY129_COIN_CHANGE_DP,
    &PY130_FLOYD_WARSHALL,
    &PY131_TWO_POINTERS,
    &PY132_SLIDING_WINDOW,
    &PY133_PERMUTATIONS,
    &PY134_NQUEENS_COUNT,
    &PY135_TRIE,
    &PY136_BIT_COUNT,
    &PY137_KADANE,
    &PY138_MERGE_INTERVALS,
    &PY139_LOWER_BOUND,
    &PY140_ROTATE_MATRIX,
    &PY141_VALID_PARENS,
    &PY142_ANAGRAM,
    &PY143_CLIMB_STAIRS,
    &PY144_HOUSE_ROBBER,
    &PY145_UNIQUE_PATHS,
    &PY146_MAJORITY,
    &PY147_MISSING_NUMBER,
    &PY148_SINGLE_NUMBER,
    &PY149_LIS,
    &PY150_EDIT_DISTANCE,
    &PY151_WORD_BREAK,
    &PY152_MIN_PATH_SUM,
    &PY153_DECODE_WAYS,
    &PY154_LONGEST_PALINDROME,
    &PY155_CONTAINS_DUP,
    &PY156_BEST_STOCK,
    &PY157_MOVE_ZEROES,
    &PY158_PRODUCT_EXCEPT,
    &PY159_FIRST_UNIQUE,
    &PY160_HAPPY_NUMBER,
    &PY161_REVERSE_LIST,
    &PY162_MERGE_SORTED,
    &PY163_LINKED_CYCLE,
    &PY164_VALID_PALINDROME,
    &PY165_COMMON_PREFIX,
    &PY166_ROMAN_TO_INT,
    &PY167_INVERT_TREE,
    &PY168_SAME_TREE,
    &PY169_MAX_DEPTH,
    &PY170_SPIRAL_MATRIX,
    &PY171_SET_ZEROES,
    &PY172_SUBSETS,
    &PY173_JUMP_GAME,
    &PY174_GAS_STATION,
    &PY175_CONTAINER_WATER,
    &PY176_THREE_SUM,
    &PY177_TRAPPING_RAIN,
    &PY178_GROUP_ANAGRAMS,
    &PY179_DAILY_TEMPS,
    &PY180_NEXT_GREATER,
    &PY181_EVAL_RPN,
    &PY182_QUEUE_STACKS,
    &PY183_SLIDING_MAX,
    &PY184_MIN_STACK,
    &PY185_FIRST_LAST,
    &PY186_PEAK_ELEMENT,
    &PY187_SEARCH_ROTATED,
    &PY188_SQRT,
    &PY189_SHIP_CAPACITY,
    &PY190_MIN_ROTATED,
    &PY191_KTH_LARGEST,
    &PY192_TOP_K_FREQ,
    &PY193_MERGE_K_LISTS,
    &PY194_MEETING_ROOMS,
    &PY195_UGLY_NUMBER,
    &PY196_K_CLOSEST,
    &PY197_COIN_CHANGE_II,
    &PY198_HOUSE_ROBBER_II,
    &PY199_UNIQUE_PATHS_II,
    &PY200_MAX_PRODUCT,
    &PY201_PARTITION_SUBSET,
    &PY202_PERFECT_SQUARES,
    &PY203_NUM_ISLANDS,
    &PY204_CLONE_GRAPH,
    &PY205_COURSE_SCHEDULE,
    &PY206_PACIFIC_ATLANTIC,
    &PY207_ROT_ORANGES,
    &PY208_WORD_LADDER,
    &PY209_LRU_CACHE,
    &PY210_BASIC_CALC,
    &PY211_ENCODE_DECODE,
    &PY212_RANDOMIZED_SET,
    &PY213_TIME_KV,
    &PY214_SNAPSHOT_ARRAY,
    &PY215_MIN_WINDOW,
    &PY216_CHAR_REPLACE,
    &PY217_FIND_ANAGRAMS,
    &PY218_DECODE_STRING,
    &PY219_STR_COMPRESS,
    &PY220_MULTIPLY_STRINGS,
    &PY221_INSERT_INTERVAL,
    &PY222_ERASE_OVERLAP,
    &PY223_MEETING_ROOMS_II,
    &PY224_SINGLE_NUMBER_II,
    &PY225_COUNTING_BITS,
    &PY226_REVERSE_BITS,
    &PY227_GENERATE_PARENS,
    &PY228_COMBINATION_SUM,
    &PY229_WORD_SEARCH,
    &PY230_LETTER_COMBOS,
    &PY231_SUBSETS_II,
    &PY232_PALINDROME_PARTITION,
    &PY233_REVERSE_INTEGER,
    &PY234_PALINDROME_NUMBER,
    &PY235_PLUS_ONE,
    &PY236_ADD_BINARY,
    &PY237_MY_POW,
    &PY238_TRAILING_ZEROES,
    &PY239_TREE_DIAMETER,
    &PY240_LCA,
    &PY241_PATH_SUM,
    &PY242_RIGHT_SIDE,
    &PY243_FLATTEN_TREE,
    &PY244_VALIDATE_BST,
    &PY245_REMOVE_NTH,
    &PY246_REORDER_LIST,
    &PY247_ADD_TWO_LISTS,
    &PY248_SWAP_PAIRS,
    &PY249_ROTATE_LIST,
    &PY250_PALINDROME_LIST,
    &PY251_COPY_RANDOM,
    &PY252_SORT_LIST,
    &PY253_MERGE_TWO_LISTS,
    &PY254_INTERSECTION,
    &PY255_CYCLE_START,
    &PY256_REMOVE_DUPES_II,
    &PY257_REMOVE_K_DIGITS,
    &PY258_ASTEROID_COLLISION,
    &PY259_SIMPLIFY_PATH,
    &PY260_CALC_II,
    &PY261_CAR_FLEET,
    &PY262_LARGEST_RECT,
    &PY263_OPEN_LOCK,
    &PY264_SHORTEST_BINARY,
    &PY265_WALLS_GATES,
    &PY266_CIRCULAR_QUEUE,
    &PY267_RECENT_COUNTER,
    &PY268_TIME_TICKETS,
    &PY269_LAST_STONE,
    &PY270_TASK_SCHEDULER,
    &PY271_REORGANIZE_STRING,
    &PY272_FIND_MEDIAN,
    &PY273_KTH_MATRIX,
    &PY274_NETWORK_DELAY,
    &PY275_COURSE_ORDER,
    &PY276_CHEAPEST_FLIGHTS,
    &PY277_REDUNDANT_EDGE,
    &PY278_ACCOUNTS_MERGE,
    &PY279_ALIEN_DICT,
    &PY280_MIN_COST_POINTS,
    &PY281_JUMP_GAME_II,
    &PY282_TARGET_SUM,
    &PY283_MAXIMAL_SQUARE,
    &PY284_STOCK_COOLDOWN,
    &PY285_INTERLEAVING,
    &PY286_PALINDROME_SUBSEQ,
    &PY287_KOKO_BANANAS,
    &PY288_SPLIT_ARRAY,
    &PY289_MEDIAN_TWO,
    &PY290_SEARCH_2D_II,
    &PY291_FIND_DUPLICATE,
    &PY292_FIRST_BAD,
    &PY293_FRUIT_BASKETS,
    &PY294_PRODUCT_LESS_K,
    &PY295_ONES_III,
    &PY296_K_DISTINCT,
    &PY297_CHECK_INCLUSION,
    &PY298_SORT_COLORS,
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
    format!("<code>{}</code>", wrap_explore_idents(inner, flash_ident))
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
            assert!(step.micro_step >= 1 && step.micro_step <= 300);
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
            (87, "py-87-queue-class", Some("py-88-linear-in")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("dsa family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py88_to_py93_algorithms_chain() {
        let ids = [
            (88, "py-88-linear-in", Some("py-89-linear-search")),
            (89, "py-89-linear-search", Some("py-90-bubble-sort")),
            (90, "py-90-bubble-sort", Some("py-91-binary-search")),
            (91, "py-91-binary-search", Some("py-92-selection-sort")),
            (92, "py-92-selection-sort", Some("py-93-insertion-sort")),
            (93, "py-93-insertion-sort", Some("py-94-linked-node")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("algorithms family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py94_to_py100_finale_chain() {
        let ids = [
            (94, "py-94-linked-node", Some("py-95-linked-traverse")),
            (95, "py-95-linked-traverse", Some("py-96-linked-lowest")),
            (96, "py-96-linked-lowest", Some("py-97-recursion")),
            (97, "py-97-recursion", Some("py-98-fibonacci")),
            (98, "py-98-fibonacci", Some("py-99-quicksort")),
            (99, "py-99-quicksort", Some("py-100-hash-count")),
            (100, "py-100-hash-count", Some("py-101-linked-delete")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("finale family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py101_to_py106_dsa_plus_chain() {
        let ids = [
            (101, "py-101-linked-delete", Some("py-102-linked-insert")),
            (102, "py-102-linked-insert", Some("py-103-merge-sort")),
            (103, "py-103-merge-sort", Some("py-104-counting-sort")),
            (104, "py-104-counting-sort", Some("py-105-tree-node")),
            (105, "py-105-tree-node", Some("py-106-tree-preorder")),
            (106, "py-106-tree-preorder", Some("py-107-tree-inorder")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("dsa-plus family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py107_to_py112_trees_graphs_chain() {
        let ids = [
            (107, "py-107-tree-inorder", Some("py-108-tree-postorder")),
            (108, "py-108-tree-postorder", Some("py-109-graph-dfs")),
            (109, "py-109-graph-dfs", Some("py-110-graph-bfs")),
            (110, "py-110-graph-bfs", Some("py-111-tree-height")),
            (111, "py-111-tree-height", Some("py-112-dijkstra")),
            (112, "py-112-dijkstra", Some("py-113-heap")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("trees-graphs family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py113_to_py118_heaps_graphs_chain() {
        let ids = [
            (113, "py-113-heap", Some("py-114-priority-queue")),
            (114, "py-114-priority-queue", Some("py-115-union-find")),
            (115, "py-115-union-find", Some("py-116-kruskal")),
            (116, "py-116-kruskal", Some("py-117-prim")),
            (117, "py-117-prim", Some("py-118-topo-sort")),
            (118, "py-118-topo-sort", Some("py-119-bellman-ford")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("heaps-graphs family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py119_to_py124_dp_paths_chain() {
        let ids = [
            (119, "py-119-bellman-ford", Some("py-120-memo-fib")),
            (120, "py-120-memo-fib", Some("py-121-tab-fib")),
            (121, "py-121-tab-fib", Some("py-122-knapsack")),
            (122, "py-122-knapsack", Some("py-123-euclidean")),
            (123, "py-123-euclidean", Some("py-124-huffman-cost")),
            (124, "py-124-huffman-cost", Some("py-125-greedy-coin")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("dp-paths family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py125_to_py130_greedy_dp_chain() {
        let ids = [
            (125, "py-125-greedy-coin", Some("py-126-activity-select")),
            (126, "py-126-activity-select", Some("py-127-tsp-nearest")),
            (127, "py-127-tsp-nearest", Some("py-128-lcs")),
            (128, "py-128-lcs", Some("py-129-coin-change-dp")),
            (129, "py-129-coin-change-dp", Some("py-130-floyd-warshall")),
            (130, "py-130-floyd-warshall", Some("py-131-two-pointers")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("greedy-dp family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py131_to_py136_backtrack_strings_chain() {
        let ids = [
            (131, "py-131-two-pointers", Some("py-132-sliding-window")),
            (132, "py-132-sliding-window", Some("py-133-permutations")),
            (133, "py-133-permutations", Some("py-134-nqueens-count")),
            (134, "py-134-nqueens-count", Some("py-135-trie")),
            (135, "py-135-trie", Some("py-136-bit-count")),
            (136, "py-136-bit-count", Some("py-137-kadane")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("backtrack-strings family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py137_to_py142_patterns_chain() {
        let ids = [
            (137, "py-137-kadane", Some("py-138-merge-intervals")),
            (138, "py-138-merge-intervals", Some("py-139-lower-bound")),
            (139, "py-139-lower-bound", Some("py-140-rotate-matrix")),
            (140, "py-140-rotate-matrix", Some("py-141-valid-parens")),
            (141, "py-141-valid-parens", Some("py-142-anagram")),
            (142, "py-142-anagram", Some("py-143-climb-stairs")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("patterns family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py143_to_py148_more_patterns_chain() {
        let ids = [
            (143, "py-143-climb-stairs", Some("py-144-house-robber")),
            (144, "py-144-house-robber", Some("py-145-unique-paths")),
            (145, "py-145-unique-paths", Some("py-146-majority")),
            (146, "py-146-majority", Some("py-147-missing-number")),
            (147, "py-147-missing-number", Some("py-148-single-number")),
            (148, "py-148-single-number", Some("py-149-lis")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("more-patterns family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py149_to_py154_dp_strings_chain() {
        let ids = [
            (149, "py-149-lis", Some("py-150-edit-distance")),
            (150, "py-150-edit-distance", Some("py-151-word-break")),
            (151, "py-151-word-break", Some("py-152-min-path-sum")),
            (152, "py-152-min-path-sum", Some("py-153-decode-ways")),
            (153, "py-153-decode-ways", Some("py-154-longest-palindrome")),
            (
                154,
                "py-154-longest-palindrome",
                Some("py-155-contains-dup"),
            ),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("dp-strings family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py155_to_py160_arrays_hash_chain() {
        let ids = [
            (155, "py-155-contains-dup", Some("py-156-best-stock")),
            (156, "py-156-best-stock", Some("py-157-move-zeroes")),
            (157, "py-157-move-zeroes", Some("py-158-product-except")),
            (158, "py-158-product-except", Some("py-159-first-unique")),
            (159, "py-159-first-unique", Some("py-160-happy-number")),
            (160, "py-160-happy-number", Some("py-161-reverse-list")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("arrays-hash family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py167_to_py172_trees_matrix_chain() {
        let ids = [
            (166, "py-166-roman-to-int", Some("py-167-invert-tree")),
            (167, "py-167-invert-tree", Some("py-168-same-tree")),
            (168, "py-168-same-tree", Some("py-169-max-depth")),
            (169, "py-169-max-depth", Some("py-170-spiral-matrix")),
            (170, "py-170-spiral-matrix", Some("py-171-set-zeroes")),
            (171, "py-171-set-zeroes", Some("py-172-subsets")),
            (172, "py-172-subsets", Some("py-173-jump-game")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("trees-matrix family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py173_to_py178_greedy_twopointers_chain() {
        let ids = [
            (173, "py-173-jump-game", Some("py-174-gas-station")),
            (174, "py-174-gas-station", Some("py-175-container-water")),
            (175, "py-175-container-water", Some("py-176-three-sum")),
            (176, "py-176-three-sum", Some("py-177-trapping-rain")),
            (177, "py-177-trapping-rain", Some("py-178-group-anagrams")),
            (178, "py-178-group-anagrams", Some("py-179-daily-temps")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("greedy/two-pointers family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }


    #[test]
    fn py179_to_py184_stacks_chain() {
        let ids = [
            (179, "py-179-daily-temps", Some("py-180-next-greater")),
            (180, "py-180-next-greater", Some("py-181-eval-rpn")),
            (181, "py-181-eval-rpn", Some("py-182-queue-stacks")),
            (182, "py-182-queue-stacks", Some("py-183-sliding-max")),
            (183, "py-183-sliding-max", Some("py-184-min-stack")),
            (184, "py-184-min-stack", Some("py-185-first-last")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("stacks family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py185_to_py190_binsearch_chain() {
        let ids = [
            (185, "py-185-first-last", Some("py-186-peak-element")),
            (186, "py-186-peak-element", Some("py-187-search-rotated")),
            (187, "py-187-search-rotated", Some("py-188-sqrt")),
            (188, "py-188-sqrt", Some("py-189-ship-capacity")),
            (189, "py-189-ship-capacity", Some("py-190-min-rotated")),
            (190, "py-190-min-rotated", Some("py-191-kth-largest")),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("binsearch family step");
            assert_eq!(step.id, id);
            assert_eq!(step.next, next);
        }
    }

    #[test]
    fn py203_to_py298_curriculum_chain() {
        let ids = [
            (202, "py-202-perfect-squares", Some("py-203-num-islands")),
            (203, "py-203-num-islands", Some("py-204-clone-graph")),
            (204, "py-204-clone-graph", Some("py-205-course-schedule")),
            (205, "py-205-course-schedule", Some("py-206-pacific-atlantic")),
            (206, "py-206-pacific-atlantic", Some("py-207-rot-oranges")),
            (207, "py-207-rot-oranges", Some("py-208-word-ladder")),
            (208, "py-208-word-ladder", Some("py-209-lru-cache")),
            (209, "py-209-lru-cache", Some("py-210-basic-calc")),
            (210, "py-210-basic-calc", Some("py-211-encode-decode")),
            (211, "py-211-encode-decode", Some("py-212-randomized-set")),
            (212, "py-212-randomized-set", Some("py-213-time-kv")),
            (213, "py-213-time-kv", Some("py-214-snapshot-array")),
            (214, "py-214-snapshot-array", Some("py-215-min-window")),
            (215, "py-215-min-window", Some("py-216-char-replace")),
            (216, "py-216-char-replace", Some("py-217-find-anagrams")),
            (217, "py-217-find-anagrams", Some("py-218-decode-string")),
            (218, "py-218-decode-string", Some("py-219-str-compress")),
            (219, "py-219-str-compress", Some("py-220-multiply-strings")),
            (220, "py-220-multiply-strings", Some("py-221-insert-interval")),
            (221, "py-221-insert-interval", Some("py-222-erase-overlap")),
            (222, "py-222-erase-overlap", Some("py-223-meeting-rooms-ii")),
            (223, "py-223-meeting-rooms-ii", Some("py-224-single-number-ii")),
            (224, "py-224-single-number-ii", Some("py-225-counting-bits")),
            (225, "py-225-counting-bits", Some("py-226-reverse-bits")),
            (226, "py-226-reverse-bits", Some("py-227-generate-parens")),
            (227, "py-227-generate-parens", Some("py-228-combination-sum")),
            (228, "py-228-combination-sum", Some("py-229-word-search")),
            (229, "py-229-word-search", Some("py-230-letter-combos")),
            (230, "py-230-letter-combos", Some("py-231-subsets-ii")),
            (231, "py-231-subsets-ii", Some("py-232-palindrome-partition")),
            (232, "py-232-palindrome-partition", Some("py-233-reverse-integer")),
            (233, "py-233-reverse-integer", Some("py-234-palindrome-number")),
            (234, "py-234-palindrome-number", Some("py-235-plus-one")),
            (235, "py-235-plus-one", Some("py-236-add-binary")),
            (236, "py-236-add-binary", Some("py-237-my-pow")),
            (237, "py-237-my-pow", Some("py-238-trailing-zeroes")),
            (238, "py-238-trailing-zeroes", Some("py-239-tree-diameter")),
            (239, "py-239-tree-diameter", Some("py-240-lca")),
            (240, "py-240-lca", Some("py-241-path-sum")),
            (241, "py-241-path-sum", Some("py-242-right-side")),
            (242, "py-242-right-side", Some("py-243-flatten-tree")),
            (243, "py-243-flatten-tree", Some("py-244-validate-bst")),
            (244, "py-244-validate-bst", Some("py-245-remove-nth")),
            (245, "py-245-remove-nth", Some("py-246-reorder-list")),
            (246, "py-246-reorder-list", Some("py-247-add-two-lists")),
            (247, "py-247-add-two-lists", Some("py-248-swap-pairs")),
            (248, "py-248-swap-pairs", Some("py-249-rotate-list")),
            (249, "py-249-rotate-list", Some("py-250-palindrome-list")),
            (250, "py-250-palindrome-list", Some("py-251-copy-random")),
            (251, "py-251-copy-random", Some("py-252-sort-list")),
            (252, "py-252-sort-list", Some("py-253-merge-two-lists")),
            (253, "py-253-merge-two-lists", Some("py-254-intersection")),
            (254, "py-254-intersection", Some("py-255-cycle-start")),
            (255, "py-255-cycle-start", Some("py-256-remove-dupes-ii")),
            (256, "py-256-remove-dupes-ii", Some("py-257-remove-k-digits")),
            (257, "py-257-remove-k-digits", Some("py-258-asteroid-collision")),
            (258, "py-258-asteroid-collision", Some("py-259-simplify-path")),
            (259, "py-259-simplify-path", Some("py-260-calc-ii")),
            (260, "py-260-calc-ii", Some("py-261-car-fleet")),
            (261, "py-261-car-fleet", Some("py-262-largest-rect")),
            (262, "py-262-largest-rect", Some("py-263-open-lock")),
            (263, "py-263-open-lock", Some("py-264-shortest-binary")),
            (264, "py-264-shortest-binary", Some("py-265-walls-gates")),
            (265, "py-265-walls-gates", Some("py-266-circular-queue")),
            (266, "py-266-circular-queue", Some("py-267-recent-counter")),
            (267, "py-267-recent-counter", Some("py-268-time-tickets")),
            (268, "py-268-time-tickets", Some("py-269-last-stone")),
            (269, "py-269-last-stone", Some("py-270-task-scheduler")),
            (270, "py-270-task-scheduler", Some("py-271-reorganize-string")),
            (271, "py-271-reorganize-string", Some("py-272-find-median")),
            (272, "py-272-find-median", Some("py-273-kth-matrix")),
            (273, "py-273-kth-matrix", Some("py-274-network-delay")),
            (274, "py-274-network-delay", Some("py-275-course-order")),
            (275, "py-275-course-order", Some("py-276-cheapest-flights")),
            (276, "py-276-cheapest-flights", Some("py-277-redundant-edge")),
            (277, "py-277-redundant-edge", Some("py-278-accounts-merge")),
            (278, "py-278-accounts-merge", Some("py-279-alien-dict")),
            (279, "py-279-alien-dict", Some("py-280-min-cost-points")),
            (280, "py-280-min-cost-points", Some("py-281-jump-game-ii")),
            (281, "py-281-jump-game-ii", Some("py-282-target-sum")),
            (282, "py-282-target-sum", Some("py-283-maximal-square")),
            (283, "py-283-maximal-square", Some("py-284-stock-cooldown")),
            (284, "py-284-stock-cooldown", Some("py-285-interleaving")),
            (285, "py-285-interleaving", Some("py-286-palindrome-subseq")),
            (286, "py-286-palindrome-subseq", Some("py-287-koko-bananas")),
            (287, "py-287-koko-bananas", Some("py-288-split-array")),
            (288, "py-288-split-array", Some("py-289-median-two")),
            (289, "py-289-median-two", Some("py-290-search-2d-ii")),
            (290, "py-290-search-2d-ii", Some("py-291-find-duplicate")),
            (291, "py-291-find-duplicate", Some("py-292-first-bad")),
            (292, "py-292-first-bad", Some("py-293-fruit-baskets")),
            (293, "py-293-fruit-baskets", Some("py-294-product-less-k")),
            (294, "py-294-product-less-k", Some("py-295-ones-iii")),
            (295, "py-295-ones-iii", Some("py-296-k-distinct")),
            (296, "py-296-k-distinct", Some("py-297-check-inclusion")),
            (297, "py-297-check-inclusion", Some("py-298-sort-colors")),
            (298, "py-298-sort-colors", None),
        ];
        for (n, id, next) in ids {
            let step = coding_step_by_micro_step(n).expect("curriculum family step");
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
                    .matches(r#"class="learn__ident learn__ident--flash" id="learn-ident-nombre""#)
                    .count(),
            2
        );
        assert!(html.contains(r#"class="learn__ident" id="learn-ident-edad""#));
        assert!(html.contains(r#"class="learn__ident" data-ident="edad""#));
    }
}
