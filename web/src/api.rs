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
    fn auth_success_deserializes_like_go() {
        let raw = r#"{"user":{"id":"u1","email":"a@b.com"},"token":"jwt"}"#;
        let parsed: AuthSuccess = serde_json::from_str(raw).expect("decode");
        assert_eq!(parsed.user.id, "u1");
        assert_eq!(parsed.user.email, "a@b.com");
        assert_eq!(parsed.token, "jwt");
    }
}
