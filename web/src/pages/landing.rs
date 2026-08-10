use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::BrandMark;
use crate::session::SessionCtx;

#[component]
pub fn LandingPage() -> impl IntoView {
    let session = expect_context::<SessionCtx>();

    view! {
        <section class="hero">
            <p class="hero__eyebrow">"Pensamiento ingenieril"</p>
            <BrandMark class="hero__title" heading=true />
            <p class="hero__headline">
                "Aprendé a programar resolviendo problemas reales, paso a paso."
            </p>
            <p class="hero__support">
                "Abstracción → Diseño → Implementación → Pruebas. Tu progreso queda atado a tu cuenta."
            </p>
            <Show
                when=move || session.user.get().is_some()
                fallback=move || {
                    view! {
                        <div class="hero__ctas">
                            <A href="/register" attr:class="cta cta--primary">
                                "Crear cuenta"
                            </A>
                            <A href="/login" attr:class="cta cta--secondary">
                                "Iniciar sesión"
                            </A>
                        </div>
                        <p class="hero__guest">
                            <A href="/workspace" attr:class="hero__guest-link">
                                "Entrar al workspace (requiere sesión)"
                            </A>
                        </p>
                    }
                }
            >
                <div class="hero__ctas">
                    <A href="/workspace" attr:class="cta cta--primary">
                        "Ir al workspace"
                    </A>
                </div>
                <p class="hero__guest">
                    {move || {
                        session
                            .user
                            .get()
                            .map(|u| format!("Sesión activa como {}", u.email))
                            .unwrap_or_default()
                    }}
                </p>
            </Show>
        </section>
    }
}
