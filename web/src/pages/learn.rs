//! Paso 2 — coding micro-exercise with browser Pyodide (ADR 002).
//!
//! Title, objective and prompt come from the embedded curriculum step
//! (`web/src/curriculum.rs`), never from a frozen `GET /api/levels/current`
//! overlay. Learner Python runs only in the browser. Progress is reported
//! with `POST /api/progress/complete` (pass/fail only).
//!
//! Routes: `/learn` (first step) and `/learn/:step` (seed step id).

use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_location, use_navigate, use_params_map};
use leptos_router::NavigateOptions;
use wasm_bindgen::JsCast;

use crate::analytics::{emit_learn_step_enter, emit_learn_validate};
use crate::auth::{complete_progress, input_value};
use crate::components::{
    level_completed, ConceptLensWidget, FabState, PartitionBadges, ProgressCheck, VariableTypeChips,
};
use crate::curriculum::{
    coding_step_or_default, first_coding_step, prompt_to_html_with_flash, DEFAULT_CODING_STEP_ID,
};
use crate::interop::pyodide::{
    check_student_code, ensure_engine, format_check_log, run_stderr_body, run_stdout_body,
    run_student_code, CheckCase,
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
enum ConsoleKind {
    Idle,
    Running,
    RunOk,
    RunErr,
    Validating,
    CheckPass,
    CheckFail,
}

/// Visual FSM for the «Validar solución» CTA.
#[derive(Clone, Copy, PartialEq, Eq)]
enum ValidateBtn {
    /// 1) Ready to validate for the first time.
    Idle,
    /// 2) In-flight checks.
    Validating,
    /// 3) Step already passed; code unchanged since last success.
    Passed,
    /// 4) Passed before, but editor changed → invite re-validation.
    DirtyRevalidate,
}

fn schedule_clear_toast(toast: RwSignal<Option<String>>, ms: i32) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let cb = wasm_bindgen::closure::Closure::once_into_js(move || {
        toast.set(None);
    });
    let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
        cb.as_ref().unchecked_ref(),
        ms,
    );
}

