use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

use crate::auth::{input_value, register_user};
use crate::session::SessionCtx;

#[component]
pub fn RegisterPage() -> impl IntoView {
    let session = expect_context::<SessionCtx>();
    let navigate = use_navigate();

    let email = RwSignal::new(String::new());
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

        let email_v = email.get_untracked();
        let password_v = password.get_untracked();
        let navigate = navigate.clone();

        leptos::task::spawn_local(async move {
            match register_user(email_v, password_v).await {
                Ok(result) => {
                    session.establish(result.user, result.token);
                    busy.set(false);
                    navigate("/workspace", Default::default());
                }
                Err(err) => {
                    if matches!(err.status, Some(401 | 409)) {
                        session.clear();
                    }
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
                <h1 class="auth-page__title">"Crear cuenta"</h1>
                <p class="auth-page__lead">
                    "Registrate con correo y contraseña (mínimo 8 caracteres)."
                </p>
                <form class="auth-form" on:submit=on_submit>
                    <label class="auth-form__label" for="register-email">
                        "Correo"
                    </label>
                    <input
                        id="register-email"
                        class="auth-form__input"
                        type="email"
                        autocomplete="email"
                        required
                        prop:value=move || email.get()
                        on:input=move |ev| email.set(input_value(&ev))
                    />
                    <label class="auth-form__label" for="register-password">
                        "Contraseña"
                    </label>
                    <input
                        id="register-password"
                        class="auth-form__input"
                        type="password"
                        autocomplete="new-password"
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
                                "Creando…"
                            } else {
                                "Crear cuenta"
                            }
                        }}
                    </button>
                </form>
                <p class="auth-page__switch">
                    "¿Ya tenés cuenta? "
                    <A href="/login">"Iniciar sesión"</A>
                </p>
            </div>
        </section>
    }
}
