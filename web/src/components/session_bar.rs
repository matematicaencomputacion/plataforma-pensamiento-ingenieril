use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_navigate;

use crate::auth::logout_session;
use crate::session::SessionCtx;

#[component]
pub fn SessionBar() -> impl IntoView {
    let session = expect_context::<SessionCtx>();
    let navigate = use_navigate();
    let logging_out = RwSignal::new(false);

    view! {
        <div class="session-bar">
            <Show
                when=move || session.user.get().is_some()
                fallback=move || {
                    view! {
                        <nav class="session-bar__links">
                            <A href="/login" attr:class="session-bar__link">
                                "Iniciar sesión"
                            </A>
                            <A href="/register" attr:class="session-bar__link session-bar__link--emph">
                                "Crear cuenta"
                            </A>
                        </nav>
                    }
                }
            >
                <span class="session-bar__email">
                    {move || {
                        session
                            .user
                            .get()
                            .map(|u| u.email)
                            .unwrap_or_default()
                    }}
                </span>
                <button
                    class="session-bar__logout"
                    type="button"
                    prop:disabled=move || logging_out.get()
                    on:click={
                        let navigate = navigate.clone();
                        move |_| {
                            if logging_out.get_untracked() {
                                return;
                            }
                            logging_out.set(true);
                            let navigate = navigate.clone();
                            leptos::task::spawn_local(async move {
                                logout_session().await;
                                session.clear();
                                logging_out.set(false);
                                navigate("/", Default::default());
                            });
                        }
                    }
                >
                    "Salir"
                </button>
            </Show>
        </div>
    }
}
