use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

use crate::auth::{input_value, request_password_reset};

#[component]
pub fn ForgotPasswordPage() -> impl IntoView {
    let navigate = use_navigate();
    let email = RwSignal::new(String::new());
    let error = RwSignal::new(String::new());
    let message = RwSignal::new(String::new());
    let reset_token = RwSignal::new(String::new());
    let busy = RwSignal::new(false);

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        if busy.get_untracked() {
            return;
        }
        error.set(String::new());
        message.set(String::new());
        reset_token.set(String::new());
        busy.set(true);

        let email_v = email.get_untracked();
        let navigate = navigate.clone();
        leptos::task::spawn_local(async move {
            match request_password_reset(email_v).await {
                Ok(res) => {
                    message.set(res.message);
                    if let Some(tok) = res.reset_token.filter(|t| !t.is_empty()) {
                        // DX: backend only includes resetToken in local/dev exposure modes.
                        reset_token.set(tok.clone());
                        navigate(
                            &format!("/reset-password?token={tok}"),
                            Default::default(),
                        );
                    }
                    busy.set(false);
                }
                Err(err) => {
                    error.set(err.message);
                    busy.set(false);
                }
            }
        });
    };

    view! {
        <section class="auth-page">
            <div class="auth-page__card">
                <A href="/" attr:class="auth-page__brand">
                    "IngenierIA"
                </A>
                <h1 class="auth-page__title">"Recuperar contraseña"</h1>
                <p class="auth-page__lead">
                    "Ingresá tu correo y te indicaremos cómo restablecer el acceso."
                </p>
                <form class="auth-form" on:submit=on_submit>
                    <label class="auth-form__label" for="forgot-email">
                        "Correo"
                    </label>
                    <input
                        id="forgot-email"
                        class="auth-form__input"
                        type="email"
                        autocomplete="email"
                        required
                        prop:value=move || email.get()
                        on:input=move |ev| email.set(input_value(&ev))
                    />
                    <Show when=move || !error.get().is_empty()>
                        <p class="auth-form__error" role="alert">
                            {move || error.get()}
                        </p>
                    </Show>
                    <Show when=move || !message.get().is_empty()>
                        <p class="auth-form__ok" role="status">
                            {move || message.get()}
                        </p>
                    </Show>
                    <Show when=move || !reset_token.get().is_empty()>
                        <p class="auth-form__dev" data-testid="reset-token-hint">
                            "Entorno de desarrollo: "
                            <A
                                href=move || format!("/reset-password?token={}", reset_token.get())
                                attr:class="auth-form__dev-link"
                            >
                                "continuar al reseteo"
                            </A>
                        </p>
                    </Show>
                    <button
                        class="cta cta--primary auth-form__submit"
                        type="submit"
                        prop:disabled=move || busy.get()
                    >
                        {move || {
                            if busy.get() {
                                "Enviando…"
                            } else {
                                "Enviar instrucciones"
                            }
                        }}
                    </button>
                </form>
                <p class="auth-page__switch">
                    <A href="/login">"Volver a iniciar sesión"</A>
                </p>
            </div>
        </section>
    }
}
