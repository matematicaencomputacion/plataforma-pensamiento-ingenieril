//! Browser session + HTTP client for Go auth endpoints (CSR / Wasm only).

use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{CustomEvent, CustomEventInit, HtmlInputElement, HtmlTextAreaElement};

use crate::api::{
    current_level_url, forgot_password_url, is_auth_rejection, parse_auth_error_body,
    reset_password_url, sanitize_email, synthesize_profile_url, AuthCredentials, AuthSuccess,
    AuthUser, ForgotPasswordRequest, ForgotPasswordResponse, Level, ProfileSynthesis,
    ResetPasswordRequest, SynthesizeProfileRequest, AUTH_TOKEN_KEY, MSG_INVALID_RESPONSE,
    MSG_NETWORK_UNAVAILABLE, login_url, logout_url, me_url, register_url,
};

/// Same-tab signal that `SessionCtx` should drop in-memory auth after a storage purge.
pub const AUTH_CLEARED_EVENT: &str = "ppi:auth-cleared";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthError {
    pub message: String,
    pub status: Option<u16>,
}

impl AuthError {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            status: None,
        }
    }

    pub fn with_status(message: impl Into<String>, status: u16) -> Self {
        Self {
            message: message.into(),
            status: Some(status),
        }
    }

    pub fn is_unauthorized(&self) -> bool {
        self.status.is_some_and(is_auth_rejection)
    }
}

fn window() -> Option<web_sys::Window> {
    web_sys::window()
}

