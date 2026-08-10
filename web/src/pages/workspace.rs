use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_location, use_navigate};
use leptos_router::NavigateOptions;

use crate::api::Level;
use crate::auth::fetch_current_level;
use crate::session::SessionCtx;

#[component]
pub fn WorkspacePage() -> impl IntoView {
    let session = expect_context::<SessionCtx>();
    let navigate = use_navigate();
    let location = use_location();

    let level = RwSignal::new(Option::<Level>::None);
    let level_error = RwSignal::new(Option::<String>::None);
    let level_loading = RwSignal::new(false);

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

    // Load the operational "nivel actual" once the session is live.
    Effect::new(move |_| {
        if session.user.get().is_none() {
            return;
        }
        if level.get_untracked().is_some() || level_loading.get_untracked() {
            return;
        }
        level_loading.set(true);
        level_error.set(None);
        leptos::task::spawn_local(async move {
            match fetch_current_level().await {
                Ok(current) => {
                    level.set(Some(current));
                    level_error.set(None);
                }
                Err(err) => {
                    level_error.set(Some(err.message));
                }
            }
            level_loading.set(false);
        });
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
                        "Tu espacio operativo: seguí el nivel actual y movete entre portada y workspace sin perder la sesión."
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
                        <h2 id="workspace-level-heading" class="workspace__panel-title">
                            "Nivel actual"
                        </h2>
                        <LevelPanel
                            loading=level_loading
                            error=level_error
                            level=level
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
                            "Variables, tipos y foundations declarativas en Python. El enunciado vive en «Nivel actual»; el editor Pyodide del harness sigue el cutover desde el frontend legacy."
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
fn LevelPanel(
    loading: RwSignal<bool>,
    error: RwSignal<Option<String>>,
    level: RwSignal<Option<Level>>,
) -> impl IntoView {
    view! {
        <Show when=move || loading.get()>
            <p class="workspace__muted">"Cargando nivel…"</p>
        </Show>
        <Show when=move || !loading.get() && level.get().is_some()>
            {move || {
                level.get().map(|lvl| {
                    let track = track_label(&lvl.track_type);
                    view! {
                        <p class="workspace__meta">
                            {format!("#{id} · {track}", id = lvl.id, track = track)}
                        </p>
                        <h3 class="workspace__level-title">{lvl.title.clone()}</h3>
                        <p class="workspace__statement">{lvl.statement.clone()}</p>
                    }
                })
            }}
        </Show>
        <Show when=move || !loading.get() && level.get().is_none()>
            <p
                class="workspace__muted"
                role=move || if error.get().is_some() { "alert" } else { "status" }
                aria-live="polite"
            >
                {move || {
                    error
                        .get()
                        .unwrap_or_else(|| "No hay un nivel cargado todavía.".into())
                }}
            </p>
        </Show>
    }
}

fn track_label(track_type: &str) -> &'static str {
    match track_type {
        "micro_paso" => "Micro-paso",
        "reto_ingenieril" => "Reto ingenieril",
        _ => "Nivel",
    }
}
