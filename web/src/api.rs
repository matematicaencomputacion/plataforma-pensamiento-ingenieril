//! HTTP client constants for the Go API.
//!
//! Contracts (JSON paths/fields) are owned by the Go backend; this module only
//! points the Leptos CSR shell at the existing service.

/// Base URL for the Go Clean Architecture API (local default).
pub const API_BASE_URL: &str = "http://localhost:8080";

#[cfg(test)]
mod tests {
    use super::API_BASE_URL;

    #[test]
    fn api_base_url_points_at_local_go_api() {
        assert_eq!(API_BASE_URL, "http://localhost:8080");
        assert!(API_BASE_URL.starts_with("http://"), "API base must be absolute for browser fetch");
    }
}
