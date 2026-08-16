use leptos::ev;
use leptos::leptos_dom::helpers::window_event_listener;
use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::{use_navigate, use_params_map};
use leptos_router::NavigateOptions;

use crate::auth::input_value;
use crate::components::level_completed;
use crate::concepts::{
    filtered_drills_for_partition, heatmap_cells_for_drills, heatmap_decade_drills_in,
    mastery_percent, partition_by_id, ConceptFacetFilter, HeatmapBand, HeatmapCellState,
    PARTITIONS,
};
use crate::curriculum::coding_step_by_micro_step;
use crate::session::SessionCtx;

#[component]
pub fn ConceptsPage() -> impl IntoView {
    let session = expect_context::<SessionCtx>();
    let navigate = use_navigate();
    let params = use_params_map();
    let open_decade = RwSignal::new(None::<HeatmapBand>);
    let query = RwSignal::new(String::new());
    let extra_partitions = RwSignal::new(Vec::<u8>::new());

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

    Effect::new(move |_| {
        let id = partition_id.get();
        extra_partitions.update(|xs| xs.retain(|&p| p != id));
        open_decade.set(None);
    });

    Effect::new(move |_| {
        let _ = query.get();
        let _ = extra_partitions.get();
        open_decade.set(None);
    });

    Effect::new(move |_| {
        if open_decade.get().is_none() {
            return;
        }
        let handle = window_event_listener(ev::keydown, move |ev| {
            if ev.key() == "Escape" {
                ev.prevent_default();
                open_decade.set(None);
            }
        });
        on_cleanup(move || handle.remove());
    });

    let completed = Signal::derive(move || {
        session
            .user
            .get()
            .map(|u| u.completed_levels)
            .unwrap_or_default()
    });

    let toggle_extra = move |chip_id: u8| {
        extra_partitions.update(|xs| {
            if let Some(i) = xs.iter().position(|&p| p == chip_id) {
                xs.remove(i);
            } else {
                xs.push(chip_id);
                xs.sort_unstable();
            }
        });
    };

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
                    let completed_now = completed.get();
                    let pct = mastery_percent(id, &completed_now);
                    let (done, total) = crate::concepts::partition_mastery(id, &completed_now);
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
                                <div
                                    id="concept-facet-bar"
                                    class="concept-facet-bar"
                                    role="search"
                                    aria-label="Filtro conceptual"
                                >
                                    <label class="concept-facet-bar__sr" for="concept-facet-query">
                                        "Buscar concepto"
                                    </label>
                                    <input
                                        id="concept-facet-query"
                                        class="concept-facet-bar__query"
                                        type="search"
                                        placeholder="Buscar (append, recursion, dfs…)"
                                        autocomplete="off"
                                        prop:value=move || query.get()
                                        on:input=move |ev| query.set(input_value(&ev))
                                    />
                                    <div
                                        class="concept-facet-bar__lenses"
                                        role="group"
                                        aria-label="Lentes adicionales (AND)"
                                    >
                                        {PARTITIONS
                                            .iter()
                                            .filter(|chip| chip.id != id)
                                            .map(|chip| {
                                                let chip_id = chip.id;
                                                view! {
                                                    <button
                                                        type="button"
                                                        class=move || {
                                                            if extra_partitions.get().contains(&chip_id) {
                                                                "concept-facet-bar__chip concept-facet-bar__chip--on"
                                                            } else {
                                                                "concept-facet-bar__chip"
                                                            }
                                                        }
                                                        id=format!("concept-facet-p{chip_id}")
                                                        aria-pressed=move || {
                                                            if extra_partitions.get().contains(&chip_id) {
                                                                "true"
                                                            } else {
                                                                "false"
                                                            }
                                                        }
                                                        on:click=move |_| toggle_extra(chip_id)
                                                    >
                                                        {format!("+{}", chip.short_label)}
                                                    </button>
                                                }
                                            })
                                            .collect_view()}
                                    </div>
                                    <p class="concept-facet-bar__count" id="concept-facet-count">
                                        {move || {
                                            let filter = ConceptFacetFilter {
                                                extra_partitions: extra_partitions.get(),
                                                query: query.get(),
                                            };
                                            let n = filtered_drills_for_partition(id, &filter).len();
                                            if filter.is_active() {
                                                format!("{n} drills en esta ruta")
                                            } else {
                                                format!("{n} drills")
                                            }
                                        }}
                                    </p>
                                    <Show when=move || {
                                        ConceptFacetFilter {
                                            extra_partitions: extra_partitions.get(),
                                            query: query.get(),
                                        }
                                        .is_active()
                                    }>
                                        <button
                                            id="concept-facet-clear"
                                            class="concept-facet-bar__clear"
                                            type="button"
                                            on:click=move |_| {
                                                query.set(String::new());
                                                extra_partitions.set(Vec::new());
                                            }
                                        >
                                            "Limpiar filtros"
                                        </button>
                                    </Show>
                                </div>
                                {move || {
                                    let filter = ConceptFacetFilter {
                                        extra_partitions: extra_partitions.get(),
                                        query: query.get(),
                                    };
                                    let drills = filtered_drills_for_partition(id, &filter);
                                    let completed_now = completed.get();
                                    let cells = heatmap_cells_for_drills(&drills, &completed_now);
                                    view! {
                                        <div
                                            id="concept-heatmap"
                                            class="concept-heatmap"
                                            aria-label="Cobertura por décadas del rail"
                                        >
                                            {cells
                                                .into_iter()
                                                .map(|cell| {
                                                    let empty = cell.state == HeatmapCellState::Empty;
                                                    let lo = cell.band.lo;
                                                    let band = cell.band;
                                                    let state = cell.state.as_str();
                                                    let label = cell.accessible_name();
                                                    let count = format!("{}/{}", cell.done, cell.total);
                                                    let facet = if cell.total > 0 { "hit" } else { "" };
                                                    if empty {
                                                        view! {
                                                            <button
                                                                type="button"
                                                                class="concept-heatmap__cell"
                                                                id=format!("concept-heat-{lo}")
                                                                data-state=state
                                                                data-facet=facet
                                                                aria-label=label
                                                                prop:disabled=true
                                                            >
                                                                {count}
                                                            </button>
                                                        }
                                                        .into_any()
                                                    } else {
                                                        view! {
                                                            <button
                                                                type="button"
                                                                class="concept-heatmap__cell"
                                                                id=format!("concept-heat-{lo}")
                                                                data-state=state
                                                                data-facet=facet
                                                                aria-label=label
                                                                aria-haspopup="dialog"
                                                                aria-controls="concept-decade-drawer"
                                                                aria-expanded=move || {
                                                                    if open_decade.get() == Some(band) {
                                                                        "true"
                                                                    } else {
                                                                        "false"
                                                                    }
                                                                }
                                                                on:click=move |_| open_decade.set(Some(band))
                                                            >
                                                                {count}
                                                            </button>
                                                        }
                                                        .into_any()
                                                    }
                                                })
                                                .collect_view()}
                                        </div>
                                        <ul class="concepts__drills" id="concepts-drill-list">
                                            {drills
                                                .into_iter()
                                                .filter_map(|n| {
                                                    coding_step_by_micro_step(n).map(|step| (n, step))
                                                })
                                                .map(|(n, step)| {
                                                    let href = format!("/learn/{}", step.id);
                                                    let done = level_completed(&completed_now, n);
                                                    let mut class = String::from("concepts__drill");
                                                    if done {
                                                        class.push_str(" concepts__drill--done");
                                                    }
                                                    let tags = crate::concepts::partitions_for_micro_step(n)
                                                        .iter()
                                                        .map(ToString::to_string)
                                                        .collect::<Vec<_>>()
                                                        .join(",");
                                                    view! {
                                                        <li class=class>
                                                            <A
                                                                href=href
                                                                attr:class="concepts__drill-link"
                                                                attr:id=format!("concepts-drill-{n}")
                                                                attr:data-partitions=tags
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
                                        {move || {
                                            let Some(band) = open_decade.get() else {
                                                return ().into_any();
                                            };
                                            let filter = ConceptFacetFilter {
                                                extra_partitions: extra_partitions.get(),
                                                query: query.get(),
                                            };
                                            let drills = filtered_drills_for_partition(id, &filter);
                                            let decade_drills =
                                                heatmap_decade_drills_in(&drills, band);
                                            if decade_drills.is_empty() {
                                                return ().into_any();
                                            }
                                            let completed_now = completed.get();
                                            let title = format!("Década {}–{}", band.lo, band.hi);
                                            view! {
                                                <div class="concept-decade-overlay">
                                                    <button
                                                        type="button"
                                                        class="concept-decade-overlay__backdrop"
                                                        aria-label="Cerrar lista de década"
                                                        on:click=move |_| open_decade.set(None)
                                                    ></button>
                                                    <aside
                                                        id="concept-decade-drawer"
                                                        class="concept-decade-drawer"
                                                        role="dialog"
                                                        aria-modal="true"
                                                        aria-labelledby="concept-decade-title"
                                                    >
                                                        <header class="concept-decade-drawer__head">
                                                            <h3 id="concept-decade-title" class="concept-decade-drawer__title">
                                                                {title}
                                                            </h3>
                                                            <button
                                                                id="concept-decade-close"
                                                                class="concept-decade-drawer__close"
                                                                type="button"
                                                                aria-label="Cerrar lista de década"
                                                                on:click=move |_| open_decade.set(None)
                                                            >
                                                                "Esc"
                                                            </button>
                                                        </header>
                                                        <ul
                                                            class="concepts__drills"
                                                            id="concept-decade-list"
                                                            role="list"
                                                        >
                                                            {decade_drills
                                                                .into_iter()
                                                                .filter_map(|n| {
                                                                    coding_step_by_micro_step(n).map(|step| (n, step))
                                                                })
                                                                .map(|(n, step)| {
                                                                    let href = format!("/learn/{}", step.id);
                                                                    let done = level_completed(&completed_now, n);
                                                                    let mut class = String::from("concepts__drill");
                                                                    if done {
                                                                        class.push_str(" concepts__drill--done");
                                                                    }
                                                                    view! {
                                                                        <li class=class role="listitem">
                                                                            <A
                                                                                href=href
                                                                                attr:class="concepts__drill-link"
                                                                                attr:id=format!("concept-decade-drill-{n}")
                                                                                attr:data-micro=n.to_string()
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
                                                    </aside>
                                                </div>
                                            }
                                            .into_any()
                                        }}
                                    }
                                    .into_any()
                                }}
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
