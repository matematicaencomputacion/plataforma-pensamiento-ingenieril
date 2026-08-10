//! Shared brand mark: «Ingenier» + green «IA».

use leptos::prelude::*;
use leptos_router::components::A;

/// Wordmark for shell header / auth cards / hero.
#[component]
pub fn BrandMark(
    /// Extra CSS classes (e.g. `shell__brand`, `hero__title`).
    #[prop(into, optional)]
    class: String,
    /// When true, render as `<h1>`; otherwise as `<span>` inside a link parent.
    #[prop(optional)]
    heading: bool,
) -> impl IntoView {
    let class_attr = if class.is_empty() {
        "brand-mark".to_string()
    } else {
        format!("brand-mark {class}")
    };

    if heading {
        view! {
            <h1 class=class_attr>
                <span class="brand-mark__base">"Ingenier"</span>
                <span class="brand-mark__ia">"IA"</span>
            </h1>
        }
        .into_any()
    } else {
        view! {
            <span class=class_attr>
                <span class="brand-mark__base">"Ingenier"</span>
                <span class="brand-mark__ia">"IA"</span>
            </span>
        }
        .into_any()
    }
}

/// Linked wordmark (header / auth).
#[component]
pub fn BrandLink(
    #[prop(into, optional)]
    class: String,
) -> impl IntoView {
    let class_attr = if class.is_empty() {
        "brand-mark".to_string()
    } else {
        format!("brand-mark {class}")
    };
    view! {
        <A href="/" attr:class=class_attr>
            <span class="brand-mark__base">"Ingenier"</span>
            <span class="brand-mark__ia">"IA"</span>
        </A>
    }
}
