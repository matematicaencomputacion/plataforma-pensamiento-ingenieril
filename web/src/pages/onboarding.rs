//! Onboarding / coaching (Paso 1) — draft → synthesize → review.

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::NavigateOptions;

use crate::api::{ProfileSynthesis, MIN_LEARNER_NOTES_RUNES};
use crate::auth::{input_value, synthesize_learner_profile};
use crate::session::SessionCtx;

const COACHING_PROMPTS: &[&str] = &[
    "¿Qué te trae a aprender Python ahora?",
    "¿Con qué urgencia necesitás estos resultados?",
    "¿Qué visión tenés a 5 años?",
    "¿Qué entornos conocés? (Jupyter, Cursor, Positron…)",
];

const SOURCE_STEP_ID: &str = "leptos-onboarding-v1";

#[derive(Clone, Copy, PartialEq, Eq)]
enum CoachingPhase {
    Drafting,
    Analyzing,
    Reviewing,
}

#[component]
pub fn OnboardingPage() -> impl IntoView {
    let session = expect_context::<SessionCtx>();
    let navigate = use_navigate();
    let location = use_location();

    let notes = RwSignal::new(String::new());
    let phase = RwSignal::new(CoachingPhase::Drafting);
    let error = RwSignal::new(String::new());
    let purpose = RwSignal::new(String::new());
    let urgency = RwSignal::new(String::new());
    let vision = RwSignal::new(String::new());
    let stack = RwSignal::new(String::new());

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

    let notes_ready = move || {
        notes.get().trim().chars().count() >= MIN_LEARNER_NOTES_RUNES
    };

    let on_analyze = move |_| {
        if phase.get_untracked() == CoachingPhase::Analyzing {
            return;
        }
        error.set(String::new());
        let raw = notes.get_untracked();
        if raw.trim().chars().count() < MIN_LEARNER_NOTES_RUNES {
            error.set("El relato es demasiado corto para analizar.".into());
            return;
        }
        phase.set(CoachingPhase::Analyzing);
        leptos::task::spawn_local(async move {
            let outcome =
                synthesize_learner_profile(raw, SOURCE_STEP_ID.to_string()).await;
            match outcome {
                Ok(ProfileSynthesis {
                    purpose: p,
                    urgency: u,
                    vision: v,
                    stack: s,
                }) => {
                    purpose.set(p);
                    urgency.set(u);
                    vision.set(v);
                    stack.set(s);
                    error.set(String::new());
                    phase.set(CoachingPhase::Reviewing);
                }
                Err(err) => {
                    error.set(err.message);
                    phase.set(CoachingPhase::Drafting);
                }
            }
        });
    };

    let on_back_to_draft = move |_| {
        error.set(String::new());
        phase.set(CoachingPhase::Drafting);
    };

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
                    prop:readOnly=move || phase.get() != CoachingPhase::Drafting
                    on:input=move |ev| {
                        if phase.get_untracked() == CoachingPhase::Drafting {
                            notes.set(input_value(&ev));
                        }
                    }
                />

                <Show when=move || !error.get().is_empty()>
                    <p class="onboarding__error" role="alert" aria-live="polite">
                        {move || error.get()}
                    </p>
                </Show>

                <div class="onboarding__actions">
                    <Show when=move || {
                        matches!(
                            phase.get(),
                            CoachingPhase::Drafting | CoachingPhase::Analyzing
                        )
                    }>
                        <button
                            class="cta cta--primary"
                            type="button"
                            id="coaching-analyze"
                            prop:disabled=move || {
                                phase.get() == CoachingPhase::Analyzing || !notes_ready()
                            }
                            attr:aria-busy=move || {
                                (phase.get() == CoachingPhase::Analyzing).to_string()
                            }
                            on:click=on_analyze
                        >
                            {move || {
                                if phase.get() == CoachingPhase::Analyzing {
                                    "Analizando…"
                                } else {
                                    "Enviar para análisis"
                                }
                            }}
                        </button>
                        <p class="onboarding__hint" role="status" aria-live="polite">
                            {move || {
                                if phase.get() == CoachingPhase::Analyzing {
                                    "Estamos escuchando tu relato…"
                                } else if notes_ready() {
                                    "Cuando envíes, IngenierIA sintetizará propósito, urgencia, visión y stack."
                                } else {
                                    "Escribí un poco más (al menos unas pocas frases) para poder analizar."
                                }
                            }}
                        </p>
                    </Show>

                    <Show when=move || phase.get() == CoachingPhase::Reviewing>
                        <button
                            class="cta cta--secondary"
                            type="button"
                            on:click=on_back_to_draft
                        >
                            "Editar relato"
                        </button>
                        <button
                            class="cta cta--primary"
                            type="button"
                            disabled
                            title="Persistencia de perfil: rebanada siguiente"
                            aria-disabled="true"
                        >
                            "Guardar perfil"
                        </button>
                        <p class="onboarding__hint" role="status" aria-live="polite">
                            "Revisá y editá los campos. Guardar en tu cuenta llega en la próxima rebanada."
                        </p>
                    </Show>
                </div>

                <Show when=move || phase.get() == CoachingPhase::Reviewing>
                    <section
                        class="onboarding__profile"
                        aria-label="Resumen de perfil"
                        aria-live="polite"
                    >
                        <p class="onboarding__profile-eyebrow">"Perfil · revisión"</p>
                        <h2 class="onboarding__profile-title">"Lo que estamos escuchando"</h2>
                        <p class="onboarding__muted">
                            "Revisá lo que dedujimos. Si no refleja tu historia, editá los campos o volvé al relato."
                        </p>

                        <label class="onboarding__label" for="profile-purpose">
                            "Propósito"
                        </label>
                        <textarea
                            id="profile-purpose"
                            class="onboarding__field"
                            rows="2"
                            prop:value=move || purpose.get()
                            on:input=move |ev| purpose.set(input_value(&ev))
                        />

                        <label class="onboarding__label" for="profile-urgency">
                            "Urgencia"
                        </label>
                        <textarea
                            id="profile-urgency"
                            class="onboarding__field"
                            rows="2"
                            prop:value=move || urgency.get()
                            on:input=move |ev| urgency.set(input_value(&ev))
                        />

                        <label class="onboarding__label" for="profile-vision">
                            "Visión a 5 años"
                        </label>
                        <textarea
                            id="profile-vision"
                            class="onboarding__field"
                            rows="2"
                            prop:value=move || vision.get()
                            on:input=move |ev| vision.set(input_value(&ev))
                        />

                        <label class="onboarding__label" for="profile-stack">
                            "Stack previo"
                        </label>
                        <textarea
                            id="profile-stack"
                            class="onboarding__field"
                            rows="2"
                            prop:value=move || stack.get()
                            on:input=move |ev| stack.set(input_value(&ev))
                        />
                    </section>
                </Show>

                <nav class="onboarding__nav" aria-label="Navegación del onboarding">
                    <A href="/workspace" attr:class="cta cta--secondary">
                        "Volver al workspace"
                    </A>
                </nav>
            </Show>
        </section>
    }
}
