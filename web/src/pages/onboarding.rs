//! Onboarding / coaching (Paso 1) — draft → synthesize → review → save.

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::NavigateOptions;
use wasm_bindgen::closure::Closure;

use crate::api::{ProfileSynthesis, UserProfile, MIN_LEARNER_NOTES_RUNES};
use crate::auth::{
    fetch_user_profile, input_value, put_user_profile, synthesize_learner_profile,
};
use crate::interop::speech;
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
    Saving,
    Saved,
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
    let hydrate_loading = RwSignal::new(false);
    let hydrate_done = RwSignal::new(false);
    let voice_supported = RwSignal::new(false);
    let listening = RwSignal::new(false);

    Effect::new(move |_| {
        voice_supported.set(speech::is_supported());
    });

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

    // Rehydrate saved profile once the session is live.
    Effect::new(move |_| {
        if session.user.get().is_none() {
            return;
        }
        if hydrate_done.get_untracked() || hydrate_loading.get_untracked() {
            return;
        }
        hydrate_loading.set(true);
        leptos::task::spawn_local(async move {
            match fetch_user_profile().await {
                Ok(profile) if !profile.is_empty() => {
                    let syn = profile.to_synthesis();
                    purpose.set(syn.purpose);
                    urgency.set(syn.urgency);
                    vision.set(syn.vision);
                    stack.set(syn.stack);
                    error.set(String::new());
                    phase.set(CoachingPhase::Saved);
                }
                Ok(_) => {
                    // Empty profile — stay drafting.
                }
                Err(err) => {
                    // Soft-fail: learner can still draft/synthesize.
                    if err.status != Some(401) {
                        error.set(err.message);
                    }
                }
            }
            hydrate_loading.set(false);
            hydrate_done.set(true);
        });
    });

    let notes_ready = move || notes.get().trim().chars().count() >= MIN_LEARNER_NOTES_RUNES;

    let apply_synthesis = move |syn: ProfileSynthesis| {
        purpose.set(syn.purpose);
        urgency.set(syn.urgency);
        vision.set(syn.vision);
        stack.set(syn.stack);
    };

    let current_profile = move || {
        UserProfile::from_synthesis(&ProfileSynthesis {
            purpose: purpose.get_untracked(),
            urgency: urgency.get_untracked(),
            vision: vision.get_untracked(),
            stack: stack.get_untracked(),
        })
    };

    let stop_voice = move || {
        speech::stop();
        listening.set(false);
    };

    let on_mic = move |_| {
        if phase.get_untracked() != CoachingPhase::Drafting {
            return;
        }
        error.set(String::new());
        if listening.get_untracked() {
            stop_voice();
            return;
        }
        if !speech::is_supported() {
            error.set("Tu navegador no soporta dictado por voz.".into());
            return;
        }

        let base = notes.get_untracked();
        let on_update = Closure::wrap(Box::new(move |text: String| {
            notes.set(text);
        }) as Box<dyn FnMut(String)>);
        let on_error = Closure::wrap(Box::new(move |msg: String| {
            if !msg.trim().is_empty() {
                error.set(msg);
            }
            listening.set(false);
        }) as Box<dyn FnMut(String)>);
        let on_end = Closure::wrap(Box::new(move || {
            listening.set(false);
        }) as Box<dyn FnMut()>);

        match speech::start(&base, "es-AR", &on_update, &on_error, &on_end) {
            Ok(()) => {
                listening.set(true);
                // Keep handlers alive for the recognition session lifetime.
                on_update.forget();
                on_error.forget();
                on_end.forget();
            }
            Err(err) => {
                error.set(err.message);
                listening.set(false);
            }
        }
    };

    let on_analyze = move |_| {
        if phase.get_untracked() == CoachingPhase::Analyzing {
            return;
        }
        stop_voice();
        error.set(String::new());
        let raw = notes.get_untracked();
        if raw.trim().chars().count() < MIN_LEARNER_NOTES_RUNES {
            error.set("El relato es demasiado corto para analizar.".into());
            return;
        }
        phase.set(CoachingPhase::Analyzing);
        leptos::task::spawn_local(async move {
            let outcome = synthesize_learner_profile(raw, SOURCE_STEP_ID.to_string()).await;
            match outcome {
                Ok(syn) if syn.is_empty() => {
                    error.set(
                        "La IA no pudo completar el perfil con ese relato. Ampliá tu respuesta o editá los campos a mano."
                            .into(),
                    );
                    phase.set(CoachingPhase::Drafting);
                }
                Ok(syn) => {
                    apply_synthesis(syn);
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

    let on_edit_profile = move |_| {
        error.set(String::new());
        phase.set(CoachingPhase::Reviewing);
    };

    let on_save = move |_| {
        if matches!(
            phase.get_untracked(),
            CoachingPhase::Saving | CoachingPhase::Analyzing
        ) {
            return;
        }
        error.set(String::new());
        let payload = current_profile();
        if payload.is_empty() {
            error.set(
                "El perfil está vacío. Completá al menos un campo antes de guardar.".into(),
            );
            return;
        }
        phase.set(CoachingPhase::Saving);
        leptos::task::spawn_local(async move {
            match put_user_profile(payload).await {
                Ok(saved) => {
                    apply_synthesis(saved.to_synthesis());
                    error.set(String::new());
                    phase.set(CoachingPhase::Saved);
                }
                Err(err) => {
                    error.set(err.message);
                    phase.set(CoachingPhase::Reviewing);
                }
            }
        });
    };

    let show_profile = move || {
        matches!(
            phase.get(),
            CoachingPhase::Reviewing | CoachingPhase::Saving | CoachingPhase::Saved
        )
    };

    let profile_editable = move || phase.get() == CoachingPhase::Reviewing;

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
                <Show when=move || hydrate_loading.get()>
                    <p class="onboarding__muted" role="status" aria-live="polite">
                        "Cargando perfil guardado…"
                    </p>
                </Show>

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

                <div class="onboarding__notes-head">
                    <label class="onboarding__label" for="coaching-notes">
                        "Tu respuesta"
                    </label>
                    <Show when=move || voice_supported.get() && phase.get() == CoachingPhase::Drafting>
                        <button
                            class=move || {
                                if listening.get() {
                                    "onboarding__mic onboarding__mic--live"
                                } else {
                                    "onboarding__mic"
                                }
                            }
                            type="button"
                            id="coaching-mic"
                            title=move || {
                                if listening.get() {
                                    "Detener dictado"
                                } else {
                                    "Dictar por voz"
                                }
                            }
                            attr:aria-pressed=move || listening.get().to_string()
                            attr:aria-label=move || {
                                if listening.get() {
                                    "Detener dictado por voz"
                                } else {
                                    "Dictar respuesta por voz"
                                }
                            }
                            on:click=on_mic
                        >
                            {move || if listening.get() { "⏹ Escuchando" } else { "🎤 Dictar" }}
                        </button>
                    </Show>
                </div>
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
                <Show when=move || listening.get()>
                    <p class="onboarding__hint" role="status" aria-live="polite" id="coaching-listening">
                        "Escuchando… hablá con naturalidad; el texto se agrega a tu respuesta."
                    </p>
                </Show>

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
                                    "Analizando tu respuesta…"
                                } else {
                                    "Analizar mi respuesta con IA"
                                }
                            }}
                        </button>
                        <p class="onboarding__hint" role="status" aria-live="polite" id="coaching-analyze-hint">
                            {move || {
                                if phase.get() == CoachingPhase::Analyzing {
                                    "Analizando tu respuesta… estamos resumiendo propósito, urgencia, visión y stack."
                                } else if notes_ready() {
                                    "Al analizar, IngenierIA completa los 4 campos de abajo para que los revises."
                                } else {
                                    "Escribí un poco más (al menos unas pocas frases) para poder analizar."
                                }
                            }}
                        </p>
                    </Show>

                    <Show when=move || {
                        matches!(
                            phase.get(),
                            CoachingPhase::Reviewing | CoachingPhase::Saving
                        )
                    }>
                        <button
                            class="cta cta--secondary"
                            type="button"
                            prop:disabled=move || phase.get() == CoachingPhase::Saving
                            on:click=on_back_to_draft
                        >
                            "Editar relato"
                        </button>
                        <button
                            class="cta cta--primary"
                            type="button"
                            id="coaching-save"
                            prop:disabled=move || phase.get() == CoachingPhase::Saving
                            attr:aria-busy=move || {
                                (phase.get() == CoachingPhase::Saving).to_string()
                            }
                            on:click=on_save
                        >
                            {move || {
                                if phase.get() == CoachingPhase::Saving {
                                    "Guardando perfil…"
                                } else {
                                    "Guardar perfil"
                                }
                            }}
                        </button>
                        <p class="onboarding__hint" role="status" aria-live="polite">
                            {move || {
                                if phase.get() == CoachingPhase::Saving {
                                    "Persistiendo tu perfil en la cuenta…"
                                } else {
                                    "Revisá y editá los campos, después guardalos en tu cuenta."
                                }
                            }}
                        </p>
                    </Show>

                    <Show when=move || phase.get() == CoachingPhase::Saved>
                        <p
                            class="onboarding__saved"
                            id="coaching-saved-status"
                            role="status"
                            aria-live="polite"
                        >
                            "Perfil guardado"
                        </p>
                        <button
                            class="cta cta--secondary"
                            type="button"
                            on:click=on_edit_profile
                        >
                            "Editar perfil"
                        </button>
                        <A
                            href="/learn"
                            attr:class="cta cta--primary"
                            attr:id="coaching-continue"
                        >
                            "Continuar al Paso 2"
                        </A>
                        <A
                            href="/workspace"
                            attr:class="cta cta--secondary"
                            attr:id="coaching-workspace"
                        >
                            "Ir al workspace"
                        </A>
                        <p class="onboarding__hint" role="status">
                            "Paso 2 abre el editor Python en el navegador (Pyodide)."
                        </p>
                    </Show>
                </div>

                <Show when=show_profile>
                    <section
                        class="onboarding__profile"
                        aria-label="Resumen de perfil"
                        aria-live="polite"
                    >
                        <p class="onboarding__profile-eyebrow" id="coaching-profile-phase">
                            {move || match phase.get() {
                                CoachingPhase::Saved => "Perfil · guardado / actualizado",
                                CoachingPhase::Saving => "Perfil · guardando",
                                _ => "Perfil · revisión / Lo que estamos escuchando",
                            }}
                        </p>
                        <h2 class="onboarding__profile-title">"Lo que estamos escuchando"</h2>
                        <p class="onboarding__muted">
                            {move || {
                                if phase.get() == CoachingPhase::Saved {
                                    "Perfil listo. Podés ajustar campos y guardar de nuevo, o continuar al Paso 2."
                                } else {
                                    "Revisá lo que dedujimos. Si no refleja tu historia, editá los campos o volvé al relato."
                                }
                            }}
                        </p>

                        <label class="onboarding__label" for="profile-purpose">
                            "Propósito"
                        </label>
                        <textarea
                            id="profile-purpose"
                            class="onboarding__field"
                            rows="2"
                            prop:value=move || purpose.get()
                            prop:readOnly=move || !profile_editable()
                            on:input=move |ev| {
                                if profile_editable() {
                                    purpose.set(input_value(&ev));
                                }
                            }
                        />

                        <label class="onboarding__label" for="profile-urgency">
                            "Urgencia"
                        </label>
                        <textarea
                            id="profile-urgency"
                            class="onboarding__field"
                            rows="2"
                            prop:value=move || urgency.get()
                            prop:readOnly=move || !profile_editable()
                            on:input=move |ev| {
                                if profile_editable() {
                                    urgency.set(input_value(&ev));
                                }
                            }
                        />

                        <label class="onboarding__label" for="profile-vision">
                            "Visión a 5 años"
                        </label>
                        <textarea
                            id="profile-vision"
                            class="onboarding__field"
                            rows="2"
                            prop:value=move || vision.get()
                            prop:readOnly=move || !profile_editable()
                            on:input=move |ev| {
                                if profile_editable() {
                                    vision.set(input_value(&ev));
                                }
                            }
                        />

                        <label class="onboarding__label" for="profile-stack">
                            "Stack previo"
                        </label>
                        <textarea
                            id="profile-stack"
                            class="onboarding__field"
                            rows="2"
                            prop:value=move || stack.get()
                            prop:readOnly=move || !profile_editable()
                            on:input=move |ev| {
                                if profile_editable() {
                                    stack.set(input_value(&ev));
                                }
                            }
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