fn dispatch_auth_cleared() {
    let Some(window) = window() else {
        return;
    };
    let init = CustomEventInit::new();
    init.set_bubbles(true);
    if let Ok(ev) = CustomEvent::new_with_event_init_dict(AUTH_CLEARED_EVENT, &init) {
        let _ = window.dispatch_event(&ev);
    }
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

/// Drop every residual auth key we own (local + session storage).
///
/// Also notifies the same tab via [`AUTH_CLEARED_EVENT`] so `SessionCtx` signals
/// clear immediately (the browser `storage` event only fires in *other* tabs).
pub fn purge_auth_storage() {
    clear_token();
    if let Some(Ok(Some(storage))) = window().map(|w| w.session_storage()) {
        let _ = storage.remove_item(AUTH_TOKEN_KEY);
    }
    dispatch_auth_cleared();
}

async fn read_error(res: &gloo_net::http::Response) -> String {
    let status = res.status();
    match res.text().await {
        Ok(body) => parse_auth_error_body(&body, status),
        Err(_) => format!("Error HTTP {status}"),
    }
}

fn network_unavailable(_detail: impl std::fmt::Display) -> AuthError {
    // Do not surface raw browser/CORS strings to the learner UI.
    AuthError::new(MSG_NETWORK_UNAVAILABLE)
}

fn invalid_response(_detail: impl std::fmt::Display) -> AuthError {
    AuthError::new(MSG_INVALID_RESPONSE)
}

fn request_build_error(_detail: impl std::fmt::Display) -> AuthError {
    AuthError::new(MSG_NETWORK_UNAVAILABLE)
}

/// If the API rejects the Bearer session, scrub storage (+ SessionCtx via event)
/// before returning.
async fn reject_if_not_ok(
    res: gloo_net::http::Response,
) -> Result<gloo_net::http::Response, AuthError> {
    if res.ok() {
        return Ok(res);
    }
    let status = res.status();
    if is_auth_rejection(status) {
        purge_auth_storage();
    }
    Err(AuthError::with_status(read_error(&res).await, status))
}

pub async fn login_user(email: String, password: String) -> Result<AuthSuccess, AuthError> {
    post_credentials(login_url(), sanitize_email(email), password).await
}

pub async fn register_user(email: String, password: String) -> Result<AuthSuccess, AuthError> {
    post_credentials(register_url(), sanitize_email(email), password).await
}

pub async fn request_password_reset(email: String) -> Result<ForgotPasswordResponse, AuthError> {
    let payload = ForgotPasswordRequest {
        email: sanitize_email(email),
    };
    let res = Request::post(&forgot_password_url())
        .header("Content-Type", "application/json")
        .json(&payload)
        .map_err(request_build_error)?
        .send()
        .await
        .map_err(network_unavailable)?;

    let res = reject_if_not_ok(res).await?;
    res.json::<ForgotPasswordResponse>()
        .await
        .map_err(invalid_response)
}

pub async fn reset_password(token: String, password: String) -> Result<AuthSuccess, AuthError> {
    let payload = ResetPasswordRequest {
        token: token.trim().to_string(),
        password,
    };
    let res = Request::post(&reset_password_url())
        .header("Content-Type", "application/json")
        .json(&payload)
        .map_err(request_build_error)?
        .send()
        .await
        .map_err(network_unavailable)?;

    let res = reject_if_not_ok(res).await?;
    res.json::<AuthSuccess>().await.map_err(invalid_response)
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
        .map_err(request_build_error)?
        .send()
        .await
        .map_err(network_unavailable)?;

    // Login/register 401 is "bad credentials", not an orphan Bearer session —
    // do not scrub storage unless a stale token somehow remains.
    if !res.ok() {
        let status = res.status();
        return Err(AuthError::with_status(read_error(&res).await, status));
    }

    res.json::<AuthSuccess>().await.map_err(invalid_response)
}

pub async fn fetch_me(token: &str) -> Result<AuthUser, AuthError> {
    let res = Request::get(&me_url())
        .header("Authorization", &format!("Bearer {token}"))
        .send()
        .await
        .map_err(network_unavailable)?;

    let res = reject_if_not_ok(res).await?;
    res.json::<AuthUser>().await.map_err(invalid_response)
}

/// Public curriculum entry (`GET /api/levels/current`) — no Bearer required.
pub async fn fetch_current_level() -> Result<Level, AuthError> {
    let res = Request::get(&current_level_url())
        .send()
        .await
        .map_err(network_unavailable)?;

    if !res.ok() {
        let status = res.status();
        return Err(AuthError::with_status(read_error(&res).await, status));
    }

    res.json::<Level>().await.map_err(invalid_response)
}

/// Onboarding synthesize (`POST /api/learner/profile/synthesize`) — no Bearer required.
pub async fn synthesize_learner_profile(
    raw_notes: String,
    source_step_id: String,
) -> Result<ProfileSynthesis, AuthError> {
    let payload = SynthesizeProfileRequest {
        raw_notes,
        source_step_id,
    };
    let res = Request::post(&synthesize_profile_url())
        .header("Content-Type", "application/json")
        .json(&payload)
        .map_err(request_build_error)?
        .send()
        .await
        .map_err(network_unavailable)?;

    if !res.ok() {
        let status = res.status();
        let message = if status == 400 {
            let body = read_error(&res).await;
            if body.contains("raw_notes") || body.contains("HTTP 400") {
                "El relato es demasiado corto para analizar.".into()
            } else {
                body
            }
        } else {
            read_error(&res).await
        };
        return Err(AuthError::with_status(message, status));
    }

    res.json::<ProfileSynthesis>()
        .await
        .map_err(invalid_response)
}

pub async fn logout_session() {
    let token = get_stored_token();
    let mut req = Request::post(&logout_url());
    if let Some(token) = token.as_deref() {
        req = req.header("Authorization", &format!("Bearer {token}"));
    }
    let _ = req.send().await;
    purge_auth_storage();
}

/// Helper for uncontrolled-looking inputs bound to signals via on:input.
pub fn input_value(ev: &web_sys::Event) -> String {
    let Some(target) = ev.target() else {
        return String::new();
    };
    if let Ok(el) = target.clone().dyn_into::<HtmlInputElement>() {
        return el.value();
    }
    if let Ok(el) = target.dyn_into::<HtmlTextAreaElement>() {
        return el.value();
    }
    String::new()
}
