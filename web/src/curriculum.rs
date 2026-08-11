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
    next: None,
    show_type_chips: false,
    micro_step: 7,
};

pub const CODING_STEPS: &[&CodingStep] = &[
    &PY02_VARIABLES,
    &PY02_INTRO,
    &PY03_GET_STARTED,
    &PY04_SYNTAX,
    &PY05_OUTPUT,
    &PY06_COMMENTS,
    &PY07_DATA_TYPES,
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
        assert!(step.next.is_none());
        assert!(step.pytest.contains("test_data_types"));
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