#[component]
pub fn LearnPage() -> impl IntoView {
    let session = expect_context::<SessionCtx>();
    let navigate = use_navigate();
    let location = use_location();
    let params = use_params_map();

    let step_id = Memo::new(move |_| {
        params.with(|p| {
            p.get("step")
                .map(|s| s.to_string())
                .filter(|s| !s.is_empty())
                .unwrap_or_else(|| DEFAULT_CODING_STEP_ID.to_string())
        })
    });

    let has_step_param = Memo::new(move |_| {
        params.with(|p| p.get("step").map(|s| !s.is_empty()).unwrap_or(false))
    });

    let step = Memo::new(move |_| coding_step_or_default(&step_id.get()));

    let code = RwSignal::new(first_coding_step().starter_code.to_string());
    let fab_state = RwSignal::new(FabState::Collapsed);
    let engine = RwSignal::new(EngineUi::Idle);
    let engine_msg = RwSignal::new(String::from("Motor Python en espera."));
    let busy = RwSignal::new(false);
    let console_kind = RwSignal::new(ConsoleKind::Idle);
    let stdout = RwSignal::new(String::new());
    let stderr = RwSignal::new(String::new());
    let check_log = RwSignal::new(String::new());
    let check_cases = RwSignal::new(Vec::<CheckCase>::new());
    let progress_note = RwSignal::new(Option::<String>::None);
    let show_hint = RwSignal::new(false);
    let show_solution = RwSignal::new(false);
    let can_continue = RwSignal::new(false);
    let code_dirty_since_pass = RwSignal::new(false);
    let already_passed_toast = RwSignal::new(Option::<String>::None);
    // Flash `nombre`/`edad` in the enunciado when exploring type chips (str/int).
    let flash_ident = RwSignal::new(Option::<&'static str>::None);

    let validate_btn = Memo::new(move |_| {
        if console_kind.get() == ConsoleKind::Validating {
            return ValidateBtn::Validating;
        }
        match (can_continue.get(), code_dirty_since_pass.get()) {
            (true, true) => ValidateBtn::DirtyRevalidate,
            (true, false) => ValidateBtn::Passed,
            _ => ValidateBtn::Idle,
        }
    });

    // Reset editor / console when the route step changes.
    Effect::new(move |_| {
        let s = step.get();
        code.set(s.starter_code.to_string());
        fab_state.set(FabState::Collapsed);
        busy.set(false);
        console_kind.set(ConsoleKind::Idle);
        stdout.set(String::new());
        stderr.set(String::new());
        check_log.set(String::new());
        check_cases.set(Vec::new());
        progress_note.set(None);
        show_hint.set(false);
        show_solution.set(false);
        can_continue.set(false);
        code_dirty_since_pass.set(false);
        already_passed_toast.set(None);
        flash_ident.set(None);
    });

    Effect::new(move |_| {
        if session.user.get().is_none() {
            return;
        }
        let s = step.get();
        emit_learn_step_enter(s.id, s.micro_step);
    });

    Effect::new(move |_| {
        let ready = session.bootstrapped.get();
        let live = session.user.get().is_some();
        let pending = session.token.get().is_some() && !live;
        let on_learn = location.pathname.get().starts_with("/learn");
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
        can_continue.set(false);
        progress_note.set(None);
        console_kind.set(ConsoleKind::Running);
        stdout.set(String::new());
        stderr.set(String::new());
        check_log.set(String::new());
        check_cases.set(Vec::new());
        let source = code.get_untracked();
        leptos::task::spawn_local(async move {
            match run_student_code(source).await {
                Ok(result) => {
                    stdout.set(run_stdout_body(&result));
                    stderr.set(run_stderr_body(&result));
                    console_kind.set(if result.ok {
                        ConsoleKind::RunOk
                    } else {
                        ConsoleKind::RunErr
                    });
                }
                Err(err) => {
                    stdout.set(String::new());
                    stderr.set(err.message);
                    console_kind.set(ConsoleKind::RunErr);
                }
            }
            busy.set(false);
        });
    };

    let on_validate = move |_| {
        if busy.get_untracked() || engine.get_untracked() != EngineUi::Ready {
            return;
        }
        // Already passed + code unchanged → toast only (no re-run).
        if can_continue.get_untracked() && !code_dirty_since_pass.get_untracked() {
            already_passed_toast.set(Some(
                "Ya pasaste esta prueba. Si cambiás el código, podés volver a validar.".into(),
            ));
            schedule_clear_toast(already_passed_toast, 5000);
            return;
        }
        busy.set(true);
        progress_note.set(None);
        already_passed_toast.set(None);
        console_kind.set(ConsoleKind::Validating);
        stdout.set(String::new());
        stderr.set(String::new());
        check_log.set(String::new());
        check_cases.set(Vec::new());
        let source = code.get_untracked();
        let current = step.get_untracked();
        let tests = current.pytest.to_string();
        let step_key = current.id.to_string();
        // Progress cursor uses the rail micro-step index (not GET /api/levels/current id).
        let level_id = current.micro_step;
        leptos::task::spawn_local(async move {
            match check_student_code(source, tests).await {
                Ok(result) => {
                    check_log.set(format_check_log(&result));
                    check_cases.set(result.cases.clone());
                    if result.passed {
                        console_kind.set(ConsoleKind::CheckPass);
                        can_continue.set(true);
                        code_dirty_since_pass.set(false);
                        emit_learn_validate(&step_key, level_id, true);
                        match complete_progress(level_id, step_key, true).await {
                            Ok(prog) => {
                                session.set_progress(
                                    prog.current_level,
                                    prog.completed_levels.clone(),
                                );
                                if prog.advanced {
                                    progress_note.set(Some(format!(
                                        "Progreso guardado · nivel actual {}",
                                        prog.current_level
                                    )));
                                } else {
                                    progress_note.set(Some("Progreso guardado.".into()));
                                }
                            }
                            Err(err) => {
                                progress_note.set(Some(format!(
                                    "Checks OK en el browser, pero no se pudo guardar el avance: {}",
                                    err.message
                                )));
                            }
                        }
                    } else {
                        console_kind.set(ConsoleKind::CheckFail);
                        can_continue.set(false);
                        code_dirty_since_pass.set(false);
                        emit_learn_validate(&step_key, level_id, false);
                        let _ = complete_progress(level_id, step_key, false).await;
                    }
                }
                Err(err) => {
                    check_log.set(format!("=== Validar ===\n{}", err.message));
                    check_cases.set(vec![CheckCase {
                        name: "(runtime)".into(),
                        passed: false,
                        message: err.message.clone(),
                    }]);
                    console_kind.set(ConsoleKind::CheckFail);
                    can_continue.set(false);
                    code_dirty_since_pass.set(false);
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
                <p class="learn__eyebrow">
                    {move || format!("Paso 2 · Coding · {}", step.get().id)}
                </p>
                <header class="learn__header">
                    <h1 class="learn__title">{move || step.get().title.to_string()}</h1>
                    <p class="learn__lead">{move || step.get().objective.to_string()}</p>
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

                <div
                    class=move || {
                        if fab_state.get() == FabState::Docked {
                            "learn__workspace learn__workspace--with-drawer"
                        } else {
                            "learn__workspace"
                        }
                    }
                >
                    <ConceptLensWidget
                        visible=Signal::derive(move || has_step_param.get())
                        state=fab_state
                    />
                    <div
                        class=move || {
                            if fab_state.get() == FabState::Docked {
                                "learn__grid learn__grid--with-drawer"
                            } else {
                                "learn__grid"
                            }
                        }
                    >
                    <section class="learn__theory" aria-label="Teoría y enunciado">
                        <h2 class="learn__section-title">"Enunciado"</h2>
                        <PartitionBadges micro_step=Signal::derive(move || step.get().micro_step) />
                        <Show when=move || step.get().show_type_chips>
                            <VariableTypeChips flash_ident=flash_ident />
                        </Show>
                        <div
                            class="learn__prompt"
                            inner_html=move || {
                                prompt_to_html_with_flash(
                                    step.get().prompt_md,
                                    flash_ident.get(),
                                )
                            }
                        ></div>

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
                                {move || step.get().hint.to_string()}
                            </aside>
                        </Show>
                        <Show when=move || show_solution.get()>
                            <aside class="learn__callout learn__callout--solution" aria-live="polite">
                                <strong>"Solución ejemplo:"</strong>
                                <pre class="learn__pre">{move || step.get().solution_example.to_string()}</pre>
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
                            on:input=move |ev| {
                                code.set(input_value(&ev));
                                if can_continue.get_untracked() {
                                    code_dirty_since_pass.set(true);
                                }
                            }
                        />
                    </section>

                    <section class="learn__results" aria-label="Resultados">
                        <h2 class="learn__section-title">"Consola"</h2>
                        <div class="learn__toolbar">
                            <button
                                class="cta cta--secondary"
                                type="button"
                                id="learn-run"
                                prop:disabled=move || {
                                    busy.get() || engine.get() != EngineUi::Ready
                                }
                                attr:aria-busy=move || busy.get().to_string()
                                on:click=on_run
                            >
                                {move || {
                                    if console_kind.get() == ConsoleKind::Running {
                                        "Ejecutando Python…"
                                    } else {
                                        "Ejecutar código"
                                    }
                                }}
                            </button>
                            <button
                                class=move || match validate_btn.get() {
                                    ValidateBtn::Idle => "cta cta--primary",
                                    ValidateBtn::Validating => "cta cta--primary cta--busy",
                                    ValidateBtn::Passed => "cta cta--passed",
                                    ValidateBtn::DirtyRevalidate => "cta cta--revalidate",
                                }
                                type="button"
                                id="learn-validate"
                                data-validate-state=move || match validate_btn.get() {
                                    ValidateBtn::Idle => "idle",
                                    ValidateBtn::Validating => "validating",
                                    ValidateBtn::Passed => "passed",
                                    ValidateBtn::DirtyRevalidate => "revalidate",
                                }
                                prop:disabled=move || {
                                    busy.get() || engine.get() != EngineUi::Ready
                                }
                                on:click=on_validate
                            >
                                {move || match validate_btn.get() {
                                    ValidateBtn::Validating => "Validando…",
                                    ValidateBtn::Passed => "Prueba superada",
                                    ValidateBtn::DirtyRevalidate => "Volver a validar",
                                    ValidateBtn::Idle => "Validar solución",
                                }}
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
                                {move || {
                                    let next = step.get().next;
                                    let href = match next {
                                        Some(id) => format!("/learn/{id}"),
                                        None => "/workspace".to_string(),
                                    };
                                    let label = if next.is_some() {
                                        "Continuar al siguiente"
                                    } else {
                                        "Ir al workspace"
                                    };
                                    view! {
                                        <A
                                            href=href
                                            attr:class="cta cta--primary"
                                            attr:id="learn-continue"
                                        >
                                            {label}
                                        </A>
                                    }
                                }}
                            </Show>
                        </div>

                        <Show when=move || {
                            can_continue.get()
                                || level_completed(
                                    &session
                                        .user
                                        .get()
                                        .map(|u| u.completed_levels)
                                        .unwrap_or_default(),
                                    step.get().micro_step,
                                )
                        }>
                            <div class="learn__progress-mark" id="learn-progress-check">
                                <ProgressCheck label="Ejercicio superado" />
                            </div>
                        </Show>

                        <Show when=move || console_kind.get() == ConsoleKind::Running>
                            <p class="learn__busy" id="learn-busy" role="status" aria-live="polite">
                                "Ejecutando Python en tu navegador…"
                            </p>
                        </Show>
                        <Show when=move || console_kind.get() == ConsoleKind::Validating>
                            <p class="learn__busy" role="status" aria-live="polite">
                                "Validando checks del micro-reto en tu navegador…"
                            </p>
                        </Show>

                        <Show when=move || !check_cases.get().is_empty()>
                            <ul
                                id="learn-test-cases"
                                class="learn__cases"
                                aria-label="Resultado de test cases"
                            >
                                <For
                                    each=move || check_cases.get()
                                    key=|c| format!("{}:{}", c.name, c.passed)
                                    children=move |c| {
                                        let item_class = if c.passed {
                                            "learn__case learn__case--pass"
                                        } else {
                                            "learn__case learn__case--fail"
                                        };
                                        let status = if c.passed { "pass" } else { "fail" };
                                        let msg = c.message.clone();
                                        let show_msg = !c.message.is_empty() && !c.passed;
                                        view! {
                                            <li class=item_class data-status=status>
                                                <span class="learn__case-name">{c.name.clone()}</span>
                                                <span class="learn__case-mark">
                                                    {if c.passed { "pasa" } else { "falla" }}
                                                </span>
                                                <Show when=move || show_msg>
                                                    <pre class="learn__case-msg">{msg.clone()}</pre>
                                                </Show>
                                            </li>
                                        }
                                    }
                                />
                            </ul>
                        </Show>

                        <div
                            id="learn-console"
                            class=move || match console_kind.get() {
                                ConsoleKind::RunOk | ConsoleKind::CheckPass => {
                                    "learn__console learn__console--pass"
                                }
                                ConsoleKind::RunErr | ConsoleKind::CheckFail => {
                                    "learn__console learn__console--fail"
                                }
                                ConsoleKind::Running | ConsoleKind::Validating => {
                                    "learn__console learn__console--busy"
                                }
                                ConsoleKind::Idle => "learn__console",
                            }
                            aria-live="polite"
                        >
                            <Show
                                when=move || {
                                    matches!(
                                        console_kind.get(),
                                        ConsoleKind::CheckPass | ConsoleKind::CheckFail
                                    )
                                }
                                fallback=move || {
                                    view! {
                                        <pre id="learn-stdout" class="learn__stdout">
                                            {move || {
                                                let out = stdout.get();
                                                if out.is_empty()
                                                    && matches!(
                                                        console_kind.get(),
                                                        ConsoleKind::Idle
                                                            | ConsoleKind::Running
                                                            | ConsoleKind::Validating
                                                    )
                                                {
                                                    "La salida de Ejecutar / Validar aparece acá.".into()
                                                } else if out.is_empty()
                                                    && console_kind.get() == ConsoleKind::RunOk
                                                {
                                                    "(sin salida — usá print(...) para ver texto aquí)"
                                                        .into()
                                                } else {
                                                    out
                                                }
                                            }}
                                        </pre>
                                        <Show when=move || !stderr.get().is_empty()>
                                            <pre id="learn-stderr" class="learn__stderr">
                                                {move || stderr.get()}
                                            </pre>
                                        </Show>
                                    }
                                }
                            >
                                <pre id="learn-check-log" class="learn__stdout">
                                    {move || check_log.get()}
                                </pre>
                            </Show>
                        </div>

                        <Show when=move || can_continue.get()>
                            <p
                                id="learn-success-banner"
                                class="learn__ok learn__ok--banner"
                                role="status"
                            >
                                "¡Ejercicio completado con éxito!"
                            </p>
                        </Show>
                        <Show when=move || already_passed_toast.get().is_some()>
                            <p
                                id="learn-already-passed-toast"
                                class="learn__toast"
                                role="status"
                                aria-live="polite"
                            >
                                {move || already_passed_toast.get().unwrap_or_default()}
                            </p>
                        </Show>
                        <Show when=move || progress_note.get().is_some()>
                            <p class="learn__muted" role="status" id="learn-progress-note">
                                {move || progress_note.get().unwrap_or_default()}
                            </p>
                        </Show>
                    </section>
                </div>
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
