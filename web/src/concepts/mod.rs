//! Conceptual partitions (lentes de razonamiento) over the coding rail.
//!
//! Pedagogy: five mutually exclusive *labels* for navigation — but a micro-step
//! may carry multiple tags (multi-label index). Does not alter progress storage.
//!
//! Wave B splits this crate path into a directory: drill tags stay here;
//! the search corpus lives in [`glossary`].

pub mod glossary;

#[allow(unused_imports)]
pub use glossary::{
    entry_by_id, group_search_hits, search_glossary, search_intent, ConceptLens, GlossaryEntry,
    PartitionId, SearchIntent, GLOSSARY_ENTRIES,
};

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

/// `(micro_step, partition_ids…)` — multi-label, sparse and conservative.
/// Keep sorted by micro_step ascending for binary search.
/// Untagged steps are intentional: DSA without a material conceptual lens.
const STEP_PARTITIONS: &[(i32, &[u8])] = &[
    (1, &[1]),
    (3, &[4]),
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
    (53, &[1, 2, 3]),
    (54, &[2, 3]),
    (55, &[2, 3]),
    (56, &[1]),
    (57, &[2, 3]),
    (58, &[2, 3]),
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
    (69, &[1]),
    (70, &[3, 5]),
    (71, &[3, 5]),
    (72, &[3, 5]),
    (73, &[3, 5]),
    (74, &[3, 4, 5]),
    (75, &[3, 5]),
    (76, &[1]),
    (77, &[1]),
    (78, &[1, 3]),
    (79, &[2, 3]),
    (80, &[2, 3]),
    (81, &[2, 3]),
    (82, &[1, 3]),
    (83, &[1, 3]),
    (84, &[1, 3]),
    (85, &[1, 3]),
    (86, &[1, 2, 3]),
    (87, &[1, 2, 3]),
    (88, &[1, 3]),
    (89, &[3]),
    (90, &[1, 3]),
    (91, &[3]),
    (92, &[1, 3]),
    (93, &[1, 3]),
    (94, &[1, 2, 3]),
    (95, &[1, 2, 3]),
    (96, &[1, 2, 3]),
    (97, &[3]),
    (98, &[3]),
    (99, &[1, 3]),
    (100, &[1, 3]),
    (101, &[1]),
    (102, &[1]),
    (103, &[3]),
    (105, &[1]),
    (106, &[3]),
    (107, &[3]),
    (108, &[3]),
    (109, &[3]),
    (110, &[3]),
    (111, &[3]),
    (112, &[3]),
    (113, &[4]),
    (114, &[4]),
    (115, &[2]),
    (116, &[2, 3]),
    (117, &[3]),
    (118, &[3]),
    (119, &[3]),
    (120, &[1, 2, 3]),
    (121, &[3]),
    (122, &[3]),
    (124, &[3]),
    (125, &[3]),
    (126, &[3]),
    (127, &[3]),
    (128, &[3]),
    (129, &[3]),
    (130, &[3]),
    (133, &[1, 2, 3]),
    (134, &[1, 2, 3]),
    (135, &[1, 3]),
    (137, &[3]),
    (140, &[1]),
    (141, &[1]),
    (143, &[3]),
    (144, &[3]),
    (145, &[3]),
    (149, &[3]),
    (150, &[3]),
    (151, &[3]),
    (152, &[3]),
    (153, &[3]),
    (154, &[2]),
    (155, &[1]),
    (157, &[1]),
    (161, &[1]),
    (162, &[1]),
    (163, &[1]),
    (167, &[1, 3]),
    (168, &[3]),
    (169, &[3]),
    (171, &[1]),
    (172, &[1, 3]),
    (173, &[3]),
    (174, &[3]),
    (181, &[1]),
    (182, &[1, 3]),
    (184, &[1, 3]),
    (185, &[2]),
    (189, &[2]),
    (191, &[4]),
    (192, &[4]),
    (193, &[1, 4]),
    (195, &[3]),
    (196, &[4]),
    (197, &[3]),
    (198, &[3]),
    (199, &[3]),
    (200, &[3]),
    (201, &[3]),
    (202, &[3]),
    (203, &[1, 2, 3]),
    (204, &[1, 3]),
    (205, &[3]),
    (206, &[3]),
    (207, &[3]),
    (208, &[3]),
    (209, &[1, 3]),
    (210, &[1]),
    (211, &[1]),
    (212, &[1, 3]),
    (213, &[1, 3]),
    (214, &[1, 3]),
    (219, &[1]),
    (222, &[3]),
    (227, &[1, 2, 3]),
    (228, &[1, 2, 3]),
    (229, &[1, 2, 3]),
    (230, &[1, 3]),
    (231, &[2, 3]),
    (232, &[2, 3]),
    (237, &[3]),
    (239, &[1, 2, 3]),
    (240, &[3]),
    (241, &[3]),
    (242, &[3]),
    (243, &[1]),
    (244, &[3]),
    (245, &[1]),
    (246, &[1]),
    (247, &[1]),
    (248, &[1]),
    (249, &[1]),
    (250, &[1]),
    (251, &[1, 3]),
    (252, &[1, 3]),
    (253, &[1]),
    (254, &[1]),
    (255, &[1]),
    (256, &[1]),
    (264, &[1]),
    (265, &[1]),
    (266, &[1, 3]),
    (267, &[1, 3]),
    (269, &[4]),
    (272, &[1, 3]),
    (274, &[3]),
    (275, &[3]),
    (277, &[2]),
    (278, &[2]),
    (282, &[3]),
    (283, &[3]),
    (298, &[1]),
    (305, &[2, 3]),
    (306, &[1, 3]),
    (307, &[1, 3]),
    (308, &[3]),
    (309, &[1, 3]),
    (310, &[3]),
    (311, &[3]),
    (314, &[1, 3]),
    (315, &[3]),
    (316, &[3]),
    (317, &[1, 2, 3]),
    (318, &[1, 2, 3]),
    (319, &[1, 2, 3]),
    (320, &[1, 2, 3]),
    (322, &[1, 2, 3]),
    (325, &[1]),
    (326, &[1]),
    (335, &[1, 3]),
    (336, &[1, 2, 3]),
    (337, &[1, 2, 3]),
    (338, &[1, 3]),
    (339, &[1, 3]),
    (340, &[1, 3]),
    (341, &[1, 3]),
    (342, &[1]),
    (343, &[1]),
    (344, &[1]),
    (345, &[1]),
    (346, &[1]),
    (347, &[1]),
    (348, &[1]),
    (349, &[1]),
    (350, &[1, 3]),
    (351, &[1, 3]),
    (352, &[1]),
    (353, &[1, 2]),
    (354, &[1, 2]),
    (355, &[1, 2]),
    (356, &[1, 2]),
    (357, &[1, 2]),
    (358, &[1, 2]),
    (363, &[3]),
    (364, &[3]),
    (365, &[4]),
    (366, &[4]),
    (367, &[1, 4]),
    (369, &[4]),
    (370, &[4]),
    (383, &[1, 2, 3]),
    (384, &[1]),
    (385, &[3]),
    (386, &[1]),
    (387, &[1, 3]),
    (388, &[1]),
    (389, &[1, 3]),
    (390, &[1, 3]),
    (391, &[1, 3]),
    (392, &[1, 3]),
    (393, &[1, 3]),
    (394, &[1, 3]),
    (395, &[1]),
    (396, &[1, 3]),
    (398, &[1]),
    (399, &[1]),
    (400, &[1]),
    (401, &[1]),
    (402, &[1]),
    (403, &[1, 3]),
    (404, &[1]),
    (406, &[1]),
    (407, &[1]),
    (408, &[1]),
    (409, &[1]),
    (411, &[1]),
    (414, &[1]),
    (417, &[1, 3]),
    (419, &[1, 3]),
    (420, &[1, 3]),
    (421, &[1, 3]),
    (422, &[1, 3]),
    (423, &[1, 3]),
    (424, &[1, 3]),
    (425, &[3]),
    (426, &[3]),
    (427, &[3]),
    (431, &[1]),
    (434, &[1]),
    (435, &[1]),
    (436, &[1]),
    (443, &[1]),
    (444, &[1, 3]),
    (447, &[1]),
    (448, &[1]),
    (449, &[1]),
    (450, &[1]),
    (463, &[3]),
    (464, &[3]),
    (465, &[3]),
    (466, &[3]),
    (467, &[3]),
    (468, &[3]),
    (469, &[1, 3]),
    (470, &[1, 3]),
    (471, &[3]),
    (472, &[1]),
    (473, &[1, 2, 3]),
    (474, &[2, 3]),
    (475, &[1, 3]),
    (476, &[1, 3]),
    (477, &[1, 3]),
    (478, &[3]),
    (479, &[1, 3]),
    (480, &[3]),
    (481, &[4]),
    (482, &[1, 4]),
    (483, &[4]),
    (484, &[4]),
    (485, &[1, 4]),
    (486, &[1, 4]),
    (487, &[1]),
    (488, &[1]),
    (489, &[1, 3]),
    (490, &[1, 3]),
    (491, &[1]),
    (492, &[1]),
    (493, &[1]),
    (494, &[1, 3]),
    (495, &[1, 3]),
    (496, &[1]),
    (498, &[1]),
    (499, &[1]),
    (500, &[1]),
    (501, &[1]),
    (502, &[1]),
    (505, &[1]),
    (506, &[1]),
    (508, &[1]),
    (510, &[1]),
    (511, &[3]),
    (512, &[3]),
    (513, &[1]),
    (515, &[3]),
    (523, &[1, 3]),
    (524, &[1, 3]),
    (525, &[1]),
    (526, &[1, 3]),
    (527, &[1, 3]),
    (528, &[1, 3]),
    (529, &[1, 3]),
    (531, &[3]),
    (532, &[1, 3]),
    (533, &[1, 3]),
    (534, &[3]),
    (538, &[3]),
    (540, &[3]),
    (553, &[1, 2, 3]),
    (554, &[1, 2, 3]),
    (555, &[1, 2, 3]),
    (556, &[1, 2, 3]),
    (557, &[1, 2, 3]),
    (558, &[1, 2, 3]),
    (561, &[1]),
    (563, &[1]),
    (568, &[1]),
    (569, &[1]),
    (571, &[1]),
    (577, &[1, 3]),
    (578, &[1, 3]),
    (579, &[1, 3]),
    (580, &[1, 3]),
    (581, &[1, 3]),
    (582, &[1, 3]),
    (583, &[1, 3]),
    (584, &[3]),
    (585, &[2, 3]),
    (586, &[3]),
    (587, &[1, 3]),
    (588, &[3]),
    (589, &[1, 3]),
    (590, &[3]),
    (591, &[1, 3]),
    (592, &[1, 2]),
    (593, &[3]),
    (594, &[1, 2, 3]),
    (595, &[1]),
    (596, &[1]),
    (597, &[1]),
    (600, &[1]),
    (601, &[1]),
    (602, &[1]),
    (603, &[1, 3]),
    (604, &[1]),
    (605, &[1]),
    (606, &[1]),
    (607, &[1]),
    (608, &[1]),
    (609, &[1]),
    (611, &[1]),
    (612, &[1]),
    (619, &[1, 2]),
    (620, &[1, 2]),
    (621, &[1, 2]),
    (622, &[1, 2]),
    (623, &[1, 2]),
    (624, &[1, 2]),
    (625, &[1, 3]),
    (626, &[2]),
    (627, &[1, 3]),
    (628, &[1, 3]),
    (629, &[1, 3]),
    (631, &[1, 3]),
    (632, &[1, 3]),
    (633, &[1, 3]),
    (634, &[1]),
    (635, &[3]),
    (636, &[3]),
    (637, &[3]),
    (638, &[3]),
    (640, &[3]),
    (643, &[1, 3]),
    (644, &[1, 3]),
    (645, &[3]),
    (646, &[1, 3]),
    (647, &[3]),
    (648, &[1, 3]),
    (655, &[1, 3]),
    (656, &[1, 3]),
    (657, &[1]),
    (658, &[3]),
    (659, &[1, 3]),
    (660, &[3]),
    (661, &[1, 3]),
    (662, &[3]),
    (663, &[3]),
    (667, &[3]),
    (668, &[3]),
    (669, &[1, 3]),
    (670, &[1, 2, 3]),
    (671, &[1, 2, 3]),
    (672, &[1, 2, 3]),
    (673, &[3]),
    (674, &[1, 3]),
    (675, &[1, 3]),
    (676, &[2, 3]),
    (677, &[1, 3]),
    (678, &[1, 2, 3]),
    (679, &[1]),
    (680, &[1]),
    (681, &[1]),
    (682, &[1]),
    (683, &[1]),
    (684, &[1]),
    (685, &[4]),
    (686, &[4]),
    (687, &[1, 4]),
    (688, &[4]),
    (689, &[4]),
    (690, &[1, 4]),
    (691, &[1]),
    (692, &[1]),
    (697, &[1, 3]),
    (698, &[1, 3]),
    (699, &[1, 3]),
    (700, &[1, 3]),
    (701, &[1, 3]),
    (702, &[1, 3]),
    (703, &[1, 3]),
    (704, &[1, 3]),
    (705, &[1, 3]),
    (706, &[1, 3]),
    (707, &[1, 3]),
    (708, &[1, 3]),
    (709, &[1, 3]),
    (710, &[1, 3]),
    (711, &[1, 3]),
    (712, &[1, 3]),
    (713, &[1, 3]),
    (714, &[1, 3]),
    (715, &[1]),
    (718, &[1]),
    (721, &[1]),
    (722, &[1]),
    (727, &[3]),
    (730, &[3]),
    (745, &[1]),
    (746, &[1]),
    (748, &[1]),
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

/// Value for `data-mastery` on compass controls (`"0"`..=`"100"`).
pub fn mastery_attr(partition_id: u8, completed_levels: &[i32]) -> String {
    mastery_percent(partition_id, completed_levels).to_string()
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
        assert!(
            STEP_PARTITIONS.len() >= 200,
            "expected curriculum-wide conservative coverage"
        );
    }

    #[test]
    fn foundations_coverage() {
        assert_eq!(partitions_for_micro_step(1), &[1]);
        assert!(partitions_for_micro_step(2).is_empty());
        assert_eq!(partitions_for_micro_step(20), &[1]);
        assert_eq!(partitions_for_micro_step(62), &[2]);
        assert_eq!(partitions_for_micro_step(57), &[2, 3]);
        assert_eq!(partitions_for_micro_step(63), &[4]);
        assert_eq!(partitions_for_micro_step(69), &[1]);
        assert_eq!(partitions_for_micro_step(78), &[1, 3]);
        assert!(!drills_for_partition(1).is_empty());
        assert!(!drills_for_partition(2).is_empty());
        assert!(!drills_for_partition(3).is_empty());
    }

    #[test]
    fn applied_families_use_specific_lenses() {
        assert_eq!(partitions_for_micro_step(120), &[1, 2, 3]); // memo + closure + DP
        assert_eq!(partitions_for_micro_step(135), &[1, 3]); // trie
        assert_eq!(partitions_for_micro_step(317), &[1, 2, 3]); // backtracking
        assert_eq!(partitions_for_micro_step(353), &[1, 2]); // union-find
        assert!(partitions_for_micro_step(517).is_empty()); // two pointers: no conceptual lens
        assert_eq!(partitions_for_micro_step(625), &[1, 3]); // trie
        assert!(partitions_for_micro_step(853).is_empty()); // DSA lab without conceptual lens
        assert!(partitions_for_micro_step(913).is_empty()); // beyond tagged advanced structures
        assert_eq!(partitions_for_micro_step(708), &[1, 3]); // segment tree ADT
        assert_eq!(partitions_for_micro_step(721), &[1]); // arrays VI: prefix/data load
        assert!(partitions_for_micro_step(733).is_empty()); // window VI recipe
        assert!(partitions_for_micro_step(1001).is_empty());
    }

    #[test]
    fn expanded_drills_resolve_to_catalog() {
        for partition_id in 1..=3 {
            let drills = drills_for_partition(partition_id);
            assert!(
                drills.len() >= 10,
                "partition {partition_id} lacks representative drills"
            );
            assert!(
                drills
                    .iter()
                    .all(|&n| coding_step_by_micro_step(n).is_some()),
                "partition {partition_id} contains unknown drills"
            );
        }
        // Intentional sparsity: most advanced DSA stays untagged.
        assert!(partitions_for_micro_step(955).is_empty());
        assert!(partitions_for_micro_step(1000).is_empty());
    }

    #[test]
    fn mastery_uses_completed_set() {
        let (done, total) = partition_mastery(1, &[20, 9999]);
        assert!(total > 0);
        assert_eq!(done, 1);
        assert_eq!(mastery_percent(1, &[]), 0);
    }

    #[test]
    fn wave_a_foundations_dense_and_floors() {
        for n in 4..=100 {
            if coding_step_by_micro_step(n).is_some() {
                assert!(
                    !partitions_for_micro_step(n).is_empty(),
                    "Wave A hole at micro_step {n}"
                );
            }
        }
        assert!(drills_for_partition(1).len() >= 40);
        assert!(drills_for_partition(2).len() >= 15);
        assert!(drills_for_partition(3).len() >= 35);
    }

    #[test]
    fn wave_a_no_bulk_paradigm_on_applied_dsa() {
        let tagged: Vec<i32> = (101..=160)
            .filter(|&n| !partitions_for_micro_step(n).is_empty())
            .collect();
        assert!(!tagged.is_empty());
        let p3 = tagged
            .iter()
            .filter(|&&n| partitions_for_micro_step(n).contains(&3))
            .count();
        assert!(
            p3 < tagged.len(),
            "101..=160 must not be 100% partition 3 (got {p3}/{})",
            tagged.len()
        );
    }

    #[test]
    fn mastery_percent_reflects_synthetic_completed_levels() {
        assert_eq!(mastery_percent(2, &[]), 0);
        let after = mastery_percent(2, &[52]);
        assert!(
            after > 0,
            "completing a P2 drill must raise data-mastery above 0"
        );
        assert_eq!(mastery_attr(2, &[]), "0");
        assert_eq!(mastery_attr(2, &[52]), after.to_string());
    }

    /// C1 contract: `(micro_step, tags)` in `301..=450` at merge `d0144ea`.
    const WAVE_C1_SNAPSHOT_301_450: &[(i32, &[u8])] = &[
        (305, &[2, 3]),
        (306, &[1, 3]),
        (307, &[1, 3]),
        (308, &[3]),
        (309, &[1, 3]),
        (310, &[3]),
        (311, &[3]),
        (314, &[1, 3]),
        (315, &[3]),
        (316, &[3]),
        (317, &[1, 2, 3]),
        (318, &[1, 2, 3]),
        (319, &[1, 2, 3]),
        (320, &[1, 2, 3]),
        (322, &[1, 2, 3]),
        (325, &[1]),
        (326, &[1]),
        (335, &[1, 3]),
        (336, &[1, 2, 3]),
        (337, &[1, 2, 3]),
        (338, &[1, 3]),
        (339, &[1, 3]),
        (340, &[1, 3]),
        (341, &[1, 3]),
        (342, &[1]),
        (343, &[1]),
        (344, &[1]),
        (345, &[1]),
        (346, &[1]),
        (347, &[1]),
        (348, &[1]),
        (349, &[1]),
        (350, &[1, 3]),
        (351, &[1, 3]),
        (352, &[1]),
        (353, &[1, 2]),
        (354, &[1, 2]),
        (355, &[1, 2]),
        (356, &[1, 2]),
        (357, &[1, 2]),
        (358, &[1, 2]),
        (363, &[3]),
        (364, &[3]),
        (365, &[4]),
        (366, &[4]),
        (367, &[1, 4]),
        (369, &[4]),
        (370, &[4]),
        (383, &[1, 2, 3]),
        (384, &[1]),
        (385, &[3]),
        (386, &[1]),
        (387, &[1, 3]),
        (388, &[1]),
        (389, &[1, 3]),
        (390, &[1, 3]),
        (391, &[1, 3]),
        (392, &[1, 3]),
        (393, &[1, 3]),
        (394, &[1, 3]),
        (395, &[1]),
        (396, &[1, 3]),
        (398, &[1]),
        (399, &[1]),
        (400, &[1]),
        (401, &[1]),
        (402, &[1]),
        (403, &[1, 3]),
        (404, &[1]),
        (406, &[1]),
        (407, &[1]),
        (408, &[1]),
        (409, &[1]),
        (411, &[1]),
        (414, &[1]),
        (417, &[1, 3]),
        (419, &[1, 3]),
        (420, &[1, 3]),
        (421, &[1, 3]),
        (422, &[1, 3]),
        (423, &[1, 3]),
        (424, &[1, 3]),
        (425, &[3]),
        (426, &[3]),
        (427, &[3]),
        (431, &[1]),
        (434, &[1]),
        (435, &[1]),
        (436, &[1]),
        (443, &[1]),
        (444, &[1, 3]),
        (447, &[1]),
        (448, &[1]),
        (449, &[1]),
        (450, &[1]),
    ];

    /// C2 contract: `(micro_step, tags)` in `451..=600` at merge `7603e55`.
    const WAVE_C2_SNAPSHOT_451_600: &[(i32, &[u8])] = &[
        (463, &[3]),
        (464, &[3]),
        (465, &[3]),
        (466, &[3]),
        (467, &[3]),
        (468, &[3]),
        (469, &[1, 3]),
        (470, &[1, 3]),
        (471, &[3]),
        (472, &[1]),
        (473, &[1, 2, 3]),
        (474, &[2, 3]),
        (475, &[1, 3]),
        (476, &[1, 3]),
        (477, &[1, 3]),
        (478, &[3]),
        (479, &[1, 3]),
        (480, &[3]),
        (481, &[4]),
        (482, &[1, 4]),
        (483, &[4]),
        (484, &[4]),
        (485, &[1, 4]),
        (486, &[1, 4]),
        (487, &[1]),
        (488, &[1]),
        (489, &[1, 3]),
        (490, &[1, 3]),
        (491, &[1]),
        (492, &[1]),
        (493, &[1]),
        (494, &[1, 3]),
        (495, &[1, 3]),
        (496, &[1]),
        (498, &[1]),
        (499, &[1]),
        (500, &[1]),
        (501, &[1]),
        (502, &[1]),
        (505, &[1]),
        (506, &[1]),
        (508, &[1]),
        (510, &[1]),
        (511, &[3]),
        (512, &[3]),
        (513, &[1]),
        (515, &[3]),
        (523, &[1, 3]),
        (524, &[1, 3]),
        (525, &[1]),
        (526, &[1, 3]),
        (527, &[1, 3]),
        (528, &[1, 3]),
        (529, &[1, 3]),
        (531, &[3]),
        (532, &[1, 3]),
        (533, &[1, 3]),
        (534, &[3]),
        (538, &[3]),
        (540, &[3]),
        (553, &[1, 2, 3]),
        (554, &[1, 2, 3]),
        (555, &[1, 2, 3]),
        (556, &[1, 2, 3]),
        (557, &[1, 2, 3]),
        (558, &[1, 2, 3]),
        (561, &[1]),
        (563, &[1]),
        (568, &[1]),
        (569, &[1]),
        (571, &[1]),
        (577, &[1, 3]),
        (578, &[1, 3]),
        (579, &[1, 3]),
        (580, &[1, 3]),
        (581, &[1, 3]),
        (582, &[1, 3]),
        (583, &[1, 3]),
        (584, &[3]),
        (585, &[2, 3]),
        (586, &[3]),
        (587, &[1, 3]),
        (588, &[3]),
        (589, &[1, 3]),
        (590, &[3]),
        (591, &[1, 3]),
        (592, &[1, 2]),
        (593, &[3]),
        (594, &[1, 2, 3]),
        (595, &[1]),
        (596, &[1]),
        (597, &[1]),
        (600, &[1]),
    ];

    /// Frozen `(micro_step, tags)` pairs with `micro_step ≥ 751` at C3 merge `7747cb1`.
    const WAVE_C3_FROZEN_751: &[(i32, &[u8])] = &[];

    #[test]
    fn wave_b_applied_floor_101_to_300() {
        let tagged: Vec<i32> = (101..=300)
            .filter(|&n| {
                coding_step_by_micro_step(n).is_some() && !partitions_for_micro_step(n).is_empty()
            })
            .collect();
        assert!(
            tagged.len() >= 120,
            "101..=300 tagged floor is 120, got {}",
            tagged.len()
        );
        for n in &tagged {
            for &t in partitions_for_micro_step(*n) {
                assert!((1..=5).contains(&t), "bad tag {t} on {n}");
            }
            assert!(coding_step_by_micro_step(*n).is_some());
        }
        let p3 = tagged
            .iter()
            .filter(|&&n| partitions_for_micro_step(n).contains(&3))
            .count();
        assert!(
            p3 < tagged.len(),
            "101..=300 must not be 100% partition 3 (got {p3}/{})",
            tagged.len()
        );
    }

    #[test]
    fn wave_b_two_pointer_family_stays_untagged() {
        for n in [131, 132, 175] {
            assert!(
                partitions_for_micro_step(n).is_empty(),
                "micro_step {n} must stay untagged (two pointers / window)"
            );
        }
    }

    #[test]
    fn wave_c1_applied_floor_301_to_450() {
        let tagged: Vec<i32> = (301..=450)
            .filter(|&n| {
                coding_step_by_micro_step(n).is_some() && !partitions_for_micro_step(n).is_empty()
            })
            .collect();
        assert!(
            tagged.len() >= 90,
            "301..=450 tagged floor is 90, got {}",
            tagged.len()
        );
        for n in &tagged {
            for &t in partitions_for_micro_step(*n) {
                assert!((1..=5).contains(&t), "bad tag {t} on {n}");
            }
            assert!(coding_step_by_micro_step(*n).is_some());
        }
        let p3 = tagged
            .iter()
            .filter(|&&n| partitions_for_micro_step(n).contains(&3))
            .count();
        assert!(
            p3 < tagged.len(),
            "301..=450 must not be 100% partition 3 (got {p3}/{})",
            tagged.len()
        );
    }

    #[test]
    fn wave_c1_bit_window_two_pointer_family_stays_untagged() {
        for n in [301, 329, 371] {
            assert!(
                partitions_for_micro_step(n).is_empty(),
                "micro_step {n} must stay untagged (bits / two pointers / window)"
            );
        }
    }

    #[test]
    fn wave_c1_range_301_to_450_unchanged() {
        let current: Vec<(i32, &[u8])> = STEP_PARTITIONS
            .iter()
            .copied()
            .filter(|(n, _)| (301..=450).contains(n))
            .collect();
        assert_eq!(
            current.len(),
            WAVE_C1_SNAPSHOT_301_450.len(),
            "do not add or remove rows in 301..=450"
        );
        for (got, expected) in current.iter().zip(WAVE_C1_SNAPSHOT_301_450.iter()) {
            assert_eq!(got.0, expected.0, "micro_step drift in 301..=450");
            assert_eq!(got.1, expected.1, "C1 tags changed for micro_step {}", got.0);
        }
    }

    #[test]
    fn wave_c2_applied_floor_451_to_600() {
        let tagged: Vec<i32> = (451..=600)
            .filter(|&n| {
                coding_step_by_micro_step(n).is_some() && !partitions_for_micro_step(n).is_empty()
            })
            .collect();
        assert!(
            tagged.len() >= 90,
            "451..=600 tagged floor is 90, got {}",
            tagged.len()
        );
        for n in &tagged {
            for &t in partitions_for_micro_step(*n) {
                assert!((1..=5).contains(&t), "bad tag {t} on {n}");
            }
            assert!(coding_step_by_micro_step(*n).is_some());
        }
        let p3 = tagged
            .iter()
            .filter(|&&n| partitions_for_micro_step(n).contains(&3))
            .count();
        assert!(
            p3 < tagged.len(),
            "451..=600 must not be 100% partition 3 (got {p3}/{})",
            tagged.len()
        );
    }

    #[test]
    fn wave_c2_recipe_families_stay_untagged() {
        for n in [518, 543, 547] {
            assert!(
                partitions_for_micro_step(n).is_empty(),
                "micro_step {n} must stay untagged (two pointers / window / bits)"
            );
        }
    }

    #[test]
    fn wave_c2_range_451_to_600_unchanged() {
        let current: Vec<(i32, &[u8])> = STEP_PARTITIONS
            .iter()
            .copied()
            .filter(|(n, _)| (451..=600).contains(n))
            .collect();
        assert_eq!(
            current.len(),
            WAVE_C2_SNAPSHOT_451_600.len(),
            "do not add or remove rows in 451..=600"
        );
        for (got, expected) in current.iter().zip(WAVE_C2_SNAPSHOT_451_600.iter()) {
            assert_eq!(got.0, expected.0, "micro_step drift in 451..=600");
            assert_eq!(got.1, expected.1, "C2 tags changed for micro_step {}", got.0);
        }
    }

    #[test]
    fn wave_c4_applied_floor_601_to_750() {
        let tagged: Vec<i32> = (601..=750)
            .filter(|&n| {
                coding_step_by_micro_step(n).is_some() && !partitions_for_micro_step(n).is_empty()
            })
            .collect();
        assert!(
            tagged.len() >= 90,
            "601..=750 tagged floor is 90, got {}",
            tagged.len()
        );
        for n in &tagged {
            for &t in partitions_for_micro_step(*n) {
                assert!((1..=5).contains(&t), "bad tag {t} on {n}");
            }
            assert!(coding_step_by_micro_step(*n).is_some());
        }
        let p3 = tagged
            .iter()
            .filter(|&&n| partitions_for_micro_step(n).contains(&3))
            .count();
        assert!(
            p3 < tagged.len(),
            "601..=750 must not be 100% partition 3 (got {p3}/{})",
            tagged.len()
        );
    }

    #[test]
    fn wave_c4_recipe_families_stay_untagged() {
        for n in [613, 630, 739] {
            assert!(
                partitions_for_micro_step(n).is_empty(),
                "micro_step {n} must stay untagged (window / bits / two pointers)"
            );
        }
    }

    #[test]
    fn wave_c4_freeze_rows_751_and_up() {
        let current: Vec<(i32, &[u8])> = STEP_PARTITIONS
            .iter()
            .copied()
            .filter(|(n, _)| *n >= 751)
            .collect();
        assert_eq!(
            current.as_slice(),
            WAVE_C3_FROZEN_751,
            "do not add or remove rows ≥ 751"
        );
    }
}
