//! Paso 2 — coding micro-exercise with browser Pyodide (ADR 002).

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::NavigateOptions;

use crate::auth::input_value;
use crate::curriculum::{first_coding_step, prompt_to_html};
use crate::pyodide::{
    check_student_code, ensure_engine, format_check_log, format_run_log, run_student_code,
};
use crate::session::SessionCtx;

#[derive(Clone, Copy, PartialEq, Eq)]
enum EngineUi {
    Idle,
    Loading,
    Ready,
    Error,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum CheckUi {
    Idle,
    Pass,
    Fail,
}

#[component]
pub fn LearnPage() -> impl IntoView {
    let session = expect_context::<SessionCtx>();
    let navigate = use_navigate();
    let location = use_location();

    let step = first_coding_step();
    let code = RwSignal::new(step.starter_code.to_string());
    let engine = RwSignal::new(EngineUi::Idle);
    let engine_msg = RwSignal::new(String::from("Motor Python en espera."));
    let busy = RwSignal::new(false);
    let results = RwSignal::new(String::new());
    let check_ui = RwSignal::new(CheckUi::Idle);
    let show_hint = RwSignal::new(false);
    let show_solution = RwSignal::new(false);
    let can_continue = RwSignal::new(false);
    let prompt_html = prompt_to_html(step.prompt_md);

    Effect::new(move |_| {
        let ready = session.bootstrapped.get();
        let live = session.user.get().is_some();
        let pending = session.token.get().is_some() && !live;
        let on_learn = location.pathname.get() == "/learn";
        if ready && !live && !pending && on_learn {
            navigate(
                "/login",
                NavigateOptions {
                    replace: true,
                    ..Default::default()
                },
            );
        }
    });

    Effect::new(move |_| {
        if session.user.get().is_none() {
            return;
        }
        if !matches!(engine.get_untracked(), EngineUi::Idle) {
            return;
        }
        engine.set(EngineUi::Loading);
        engine_msg.set(
            "Preparando motor Python… (primera carga puede tardar unos segundos)".into(),
        );
        leptos::task::spawn_local(async move {
            match ensure_engine().await {
                Ok(state) => {
                    let ui = match state.status.as_str() {
                        "ready" => EngineUi::Ready,
                        "error" => EngineUi::Error,
                        "loading" => EngineUi::Loading,
                        _ => EngineUi::Idle,
                    };
                    engine.set(ui);
                    engine_msg.set(state.message);
                }
                Err(err) => {
                    engine.set(EngineUi::Error);
                    engine_msg.set(err.message);
                }
            }
        });
    });

    let on_run = move |_| {
        if busy.get_untracked() || engine.get_untracked() != EngineUi::Ready {
            return;
        }
        busy.set(true);
        check_ui.set(CheckUi::Idle);
        let source = code.get_untracked();
        leptos::task::spawn_local(async move {
            match run_student_code(source).await {
                Ok(result) => results.set(format_run_log(&result)),
                Err(err) => results.set(format!("=== Run ===\n{}", err.message)),
            }
            busy.set(false);
        });
    };

    let on_validate = move |_| {
        if busy.get_untracked() || engine.get_untracked() != EngineUi::Ready {
            return;
        }
        busy.set(true);
        let source = code.get_untracked();
        let tests = first_coding_step().pytest.to_string();
        leptos::task::spawn_local(async move {
            match check_student_code(source, tests).await {
                Ok(result) => {
                    results.set(format_check_log(&result));
                    if result.passed {
                        check_ui.set(CheckUi::Pass);
                        can_continue.set(true);
                    } else {
                        check_ui.set(CheckUi::Fail);
                        can_continue.set(false);
                    }
                }
                Err(err) => {
                    results.set(format!("=== Validar ===\n{}", err.message));
                    check_ui.set(CheckUi::Fail);
                    can_continue.set(false);
                }
            }
            busy.set(false);
        });
    };

    view! {
        <section class="learn">
            <Show
                when=move || session.user.get().is_some()
                fallback=move || {
                    view! {
                        <p class="learn__muted">
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
                <p class="learn__eyebrow">"Paso 2 · Coding"</p>
                <header class="learn__header">
                    <h1 class="learn__title">{step.title}</h1>
                    <p class="learn__lead">{step.objective}</p>
                    <p
                        class="learn__engine"
                        id="learn-engine-status"
                        role="status"
                        aria-live="polite"
                        data-status=move || match engine.get() {
                            EngineUi::Idle => "idle",
                            EngineUi::Loading => "loading",
                            EngineUi::Ready => "ready",
                            EngineUi::Error => "error",
                        }
                    >
                        {move || engine_msg.get()}
                    </p>
                </header>

                <div class="learn__grid">
                    <section class="learn__theory" aria-label="Teoría y enunciado">
                        <h2 class="learn__section-title">"Enunciado"</h2>
                        <div class="learn__prompt" inner_html=prompt_html.clone()></div>
                        <div class="learn__aids">
                            <button
                                class="learn__linkish"
                                type="button"
                                on:click=move |_| show_hint.update(|v| *v = !*v)
                            >
                                {move || if show_hint.get() { "Ocultar pista" } else { "Mostrar pista" }}
                            </button>
                            <button
                                class="learn__linkish"
                                type="button"
                                on:click=move |_| show_solution.update(|v| *v = !*v)
                            >
                                {move || if show_solution.get() { "Ocultar solución" } else { "Ver solución ejemplo" }}
                            </button>
                        </div>
                        <Show when=move || show_hint.get()>
                            <aside class="learn__callout" aria-live="polite">
                                <strong>"Pista: "</strong>
                                {step.hint}
                            </aside>
                        </Show>
                        <Show when=move || show_solution.get()>
                            <aside class="learn__callout learn__callout--solution" aria-live="polite">
                                <strong>"Solución ejemplo:"</strong>
                                <pre class="learn__pre">{step.solution_example}</pre>
                            </aside>
                        </Show>
                    </section>

                    <section class="learn__editor-pane" aria-label="Editor">
                        <h2 class="learn__section-title">"Editor"</h2>
                        <label class="learn__sr-only" for="learn-editor">
                            "Código Python"
                        </label>
                        <textarea
                            id="learn-editor"
                            class="learn__editor"
                            spellcheck="false"
                            rows="12"
                            prop:value=move || code.get()
                            on:input=move |ev| code.set(input_value(&ev))
                        />
                    </section>

                    <section class="learn__results" aria-label="Resultados">
                        <h2 class="learn__section-title">"Resultados"</h2>
                        <div class="learn__toolbar">
                            <button
                                class="cta cta--secondary"
                                type="button"
                                id="learn-run"
                                prop:disabled=move || {
                                    busy.get() || engine.get() != EngineUi::Ready
                                }
                                on:click=on_run
                            >
                                {move || if busy.get() { "Ejecutando…" } else { "Run" }}
                            </button>
                            <button
                                class="cta cta--primary"
                                type="button"
                                id="learn-validate"
                                prop:disabled=move || {
                                    busy.get() || engine.get() != EngineUi::Ready
                                }
                                on:click=on_validate
                            >
                                {move || if busy.get() { "Validando…" } else { "Validar" }}
                            </button>
                            <Show
                                when=move || can_continue.get()
                                fallback=move || {
                                    view! {
                                        <button
                                            class="cta cta--secondary"
                                            type="button"
                                            id="learn-continue"
                                            disabled
                                            title="Completá Validar con éxito para continuar"
                                        >
                                            "Continuar"
                                        </button>
                                    }
                                }
                            >
                                <A
                                    href="/workspace"
                                    attr:class="cta cta--primary"
                                    attr:id="learn-continue"
                                >
                                    "Continuar"
                                </A>
                            </Show>
                        </div>
                        <pre
                            id="learn-console"
                            class=move || {
                                match check_ui.get() {
                                    CheckUi::Pass => "learn__console learn__console--pass",
                                    CheckUi::Fail => "learn__console learn__console--fail",
                                    CheckUi::Idle => "learn__console",
                                }
                            }
                            aria-live="polite"
                        >
                            {move || {
                                let log = results.get();
                                if log.is_empty() {
                                    "La salida de Run / Validar aparece acá.".into()
                                } else {
                                    log
                                }
                            }}
                        </pre>
                        <Show when=move || can_continue.get()>
                            <p class="learn__ok" role="status">
                                "Checks OK. Podés continuar al workspace o seguir practicando."
                            </p>
                        </Show>
                    </section>
                </div>

                <nav class="learn__nav" aria-label="Navegación del Paso 2">
                    <A href="/onboarding" attr:class="cta cta--secondary">
                        "Volver al coaching"
                    </A>
                    <A href="/workspace" attr:class="cta cta--secondary">
                        "Workspace"
                    </A>
                </nav>
            </Show>
        </section>
    }
}
