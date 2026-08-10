//! HTTP constants and wire types for the Go API.
//!
//! JSON field names mirror the existing auth handlers — do not rename without
//! coordinating a backend change.

use serde::{Deserialize, Serialize};

/// API origin for the Go backend.
///
/// Empty string → same-origin relative URLs (`/api/...`) so `trunk serve`'s
/// `[[proxy]]` (`rewrite = "/api/"` → `http://127.0.0.1:8080/api/`) can forward
/// POSTs to Go. A misconfigured proxy falls through to Trunk's SPA and returns
/// HTTP 405 (Allow: GET, HEAD) on auth endpoints.
pub const API_BASE_URL: &str = "";

/// localStorage key (shared semantic with the legacy Qwik client).
pub const AUTH_TOKEN_KEY: &str = "ppi.auth.token";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: String,
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AuthSuccess {
    pub user: AuthUser,
    pub token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
struct AuthErrorBody {
    error: Option<String>,
}

fn api_url(path: &str) -> String {
    debug_assert!(path.starts_with('/'), "API paths must be absolute from the origin");
    format!("{API_BASE_URL}{path}")
}

pub fn login_url() -> String {
    api_url("/api/auth/login")
}

pub fn register_url() -> String {
    api_url("/api/auth/register")
}

pub fn logout_url() -> String {
    api_url("/api/auth/logout")
}

pub fn me_url() -> String {
    api_url("/api/me")
}

/// True when the API rejected the Bearer session (orphan / wiped DB / bad JWT).
pub fn is_auth_rejection(status: u16) -> bool {
    status == 401 || status == 403
}

pub fn forgot_password_url() -> String {
    api_url("/api/auth/forgot-password")
}

pub fn reset_password_url() -> String {
    api_url("/api/auth/reset-password")
}

pub fn current_level_url() -> String {
    api_url("/api/levels/current")
}

/// Wire type for `GET /api/levels/current` (mirrors Go `domain.Level`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Level {
    pub id: i32,
    pub title: String,
    pub statement: String,
    pub track_type: String,
    #[serde(default)]
    pub evaluation_prompt: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ForgotPasswordResponse {
    pub message: String,
    #[serde(default, rename = "resetToken")]
    pub reset_token: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub password: String,
}

/// Prefer API `error` field; fall back to HTTP status text.
pub fn parse_auth_error_body(body: &str, status: u16) -> String {
    if let Ok(parsed) = serde_json::from_str::<AuthErrorBody>(body) {
        if let Some(msg) = parsed.error {
            if !msg.trim().is_empty() {
                return msg;
            }
        }
    }
    format!("Error HTTP {status}")
}

/// Normalize email before wire: trim + lowercase (mirrors Go `normalizeEmail`).
pub fn sanitize_email(email: impl AsRef<str>) -> String {
    email.as_ref().trim().to_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_base_url_is_same_origin_for_trunk_proxy() {
        assert_eq!(API_BASE_URL, "");
    }

    #[test]
    fn auth_urls_are_relative_api_paths() {
        assert_eq!(login_url(), "/api/auth/login");
        assert_eq!(register_url(), "/api/auth/register");
        assert_eq!(logout_url(), "/api/auth/logout");
        assert_eq!(me_url(), "/api/me");
        assert_eq!(forgot_password_url(), "/api/auth/forgot-password");
        assert_eq!(reset_password_url(), "/api/auth/reset-password");
        assert_eq!(current_level_url(), "/api/levels/current");
    }

    #[test]
    fn level_json_matches_backend_contract() {
        let raw = r#"{
            "id": 1,
            "title": "Hola mundo",
            "statement": "Imprimí un saludo",
            "track_type": "micro_paso",
            "evaluation_prompt": "eval"
        }"#;
        let level: Level = serde_json::from_str(raw).expect("decode level");
        assert_eq!(level.id, 1);
        assert_eq!(level.title, "Hola mundo");
        assert_eq!(level.track_type, "micro_paso");
    }

    #[test]
    fn forgot_reset_json_matches_backend_contract() {
        let forgot = serde_json::to_string(&ForgotPasswordRequest {
            email: "alum@example.com".into(),
        })
        .expect("serialize forgot");
        assert!(forgot.contains("\"email\""));

        let reset = serde_json::to_string(&ResetPasswordRequest {
            token: "abc".into(),
            password: "secreto12".into(),
        })
        .expect("serialize reset");
        assert!(reset.contains("\"token\""));
        assert!(reset.contains("\"password\""));

        let parsed: ForgotPasswordResponse = serde_json::from_str(
            r#"{"message":"ok","resetToken":"deadbeef"}"#,
        )
        .expect("decode forgot");
        assert_eq!(parsed.reset_token.as_deref(), Some("deadbeef"));
    }

    #[test]
    fn credentials_json_matches_backend_contract() {
        let body = serde_json::to_string(&AuthCredentials {
            email: "alum@example.com".into(),
            password: "secreto12".into(),
        })
        .expect("serialize");
        assert!(body.contains("\"email\""));
        assert!(body.contains("\"password\""));
        assert!(!body.contains("Email"));
    }

    #[test]
    fn parse_auth_error_prefers_json_message() {
        assert_eq!(
            parse_auth_error_body(r#"{"error":"credenciales inválidas"}"#, 401),
            "credenciales inválidas"
        );
        assert_eq!(parse_auth_error_body("not-json", 500), "Error HTTP 500");
    }

    #[test]
    fn auth_rejection_detects_unauthorized_and_forbidden() {
        assert!(is_auth_rejection(401));
        assert!(is_auth_rejection(403));
        assert!(!is_auth_rejection(400));
        assert!(!is_auth_rejection(500));
    }

    #[test]
    fn sanitize_email_trims_and_lowercases() {
        assert_eq!(sanitize_email("  Alum@Example.COM "), "alum@example.com");
        assert_eq!(sanitize_email("ok@x.com"), "ok@x.com");
    }

    #[test]
    fn auth_success_deserializes_like_go() {
        let raw = r#"{"user":{"id":"u1","email":"a@b.com"},"token":"jwt"}"#;
        let parsed: AuthSuccess = serde_json::from_str(raw).expect("decode");
        assert_eq!(parsed.user.id, "u1");
        assert_eq!(parsed.user.email, "a@b.com");
        assert_eq!(parsed.token, "jwt");
    }
}
