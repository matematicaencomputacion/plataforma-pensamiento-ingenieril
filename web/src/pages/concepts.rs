use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_params_map};
use leptos_router::NavigateOptions;

use crate::components::level_completed;
use crate::concepts::{
    drills_for_partition, mastery_percent, partition_by_id, PARTITIONS,
};
use crate::curriculum::coding_step_by_micro_step;
use crate::session::SessionCtx;

#[component]
pub fn ConceptsPage() -> impl IntoView {
    let session = expect_context::<SessionCtx>();
    let navigate = use_navigate();
    let params = use_params_map();

    Effect::new(move |_| {
        let ready = session.bootstrapped.get();
        let live = session.user.get().is_some();
        let pending = session.token.get().is_some() && !live;
        if ready && !live && !pending {
            navigate(
                "/login",
                NavigateOptions {
                    replace: true,
                    ..Default::default()
                },
            );
        }
    });

    let partition_id = Signal::derive(move || {
        params
            .with(|p| p.get("id").and_then(|s| s.parse::<u8>().ok()))
            .unwrap_or(1)
            .clamp(1, 5)
    });

    let completed = Signal::derive(move || {
        session
            .user
            .get()
            .map(|u| u.completed_levels)
            .unwrap_or_default()
    });

    view! {
        <section class="concepts">
            <Show
                when=move || session.user.get().is_some()
                fallback=move || {
                    view! { <p class="concepts__muted">"Comprobando sesión…"</p> }
                }
            >
                <header class="concepts__header">
                    <p class="concepts__eyebrow">"Compás cognitivo"</p>
                    <h1 class="concepts__title">"Particiones conceptuales"</h1>
                    <p class="concepts__lead">
                        "Cinco lentes para razonar Python. El rail lineal sigue ahí; acá elegís "
                        "qué modelo mental practicar y saltás al drill."
                    </p>
                    <nav class="concepts__tabs" aria-label="Elegir partición">
                        {PARTITIONS
                            .iter()
                            .map(|p| {
                                let id = p.id;
                                let href = format!("/concepts/{id}");
                                view! {
                                    <A
                                        href=href
                                        attr:class=move || {
                                            if partition_id.get() == id {
                                                "concepts__tab concepts__tab--active"
                                            } else {
                                                "concepts__tab"
                                            }
                                        }
                                        attr:id=format!("concepts-tab-{id}")
                                    >
                                        <span class="concepts__tab-num">{id}</span>
                                        {p.short_label}
                                    </A>
                                }
                            })
                            .collect_view()}
                    </nav>
                </header>

                {move || {
                    let id = partition_id.get();
                    let Some(p) = partition_by_id(id) else {
                        return view! {
                            <p class="concepts__muted" role="alert">"Partición desconocida."</p>
                        }
                        .into_any();
                    };
                    let drills = drills_for_partition(id);
                    let pct = mastery_percent(id, &completed.get());
                    let (done, total) = crate::concepts::partition_mastery(id, &completed.get());
                    view! {
                        <article class="concepts__panel" data-partition=id.to_string()>
                            <header class="concepts__panel-head">
                                <h2 class="concepts__panel-title">
                                    {format!("{} · {}", p.id, p.title)}
                                </h2>
                                <p class="concepts__mastery" id=format!("concepts-mastery-{id}")>
                                    {format!("Dominio {pct}% ({done}/{total} drills)")}
                                </p>
                                <Show when=move || p.map_only>
                                    <p class="concepts__map-note" role="note">
                                        "Mapa conceptual (ADR 002): drills livianos en Pyodide; "
                                        "sin labs PyPI pesados en el browser."
                                    </p>
                                </Show>
                            </header>

                            <section class="concepts__block" aria-labelledby="concepts-model">
                                <h3 id="concepts-model" class="concepts__block-title">
                                    "1. Modelo mental"
                                </h3>
                                <p class="concepts__model">{p.mental_model}</p>
                            </section>

                            <section class="concepts__block" aria-labelledby="concepts-axes">
                                <h3 id="concepts-axes" class="concepts__block-title">
                                    "2. Ejes temáticos"
                                </h3>
                                <ul class="concepts__axes">
                                    {p.axes
                                        .iter()
                                        .map(|axis| {
                                            view! { <li>{*axis}</li> }
                                        })
                                        .collect_view()}
                                </ul>
                            </section>

                            <section class="concepts__block" aria-labelledby="concepts-drills">
                                <h3 id="concepts-drills" class="concepts__block-title">
                                    "3. Práctica (drills)"
                                </h3>
                                <ul class="concepts__drills" id="concepts-drill-list">
                                    {drills
                                        .into_iter()
                                        .filter_map(|n| {
                                            coding_step_by_micro_step(n).map(|step| (n, step))
                                        })
                                        .map(|(n, step)| {
                                            let href = format!("/learn/{}", step.id);
                                            let done = level_completed(&completed.get(), n);
                                            let mut class = String::from("concepts__drill");
                                            if done {
                                                class.push_str(" concepts__drill--done");
                                            }
                                            view! {
                                                <li class=class>
                                                    <A
                                                        href=href
                                                        attr:class="concepts__drill-link"
                                                        attr:id=format!("concepts-drill-{n}")
                                                    >
                                                        <span class="concepts__drill-num">{format!("Ej {n:02}")}</span>
                                                        <span class="concepts__drill-title">{step.title}</span>
                                                        <span class="concepts__drill-status">
                                                            {if done { "Completado" } else { "Pendiente" }}
                                                        </span>
                                                    </A>
                                                </li>
                                            }
                                        })
                                        .collect_view()}
                                </ul>
                            </section>

                            <nav class="concepts__footer-nav">
                                <A href="/workspace" attr:class="cta cta--secondary">
                                    "Volver al workspace"
                                </A>
                                <A href="/learn" attr:class="cta cta--primary">
                                    "Ir a Coding"
                                </A>
                            </nav>
                        </article>
                    }
                    .into_any()
                }}
            </Show>
        </section>
    }
}
