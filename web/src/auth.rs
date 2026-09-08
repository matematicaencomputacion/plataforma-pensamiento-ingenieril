//! Browser session + HTTP client for Go auth endpoints (CSR / Wasm only).

use gloo_net::http::Request;
use wasm_bindgen::JsCast;
use web_sys::{CustomEvent, CustomEventInit, HtmlInputElement, HtmlTextAreaElement};

use crate::api::{
    concept_analytics_url, concept_events_url, current_level_url, forgot_password_url,
    is_auth_rejection, login_url, logout_url, me_url, parse_auth_error_body, progress_complete_url,
    progress_reset_url, register_url, reset_password_url, sanitize_email, synthesize_profile_url,
    user_profile_url, AuthCredentials, AuthSuccess, AuthUser, ConceptAnalyticsSummary,
    ConceptEventRequest, ForgotPasswordRequest, ForgotPasswordResponse, Level, ProfileSynthesis,
    ProgressCompleteRequest, ProgressCompleteResponse, ResetPasswordRequest,
    SynthesizeProfileRequest, UserProfile, AUTH_TOKEN_KEY, MSG_INVALID_RESPONSE,
    MSG_NETWORK_UNAVAILABLE,
};

/// Same-tab signal that `SessionCtx` should drop in-memory auth after a storage purge.
pub const AUTH_CLEARED_EVENT: &str = "ppi:auth-cleared";

const AUTH_ROUTES: [&str; 4] = ["/login", "/register", "/forgot-password", "/reset-password"];

/// Accept only same-origin absolute paths and avoid authentication redirect loops.
pub fn safe_return_to(candidate: &str) -> Option<String> {
    let value = candidate.trim();
    if !value.starts_with('/')
        || value.starts_with("//")
        || value.contains('\\')
        || value.chars().any(char::is_control)
    {
        return None;
    }

    let path = value.split(['?', '#']).next().unwrap_or(value);
    if AUTH_ROUTES
        .iter()
        .any(|route| path == *route || path.starts_with(&format!("{route}/")))
    {
        return None;
    }
    Some(value.to_string())
}

pub fn path_with_search(pathname: &str, search: &str) -> String {
    if search.is_empty() {
        return pathname.to_string();
    }
    if search.starts_with('?') {
        format!("{pathname}{search}")
    } else {
        format!("{pathname}?{search}")
    }
}

pub fn login_path(return_to: &str) -> String {
    match safe_return_to(return_to) {
        Some(path) => format!("/login?return_to={}", percent_encode_query_value(&path)),
        None => "/login".to_string(),
    }
}

fn percent_encode_query_value(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                encoded.push(byte as char)
            }
            _ => encoded.push_str(&format!("%{byte:02X}")),
        }
    }
    encoded
}

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

#[cfg(test)]
mod tests {
    use super::{login_path, path_with_search, safe_return_to};

    #[test]
    fn safe_return_to_accepts_and_encodes_internal_routes() {
        assert_eq!(
            safe_return_to("/learn/42?mode=review#result"),
            Some("/learn/42?mode=review#result".into())
        );
        assert_eq!(
            login_path("/learn/42?mode=review"),
            "/login?return_to=%2Flearn%2F42%3Fmode%3Dreview"
        );
        assert_eq!(
            path_with_search("/concepts/2", "q=list"),
            "/concepts/2?q=list"
        );
        assert_eq!(path_with_search("/workspace", ""), "/workspace");
    }

    #[test]
    fn safe_return_to_rejects_external_auth_and_malformed_routes() {
        for value in [
            "https://evil.test",
            "//evil.test",
            "/\\evil.test",
            "/login",
            "/login/again",
            "/register?next=/workspace",
            "workspace",
            "/learn\n/42",
        ] {
            assert_eq!(safe_return_to(value), None, "{value:?}");
            assert_eq!(login_path(value), "/login", "{value:?}");
        }
    }
}

/// Public curriculum entry (`GET /api/levels/current`) — no Bearer required.
/// Kept for the Go contract / Qwik parity; `/learn` no longer overlays this seed.
#[allow(dead_code)]
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

