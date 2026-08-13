use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::NavigateOptions;

use crate::auth::reset_progress;
use crate::components::{level_completed, ProgressCheck};
use crate::curriculum::{coding_step_by_micro_step, micro_step_unlocked};
use crate::session::SessionCtx;

/// Placeholder rail for the upcoming Python micro-challenges (scaffold only).
/// Rail capacity toward the 316-micro-step roadmap (cells beyond filled curriculum stay locked).
const MICRO_STEP_COUNT: i32 = 316;

#[component]
pub fn WorkspacePage() -> impl IntoView {
    let session = expect_context::<SessionCtx>();
    let navigate = use_navigate();
    let location = use_location();

    let resetting = RwSignal::new(false);
    let reset_note = RwSignal::new(Option::<String>::None);

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

    let on_reset = move |_| {
        if resetting.get_untracked() {
            return;
        }
        resetting.set(true);
        reset_note.set(None);
        leptos::task::spawn_local(async move {
            match reset_progress().await {
                Ok(prog) => {
                    session.set_progress(prog.current_level, prog.completed_levels);
                    reset_note.set(Some(
                        "Avance reiniciado. Los checks verdes se borraron.".into(),
                    ));
                }
                Err(err) => {
                    reset_note.set(Some(err.message));
                }
            }
            resetting.set(false);
        });
    };

    let current_level =
        Signal::derive(move || session.user.get().map(|u| u.current_level).unwrap_or(1));
    let completed_levels = Signal::derive(move || {
        session
            .user
            .get()
            .map(|u| u.completed_levels)
            .unwrap_or_default()
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
                <header class="workspace__header">
                    <h1 class="workspace__title">"Workspace"</h1>
                    <p class="workspace__lead">
                        "Tu espacio operativo: seguí el micro-paso actual y movete entre portada y workspace sin perder la sesión."
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
                </header>

                <div class="workspace__grid">
                    <section class="workspace__panel" aria-labelledby="workspace-level-heading">
                        <div class="workspace__panel-head">
                            <h2 id="workspace-level-heading" class="workspace__panel-title">
                                "Current level micro-step"
                            </h2>
                            <button
                                type="button"
                                class="workspace__reset"
                                id="workspace-reset-progress"
                                prop:disabled=move || resetting.get()
                                attr:aria-busy=move || resetting.get().to_string()
                                on:click=on_reset
                            >
                                {move || {
                                    if resetting.get() {
                                        "Reiniciando…"
                                    } else {
                                        "Volver a empezar"
                                    }
                                }}
                            </button>
                        </div>
                        <Show when=move || reset_note.get().is_some()>
                            <p
                                class="workspace__muted"
                                id="workspace-reset-note"
                                role="status"
                                aria-live="polite"
                            >
                                {move || reset_note.get().unwrap_or_default()}
                            </p>
                        </Show>
                        <MicroStepRail
                            current_level=current_level
                            completed_levels=completed_levels
                        />
                    </section>

                    <section class="workspace__panel" aria-labelledby="workspace-path-heading">
                        <h2 id="workspace-path-heading" class="workspace__panel-title">
                            "Ruta de aprendizaje"
                        </h2>
                        <h3 class="workspace__level-title">
                            "Module 1 — Declarative Foundations"
                        </h3>
                        <p class="workspace__statement">
                            "Variables, tipos y foundations declarativas en Python. Cada número del rail abre su ejercicio en Coding; el editor Pyodide vive en Paso 2."
                        </p>
                        <ul class="workspace__list">
                            <li>"Variables e enteros"</li>
                            <li>"Strings y expresiones"</li>
                            <li>"Tipos y estudio declarativo"</li>
                        </ul>
                    </section>
                </div>

                <nav class="workspace__nav" aria-label="Navegación del workspace">
                    <A href="/onboarding" attr:class="cta cta--primary">
                        "Empezar coaching"
                    </A>
                    <A href="/learn" attr:class="cta cta--secondary">
                        "Paso 2 · Coding"
                    </A>
                    <A href="/" attr:class="cta cta--secondary">
                        "Portada"
                    </A>
                </nav>
            </Show>
        </section>
    }
}

#[component]
fn MicroStepRail(
    current_level: Signal<i32>,
    completed_levels: Signal<Vec<i32>>,
) -> impl IntoView {
    view! {
        <ol
            class="workspace__microsteps"
            id="workspace-microsteps"
            aria-label="Python micro-step challenges 1 to 316"
            data-current-level=move || current_level.get().to_string()
        >
            {(1..=MICRO_STEP_COUNT)
                .map(|n| {
                    let badge_label = format!("Micro-paso {n} superado");
                    let step_href = coding_step_by_micro_step(n)
                        .map(|s| format!("/learn/{}", s.id));
                    view! {
                        <li
                            class=move || {
                                let cur = current_level.get();
                                let earned = completed_levels.get();
                                let mut class = String::from("workspace__microstep");
                                if level_completed(&earned, n) {
                                    class.push_str(" workspace__microstep--done");
                                } else if cur == n {
                                    class.push_str(" workspace__microstep--current");
                                }
                                if micro_step_unlocked(cur, n) && step_href.is_some() {
                                    class.push_str(" workspace__microstep--open");
                                } else if step_href.is_some() {
                                    // Future step: muted, but still a link so authors/students
                                    // can jump to any exercise from the rail.
                                    class.push_str(" workspace__microstep--jumpable");
                                } else {
                                    class.push_str(" workspace__microstep--locked");
                                }
                                class
                            }
                            data-microstep=n.to_string()
                            attr:aria-current=move || {
                                (current_level.get() == n).then_some("step")
                            }
                        >
                            {match step_href.clone() {
                                Some(href) => view! {
                                    <A
                                        href=href.clone()
                                        attr:class="workspace__microstep-link"
                                        attr:id=format!("workspace-microstep-link-{n}")
                                        attr:title=format!("Abrir micro-paso {n}")
                                    >
                                        <span class="workspace__microstep-num">{n}</span>
                                    </A>
                                }
                                .into_any(),
                                None => view! {
                                    <span class="workspace__microstep-num">{n}</span>
                                }
                                .into_any(),
                            }}
                            <Show when=move || level_completed(&completed_levels.get(), n)>
                                <span
                                    class="workspace__microstep-badge"
                                    data-testid="microstep-done-badge"
                                >
                                    <ProgressCheck label=badge_label.clone() />
                                </span>
                            </Show>
                        </li>
                    }
                })
                .collect_view()}
        </ol>
    }
}
