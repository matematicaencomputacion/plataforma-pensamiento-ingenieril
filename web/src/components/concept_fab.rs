//! Progressive conceptual FAB (4 states) on `/learn/:step`.

use leptos::ev;
use leptos::html;
use leptos::leptos_dom::helpers::window_event_listener;
use leptos::prelude::*;
use leptos_router::components::A;

use crate::auth::input_value;
use crate::concepts::{
    entry_by_id, group_search_hits, search_glossary, ConceptLens, GlossaryEntry, PartitionId,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FabState {
    Collapsed,
    Search,
    MicroCard,
    Docked,
}

impl FabState {
    pub fn as_u8(self) -> u8 {
        match self {
            FabState::Collapsed => 0,
            FabState::Search => 1,
            FabState::MicroCard => 2,
            FabState::Docked => 3,
        }
    }

    fn shows_search(self) -> bool {
        matches!(self, FabState::Search | FabState::MicroCard | FabState::Docked)
    }
}

fn focus_search_input(node: NodeRef<html::Input>) {
    if let Some(el) = node.get_untracked() {
        let _ = el.focus();
    }
}

fn lens_for_entry(entry: &GlossaryEntry, preferred: Option<PartitionId>) -> PartitionId {
    if let Some(p) = preferred {
        if entry.lenses.iter().any(|l| l.partition == p) {
            return p;
        }
    }
    entry
        .lenses
        .first()
        .map(|l| l.partition)
        .unwrap_or(PartitionId::P1MemoryData)
}

fn active_lens<'a>(entry: &'a GlossaryEntry, partition: PartitionId) -> Option<&'a ConceptLens> {
    entry
        .lenses
        .iter()
        .find(|l| l.partition == partition)
        .or(entry.lenses.first())
}

