//! Browser session + HTTP client for Go auth endpoints (CSR / Wasm only).

use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

use crate::api::{
    forgot_password_url, parse_auth_error_body, reset_password_url, AuthCredentials, AuthSuccess,
    AuthUser, ForgotPasswordRequest, ForgotPasswordResponse, ResetPasswordRequest, AUTH_TOKEN_KEY,
    login_url, logout_url, me_url, register_url,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthError {
    pub message: String,
}

impl AuthError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

fn window() -> Option<web_sys::Window> {
    web_sys::window()
}

pub fn get_stored_token() -> Option<String> {
    let storage = window()?.local_storage().ok()??;
    storage.get_item(AUTH_TOKEN_KEY).ok()?
}

pub fn store_token(token: &str) {
    if let Some(Ok(Some(storage))) = window().map(|w| w.local_storage()) {
        let _ = storage.set_item(AUTH_TOKEN_KEY, token);
    }
}

pub fn clear_token() {
    if let Some(Ok(Some(storage))) = window().map(|w| w.local_storage()) {
        let _ = storage.remove_item(AUTH_TOKEN_KEY);
    }
}

async fn read_error(res: &gloo_net::http::Response) -> String {
    let status = res.status();
    match res.text().await {
        Ok(body) => parse_auth_error_body(&body, status),
        Err(_) => format!("Error HTTP {status}"),
    }
}

pub async fn login_user(email: String, password: String) -> Result<AuthSuccess, AuthError> {
    post_credentials(login_url(), email, password).await
}

pub async fn register_user(email: String, password: String) -> Result<AuthSuccess, AuthError> {
    post_credentials(register_url(), email, password).await
}

pub async fn request_password_reset(email: String) -> Result<ForgotPasswordResponse, AuthError> {
    let payload = ForgotPasswordRequest { email };
    let res = Request::post(&forgot_password_url())
        .header("Content-Type", "application/json")
        .json(&payload)
        .map_err(|e| AuthError::new(e.to_string()))?
        .send()
        .await
        .map_err(|e| AuthError::new(format!("No se pudo contactar la API: {e}")))?;

    if !res.ok() {
        return Err(AuthError::new(read_error(&res).await));
    }

    res.json::<ForgotPasswordResponse>()
        .await
        .map_err(|e| AuthError::new(format!("Respuesta inválida: {e}")))
}

pub async fn reset_password(token: String, password: String) -> Result<AuthSuccess, AuthError> {
    let payload = ResetPasswordRequest { token, password };
    let res = Request::post(&reset_password_url())
        .header("Content-Type", "application/json")
        .json(&payload)
        .map_err(|e| AuthError::new(e.to_string()))?
        .send()
        .await
        .map_err(|e| AuthError::new(format!("No se pudo contactar la API: {e}")))?;

    if !res.ok() {
        return Err(AuthError::new(read_error(&res).await));
    }

    res.json::<AuthSuccess>()
        .await
        .map_err(|e| AuthError::new(format!("Respuesta inválida: {e}")))
}

async fn post_credentials(
    url: String,
    email: String,
    password: String,
) -> Result<AuthSuccess, AuthError> {
    let payload = AuthCredentials { email, password };
    let res = Request::post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
        .map_err(|e| AuthError::new(e.to_string()))?
        .send()
        .await
        .map_err(|e| AuthError::new(format!("No se pudo contactar la API: {e}")))?;

    if !res.ok() {
        return Err(AuthError::new(read_error(&res).await));
    }

    res.json::<AuthSuccess>()
        .await
        .map_err(|e| AuthError::new(format!("Respuesta inválida: {e}")))
}

pub async fn fetch_me(token: &str) -> Result<AuthUser, AuthError> {
    let res = Request::get(&me_url())
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| AuthError::new(format!("No se pudo contactar la API: {e}")))?;

    if !res.ok() {
        return Err(AuthError::new(read_error(&res).await));
    }

    res.json::<AuthUser>()
        .await
        .map_err(|e| AuthError::new(format!("Respuesta inválida: {e}")))
}

pub async fn logout_session() {
    let token = get_stored_token();
    let mut req = Request::post(&logout_url());
    if let Some(token) = token.as_deref() {
        req = req.header("Authorization", &format!("Bearer {token}"));
    }
    let _ = req.send().await;
    clear_token();
}

/// Helper for uncontrolled-looking inputs bound to signals via on:input.
pub fn input_value(ev: &web_sys::Event) -> String {
    ev.target()
        .and_then(|t| t.dyn_into::<HtmlInputElement>().ok())
        .map(|el| el.value())
        .unwrap_or_default()
}
