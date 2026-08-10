//! Browser interop for `window.ppiPyodide` (Trunk-served JS glue).

use js_sys::{Function, Promise, Reflect, JSON};
use serde::Deserialize;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct EngineState {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct RunResult {
    pub ok: bool,
    #[serde(default)]
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
    #[serde(default)]
    pub error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct CheckResult {
    pub passed: bool,
    #[serde(default)]
    pub stdout: String,
    #[serde(default)]
    pub stderr: String,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub details: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PyodideError {
    pub message: String,
}

impl PyodideError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

fn window() -> Result<web_sys::Window, PyodideError> {
    web_sys::window().ok_or_else(|| PyodideError::new("window no disponible"))
}

fn api_object() -> Result<JsValue, PyodideError> {
    let win = window()?;
    let api = Reflect::get(&win, &JsValue::from_str("ppiPyodide")).map_err(|_| {
        PyodideError::new(
            "ppiPyodide no está cargado. Revisá que Trunk sirva /ppi-pyodide.js.",
        )
    })?;
    if api.is_undefined() || api.is_null() {
        return Err(PyodideError::new(
            "ppiPyodide no está cargado. Revisá que Trunk sirva /ppi-pyodide.js.",
        ));
    }
    Ok(api)
}

fn api_fn(name: &str) -> Result<(JsValue, Function), PyodideError> {
    let api = api_object()?;
    let value = Reflect::get(&api, &JsValue::from_str(name))
        .map_err(|_| PyodideError::new(format!("ppiPyodide.{name} ausente")))?;
    let func = value
        .dyn_into::<Function>()
        .map_err(|_| PyodideError::new(format!("ppiPyodide.{name} no es función")))?;
    Ok((api, func))
}

async fn call0_json<T: for<'de> Deserialize<'de>>(name: &str) -> Result<T, PyodideError> {
    let (api, func) = api_fn(name)?;
    let ret = func
        .call0(&api)
        .map_err(|_| PyodideError::new(format!("falló ppiPyodide.{name}")))?;
    await_json(ret).await
}

async fn call1_json<T: for<'de> Deserialize<'de>>(
    name: &str,
    arg: &str,
) -> Result<T, PyodideError> {
    let (api, func) = api_fn(name)?;
    let ret = func
        .call1(&api, &JsValue::from_str(arg))
        .map_err(|_| PyodideError::new(format!("falló ppiPyodide.{name}")))?;
    await_json(ret).await
}

async fn call2_json<T: for<'de> Deserialize<'de>>(
    name: &str,
    arg1: &str,
    arg2: &str,
) -> Result<T, PyodideError> {
    let (api, func) = api_fn(name)?;
    let ret = func
        .call2(&api, &JsValue::from_str(arg1), &JsValue::from_str(arg2))
        .map_err(|_| PyodideError::new(format!("falló ppiPyodide.{name}")))?;
    await_json(ret).await
}

async fn await_json<T: for<'de> Deserialize<'de>>(ret: JsValue) -> Result<T, PyodideError> {
    let resolved = match ret.dyn_into::<Promise>() {
        Ok(promise) => JsFuture::from(promise)
            .await
            .map_err(|err| PyodideError::new(js_error_message(&err)))?,
        Err(value) => value,
    };
    let json = JSON::stringify(&resolved)
        .map_err(|_| PyodideError::new("no se pudo serializar la respuesta de Pyodide"))?
        .as_string()
        .ok_or_else(|| PyodideError::new("JSON.stringify devolvió valor no-string"))?;
    serde_json::from_str(&json)
        .map_err(|err| PyodideError::new(format!("JSON Pyodide inválido: {err}")))
}

fn js_error_message(err: &JsValue) -> String {
    if let Some(s) = err.as_string() {
        return s;
    }
    JSON::stringify(err)
        .ok()
        .and_then(|v| v.as_string())
        .unwrap_or_else(|| "error desconocido en ppiPyodide".into())
}

/// Lazy-load Pyodide via the JS glue.
pub async fn ensure_engine() -> Result<EngineState, PyodideError> {
    call0_json("ensure").await
}

pub async fn run_student_code(code: String) -> Result<RunResult, PyodideError> {
    call1_json("run", &code).await
}

pub async fn check_student_code(
    code: String,
    test_source: String,
) -> Result<CheckResult, PyodideError> {
    call2_json("check", &code, &test_source).await
}

/// Formats a full Run transcript (parity with the JS `formatRunLog` helper).
#[allow(dead_code)] // Kept for Qwik-parity logging / future structured export.
pub fn format_run_log(result: &RunResult) -> String {
    let mut parts = vec!["=== Run ===".to_string()];
    let stdout = result.stdout.trim_end_matches('\n');
    if !stdout.trim().is_empty() {
        parts.push(stdout.to_string());
    } else if result.ok {
        parts.push("(sin salida — usá print(...) para ver texto aquí)".into());
    }
    let stderr = result.stderr.trim_end_matches('\n');
    if !stderr.trim().is_empty() {
        parts.push("--- stderr ---".into());
        parts.push(stderr.to_string());
    }
    if !result.ok {
        if let Some(error) = &result.error {
            parts.push("--- error ---".into());
            parts.push(error.clone());
        }
    }
    if result.ok {
        parts.push(String::new());
        parts.push("✓ Ejecución finalizada".into());
    }
    parts.join("\n")
}

pub fn format_check_log(result: &CheckResult) -> String {
    let mut parts = vec![
        "=== Validar ===".into(),
        result.summary.clone(),
        String::new(),
    ];
    let details = result.details.trim_end_matches('\n');
    if !details.trim().is_empty() {
        parts.push(details.to_string());
    }
    parts.join("\n")
}

/// Pure stdout body for the console pane (no Run banners).
pub fn run_stdout_body(result: &RunResult) -> String {
    result.stdout.trim_end_matches('\n').to_string()
}

/// stderr + error message for the error pane.
pub fn run_stderr_body(result: &RunResult) -> String {
    let mut parts = Vec::new();
    let stderr = result.stderr.trim_end_matches('\n');
    if !stderr.trim().is_empty() {
        parts.push(stderr.to_string());
    }
    if !result.ok {
        if let Some(error) = &result.error {
            let err = error.trim();
            if !err.is_empty() && !parts.iter().any(|p| p.contains(err)) {
                parts.push(error.clone());
            }
        }
    }
    parts.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_run_log_ok_empty_stdout() {
        let log = format_run_log(&RunResult {
            ok: true,
            stdout: String::new(),
            stderr: String::new(),
            error: None,
        });
        assert!(log.contains("sin salida"));
        assert!(log.contains("✓ Ejecución finalizada"));
    }

    #[test]
    fn format_check_log_includes_summary() {
        let log = format_check_log(&CheckResult {
            passed: true,
            stdout: String::new(),
            stderr: String::new(),
            summary: "✓ Checks OK".into(),
            details: "OK".into(),
        });
        assert!(log.contains("✓ Checks OK"));
        assert!(log.contains("OK"));
    }

    #[test]
    fn run_bodies_split_stdout_and_stderr() {
        let ok = RunResult {
            ok: true,
            stdout: "Hola IngenierIA\n".into(),
            stderr: String::new(),
            error: None,
        };
        assert_eq!(run_stdout_body(&ok), "Hola IngenierIA");
        assert!(run_stderr_body(&ok).is_empty());

        let bad = RunResult {
            ok: false,
            stdout: String::new(),
            stderr: "Traceback".into(),
            error: Some("NameError: x".into()),
        };
        assert!(run_stderr_body(&bad).contains("Traceback"));
        assert!(run_stderr_body(&bad).contains("NameError"));
    }
}
