use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

use crate::session::SessionCtx;

#[component]
pub fn WorkspacePage() -> impl IntoView {
    let session = expect_context::<SessionCtx>();
    let navigate = use_navigate();

    Effect::new(move |_| {
        if session.bootstrapped.get() && session.token.get().is_none() {
            navigate("/login", Default::default());
        }
    });

    view! {
        <section class="workspace">
            <Show
                when=move || session.user.get().is_some()
                fallback=|| {
                    view! {
                        <p class="workspace__muted">"Comprobando sesión…"</p>
                    }
                }
            >
                <h1 class="workspace__title">"Workspace"</h1>
                <p class="workspace__lead">
                    "Sesión activa en Leptos CSR. El harness completo (ejercicios / Pyodide) sigue en el frontend Qwik hasta el cutover."
                </p>
                <p class="workspace__user">
                    "Conectado como "
                    <strong>
                        {move || {
                            session
                                .user
                                .get()
                                .map(|u| u.email)
                                .unwrap_or_default()
                        }}
                    </strong>
                </p>
                <A href="/" attr:class="cta cta--secondary">
                    "Volver a la portada"
                </A>
            </Show>
        </section>
    }
}