/// Progressive glossary widget. Mount only when the route has `:step`.
#[component]
pub fn ConceptLensWidget(
    visible: Signal<bool>,
    state: RwSignal<FabState>,
) -> impl IntoView {
    let query = RwSignal::new(String::new());
    let lens_filter = RwSignal::new(Option::<PartitionId>::None);
    let selected_id = RwSignal::new(Option::<&'static str>::None);
    let peek_partition = RwSignal::new(PartitionId::P1MemoryData);
    let search_ref: NodeRef<html::Input> = NodeRef::new();
    let focus_nonce = RwSignal::new(0u32);

    let open_search = move || {
        if state.get_untracked() == FabState::Collapsed {
            state.set(FabState::Search);
        }
        focus_nonce.update(|n| *n += 1);
    };

    let collapse = move || {
        state.set(FabState::Collapsed);
    };

    let select_hit = move |id: &'static str| {
        if let Some(entry) = entry_by_id(id) {
            selected_id.set(Some(id));
            peek_partition.set(lens_for_entry(entry, lens_filter.get_untracked()));
            if state.get_untracked() != FabState::Docked {
                state.set(FabState::MicroCard);
            }
        }
    };

    let toggle_lens = move |p: PartitionId| {
        let next = if lens_filter.get_untracked() == Some(p) {
            None
        } else {
            Some(p)
        };
        lens_filter.set(next);
        let q = query.get_untracked();
        if !q.trim().is_empty() {
            let hits = search_glossary(&q, next);
            if hits.len() == 1 {
                select_hit(hits[0].id);
            }
        }
    };

    Effect::new(move |_| {
        let _ = focus_nonce.get();
        if !visible.get() || !state.get().shows_search() {
            return;
        }
        request_animation_frame(move || {
            focus_search_input(search_ref);
        });
    });

    Effect::new(move |_| {
        if !visible.get() {
            return;
        }
        let handle = window_event_listener(ev::keydown, move |ev| {
            if !visible.get_untracked() {
                return;
            }
            let key = ev.key();
            let is_k = key.eq_ignore_ascii_case("k");
            if is_k && (ev.ctrl_key() || ev.meta_key()) {
                ev.prevent_default();
                open_search();
                return;
            }
            if key == "Escape" && state.get_untracked() != FabState::Collapsed {
                ev.prevent_default();
                collapse();
            }
        });
        on_cleanup(move || handle.remove());
    });

    let hits = Memo::new(move |_| {
        if !state.get().shows_search() {
            return Vec::new();
        }
        search_glossary(&query.get(), lens_filter.get())
    });

    let selected = Memo::new(move |_| selected_id.get().and_then(entry_by_id));

    view! {
        <Show when=move || visible.get()>
            {move || match state.get() {
                FabState::Collapsed => view! {
                    <div class="concept-fab-wrap" data-fab-state="0">
                        <button
                            id="concept-fab"
                            class="concept-fab"
                            type="button"
                            aria-label="Lentes [1]…[5]"
                            title="Lentes [1]…[5]"
                            on:click=move |_| open_search()
                        >
                            "🔮"
                            <span class="concept-fab__sr">" Lentes [1]…[5]"</span>
                        </button>
                    </div>
                }.into_any(),
                FabState::Search | FabState::MicroCard | FabState::Docked => {
                    let docked = state.get() == FabState::Docked;
                    let peek = state.get() != FabState::Search;
                    let wrap_class = if docked {
                        "concept-drawer"
                    } else {
                        "concept-fab-wrap concept-fab-wrap--open"
                    };
                    let wrap_id = if docked { "concept-drawer" } else { "concept-fab-panel" };
                    view! {
                        <div
                            class=wrap_class
                            id=wrap_id
                            data-fab-state=state.get().as_u8().to_string()
                        >
                            <header class="concept-panel__head">
                                <p class="concept-panel__eyebrow">"Glosario · lentes [1]…[5]"</p>
                                <div class="concept-panel__actions">
                                    <Show when=move || docked>
                                        <button
                                            id="concept-undock"
                                            class="concept-panel__text-btn"
                                            type="button"
                                            on:click=move |_| collapse()
                                        >
                                            "Desanclar"
                                        </button>
                                    </Show>
                                    <button
                                        class="concept-panel__text-btn"
                                        type="button"
                                        aria-label="Cerrar glosario"
                                        on:click=move |_| collapse()
                                    >
                                        "Esc"
                                    </button>
                                </div>
                            </header>
                            <label class="concept-panel__sr" for="concept-glossary-search">
                                "Buscar concepto"
                            </label>
                            <input
                                id="concept-glossary-search"
                                class="concept-panel__search"
                                type="search"
                                placeholder="Buscar (append, LEGB, BFS…)"
                                autocomplete="off"
                                node_ref=search_ref
                                prop:value=move || query.get()
                                on:input=move |ev| query.set(input_value(&ev))
                            />
                            <div class="concept-lens-row" role="group" aria-label="Filtro de lente">
                                {PartitionId::ALL.into_iter().map(|p| {
                                    let id = p.as_u8();
                                    view! {
                                        <button
                                            id=format!("concept-lens-{id}")
                                            class=format!("concept-lens-chip {}", p.color_badge())
                                            type="button"
                                            data-lens=id.to_string()
                                            aria-pressed=move || {
                                                (lens_filter.get() == Some(p)).to_string()
                                            }
                                            on:click=move |_| toggle_lens(p)
                                        >
                                            {p.label()}
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                            <Show when=move || state.get() == FabState::Search>
                                <div class="concept-hits" id="concept-hits">
                                    {move || {
                                        let grouped = group_search_hits(&hits.get());
                                        if grouped.is_empty() {
                                            view! {
                                                <p class="concept-hits__empty">
                                                    "Sin coincidencias en el glosario."
                                                </p>
                                            }.into_any()
                                        } else {
                                            view! {
                                                <For
                                                    each=move || group_search_hits(&hits.get())
                                                    key=|(intent, _)| format!("{intent:?}")
                                                    children=move |(intent, group)| {
                                                        view! {
                                                            <section class="concept-hits__group">
                                                                <h3 class="concept-hits__intent">{intent.label()}</h3>
                                                                <ul>
                                                                    {group.into_iter().map(|entry| {
                                                                        let id = entry.id;
                                                                        view! {
                                                                            <li>
                                                                                <button
                                                                                    class="concept-hits__hit"
                                                                                    type="button"
                                                                                    id=format!("concept-hit-{id}")
                                                                                    data-glossary-id=id
                                                                                    on:click=move |_| select_hit(id)
                                                                                >
                                                                                    {entry.title}
                                                                                </button>
                                                                            </li>
                                                                        }
                                                                    }).collect_view()}
                                                                </ul>
                                                            </section>
                                                        }
                                                    }
                                                />
                                            }.into_any()
                                        }
                                    }}
                                </div>
                            </Show>
                            <Show when=move || peek>
                                {move || selected.get().map(|entry| {
                                    view! {
                                        <ConceptMicroCard
                                            entry=entry
                                            partition=peek_partition
                                            state=state
                                        />
                                    }
                                })}
                            </Show>
                        </div>
                    }.into_any()
                }
            }}
        </Show>
    }
}

#[component]
fn ConceptMicroCard(
    entry: &'static GlossaryEntry,
    partition: RwSignal<PartitionId>,
    state: RwSignal<FabState>,
) -> impl IntoView {
    let lens = Memo::new(move |_| {
        active_lens(entry, partition.get())
            .copied()
            .unwrap_or(entry.lenses[0])
    });
    let hub = move || format!("/concepts/{}", lens.get().partition.as_u8());
    let related = move || lens.get().related_step_id;
    let cta = move || {
        format!(
            "Ver modelo mental en Partición {}",
            lens.get().partition.as_u8()
        )
    };
    let docked = move || state.get() == FabState::Docked;

    view! {
        <article class="concept-card" id="concept-peek" data-entry=entry.id>
            <div class="concept-card__pills" role="group" aria-label="Lentes del término">
                {entry.lenses.iter().map(|l| {
                    let p = l.partition;
                    let id = p.as_u8();
                    view! {
                        <button
                            id=format!("concept-pill-{id}")
                            class=format!("concept-pill {}", p.color_badge())
                            type="button"
                            data-lens=id.to_string()
                            aria-pressed=move || (partition.get() == p).to_string()
                            on:click=move |_| partition.set(p)
                        >
                            {p.label()}
                        </button>
                    }
                }).collect_view()}
            </div>
            <h3 class="concept-card__title">{entry.title}</h3>
            <p class="concept-card__tldr">{move || lens.get().tldr}</p>
            <p class="concept-card__headline">{move || lens.get().headline}</p>
            <pre class="concept-card__code">{move || lens.get().code_example}</pre>
            <Show when=move || entry.common_pitfall.is_some()>
                <p class="concept-card__pitfall">
                    {entry.common_pitfall.unwrap_or_default()}
                </p>
            </Show>
            <div class="concept-card__cta-row">
                <A href=hub attr:class="concept-card__cta" attr:id="concept-cta-hub">
                    {cta}
                </A>
                {move || related().map(|step_id| {
                    view! {
                        <A
                            href=format!("/learn/{step_id}")
                            attr:class="concept-card__related"
                        >
                            "Ir al drill"
                        </A>
                    }
                })}
                <Show when=move || !docked()>
                    <button
                        id="concept-dock"
                        class="concept-card__dock"
                        type="button"
                        on:click=move |_| state.set(FabState::Docked)
                    >
                        "Anclar"
                    </button>
                </Show>
            </div>
        </article>
    }
}
