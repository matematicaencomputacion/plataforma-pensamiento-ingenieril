//! Shared session signals for the CSR shell.

use leptos::prelude::*;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::StorageEvent;

use crate::api::{AuthUser, AUTH_TOKEN_KEY};
use crate::auth::{
    fetch_me, get_stored_token, purge_auth_storage, store_token, AuthError, AUTH_CLEARED_EVENT,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionStatus {
    Loading,
    Authenticated,
    Anonymous,
    TemporarilyUnavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RestoreFailure {
    Rejected,
    Transient,
}

fn classify_restore_error(error: &AuthError) -> RestoreFailure {
    if error.is_unauthorized() {
        RestoreFailure::Rejected
    } else {
        RestoreFailure::Transient
    }
}

#[derive(Clone, Copy)]
pub struct SessionCtx {
    pub token: RwSignal<Option<String>>,
    pub user: RwSignal<Option<AuthUser>>,
    /// True after the initial localStorage restore finished (sync step).
    pub bootstrapped: RwSignal<bool>,
    pub status: RwSignal<SessionStatus>,
}

impl SessionCtx {
    pub fn provide() -> Self {
        let ctx = Self {
            token: RwSignal::new(None),
            user: RwSignal::new(None),
            bootstrapped: RwSignal::new(false),
            status: RwSignal::new(SessionStatus::Loading),
        };
        provide_context(ctx);
        ctx
    }

    pub fn establish(&self, user: AuthUser, token: String) {
        store_token(&token);
        self.token.set(Some(token));
        self.user.set(Some(user));
        self.status.set(SessionStatus::Authenticated);
    }

    /// Patch progress cursor + earned set on the live session user.
    /// Replaces the `Option` so derived signals (workspace rail) always re-run.
    pub fn set_progress(&self, current_level: i32, completed_levels: Vec<i32>) {
        let next = current_level.max(1);
        let Some(mut user) = self.user.get_untracked() else {
            return;
        };
        if user.current_level == next && user.completed_levels == completed_levels {
            return;
        }
        user.current_level = next;
        user.completed_levels = completed_levels;
        self.user.set(Some(user));
    }

    /// Drop in-memory + browser storage (orphan JWT after DB wipe, logout, etc.).
    ///
    /// Idempotent: skips work when already cleared to avoid event feedback loops.
    pub fn clear(&self) {
        let already_clear =
            self.token.get_untracked().is_none() && self.user.get_untracked().is_none();
        if already_clear && get_stored_token().is_none() {
            self.status.set(SessionStatus::Anonymous);
            return;
        }
        // Clear signals first so UI reacts, then storage (which emits AUTH_CLEARED_EVENT).
        self.token.set(None);
        self.user.set(None);
        self.status.set(SessionStatus::Anonymous);
        purge_auth_storage();
    }

    /// Apply an external purge (storage event / AUTH_CLEARED_EVENT) without re-purging.
    fn drop_memory_only(&self) {
        self.token.set(None);
        self.user.set(None);
        self.status.set(SessionStatus::Anonymous);
    }

    pub fn retry_restore(&self) {
        let Some(token) = self.token.get_untracked().or_else(get_stored_token) else {
            self.status.set(SessionStatus::Anonymous);
            return;
        };
        self.token.set(Some(token.clone()));
        self.status.set(SessionStatus::Loading);
        restore_session(*self, token);
    }
}

fn restore_session(session: SessionCtx, token: String) {
    leptos::task::spawn_local(async move {
        match fetch_me(&token).await {
            Ok(user) => {
                session.user.set(Some(user));
                session.status.set(SessionStatus::Authenticated);
            }
            Err(error) => match classify_restore_error(&error) {
                RestoreFailure::Rejected => session.clear(),
                RestoreFailure::Transient => {
                    session.user.set(None);
                    session.status.set(SessionStatus::TemporarilyUnavailable);
                }
            },
        }
    });
}

/// Load token from localStorage and hydrate `/api/me` once on startup.
///
/// Critical ordering: restore `token` from storage **before** flipping
/// `bootstrapped`, so route guards never observe `(bootstrapped ∧ !token)`
/// spuriously and bounce the user (logout / home ↔ workspace thrash).
///
/// On `/api/me` 401/403 the bearer is treated as orphaned: storage and signals
/// are wiped so the UI cannot look "logged in" against a wiped SQLite.
///
/// Also listens for cross-tab `storage` removals of the auth key and same-tab
/// [`AUTH_CLEARED_EVENT`] so SessionCtx stays coherent without F5.
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
            restore_session(session, token);
        } else {
            // Ensure no stale key survives an interrupted clear.
            purge_auth_storage();
            session.bootstrapped.set(true);
            session.status.set(SessionStatus::Anonymous);
        }

        attach_auth_sync_listeners(session);
    });

    view! { <></> }
}

#[component]
pub fn SessionRecoveryNotice() -> impl IntoView {
    let session = expect_context::<SessionCtx>();

    view! {
        <Show when=move || session.status.get() == SessionStatus::TemporarilyUnavailable>
            <div id="session-recovery" class="session-recovery" role="alert" aria-live="polite">
                <span>"No pudimos verificar tu sesión. Conservamos tu acceso para reintentar."</span>
                <button id="session-retry" type="button" on:click=move |_| session.retry_restore()>
                    "Reintentar"
                </button>
            </div>
        </Show>
    }
}

fn attach_auth_sync_listeners(session: SessionCtx) {
    let Some(window) = web_sys::window() else {
        return;
    };

    // Other tabs: storage event when ppi.auth.token is removed/changed.
    let on_storage = Closure::wrap(Box::new(move |ev: web_sys::Event| {
        let Ok(storage_ev) = ev.dyn_into::<StorageEvent>() else {
            return;
        };
        let key = storage_ev.key();
        if key.as_deref() != Some(AUTH_TOKEN_KEY) && key.is_some() {
            return;
        }
        // key == None means clear(); key matched means our auth token changed.
        if storage_ev.new_value().is_none() {
            session.drop_memory_only();
        }
    }) as Box<dyn FnMut(_)>);

    let _ = window.add_event_listener_with_callback("storage", on_storage.as_ref().unchecked_ref());
    on_storage.forget();

    // Same tab: purge_auth_storage / 401 interceptor.
    let on_cleared = Closure::wrap(Box::new(move |_ev: web_sys::Event| {
        session.drop_memory_only();
    }) as Box<dyn FnMut(_)>);
    let _ = window
        .add_event_listener_with_callback(AUTH_CLEARED_EVENT, on_cleared.as_ref().unchecked_ref());
    on_cleared.forget();
}

#[cfg(test)]
mod tests {
    use super::{classify_restore_error, RestoreFailure};
    use crate::auth::AuthError;

    #[test]
    fn unauthorized_and_forbidden_are_rejected() {
        for status in [401, 403] {
            assert_eq!(
                classify_restore_error(&AuthError::with_status("rejected", status)),
                RestoreFailure::Rejected
            );
        }
    }

    #[test]
    fn network_and_server_failures_are_transient() {
        assert_eq!(
            classify_restore_error(&AuthError::new("offline")),
            RestoreFailure::Transient
        );
        assert_eq!(
            classify_restore_error(&AuthError::with_status("unavailable", 503)),
            RestoreFailure::Transient
        );
    }
}
