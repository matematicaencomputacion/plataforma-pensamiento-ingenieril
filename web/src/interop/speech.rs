//! Browser interop for `window.ppiSpeech` (Web Speech dictation).

use js_sys::{Function, Object, Reflect};
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpeechError {
    pub message: String,
}

fn window() -> Result<web_sys::Window, SpeechError> {
    web_sys::window().ok_or_else(|| SpeechError {
        message: "window no disponible".into(),
    })
}

fn api_object() -> Result<JsValue, SpeechError> {
    let win = window()?;
    let api = Reflect::get(&win, &JsValue::from_str("ppiSpeech")).map_err(|_| SpeechError {
        message: "ppiSpeech no está cargado. Revisá que Trunk sirva /ppi-speech.js.".into(),
    })?;
    if api.is_undefined() || api.is_null() {
        return Err(SpeechError {
            message: "ppiSpeech no está cargado. Revisá que Trunk sirva /ppi-speech.js.".into(),
        });
    }
    Ok(api)
}

pub fn is_supported() -> bool {
    let Ok(api) = api_object() else {
        return false;
    };
    let Ok(value) = Reflect::get(&api, &JsValue::from_str("isSupported")) else {
        return false;
    };
    let Ok(func) = value.dyn_into::<Function>() else {
        return false;
    };
    func.call0(&api)
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

#[allow(dead_code)] // Exposed for future listening badge parity with JS.
pub fn is_listening() -> bool {
    let Ok(api) = api_object() else {
        return false;
    };
    let Ok(value) = Reflect::get(&api, &JsValue::from_str("isListening")) else {
        return false;
    };
    let Ok(func) = value.dyn_into::<Function>() else {
        return false;
    };
    func.call0(&api)
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(false)
}

pub fn stop() {
    let Ok(api) = api_object() else {
        return;
    };
    let Ok(value) = Reflect::get(&api, &JsValue::from_str("stop")) else {
        return;
    };
    if let Ok(func) = value.dyn_into::<Function>() {
        let _ = func.call0(&api);
    }
}

/// Start continuous dictation. Callbacks must outlive the recognition session;
/// callers typically leak `Closure`s for the page lifetime of a listen session.
pub fn start(
    base: &str,
    lang: &str,
    on_update: &Closure<dyn FnMut(String)>,
    on_error: &Closure<dyn FnMut(String)>,
    on_end: &Closure<dyn FnMut()>,
) -> Result<(), SpeechError> {
    let api = api_object()?;
    let start_fn = Reflect::get(&api, &JsValue::from_str("start"))
        .map_err(|_| SpeechError {
            message: "ppiSpeech.start ausente".into(),
        })?
        .dyn_into::<Function>()
        .map_err(|_| SpeechError {
            message: "ppiSpeech.start no es función".into(),
        })?;

    let opts = Object::new();
    Reflect::set(&opts, &JsValue::from_str("base"), &JsValue::from_str(base))
        .map_err(|_| SpeechError {
            message: "no se pudo armar opts.base".into(),
        })?;
    Reflect::set(&opts, &JsValue::from_str("lang"), &JsValue::from_str(lang))
        .map_err(|_| SpeechError {
            message: "no se pudo armar opts.lang".into(),
        })?;
    Reflect::set(
        &opts,
        &JsValue::from_str("onUpdate"),
        on_update.as_ref().unchecked_ref(),
    )
    .map_err(|_| SpeechError {
        message: "no se pudo armar opts.onUpdate".into(),
    })?;
    Reflect::set(
        &opts,
        &JsValue::from_str("onError"),
        on_error.as_ref().unchecked_ref(),
    )
    .map_err(|_| SpeechError {
        message: "no se pudo armar opts.onError".into(),
    })?;
    Reflect::set(
        &opts,
        &JsValue::from_str("onEnd"),
        on_end.as_ref().unchecked_ref(),
    )
    .map_err(|_| SpeechError {
        message: "no se pudo armar opts.onEnd".into(),
    })?;

    let ret = start_fn
        .call1(&api, &opts)
        .map_err(|_| SpeechError {
            message: "falló ppiSpeech.start".into(),
        })?;
    let ok = Reflect::get(&ret, &JsValue::from_str("ok"))
        .ok()
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    if !ok {
        return Err(SpeechError {
            message: "No se pudo iniciar el dictado por voz.".into(),
        });
    }
    Ok(())
}

/// Pure join helper (parity with JS `compose`) — unit-testable without DOM.
pub fn compose_notes(base: &str, finals: &str, interim: &str) -> String {
    let spoken = format!("{finals}{interim}");
    if spoken.is_empty() {
        return base.to_string();
    }
    if base.is_empty() {
        return spoken.trim_start().to_string();
    }
    if base.ends_with(char::is_whitespace) || spoken.starts_with(char::is_whitespace) {
        return format!("{base}{spoken}");
    }
    format!("{base} {}", spoken.trim_start())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compose_joins_with_space() {
        assert_eq!(compose_notes("Hola", "mundo", ""), "Hola mundo");
        assert_eq!(compose_notes("Hola ", "mundo", ""), "Hola mundo");
        assert_eq!(compose_notes("", "hola", " ya"), "hola ya");
    }
}
