//! Conceptual partitions (lentes de razonamiento) over the coding rail.
//!
//! Pedagogy: five mutually exclusive *labels* for navigation — but a micro-step
//! may carry multiple tags (multi-label index). Does not alter progress storage.

use crate::curriculum::coding_step_by_micro_step;

/// Stable partition ids shown in the cognitive compass `[1]…[5]`.
pub const PARTITION_COUNT: u8 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConceptPartition {
    pub id: u8,
    pub slug: &'static str,
    pub short_label: &'static str,
    pub title: &'static str,
    pub mental_model: &'static str,
    pub axes: &'static [&'static str],
    /// When true, hub is primarily a map (ADR 002: no heavy PyPI labs).
    pub map_only: bool,
}

pub const PARTITIONS: &[ConceptPartition] = &[
    ConceptPartition {
        id: 1,
        slug: "data-model",
        short_label: "Modelo",
        title: "Modelo de Datos & Mutabilidad",
        mental_model: "En Python las variables son etiquetas que apuntan a objetos. Saber si el objeto puede mutar in-place o exige uno nuevo es la diferencia entre código predecible y bugs de aliasing.",
        axes: &[
            "Identidad vs igualdad (id / is / ==)",
            "Escalares e inmutables (int, str, tuple)",
            "Colecciones mutables y efectos (list, dict, set)",
            "Copia superficial y aliasing",
            "Paso de argumentos por asignación de objetos",
        ],
        map_only: false,
    },
    ConceptPartition {
        id: 2,
        slug: "scope-legb",
        short_label: "LEGB",
        title: "Ámbitos y Nombres (LEGB)",
        mental_model: "Python resuelve nombres en un orden estricto: Local → Enclosing → Global → Built-in. Entender ese recorrido evita shadowing accidental y closures rotas.",
        axes: &[
            "Local (L): nombres dentro de la función",
            "Enclosing (E): closures y funciones anidadas",
            "Global (G): nivel de módulo",
            "Built-in (B): len, print, range…",
            "self y namespaces de instancia (relación)",
        ],
        map_only: false,
    },
    ConceptPartition {
        id: 3,
        slug: "paradigms",
        short_label: "Paradigmas",
        title: "Paradigmas de Programación",
        mental_model: "El mismo problema se puede modelar de formas distintas. Imperativo, OOP y funcional son lentes de diseño — no religiones. Elegí la que hace el código más claro y testeable.",
        axes: &[
            "Imperativo / estructurado (secuencia, if, bucles)",
            "Orientado a objetos (clases, estado, comportamiento)",
            "Funcional (lambda, comprensiones, funciones como valores)",
            "Recursión como herramienta de diseño",
        ],
        map_only: false,
    },
    ConceptPartition {
        id: 4,
        slug: "ecosystem",
        short_label: "Ecosistema",
        title: "Ecosistema de Librerías",
        mental_model: "Todo código Python viene de tres orígenes: librería estándar, paquetes de terceros (PyPI) o tu aplicación. En PPI el runtime del alumno es Pyodide: practicar stdlib y código propio; PyPI pesado queda como mapa.",
        axes: &[
            "Built-in / stdlib (batteries included)",
            "PyPI / terceros (mapa conceptual — ADR 002)",
            "Código de aplicación (tus módulos)",
            "Importar y usar módulos del entorno",
        ],
        map_only: true,
    },
    ConceptPartition {
        id: 5,
        slug: "application-domains",
        short_label: "Dominios",
        title: "Áreas de Aplicación",
        mental_model: "Python cubre muchos oficios. Esta partición orienta: web/APIs, datos/IA, scripting y CLI. En el browser practicamos lo que Pyodide permite; el resto es brújula profesional.",
        axes: &[
            "Web / APIs (mapa)",
            "Ciencia de datos e IA (mapa)",
            "Scripting y automatización (drills livianos)",
            "Herramientas CLI (mapa)",
        ],
        map_only: true,
    },
];