/// Rehydrate coaching profile (`GET /api/user/profile`) with Bearer session.
pub async fn fetch_user_profile() -> Result<UserProfile, AuthError> {
    let Some(token) = get_stored_token() else {
        return Err(AuthError::with_status("Sesión no iniciada.", 401));
    };
    let res = Request::get(&user_profile_url())
        .header("Authorization", &format!("Bearer {token}"))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(network_unavailable)?;

    let res = reject_if_not_ok(res).await?;
    res.json::<UserProfile>()
        .await
        .map(|p| p.normalize())
        .map_err(invalid_response)
}

/// Persist coaching profile (`PUT /api/user/profile`) with Bearer session.
pub async fn put_user_profile(profile: UserProfile) -> Result<UserProfile, AuthError> {
    let Some(token) = get_stored_token() else {
        return Err(AuthError::with_status(
            "Sesión expirada. Volvé a iniciar sesión para guardar tu perfil.",
            401,
        ));
    };
    let payload = profile.normalize();
    if payload.is_empty() {
        return Err(AuthError::with_status(
            "El perfil está vacío. Completá al menos un campo antes de guardar.",
            400,
        ));
    }

    let res = Request::put(&user_profile_url())
        .header("Authorization", &format!("Bearer {token}"))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&payload)
        .map_err(request_build_error)?
        .send()
        .await
        .map_err(network_unavailable)?;

    let res = reject_if_not_ok(res).await?;
    res.json::<UserProfile>()
        .await
        .map(|p| p.normalize())
        .map_err(invalid_response)
}

/// Persist exercise progress after client-side Pyodide checks (ADR 002: no student code).
pub async fn complete_progress(
    level_id: i32,
    step_id: String,
    passed: bool,
) -> Result<ProgressCompleteResponse, AuthError> {
    let Some(token) = get_stored_token() else {
        return Err(AuthError::with_status("Sesión no iniciada.", 401));
    };
    let payload = ProgressCompleteRequest {
        level_id,
        step_id,
        passed,
    };
    let res = Request::post(&progress_complete_url())
        .header("Authorization", &format!("Bearer {token}"))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&payload)
        .map_err(request_build_error)?
        .send()
        .await
        .map_err(network_unavailable)?;

    let res = reject_if_not_ok(res).await?;
    res.json::<ProgressCompleteResponse>()
        .await
        .map_err(invalid_response)
}

/// Reset learning progress to level 1 (clears persistent checkmarks).
pub async fn reset_progress() -> Result<ProgressCompleteResponse, AuthError> {
    let Some(token) = get_stored_token() else {
        return Err(AuthError::with_status("Sesión no iniciada.", 401));
    };
    let res = Request::post(&progress_reset_url())
        .header("Authorization", &format!("Bearer {token}"))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(network_unavailable)?;

    let res = reject_if_not_ok(res).await?;
    res.json::<ProgressCompleteResponse>()
        .await
        .map_err(invalid_response)
}

/// Best-effort friction event. Does not purge the session on 4xx (analytics is optional).
pub async fn post_concept_event(payload: ConceptEventRequest) -> Result<(), AuthError> {
    let Some(token) = get_stored_token() else {
        return Err(AuthError::with_status("Sesión no iniciada.", 401));
    };
    let res = Request::post(&concept_events_url())
        .header("Authorization", &format!("Bearer {token}"))
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .json(&payload)
        .map_err(request_build_error)?
        .send()
        .await
        .map_err(network_unavailable)?;
    let status = res.status();
    if status == 204 || res.ok() {
        return Ok(());
    }
    Err(AuthError::with_status(read_error(&res).await, status))
}

/// Per-user bottleneck summary for `#concept-analytics`.
pub async fn fetch_concept_analytics() -> Result<ConceptAnalyticsSummary, AuthError> {
    let Some(token) = get_stored_token() else {
        return Err(AuthError::with_status("Sesión no iniciada.", 401));
    };
    let res = Request::get(&concept_analytics_url())
        .header("Authorization", &format!("Bearer {token}"))
        .header("Accept", "application/json")
        .send()
        .await
        .map_err(network_unavailable)?;
    let res = reject_if_not_ok(res).await?;
    res.json::<ConceptAnalyticsSummary>()
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
