//! Shared session signals for the CSR shell.

use leptos::prelude::*;

use crate::api::AuthUser;
use crate::auth::{fetch_me, get_stored_token, purge_auth_storage, store_token};

#[derive(Clone, Copy)]
pub struct SessionCtx {
    pub token: RwSignal<Option<String>>,
    pub user: RwSignal<Option<AuthUser>>,
    /// True after the initial localStorage restore finished (sync step).
    pub bootstrapped: RwSignal<bool>,
}

impl SessionCtx {
    pub fn provide() -> Self {
        let ctx = Self {
            token: RwSignal::new(None),
            user: RwSignal::new(None),
            bootstrapped: RwSignal::new(false),
        };
        provide_context(ctx);
        ctx
    }

    pub fn establish(&self, user: AuthUser, token: String) {
        store_token(&token);
        self.token.set(Some(token));
        self.user.set(Some(user));
    }

    /// Drop in-memory + browser storage (orphan JWT after DB wipe, logout, etc.).
    pub fn clear(&self) {
        purge_auth_storage();
        self.token.set(None);
        self.user.set(None);
    }
}

/// Load token from localStorage and hydrate `/api/me` once on startup.
///
/// Critical ordering: restore `token` from storage **before** flipping
/// `bootstrapped`, so route guards never observe `(bootstrapped ∧ !token)`
/// spuriously and bounce the user (logout / home ↔ workspace thrash).
///
/// On `/api/me` 401/403 the bearer is treated as orphaned: storage and signals
/// are wiped so the UI cannot look "logged in" against a wiped SQLite.
#[component]
pub fn SessionBootstrap() -> impl IntoView {
    let session = expect_context::<SessionCtx>();

    Effect::new(move |_| {
        // Run once: do not re-subscribe to `bootstrapped` (would retrigger on set).
        if session.bootstrapped.get_untracked() {
            return;
        }

        if let Some(token) = get_stored_token() {
            session.token.set(Some(token.clone()));
            session.bootstrapped.set(true);
            leptos::task::spawn_local(async move {
                match fetch_me(&token).await {
                    Ok(user) => session.user.set(Some(user)),
                    Err(err) => {
                        if err.is_unauthorized() {
                            session.clear();
                        } else {
                            // Transient network / 5xx: drop optimistic UI session but
                            // leave storage so a refresh can retry (fetch_me only
                            // purges on 401/403).
                            session.token.set(None);
                            session.user.set(None);
                        }
                    }
                }
            });
        } else {
            // Ensure no stale key survives an interrupted clear.
            purge_auth_storage();
            session.bootstrapped.set(true);
        }
    });

    view! { <></> }
}