/// `(micro_step, partition_ids…)` — multi-label, foundations-first.
/// Keep sorted by micro_step ascending for binary search.
const STEP_PARTITIONS: &[(i32, &[u8])] = &[
    (4, &[3]),
    (5, &[3]),
    (6, &[3]),
    (7, &[1]),
    (8, &[1]),
    (9, &[1]),
    (10, &[1]),
    (11, &[1]),
    (12, &[1]),
    (13, &[1]),
    (14, &[1]),
    (15, &[1]),
    (16, &[1, 3]),
    (17, &[1, 3]),
    (18, &[1]),
    (19, &[1]),
    (20, &[1]),
    (21, &[1]),
    (22, &[1]),
    (23, &[1, 3]),
    (24, &[1, 3]),
    (25, &[1]),
    (26, &[1]),
    (27, &[1]),
    (28, &[1]),
    (29, &[1]),
    (30, &[1]),
    (31, &[1]),
    (32, &[1, 3]),
    (33, &[1]),
    (34, &[1]),
    (35, &[1]),
    (36, &[1]),
    (37, &[1]),
    (38, &[1, 3]),
    (39, &[1]),
    (40, &[1]),
    (41, &[1]),
    (42, &[1]),
    (43, &[1]),
    (44, &[1]),
    (45, &[1, 3]),
    (46, &[1]),
    (47, &[1]),
    (48, &[3]),
    (49, &[3]),
    (50, &[3]),
    (51, &[3]),
    (52, &[2, 3]),
    (53, &[2, 3]),
    (54, &[2, 3]),
    (55, &[2, 3]),
    (56, &[1]),
    (57, &[3]),
    (58, &[3]),
    (59, &[3]),
    (60, &[3]),
    (61, &[3]),
    (62, &[2]),
    (63, &[4]),
    (64, &[4]),
    (65, &[4]),
    (66, &[1, 4]),
    (67, &[4]),
    (68, &[3]),
    (69, &[4]),
    (70, &[3, 5]),
    (71, &[3, 5]),
    (72, &[3, 5]),
    (73, &[3, 5]),
    (74, &[3, 5]),
    (75, &[3, 5]),
    (76, &[1, 5]),
    (77, &[1, 5]),
    (78, &[5]),
    (79, &[2, 3]),
    (80, &[2, 3]),
    (81, &[2, 3]),
    (86, &[3]),
    (87, &[3]),
    (94, &[3]),
    (95, &[3]),
    (96, &[3]),
    (97, &[3]),
    (98, &[3]),
    (99, &[3]),
];

pub fn partition_by_id(id: u8) -> Option<&'static ConceptPartition> {
    PARTITIONS.iter().find(|p| p.id == id)
}

pub fn partitions_for_micro_step(micro_step: i32) -> &'static [u8] {
    match STEP_PARTITIONS.binary_search_by_key(&micro_step, |row| row.0) {
        Ok(i) => STEP_PARTITIONS[i].1,
        Err(_) => &[],
    }
}

/// Primary partition for badges (lowest id among tags, if any).
pub fn primary_partition_for_micro_step(micro_step: i32) -> Option<&'static ConceptPartition> {
    partitions_for_micro_step(micro_step)
        .iter()
        .copied()
        .min()
        .and_then(partition_by_id)
}

pub fn drills_for_partition(partition_id: u8) -> Vec<i32> {
    let mut out: Vec<i32> = STEP_PARTITIONS
        .iter()
        .filter(|(_, tags)| tags.iter().any(|&t| t == partition_id))
        .map(|(n, _)| *n)
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// Domain mastery: earned drills / tagged drills for a partition.
pub fn partition_mastery(partition_id: u8, completed_levels: &[i32]) -> (usize, usize) {
    let drills = drills_for_partition(partition_id);
    let total = drills.len();
    let done = drills
        .iter()
        .filter(|n| completed_levels.iter().any(|c| c == *n))
        .count();
    (done, total)
}

pub fn mastery_percent(partition_id: u8, completed_levels: &[i32]) -> u8 {
    let (done, total) = partition_mastery(partition_id, completed_levels);
    if total == 0 {
        return 0;
    }
    ((done * 100) / total) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_partitions_stable() {
        assert_eq!(PARTITIONS.len(), PARTITION_COUNT as usize);
        for (i, p) in PARTITIONS.iter().enumerate() {
            assert_eq!(p.id, (i + 1) as u8);
            assert!(!p.short_label.is_empty());
            assert!(!p.mental_model.is_empty());
            assert!(!p.axes.is_empty());
        }
        assert!(PARTITIONS[3].map_only && PARTITIONS[4].map_only);
    }

    #[test]
    fn step_partitions_sorted_and_valid() {
        let mut prev = 0;
        for &(n, tags) in STEP_PARTITIONS {
            assert!(n > prev, "STEP_PARTITIONS must be sorted by micro_step");
            prev = n;
            assert!(n >= 1);
            assert!(coding_step_by_micro_step(n).is_some(), "missing step {n}");
            assert!(!tags.is_empty());
            for &t in tags {
                assert!(partition_by_id(t).is_some(), "bad partition {t} on {n}");
            }
        }
    }

    #[test]
    fn foundations_coverage() {
        assert!(partitions_for_micro_step(20).contains(&1));
        assert!(partitions_for_micro_step(62).contains(&2));
        assert!(partitions_for_micro_step(57).contains(&3));
        assert!(partitions_for_micro_step(63).contains(&4));
        assert!(!drills_for_partition(1).is_empty());
        assert!(!drills_for_partition(2).is_empty());
        assert!(!drills_for_partition(3).is_empty());
    }

    #[test]
    fn mastery_uses_completed_set() {
        let (done, total) = partition_mastery(1, &[20, 9999]);
        assert!(total > 0);
        assert_eq!(done, 1);
        assert_eq!(mastery_percent(1, &[]), 0);
    }
}
