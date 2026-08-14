use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_location;

use crate::concepts::{
    mastery_percent, partition_by_id, partitions_for_micro_step, primary_partition_for_micro_step,
    PARTITIONS,
};
use crate::curriculum::coding_step_by_id;
use crate::session::SessionCtx;

/// Cognitive compass `[1]…[5]` — authenticated shell only.
#[component]
pub fn PartitionNav() -> impl IntoView {
    let session = expect_context::<SessionCtx>();
    let location = use_location();

    let active_partitions = Signal::derive(move || {
        let path = location.pathname.get();
        if let Some(rest) = path.strip_prefix("/concepts/") {
            let id_str = rest.split('/').next().unwrap_or("");
            if let Ok(id) = id_str.parse::<u8>() {
                return vec![id];
            }
        }
        if let Some(rest) = path.strip_prefix("/learn/") {
            let step_id = rest.split('/').next().unwrap_or("");
            if !step_id.is_empty() {
                if let Some(step) = coding_step_by_id(step_id) {
                    return partitions_for_micro_step(step.micro_step).to_vec();
                }
            }
        }
        Vec::new()
    });

    let completed = Signal::derive(move || {
        session
            .user
            .get()
            .map(|u| u.completed_levels)
            .unwrap_or_default()
    });

    view! {
        <Show when=move || session.user.get().is_some()>
            <nav class="partition-nav" aria-label="Compás cognitivo — particiones">
                {PARTITIONS
                    .iter()
                    .map(|p| {
                        let id = p.id;
                        let href = format!("/concepts/{id}");
                        let title = format!("{} — {}", p.id, p.title);
                        let short = p.short_label;
                        view! {
                            <A
                                href=href
                                attr:class=move || {
                                    let mut class = String::from("partition-nav__btn");
                                    if active_partitions.get().iter().any(|&a| a == id) {
                                        class.push_str(" partition-nav__btn--active");
                                    }
                                    class
                                }
                                attr:title=title
                                attr:aria-label=move || {
                                    format!(
                                        "Partición {}: {}. Dominio {}%",
                                        id,
                                        short,
                                        mastery_percent(id, &completed.get())
                                    )
                                }
                                attr:data-partition=id.to_string()
                                attr:id=format!("partition-nav-{id}")
                            >
                                <span class="partition-nav__num">{id}</span>
                                <span class="partition-nav__label">{short}</span>
                            </A>
                        }
                    })
                    .collect_view()}
            </nav>
        </Show>
    }
}

/// Compact badges under the learn enunciado.
#[component]
pub fn PartitionBadges(micro_step: Signal<i32>) -> impl IntoView {
    view! {
        <Show when=move || !partitions_for_micro_step(micro_step.get()).is_empty()>
            <div class="partition-badges" aria-label="Particiones conceptuales">
                {move || {
                    partitions_for_micro_step(micro_step.get())
                        .iter()
                        .filter_map(|&id| partition_by_id(id))
                        .map(|p| {
                            let href = format!("/concepts/{}", p.id);
                            let label = format!("Partición {}: {}", p.id, p.short_label);
                            view! {
                                <A
                                    href=href
                                    attr:class="partition-badge"
                                    attr:title=p.title
                                >
                                    {label}
                                </A>
                            }
                        })
                        .collect_view()
                }}
            </div>
        </Show>
    }
}

/// Highlight helper for tests / callers.
pub fn primary_partition_id(micro_step: i32) -> Option<u8> {
    primary_partition_for_micro_step(micro_step).map(|p| p.id)
}
