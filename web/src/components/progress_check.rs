//! Persistent green progress check (Paso 2 / workspace).

use leptos::prelude::*;

/// Circular green checkmark for completed levels / steps.
#[component]
pub fn ProgressCheck(
    /// Accessible label (e.g. "Nivel superado").
    #[prop(into)]
    label: String,
    /// Optional DOM id.
    #[prop(optional, into)]
    id: Option<String>,
) -> impl IntoView {
    let id_attr = id.unwrap_or_default();
    view! {
        <span
            class="progress-check"
            id=id_attr
            role="img"
            attr:aria-label=label.clone()
            title=label
        >
            <span class="progress-check__mark" aria-hidden="true">
                "✓"
            </span>
        </span>
    }
}

/// True when the learner earned `level_id` (explicit completed set — not cursor inference).
pub fn level_completed(completed_levels: &[i32], level_id: i32) -> bool {
    level_id > 0 && completed_levels.iter().any(|&id| id == level_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_completed_uses_earned_set_not_cursor() {
        assert!(!level_completed(&[], 1));
        assert!(level_completed(&[1], 1));
        assert!(!level_completed(&[157], 1));
        assert!(level_completed(&[157], 157));
        assert!(!level_completed(&[157], 156));
        assert!(!level_completed(&[1, 2], 0));
    }
}
