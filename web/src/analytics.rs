//! Best-effort conceptual friction telemetry (Wave D.3).
//!
//! Events never include student Python (ADR 002). Failures are silent so a
//! down analytics path cannot log the learner out.

use leptos::prelude::{RwSignal, Set};

use crate::api::{
    ConceptAnalyticsSummary, ConceptEventRequest, EVENT_DUA_FAB_OPEN, EVENT_LEARN_STEP_ENTER,
    EVENT_LEARN_VALIDATE_FAIL, EVENT_LEARN_VALIDATE_PASS,
};
use crate::auth::{fetch_concept_analytics, post_concept_event};
use crate::concepts::primary_partition_for_micro_step;

/// Inclusive decade start for a coding-rail micro-step (1, 11, …, 991).
pub fn decade_lo_for_micro(micro_step: i32) -> i32 {
    if micro_step < 1 {
        return 0;
    }
    ((micro_step - 1) / 10) * 10 + 1
}

fn emit(event_type: &str, partition_id: i32, decade_lo: i32, step_id: &str) {
    let payload = ConceptEventRequest {
        event_type: event_type.to_string(),
        partition_id,
        decade_lo,
        step_id: step_id.to_string(),
    };
    leptos::task::spawn_local(async move {
        let _ = post_concept_event(payload).await;
    });
}

pub fn emit_dua_fab_open(step_id: &str, partition_id: i32, decade_lo: i32) {
    emit(EVENT_DUA_FAB_OPEN, partition_id, decade_lo, step_id);
}

pub fn emit_learn_step_enter(step_id: &str, micro_step: i32) {
    let partition = primary_partition_for_micro_step(micro_step)
        .map(|p| i32::from(p.id))
        .unwrap_or(0);
    emit(
        EVENT_LEARN_STEP_ENTER,
        partition,
        decade_lo_for_micro(micro_step),
        step_id,
    );
}

pub fn emit_learn_validate(step_id: &str, micro_step: i32, passed: bool) {
    let partition = primary_partition_for_micro_step(micro_step)
        .map(|p| i32::from(p.id))
        .unwrap_or(0);
    let kind = if passed {
        EVENT_LEARN_VALIDATE_PASS
    } else {
        EVENT_LEARN_VALIDATE_FAIL
    };
    emit(kind, partition, decade_lo_for_micro(micro_step), step_id);
}

/// POST then GET so the hub widget can refresh after a hub action.
pub fn emit_and_refresh_summary(
    event_type: &'static str,
    partition_id: i32,
    decade_lo: i32,
    summary: RwSignal<Option<ConceptAnalyticsSummary>>,
) {
    let payload = ConceptEventRequest {
        event_type: event_type.to_string(),
        partition_id,
        decade_lo,
        step_id: String::new(),
    };
    leptos::task::spawn_local(async move {
        let _ = post_concept_event(payload).await;
        if let Ok(s) = fetch_concept_analytics().await {
            summary.set(Some(s));
        }
    });
}

#[cfg(test)]
mod tests {
    use super::decade_lo_for_micro;

    #[test]
    fn decade_lo_matches_heatmap_bands() {
        assert_eq!(decade_lo_for_micro(1), 1);
        assert_eq!(decade_lo_for_micro(10), 1);
        assert_eq!(decade_lo_for_micro(11), 11);
        assert_eq!(decade_lo_for_micro(52), 51);
        assert_eq!(decade_lo_for_micro(1000), 991);
        assert_eq!(decade_lo_for_micro(0), 0);
    }
}
