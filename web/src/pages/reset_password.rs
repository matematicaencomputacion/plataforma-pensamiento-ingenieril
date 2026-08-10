use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::BrandLink;
use leptos_router::hooks::{use_navigate, use_query_map};

use crate::auth::{input_value, reset_password};
use crate::session::SessionCtx;

#[component]
pub fn ResetPasswordPage() -> impl IntoView {
    let session = expect_context::<SessionCtx>();
    let navigate = use_navigate();
    let query = use_query_map();

    let token = RwSignal::new(
        query
            .get_untracked()
            .get_str("token")
            .unwrap_or_default()
            .to_string(),
    );
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(String::new());
    let busy = RwSignal::new(false);

    Effect::new({
        let navigate = navigate.clone();
        move |_| {
            if session.bootstrapped.get() && session.user.get().is_some() {
                navigate(
                    "/workspace",
                    leptos_router::NavigateOptions {
                        replace: true,
                        ..Default::default()
                    },
                );
            }
        }
    });

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        if busy.get_untracked() {
            return;
        }
        error.set(String::new());
        busy.set(true);

        // Re-read query in case the signal was initialized before the router hydrated.
        let from_query = query
            .get_untracked()
            .get_str("token")
            .unwrap_or_default()
            .to_string();
        let mut token_v = token.get_untracked();
        if token_v.is_empty() && !from_query.is_empty() {
            token_v = from_query;
            token.set(token_v.clone());
        }
        let password_v = password.get_untracked();
        let navigate = navigate.clone();

        leptos::task::spawn_local(async move {
            let outcome = reset_password(token_v, password_v).await;
            busy.set(false);
            match outcome {
                Ok(result) => {
                    session.establish(result.user, result.token);
                    navigate("/workspace", Default::default());
                }
                Err(err) => {
                    error.set(err.message);
                }
            }
        });
    };

    view! {
        <section class="auth-page">
            <div class="auth-page__card">
                <BrandLink class="auth-page__brand" />
                <h1 class="auth-page__title">"Nueva contraseña"</h1>
                <p class="auth-page__lead">
                    "Elegí una contraseña de al menos 8 caracteres."
                </p>
                <form class="auth-form" on:submit=on_submit>
                    <label class="auth-form__label" for="reset-token">
                        "Token de recuperación"
                    </label>
                    <input
                        id="reset-token"
                        class="auth-form__input"
                        type="text"
                        autocomplete="off"
                        required
                        prop:value=move || token.get()
                        on:input=move |ev| token.set(input_value(&ev))
                    />
                    <label class="auth-form__label" for="reset-password">
                        "Nueva contraseña"
                    </label>
                    <input
                        id="reset-password"
                        class="auth-form__input"
                        type="password"
                        autocomplete="new-password"
                        required
                        minlength=8
                        prop:value=move || password.get()
                        on:input=move |ev| password.set(input_value(&ev))
                    />
                    <Show when=move || !error.get().is_empty()>
                        <p class="auth-form__error" role="alert" aria-live="polite">
                            {move || error.get()}
                        </p>
                    </Show>
                    <button
                        class="cta cta--primary auth-form__submit"
                        type="submit"
                        prop:disabled=move || busy.get()
                        attr:aria-busy=move || busy.get().to_string()
                    >
                        {move || {
                            if busy.get() {
                                "Guardando…"
                            } else {
                                "Restablecer contraseña"
                            }
                        }}
                    </button>
                </form>
                <p class="auth-page__switch">
                    <A href="/forgot-password">"Solicitar un token nuevo"</A>
                </p>
            </div>
        </section>
    }
}
