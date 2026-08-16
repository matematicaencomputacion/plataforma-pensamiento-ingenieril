//! Static conceptual DAG (Wave D.4). Nodes are glossary ids, not micro-steps.
//!
//! `Requires` is a DAG (prerequisites). `Reinforces` is display-only and may
//! point the other way. Progress uses `completed_levels` + `current_level`.

use crate::concepts::glossary::{entry_by_id, GlossaryEntry, PartitionId, GLOSSARY_ENTRIES};
use crate::curriculum::coding_step_by_id;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeKind {
    Requires,
    Reinforces,
}

impl EdgeKind {
    pub fn as_str(self) -> &'static str {
        match self {
            EdgeKind::Requires => "requires",
            EdgeKind::Reinforces => "reinforces",
        }
    }

    pub fn verb_es(self) -> &'static str {
        match self {
            EdgeKind::Requires => "requiere",
            EdgeKind::Reinforces => "refuerza",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConceptEdge {
    pub from: &'static str,
    pub to: &'static str,
    pub kind: EdgeKind,
}

/// Pedagogical seed. Keep small; do not grow into a micro-step graph.
pub const CONCEPT_EDGES: &[ConceptEdge] = &[
    ConceptEdge {
        from: "python-lists",
        to: "model-mutability",
        kind: EdgeKind::Requires,
    },
    ConceptEdge {
        from: "syntax-extend",
        to: "python-lists",
        kind: EdgeKind::Requires,
    },
    ConceptEdge {
        from: "trap-aliasing",
        to: "model-mutability",
        kind: EdgeKind::Requires,
    },
    ConceptEdge {
        from: "trap-mutable-default",
        to: "python-lists",
        kind: EdgeKind::Requires,
    },
    ConceptEdge {
        from: "trap-unboundlocal",
        to: "model-legb",
        kind: EdgeKind::Requires,
    },
    ConceptEdge {
        from: "model-comprehension",
        to: "python-lists",
        kind: EdgeKind::Requires,
    },
    ConceptEdge {
        from: "pattern-dfs",
        to: "model-recursion",
        kind: EdgeKind::Requires,
    },
    ConceptEdge {
        from: "trap-tuple-typeerror",
        to: "model-mutability",
        kind: EdgeKind::Requires,
    },
    ConceptEdge {
        from: "model-mutability",
        to: "trap-aliasing",
        kind: EdgeKind::Reinforces,
    },
    ConceptEdge {
        from: "python-lists",
        to: "model-comprehension",
        kind: EdgeKind::Reinforces,
    },
    ConceptEdge {
        from: "model-legb",
        to: "trap-unboundlocal",
        kind: EdgeKind::Reinforces,
    },
    ConceptEdge {
        from: "model-recursion",
        to: "pattern-dfs",
        kind: EdgeKind::Reinforces,
    },
];

fn entry_has_partition(entry: &GlossaryEntry, partition_id: u8) -> bool {
    let Some(p) = PartitionId::from_u8(partition_id) else {
        return false;
    };
    entry.lenses.iter().any(|l| l.partition == p)
}

/// Related coding-rail micro-steps for a glossary concept (`related_step_id`).
pub fn concept_drill_micros(id: &str) -> Vec<i32> {
    let Some(entry) = entry_by_id(id) else {
        return Vec::new();
    };
    let mut out: Vec<i32> = entry
        .lenses
        .iter()
        .filter_map(|lens| lens.related_step_id.and_then(coding_step_by_id))
        .map(|step| step.micro_step)
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// Started = any related drill completed, or the rail cursor already reached it.
pub fn concept_started(id: &str, completed_levels: &[i32], current_level: i32) -> bool {
    concept_drill_micros(id)
        .into_iter()
        .any(|n| completed_levels.iter().any(|c| *c == n) || current_level >= n)
}

/// Edges whose `from` concept has a lens in `partition_id`.
pub fn edges_for_partition(partition_id: u8) -> Vec<&'static ConceptEdge> {
    CONCEPT_EDGES
        .iter()
        .filter(|edge| entry_by_id(edge.from).is_some_and(|e| entry_has_partition(e, partition_id)))
        .collect()
}

/// Required concepts (unique, glossary order) that are not started yet.
pub fn missing_required_bases(
    partition_id: u8,
    completed_levels: &[i32],
    current_level: i32,
) -> Vec<&'static GlossaryEntry> {
    let mut missing: Vec<&'static GlossaryEntry> = Vec::new();
    for edge in edges_for_partition(partition_id) {
        if edge.kind != EdgeKind::Requires {
            continue;
        }
        if concept_drill_micros(edge.to).is_empty() {
            continue;
        }
        if concept_started(edge.to, completed_levels, current_level) {
            continue;
        }
        let Some(entry) = entry_by_id(edge.to) else {
            continue;
        };
        if missing.iter().any(|e| e.id == entry.id) {
            continue;
        }
        missing.push(entry);
    }
    missing.sort_by_key(|e| {
        GLOSSARY_ENTRIES
            .iter()
            .position(|g| g.id == e.id)
            .unwrap_or(usize::MAX)
    });
    missing
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curriculum::coding_step_by_id;

    #[test]
    fn seed_size_and_canonical_lists_requires_mutability() {
        assert_eq!(CONCEPT_EDGES.len(), 12);
        assert!(CONCEPT_EDGES.iter().any(|e| {
            e.from == "python-lists" && e.to == "model-mutability" && e.kind == EdgeKind::Requires
        }));
        assert_eq!(EdgeKind::Requires.as_str(), "requires");
        assert_eq!(EdgeKind::Reinforces.verb_es(), "refuerza");
    }

    #[test]
    fn endpoints_exist_in_glossary_and_requires_targets_have_drills() {
        for edge in CONCEPT_EDGES {
            assert!(
                entry_by_id(edge.from).is_some(),
                "unknown from {}",
                edge.from
            );
            assert!(entry_by_id(edge.to).is_some(), "unknown to {}", edge.to);
            assert_ne!(edge.from, edge.to, "self-loop {}", edge.from);
            if edge.kind == EdgeKind::Requires {
                assert!(
                    !concept_drill_micros(edge.to).is_empty(),
                    "{} Requires {} but {} has no related drills",
                    edge.from,
                    edge.to,
                    edge.to
                );
            }
        }
    }

    #[test]
    fn requires_edges_are_acyclic() {
        use std::collections::{HashMap, HashSet};

        let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut nodes: HashSet<&str> = HashSet::new();
        for edge in CONCEPT_EDGES
            .iter()
            .filter(|e| e.kind == EdgeKind::Requires)
        {
            adj.entry(edge.from).or_default().push(edge.to);
            nodes.insert(edge.from);
            nodes.insert(edge.to);
        }

        fn visit<'a>(
            node: &'a str,
            adj: &HashMap<&str, Vec<&'a str>>,
            visiting: &mut HashSet<&'a str>,
            done: &mut HashSet<&'a str>,
        ) -> bool {
            if done.contains(node) {
                return true;
            }
            if !visiting.insert(node) {
                return false;
            }
            if let Some(nexts) = adj.get(node) {
                for n in nexts {
                    if !visit(n, adj, visiting, done) {
                        return false;
                    }
                }
            }
            visiting.remove(node);
            done.insert(node);
            true
        }

        let mut visiting = HashSet::new();
        let mut done = HashSet::new();
        for node in nodes {
            assert!(
                visit(node, &adj, &mut visiting, &mut done),
                "Requires cycle involving {node}"
            );
        }
    }

    #[test]
    fn mutability_related_step_is_py_26() {
        let micros = concept_drill_micros("model-mutability");
        assert_eq!(micros, vec![26]);
        assert_eq!(
            coding_step_by_id("py-26-list-copy").map(|s| s.micro_step),
            Some(26)
        );
        assert!(!concept_started("model-mutability", &[], 1));
        assert!(concept_started("model-mutability", &[26], 1));
        assert!(concept_started("model-mutability", &[], 26));
        assert!(!concept_started("model-mutability", &[], 25));
    }

    #[test]
    fn p1_fresh_learner_missing_mutability() {
        let edges = edges_for_partition(1);
        assert!(edges.iter().any(|e| {
            e.from == "python-lists" && e.to == "model-mutability" && e.kind == EdgeKind::Requires
        }));
        let missing = missing_required_bases(1, &[], 1);
        assert!(
            missing.iter().any(|e| e.id == "model-mutability"),
            "fresh P1 must flag mutability, got {:?}",
            missing.iter().map(|e| e.id).collect::<Vec<_>>()
        );
        assert!(missing.iter().any(|e| e.title.contains("Mutabilidad")));
    }

    #[test]
    fn completing_py26_clears_mutability_base() {
        let missing = missing_required_bases(1, &[26], 1);
        assert!(
            missing.iter().all(|e| e.id != "model-mutability"),
            "completed py-26 must clear mutability, got {:?}",
            missing.iter().map(|e| e.id).collect::<Vec<_>>()
        );
    }
}
