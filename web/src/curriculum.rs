//! First coding micro-steps for Paso 2 (embedded from foundations seed).
//!
//! Source of truth for copy: `docs/seeds/python-foundations-microsteps-v0.2.json`
//! (`py-02-variables`). Keep in sync until a JSON loader lands.

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
}

/// First coding bridge after onboarding (seed step `py-02-variables`).
pub const PY02_VARIABLES: CodingStep = CodingStep {
    id: "py-02-variables",
    title: "Variables (puente coding)",
    objective: "Primer micro-ejercicio de código tras el onboarding.",
    prompt_md: "**Variables**\n\nUna variable guarda un valor. En Python se crea al asignar.\n\n**Micro-reto:**\n1. Crea `nombre` con un texto\n2. Crea `edad` con un entero\n3. Imprime ambas con `print(nombre, edad)`",
    starter_code: "# nombre = ...\n# edad = ...\n# print(...)\n",
    pytest: "def test_variables(capsys):\n    ns = {}\n    exec(open('solution.py', encoding='utf-8').read(), ns)\n    assert 'nombre' in ns and isinstance(ns['nombre'], str)\n    assert 'edad' in ns and isinstance(ns['edad'], int)\n    out = ' '.join(capsys.readouterr().out.split())\n    assert str(ns['nombre']) in out and str(ns['edad']) in out\n",
    hint: "nombre = \"Ana\"\nedad = 25\nprint(nombre, edad)",
    solution_example: "nombre = \"Ana\"\nedad = 25\nprint(nombre, edad)",
};

pub fn first_coding_step() -> &'static CodingStep {
    &PY02_VARIABLES
}

/// Extremely light markdown → HTML for the enunciado panel (bold + newlines).
pub fn prompt_to_html(md: &str) -> String {
    let mut out = String::new();
    for line in md.lines() {
        let mut escaped = html_escape(line);
        // **bold**
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
        // `code`
        while let Some(start) = escaped.find('`') {
            if let Some(rel_end) = escaped[start + 1..].find('`') {
                let end = start + 1 + rel_end;
                let inner = escaped[start + 1..end].to_string();
                let replacement = format!("<code>{inner}</code>");
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
    }

    #[test]
    fn prompt_html_bold_and_code() {
        let html = prompt_to_html("**Variables**\nUsa `print`");
        assert!(html.contains("<strong>Variables</strong>"));
        assert!(html.contains("<code>print</code>"));
        assert!(!html.contains("<script"));
    }
}
