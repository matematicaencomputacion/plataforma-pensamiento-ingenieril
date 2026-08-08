use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn LandingPage() -> impl IntoView {
    view! {
        <section class="hero">
            <p class="hero__eyebrow">"Pensamiento ingenieril"</p>
            <h1 class="hero__title">"IngenierIA"</h1>
            <p class="hero__headline">
                "Aprendé a programar resolviendo problemas reales, paso a paso."
            </p>
            <p class="hero__support">
                "Abstracción → Diseño → Implementación → Pruebas. Tu progreso queda atado a tu cuenta."
            </p>
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
        </section>
    }
}
