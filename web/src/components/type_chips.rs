//! Accordion chips for data-type acronyms next to the Variables label.

use leptos::prelude::*;

use crate::data_types::{chip_explanation, DATA_TYPE_CHIPS};

/// Fila «Variables» + chips. Solo un chip activo a la vez; tocá el activo para apagarlo.
#[component]
pub fn VariableTypeChips() -> impl IntoView {
    let active = RwSignal::new(Option::<&'static str>::None);

    let on_toggle = move |id: &'static str| {
        active.update(|cur| {
            if *cur == Some(id) {
                *cur = None;
            } else {
                *cur = Some(id);
            }
        });
    };

    view! {
        <div class="type-chips" id="learn-type-chips" aria-label="Tipos de datos y estructuras">
            <div class="type-chips__row">
                <span class="type-chips__label">"Variables"</span>
                <div class="type-chips__buttons" role="toolbar" aria-label="Acrónimos de tipos">
                    {DATA_TYPE_CHIPS
                        .iter()
                        .copied()
                        .map(|chip| {
                            let id = chip.id;
                            let tone = chip.tone;
                            view! {
                                <button
                                    type="button"
                                    class=move || {
                                        let base = format!("type-chips__btn type-chips__btn--{tone}");
                                        if active.get() == Some(id) {
                                            format!("{base} type-chips__btn--active")
                                        } else {
                                            base
                                        }
                                    }
                                    id=format!("type-chip-{id}")
                                    attr:aria-pressed=move || {
                                        (active.get() == Some(id)).to_string()
                                    }
                                    attr:aria-expanded=move || {
                                        (active.get() == Some(id)).to_string()
                                    }
                                    attr:aria-controls="type-chip-panel"
                                    title=format!("{} — {}", chip.name_en, chip.gloss_es)
                                    on:click=move |_| on_toggle(id)
                                >
                                    {id}
                                </button>
                            }
                        })
                        .collect_view()}
                </div>
            </div>

            <Show when=move || active.get().is_some()>
                {move || {
                    active.get().and_then(|id| {
                        DATA_TYPE_CHIPS.iter().find(|c| c.id == id).map(|chip| {
                            let text = chip_explanation(chip);
                            view! {
                                <aside
                                    class="type-chips__panel"
                                    id="type-chip-panel"
                                    role="region"
                                    aria-live="polite"
                                >
                                    <p class="type-chips__panel-title">
                                        "Tipos de Datos y Estructuras"
                                    </p>
                                    <p class="type-chips__panel-body">{text}</p>
                                </aside>
                            }
                        })
                    })
                }}
            </Show>
        </div>
    }
}
