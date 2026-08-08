//! Shared session signals for the CSR shell.

use leptos::prelude::*;

use crate::api::AuthUser;
use crate::auth::{clear_token, fetch_me, get_stored_token, store_token};

#[derive(Clone, Copy)]
pub struct SessionCtx {
    pub token: RwSignal<Option<String>>,
    pub user: RwSignal<Option<AuthUser>>,
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

    pub fn clear(&self) {
        clear_token();
        self.token.set(None);
        self.user.set(None);
    }
}

/// Load token from localStorage and hydrate `/api/me` once on startup.
#[component]
pub fn SessionBootstrap() -> impl IntoView {
    let session = expect_context::<SessionCtx>();

    Effect::new(move |_| {
        if session.bootstrapped.get() {
            return;
        }
        session.bootstrapped.set(true);

        let Some(token) = get_stored_token() else {
            return;
        };
        session.token.set(Some(token.clone()));

        leptos::task::spawn_local(async move {
            match fetch_me(&token).await {
                Ok(user) => session.user.set(Some(user)),
                Err(_) => {
                    clear_token();
                    session.token.set(None);
                    session.user.set(None);
                }
            }
        });
    });

    view! { <></> }
}
