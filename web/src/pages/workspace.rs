use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::NavigateOptions;

use crate::session::SessionCtx;

#[component]
pub fn WorkspacePage() -> impl IntoView {
    let session = expect_context::<SessionCtx>();
    let navigate = use_navigate();
    let location = use_location();

    // Guard: after bootstrap, no live session → leave /workspace once (replace).
    // Require hydrated `user` (not just a stored token) so orphan JWTs that
    // `/api/me` rejected cannot keep the shell looking authenticated.
    Effect::new(move |_| {
        let ready = session.bootstrapped.get();
        let live = session.user.get().is_some();
        let pending = session.token.get().is_some() && !live;
        let on_workspace = location.pathname.get() == "/workspace";
        if ready && !live && !pending && on_workspace {
            navigate(
                "/login",
                NavigateOptions {
                    replace: true,
                    ..Default::default()
                },
            );
        }
    });

    view! {
        <section class="workspace">
            <Show
                when=move || session.user.get().is_some()
                fallback=move || {
                    view! {
                        <p class="workspace__muted">
                            {move || {
                                if !session.bootstrapped.get() || session.token.get().is_some() {
                                    "Comprobando sesión…"
                                } else {
                                    "Redirigiendo…"
                                }
                            }}
                        </p>
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
