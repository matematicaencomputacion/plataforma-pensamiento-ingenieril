//! Onboarding / coaching shell (Paso 1) — drafting surface only in this slice.

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::NavigateOptions;

use crate::auth::input_value;
use crate::session::SessionCtx;

const COACHING_PROMPTS: &[&str] = &[
    "¿Qué te trae a aprender Python ahora?",
    "¿Con qué urgencia necesitás estos resultados?",
    "¿Qué visión tenés a 5 años?",
    "¿Qué entornos conocés? (Jupyter, Cursor, Positron…)",
];

#[component]
pub fn OnboardingPage() -> impl IntoView {
    let session = expect_context::<SessionCtx>();
    let navigate = use_navigate();
    let location = use_location();

    let notes = RwSignal::new(String::new());

    // Guard: after bootstrap, no live session → leave /onboarding once (replace).
    Effect::new(move |_| {
        let ready = session.bootstrapped.get();
        let live = session.user.get().is_some();
        let pending = session.token.get().is_some() && !live;
        let on_onboarding = location.pathname.get() == "/onboarding";
        if ready && !live && !pending && on_onboarding {
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
        <section class="onboarding">
            <Show
                when=move || session.user.get().is_some()
                fallback=move || {
                    view! {
                        <p class="onboarding__muted">
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
                <p class="onboarding__eyebrow">"Paso 1 · Coaching"</p>
                <header class="onboarding__header">
                    <h1 class="onboarding__title">"Hola, ¿cómo estás?"</h1>
                    <p class="onboarding__lead">
                        "Antes de escribir una sola línea de código, quiero conocerte. Contame con tus palabras: qué te motiva, qué tan urgente es, hacia dónde vas y con qué herramientas ya te sentís a gusto."
                    </p>
                </header>

                <ol class="onboarding__prompts">
                    <li class="onboarding__prompt">{COACHING_PROMPTS[0]}</li>
                    <li class="onboarding__prompt">{COACHING_PROMPTS[1]}</li>
                    <li class="onboarding__prompt">{COACHING_PROMPTS[2]}</li>
                    <li class="onboarding__prompt">{COACHING_PROMPTS[3]}</li>
                </ol>

                <label class="onboarding__label" for="coaching-notes">
                    "Tu respuesta"
                </label>
                <textarea
                    id="coaching-notes"
                    class="onboarding__textarea"
                    rows="5"
                    placeholder="Escribí libremente… Por ejemplo: quiero automatizar reportes en el trabajo; a 5 años me veo liderando análisis de datos."
                    prop:value=move || notes.get()
                    on:input=move |ev| notes.set(input_value(&ev))
                />

                <div class="onboarding__actions">
                    <button
                        class="cta cta--primary"
                        type="button"
                        disabled
                        title="Síntesis de perfil: rebanada siguiente"
                        aria-disabled="true"
                    >
                        "Enviar para análisis"
                    </button>
                    <p class="onboarding__hint" role="status" aria-live="polite">
                        "Podés borronear acá. El análisis con la API llega en la próxima rebanada."
                    </p>
                </div>

                <nav class="onboarding__nav" aria-label="Navegación del onboarding">
                    <A href="/workspace" attr:class="cta cta--secondary">
                        "Volver al workspace"
                    </A>
                </nav>
            </Show>
        </section>
    }
}
