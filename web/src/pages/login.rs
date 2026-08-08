use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

use crate::auth::{input_value, login_user};
use crate::session::SessionCtx;

#[component]
pub fn LoginPage() -> impl IntoView {
    let session = expect_context::<SessionCtx>();
    let navigate = use_navigate();

    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let error = RwSignal::new(String::new());
    let busy = RwSignal::new(false);

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        if busy.get_untracked() {
            return;
        }
        error.set(String::new());
        busy.set(true);

        let email_v = email.get_untracked();
        let password_v = password.get_untracked();
        let navigate = navigate.clone();

        leptos::task::spawn_local(async move {
            match login_user(email_v, password_v).await {
                Ok(result) => {
                    session.establish(result.user, result.token);
                    busy.set(false);
                    navigate("/workspace", Default::default());
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
                <h1 class="auth-page__title">"Iniciar sesión"</h1>
                <p class="auth-page__lead">
                    "Entrá con tu correo para continuar en el workspace."
                </p>
                <form class="auth-form" on:submit=on_submit>
                    <label class="auth-form__label" for="login-email">
                        "Correo"
                    </label>
                    <input
                        id="login-email"
                        class="auth-form__input"
                        type="email"
                        autocomplete="email"
                        required
                        prop:value=move || email.get()
                        on:input=move |ev| email.set(input_value(&ev))
                    />
                    <label class="auth-form__label" for="login-password">
                        "Contraseña"
                    </label>
                    <input
                        id="login-password"
                        class="auth-form__input"
                        type="password"
                        autocomplete="current-password"
                        required
                        minlength="8"
                        prop:value=move || password.get()
                        on:input=move |ev| password.set(input_value(&ev))
                    />
                    <Show when=move || !error.get().is_empty()>
                        <p class="auth-form__error" role="alert">
                            {move || error.get()}
                        </p>
                    </Show>
                    <button
                        class="cta cta--primary auth-form__submit"
                        type="submit"
                        prop:disabled=move || busy.get()
                    >
                        {move || {
                            if busy.get() {
                                "Entrando…"
                            } else {
                                "Entrar"
                            }
                        }}
                    </button>
                </form>
                <p class="auth-page__switch">
                    "¿No tenés cuenta? "
                    <A href="/register">"Crear cuenta"</A>
                </p>
            </div>
        </section>
    }
}
