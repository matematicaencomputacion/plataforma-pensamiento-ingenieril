//! Conceptual partitions (lentes de razonamiento) over the coding rail.
//!
//! Pedagogy: five mutually exclusive *labels* for navigation — but a micro-step
//! may carry multiple tags (multi-label index). Does not alter progress storage.
//!
//! Wave B splits this crate path into a directory: drill tags stay here;
//! the search corpus lives in [`glossary`]. Wave D.4 adds a static DAG in [`dag`].

pub mod dag;
pub mod glossary;

#[allow(unused_imports)]
pub use dag::{
    concept_drill_micros, concept_started, edges_for_partition, missing_required_bases,
    ConceptEdge, EdgeKind, CONCEPT_EDGES,
};
#[allow(unused_imports)]
pub use glossary::{
    entry_by_id, group_search_hits, search_glossary, search_intent, ConceptLens, GlossaryEntry,
    PartitionId, SearchIntent, GLOSSARY_ENTRIES,
};

use crate::curriculum::{coding_step_by_id, coding_step_by_micro_step};

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
    (751, &[1]),
    (752, &[1]),
    (754, &[1]),
    (755, &[1]),
    (756, &[1]),
    (757, &[1]),
    (758, &[1]),
    (760, &[1]),
    (761, &[1]),
    (762, &[1]),
    (763, &[1]),
    (764, &[1]),
    (766, &[1]),
    (767, &[1]),
    (768, &[1]),
    (769, &[1, 3]),
    (770, &[1, 3]),
    (772, &[1, 3]),
    (773, &[1, 2, 3]),
    (774, &[1, 3]),
    (775, &[1, 3]),
    (776, &[1, 2, 3]),
    (778, &[1, 3]),
    (779, &[1, 2, 3]),
    (780, &[1, 3]),
    (781, &[1, 3]),
    (782, &[1, 3]),
    (784, &[1, 3]),
    (785, &[3]),
    (786, &[1, 3]),
    (787, &[1, 3]),
    (788, &[1, 3]),
    (790, &[1, 3]),
    (791, &[3]),
    (792, &[1, 3]),
    (793, &[3]),
    (794, &[3]),
    (796, &[3]),
    (797, &[1, 3]),
    (798, &[3]),
    (799, &[1, 2]),
    (800, &[1, 2]),
    (802, &[1, 2]),
    (803, &[1, 2]),
    (804, &[1, 2]),
    (805, &[1, 3]),
    (806, &[1, 3]),
    (808, &[1, 3]),
    (809, &[3]),
    (810, &[1, 3]),
    (811, &[3]),
    (812, &[3]),
    (814, &[3]),
    (815, &[1, 3]),
    (816, &[3]),
    (817, &[1, 3]),
    (818, &[1, 3]),
    (820, &[1, 3]),
    (821, &[1, 3]),
    (822, &[1, 3]),
    (823, &[4]),
    (824, &[4]),
    (826, &[1, 4]),
    (827, &[4]),
    (828, &[1, 4]),
    (829, &[1, 3]),
    (830, &[1, 3]),
    (832, &[1]),
    (833, &[3]),
    (834, &[3]),
    (835, &[3]),
    (838, &[3]),
    (840, &[3]),
    (841, &[1, 3]),
    (842, &[1, 3]),
    (844, &[3]),
    (845, &[1, 3]),
    (846, &[3]),
    (847, &[1, 3]),
    (848, &[1, 3]),
    (850, &[3]),
    (851, &[1, 3]),
    (852, &[3]),
    (855, &[1, 2, 3]),
    (856, &[1, 2, 3]),
    (857, &[1, 2, 3]),
    (858, &[1, 2, 3]),
    (883, &[1]),
    (884, &[1]),
    (886, &[1]),
    (887, &[1]),
    (888, &[1]),
    (889, &[1]),
    (890, &[1]),
    (892, &[1]),
    (893, &[1]),
    (894, &[1]),
    (895, &[1]),
    (896, &[1]),
    (898, &[1]),
    (899, &[1]),
    (900, &[1]),
    (901, &[1]),
    (902, &[1]),
    (904, &[1]),
    (905, &[1]),
    (906, &[1]),
    (907, &[1, 3]),
    (908, &[1, 3]),
    (910, &[1, 3]),
    (911, &[1, 3]),
    (912, &[1, 3]),
    (914, &[1, 3]),
    (916, &[1, 3]),
    (917, &[1, 3]),
    (918, &[1, 3]),
    (919, &[1, 3]),
    (920, &[1, 3]),
    (922, &[1, 3]),
    (923, &[1, 3]),
    (924, &[1, 3]),
    (925, &[1, 3]),
    (926, &[1, 3]),
    (928, &[1, 3]),
    (929, &[1, 3]),
    (930, &[1, 3]),
    (931, &[1]),
    (932, &[1]),
    (934, &[1]),
    (935, &[1]),
    (936, &[1]),
    (937, &[1]),
    (938, &[1]),
    (940, &[1]),
    (941, &[1]),
    (942, &[1]),
    (943, &[1, 3]),
    (944, &[1, 3]),
    (946, &[1, 3]),
    (947, &[1, 3]),
    (948, &[1, 3]),
    (949, &[3]),
    (950, &[3]),
    (952, &[3]),
    (953, &[1, 3]),
    (954, &[3]),
    (956, &[1, 3]),
    (958, &[1, 3]),
    (959, &[1, 3]),
    (960, &[1, 3]),
    (961, &[3]),
    (962, &[3]),
    (964, &[3]),
    (965, &[3]),
    (966, &[3]),
    (973, &[1, 3]),
    (974, &[1, 3]),
    (976, &[1, 3]),
    (977, &[1, 3]),
    (978, &[1, 3]),
    (979, &[3]),
    (980, &[3]),
    (982, &[3]),
    (997, &[1]),
    (998, &[1]),
    (1001, &[1, 3]),
    (1002, &[1, 3]),
    (1003, &[1, 3]),
    (1004, &[1, 3]),
    (1005, &[1, 3]),
    (1006, &[1, 3]),
    (1007, &[3]),
    (1008, &[1, 3]),
    (1009, &[1, 3]),
    (1010, &[3]),
    (1011, &[1, 3]),
    (1012, &[3]),
    (1013, &[2]),
    (1014, &[2]),
    (1015, &[2]),
    (1016, &[2, 3]),
    (1017, &[2]),
    (1018, &[2]),
    (1019, &[1]),
    (1020, &[1]),
    (1021, &[1, 3]),
    (1022, &[1]),
    (1023, &[1]),
    (1024, &[1]),
    (1025, &[3]),
    (1026, &[3]),
    (1027, &[3]),
    (1028, &[3]),
    (1029, &[3]),
    (1030, &[3]),
    (1031, &[1]),
    (1032, &[1]),
    (1033, &[1]),
    (1034, &[1]),
    (1035, &[1]),
    (1036, &[1]),
    (1037, &[4]),
    (1038, &[4]),
    (1039, &[4]),
    (1040, &[4]),
    (1041, &[4]),
    (1042, &[4]),
    (1043, &[4]),
    (1044, &[4]),
    (1045, &[4]),
    (1046, &[4]),
    (1047, &[4]),
    (1048, &[4]),
    (1049, &[1, 3]),
    (1050, &[1, 3]),
    (1051, &[1]),
    (1052, &[1, 3]),
    (1053, &[1]),
    (1054, &[1, 3]),
    (1055, &[3]),
    (1056, &[3]),
    (1057, &[3]),
    (1058, &[3]),
    (1059, &[3]),
    (1060, &[3]),
    (1061, &[4]),
    (1062, &[4]),
    (1063, &[4]),
    (1064, &[4]),
    (1065, &[4]),
    (1066, &[4]),
    (1067, &[4]),
    (1068, &[4]),
    (1069, &[4]),
    (1070, &[4]),
    (1071, &[4]),
    (1072, &[4]),
    (1073, &[3]),
    (1074, &[3]),
    (1075, &[3]),
    (1076, &[3]),
    (1077, &[3]),
    (1078, &[3]),
    (1079, &[3]),
    (1080, &[3]),
    (1081, &[3]),
    (1082, &[3]),
    (1083, &[3]),
    (1084, &[3]),
    (1085, &[3]),
    (1086, &[3]),
    (1087, &[3]),
    (1088, &[3]),
    (1089, &[3]),
    (1090, &[3]),
    (1091, &[3]),
    (1092, &[3]),
    (1093, &[3]),
    (1094, &[3]),
    (1095, &[3]),
    (1096, &[3]),
    (1097, &[1, 3]),
    (1098, &[1, 3]),
    (1099, &[1, 3]),
    (1100, &[1, 3]),
    (1101, &[1, 3]),
    (1102, &[1, 3]),
    (1103, &[3]),
    (1104, &[3]),
    (1105, &[3]),
    (1106, &[3]),
    (1107, &[3]),
    (1108, &[3]),
    (1109, &[1, 3]),
    (1110, &[1, 3]),
    (1111, &[1, 3]),
    (1112, &[1, 3]),
    (1113, &[1, 3]),
    (1114, &[1, 3]),
    (1115, &[4, 5]),
    (1116, &[4, 5]),
    (1117, &[4, 5]),
    (1118, &[4, 5]),
    (1119, &[4, 5]),
    (1120, &[4, 5]),
    (1121, &[1, 3]),
    (1122, &[1, 3]),
    (1123, &[1, 3]),
    (1124, &[1, 3]),
    (1125, &[1, 3]),
    (1126, &[1, 3]),
    (1127, &[3]),
    (1128, &[3]),
    (1129, &[3]),
    (1130, &[3]),
    (1131, &[3]),
    (1132, &[3]),
    (1133, &[1, 3]),
    (1134, &[1, 3]),
    (1135, &[1, 3]),
    (1136, &[1, 3]),
    (1137, &[1, 3]),
    (1138, &[1, 3]),
    (1139, &[3, 4]),
    (1140, &[3, 4]),
    (1141, &[3, 4]),
    (1142, &[3, 4]),
    (1143, &[3, 4]),
    (1144, &[3, 4]),
    (1145, &[3, 4]),
    (1146, &[3, 4]),
    (1147, &[3, 4]),
    (1148, &[3, 4]),
    (1149, &[3, 4]),
    (1150, &[3, 4]),
    (1151, &[3]),
    (1152, &[3]),
    (1153, &[3]),
    (1154, &[3]),
    (1155, &[3]),
    (1156, &[3]),
    (1157, &[4]),
    (1158, &[4]),
    (1159, &[4]),
    (1160, &[4]),
    (1161, &[4]),
    (1162, &[4]),
    (1163, &[5]),
    (1164, &[5]),
    (1165, &[5]),
    (1166, &[5]),
    (1167, &[5]),
    (1168, &[5]),
    (1169, &[5]),
    (1170, &[5]),
    (1171, &[5]),
    (1172, &[5]),
    (1173, &[5]),
    (1174, &[5]),
    (1175, &[1, 5]),
    (1176, &[1, 5]),
    (1177, &[1, 5]),
    (1178, &[1, 5]),
    (1179, &[1, 5]),
    (1180, &[1, 5]),
    (1181, &[1, 5]),
    (1182, &[1, 5]),
    (1183, &[1, 5]),
    (1184, &[1, 5]),
    (1185, &[1, 5]),
    (1186, &[1, 5]),
    (1187, &[1, 5]),
    (1188, &[1, 5]),
    (1189, &[1, 5]),
    (1190, &[1, 5]),
    (1191, &[1, 5]),
    (1192, &[1, 5]),
    (1193, &[1, 5]),
    (1194, &[1, 5]),
    (1195, &[1, 5]),
    (1196, &[1, 5]),
    (1197, &[1, 5]),
    (1198, &[1, 5]),
    (1199, &[1, 3]),
    (1200, &[1, 3]),
    (1201, &[1, 3]),
    (1202, &[1, 3]),
    (1203, &[1, 3]),
    (1204, &[1, 3]),
    (1205, &[1, 3]),
    (1206, &[1, 3]),
    (1207, &[1, 3]),
    (1208, &[1, 3]),
    (1209, &[1, 3]),
    (1210, &[1, 3]),
    (1211, &[1]),
    (1212, &[1]),
    (1213, &[1]),
    (1214, &[1]),
    (1215, &[1]),
    (1216, &[1]),
    (1217, &[3, 5]),
    (1218, &[3, 5]),
    (1219, &[3, 5]),
    (1220, &[3, 5]),
    (1221, &[3, 5]),
    (1222, &[3, 5]),
    (1223, &[5]),
    (1224, &[5]),
    (1225, &[5]),
    (1226, &[5]),
    (1227, &[5]),
    (1228, &[5]),
    (1229, &[1]),
    (1230, &[1]),
    (1231, &[1]),
    (1232, &[1]),
    (1233, &[1]),
    (1234, &[1]),
    (1235, &[1, 5]),
    (1236, &[1, 5]),
    (1237, &[1, 5]),
    (1238, &[1, 5]),
    (1239, &[1, 5]),
    (1240, &[1, 5]),
    (1241, &[4, 5]),
    (1242, &[4, 5]),
    (1243, &[4, 5]),
    (1244, &[4, 5]),
    (1245, &[4, 5]),
    (1246, &[4, 5]),
    (1247, &[4, 5]),
    (1248, &[4, 5]),
    (1249, &[4, 5]),
    (1250, &[4, 5]),
    (1251, &[4, 5]),
    (1252, &[4, 5]),
    (1253, &[4, 5]),
    (1254, &[4, 5]),
    (1255, &[4, 5]),
    (1256, &[4, 5]),
    (1257, &[4, 5]),
    (1258, &[4, 5]),
    (1259, &[4, 5]),
    (1260, &[4, 5]),
    (1261, &[4, 5]),
    (1262, &[4, 5]),
    (1263, &[4, 5]),
    (1264, &[4, 5]),
    (1265, &[4, 5]),
    (1266, &[4, 5]),
    (1267, &[4, 5]),
    (1268, &[4, 5]),
    (1269, &[4, 5]),
    (1270, &[4, 5]),
    (1271, &[4, 5]),
    (1272, &[4, 5]),
    (1273, &[4, 5]),
    (1274, &[4, 5]),
    (1275, &[4, 5]),
    (1276, &[4, 5]),
    (1277, &[3, 4]),
    (1278, &[3, 4]),
    (1279, &[3, 4]),
    (1280, &[3, 4]),
    (1281, &[3, 4]),
    (1282, &[3, 4]),
    (1283, &[3, 4]),
    (1284, &[3, 4]),
    (1285, &[3, 4]),
    (1286, &[3, 4]),
    (1287, &[3, 4]),
    (1288, &[3, 4]),
    (1289, &[3, 5]),
    (1290, &[3, 5]),
    (1291, &[3, 5]),
    (1292, &[3, 5]),
    (1293, &[3, 5]),
    (1294, &[3, 5]),
    (1295, &[4, 5]),
    (1296, &[4, 5]),
    (1297, &[4, 5]),
    (1298, &[4, 5]),
    (1299, &[4, 5]),
    (1300, &[4, 5]),
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

/// Hub-side faceted filter (Wave D.2). AND of extra partition tags + query tokens.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ConceptFacetFilter {
    pub extra_partitions: Vec<u8>,
    pub query: String,
}

impl ConceptFacetFilter {
    pub fn is_active(&self) -> bool {
        !self.extra_partitions.is_empty() || !self.query.trim().is_empty()
    }
}

/// Drills tagged with `partition_id` that also match `filter` (AND).
pub fn filtered_drills_for_partition(partition_id: u8, filter: &ConceptFacetFilter) -> Vec<i32> {
    drills_for_partition(partition_id)
        .into_iter()
        .filter(|&n| drill_matches_filter(n, filter))
        .collect()
}

fn drill_matches_filter(micro_step: i32, filter: &ConceptFacetFilter) -> bool {
    let tags = partitions_for_micro_step(micro_step);
    if !filter
        .extra_partitions
        .iter()
        .all(|p| tags.iter().any(|t| t == p))
    {
        return false;
    }
    filter
        .query
        .split_whitespace()
        .all(|tok| drill_matches_token(micro_step, tok))
}

fn drill_matches_token(micro_step: i32, token: &str) -> bool {
    let tok = token.trim().to_ascii_lowercase();
    if tok.is_empty() {
        return true;
    }
    if let Some(step) = coding_step_by_micro_step(micro_step) {
        if step.title.to_ascii_lowercase().contains(&tok)
            || step.id.to_ascii_lowercase().contains(&tok)
        {
            return true;
        }
    }
    search_glossary(&tok, None).iter().any(|entry| {
        entry.lenses.iter().any(|lens| {
            lens.related_step_id
                .and_then(coding_step_by_id)
                .is_some_and(|s| s.micro_step == micro_step)
        })
    })
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

/// Inclusive decade of the coding rail (`1..=10`, `11..=20`, … `991..=1000`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeatmapBand {
    pub lo: i32,
    pub hi: i32,
}

impl HeatmapBand {
    pub fn contains(self, micro_step: i32) -> bool {
        (self.lo..=self.hi).contains(&micro_step)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeatmapCellState {
    Empty,
    Pending,
    Partial,
    Done,
}

impl HeatmapCellState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Pending => "pending",
            Self::Partial => "partial",
            Self::Done => "done",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeatmapCell {
    pub band: HeatmapBand,
    pub state: HeatmapCellState,
    pub done: usize,
    pub total: usize,
}

impl HeatmapCell {
    pub fn accessible_name(&self) -> String {
        format!(
            "Década {}–{}: {}/{}",
            self.band.lo, self.band.hi, self.done, self.total
        )
    }
}

pub const HEATMAP_BAND_COUNT: usize = 100;

/// 100 fixed decades covering micro-steps `1..=1000`.
pub fn heatmap_bands() -> [HeatmapBand; HEATMAP_BAND_COUNT] {
    let mut bands = [HeatmapBand { lo: 1, hi: 10 }; HEATMAP_BAND_COUNT];
    for (i, band) in bands.iter_mut().enumerate() {
        let lo = (i as i32) * 10 + 1;
        *band = HeatmapBand { lo, hi: lo + 9 };
    }
    bands
}

/// Tagged drills for `partition_id` whose micro-step falls in `band`.
pub fn heatmap_decade_drills(partition_id: u8, band: HeatmapBand) -> Vec<i32> {
    heatmap_decade_drills_in(&drills_for_partition(partition_id), band)
}

/// Subset of `drills` whose micro-step falls in `band`.
pub fn heatmap_decade_drills_in(drills: &[i32], band: HeatmapBand) -> Vec<i32> {
    drills
        .iter()
        .copied()
        .filter(|&n| band.contains(n))
        .collect()
}

pub fn heatmap_cell(partition_id: u8, band: HeatmapBand, completed: &[i32]) -> HeatmapCell {
    heatmap_cell_for_drills(&drills_for_partition(partition_id), band, completed)
}

pub fn heatmap_cell_for_drills(
    drills: &[i32],
    band: HeatmapBand,
    completed: &[i32],
) -> HeatmapCell {
    let decade = heatmap_decade_drills_in(drills, band);
    let total = decade.len();
    let done = decade
        .iter()
        .filter(|n| completed.iter().any(|c| c == *n))
        .count();
    let state = if total == 0 {
        HeatmapCellState::Empty
    } else if done == 0 {
        HeatmapCellState::Pending
    } else if done < total {
        HeatmapCellState::Partial
    } else {
        HeatmapCellState::Done
    };
    HeatmapCell {
        band,
        state,
        done,
        total,
    }
}

pub fn heatmap_cells(partition_id: u8, completed: &[i32]) -> Vec<HeatmapCell> {
    heatmap_cells_for_drills(&drills_for_partition(partition_id), completed)
}

pub fn heatmap_cells_for_drills(drills: &[i32], completed: &[i32]) -> Vec<HeatmapCell> {
    heatmap_bands()
        .into_iter()
        .map(|band| heatmap_cell_for_drills(drills, band, completed))
        .collect()
}

/// First pending drill in the decade, or the first drill if all are done.
/// `None` when the cell is `empty` (no navigation).
pub fn heatmap_click_target(partition_id: u8, band: HeatmapBand, completed: &[i32]) -> Option<i32> {
    let drills = heatmap_decade_drills(partition_id, band);
    if drills.is_empty() {
        return None;
    }
    drills
        .iter()
        .copied()
        .find(|n| !completed.iter().any(|c| c == n))
        .or(Some(drills[0]))
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
        assert_eq!(partitions_for_micro_step(1001), &[1, 3]); // assert contract (Wave 1)
        assert_eq!(partitions_for_micro_step(1013), &[2]); // LEGB local (Wave 1)
        assert!(partitions_for_micro_step(1301).is_empty()); // frontier beyond Wave 5
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

    /// C4 contract: `(micro_step, tags)` in `601..=750` at merge `6a8ac3b`.
    const WAVE_C4_SNAPSHOT_601_750: &[(i32, &[u8])] = &[
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

    /// C5 contract: `(micro_step, tags)` in `751..=900` at merge `e200ec1`.
    const WAVE_C5_SNAPSHOT_751_900: &[(i32, &[u8])] = &[
        (751, &[1]),
        (752, &[1]),
        (754, &[1]),
        (755, &[1]),
        (756, &[1]),
        (757, &[1]),
        (758, &[1]),
        (760, &[1]),
        (761, &[1]),
        (762, &[1]),
        (763, &[1]),
        (764, &[1]),
        (766, &[1]),
        (767, &[1]),
        (768, &[1]),
        (769, &[1, 3]),
        (770, &[1, 3]),
        (772, &[1, 3]),
        (773, &[1, 2, 3]),
        (774, &[1, 3]),
        (775, &[1, 3]),
        (776, &[1, 2, 3]),
        (778, &[1, 3]),
        (779, &[1, 2, 3]),
        (780, &[1, 3]),
        (781, &[1, 3]),
        (782, &[1, 3]),
        (784, &[1, 3]),
        (785, &[3]),
        (786, &[1, 3]),
        (787, &[1, 3]),
        (788, &[1, 3]),
        (790, &[1, 3]),
        (791, &[3]),
        (792, &[1, 3]),
        (793, &[3]),
        (794, &[3]),
        (796, &[3]),
        (797, &[1, 3]),
        (798, &[3]),
        (799, &[1, 2]),
        (800, &[1, 2]),
        (802, &[1, 2]),
        (803, &[1, 2]),
        (804, &[1, 2]),
        (805, &[1, 3]),
        (806, &[1, 3]),
        (808, &[1, 3]),
        (809, &[3]),
        (810, &[1, 3]),
        (811, &[3]),
        (812, &[3]),
        (814, &[3]),
        (815, &[1, 3]),
        (816, &[3]),
        (817, &[1, 3]),
        (818, &[1, 3]),
        (820, &[1, 3]),
        (821, &[1, 3]),
        (822, &[1, 3]),
        (823, &[4]),
        (824, &[4]),
        (826, &[1, 4]),
        (827, &[4]),
        (828, &[1, 4]),
        (829, &[1, 3]),
        (830, &[1, 3]),
        (832, &[1]),
        (833, &[3]),
        (834, &[3]),
        (835, &[3]),
        (838, &[3]),
        (840, &[3]),
        (841, &[1, 3]),
        (842, &[1, 3]),
        (844, &[3]),
        (845, &[1, 3]),
        (846, &[3]),
        (847, &[1, 3]),
        (848, &[1, 3]),
        (850, &[3]),
        (851, &[1, 3]),
        (852, &[3]),
        (855, &[1, 2, 3]),
        (856, &[1, 2, 3]),
        (857, &[1, 2, 3]),
        (858, &[1, 2, 3]),
        (883, &[1]),
        (884, &[1]),
        (886, &[1]),
        (887, &[1]),
        (888, &[1]),
        (889, &[1]),
        (890, &[1]),
        (892, &[1]),
        (893, &[1]),
        (894, &[1]),
        (895, &[1]),
        (896, &[1]),
        (898, &[1]),
        (899, &[1]),
        (900, &[1]),
    ];

    /// Frozen `(micro_step, tags)` pairs with `micro_step > 1300` (Wave 5 ceiling).
    const WAVE5_FROZEN_BEYOND_1300: &[(i32, &[u8])] = &[];

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
            assert_eq!(
                got.1, expected.1,
                "C1 tags changed for micro_step {}",
                got.0
            );
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
            assert_eq!(
                got.1, expected.1,
                "C2 tags changed for micro_step {}",
                got.0
            );
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
    fn wave_c4_range_601_to_750_unchanged() {
        let current: Vec<(i32, &[u8])> = STEP_PARTITIONS
            .iter()
            .copied()
            .filter(|(n, _)| (601..=750).contains(n))
            .collect();
        assert_eq!(
            current.len(),
            WAVE_C4_SNAPSHOT_601_750.len(),
            "do not add or remove rows in 601..=750"
        );
        for (got, expected) in current.iter().zip(WAVE_C4_SNAPSHOT_601_750.iter()) {
            assert_eq!(got.0, expected.0, "micro_step drift in 601..=750");
            assert_eq!(
                got.1, expected.1,
                "C4 tags changed for micro_step {}",
                got.0
            );
        }
    }

    #[test]
    fn wave_c5_applied_floor_751_to_900() {
        let tagged: Vec<i32> = (751..=900)
            .filter(|&n| {
                coding_step_by_micro_step(n).is_some() && !partitions_for_micro_step(n).is_empty()
            })
            .collect();
        assert!(
            tagged.len() >= 90,
            "751..=900 tagged floor is 90, got {}",
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
            "751..=900 must not be 100% partition 3 (got {p3}/{})",
            tagged.len()
        );
    }

    #[test]
    fn wave_c5_recipe_families_stay_untagged() {
        for n in [753, 853, 859] {
            assert!(
                partitions_for_micro_step(n).is_empty(),
                "micro_step {n} must stay untagged (window / lab / bits)"
            );
        }
    }

    #[test]
    fn wave_c5_range_751_to_900_unchanged() {
        let current: Vec<(i32, &[u8])> = STEP_PARTITIONS
            .iter()
            .copied()
            .filter(|(n, _)| (751..=900).contains(n))
            .collect();
        assert_eq!(
            current.len(),
            WAVE_C5_SNAPSHOT_751_900.len(),
            "do not add or remove rows in 751..=900"
        );
        for (got, expected) in current.iter().zip(WAVE_C5_SNAPSHOT_751_900.iter()) {
            assert_eq!(got.0, expected.0, "micro_step drift in 751..=900");
            assert_eq!(
                got.1, expected.1,
                "C5 tags changed for micro_step {}",
                got.0
            );
        }
    }

    #[test]
    fn wave_c6_applied_floor_901_to_1000() {
        let tagged: Vec<i32> = (901..=1000)
            .filter(|&n| {
                coding_step_by_micro_step(n).is_some() && !partitions_for_micro_step(n).is_empty()
            })
            .collect();
        assert!(
            tagged.len() >= 60,
            "901..=1000 tagged floor is 60, got {}",
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
            "901..=1000 must not be 100% partition 3 (got {p3}/{})",
            tagged.len()
        );
    }

    #[test]
    fn wave_c6_recipe_families_stay_untagged() {
        for n in [913, 955, 1000] {
            assert!(
                partitions_for_micro_step(n).is_empty(),
                "micro_step {n} must stay untagged (window / matching / review recipe)"
            );
        }
    }

    #[test]
    fn wave5_freeze_rows_beyond_1300() {
        let current: Vec<(i32, &[u8])> = STEP_PARTITIONS
            .iter()
            .copied()
            .filter(|(n, _)| *n > 1300)
            .collect();
        assert_eq!(
            current.as_slice(),
            WAVE5_FROZEN_BEYOND_1300,
            "do not add or remove rows > 1300"
        );
    }

    #[test]
    fn wave_d_heatmap_has_100_decade_cells() {
        let bands = heatmap_bands();
        assert_eq!(bands.len(), HEATMAP_BAND_COUNT);
        assert_eq!(bands[0], HeatmapBand { lo: 1, hi: 10 });
        assert_eq!(bands[5], HeatmapBand { lo: 51, hi: 60 });
        assert_eq!(bands[99], HeatmapBand { lo: 991, hi: 1000 });
        for (i, band) in bands.iter().enumerate() {
            assert_eq!(band.hi - band.lo, 9);
            if i > 0 {
                assert_eq!(band.lo, bands[i - 1].hi + 1);
            }
        }
        for id in 1..=5 {
            let cells = heatmap_cells(id, &[]);
            assert_eq!(cells.len(), 100, "partition {id} must render 100 cells");
        }
    }

    #[test]
    fn wave_d_p2_decade_of_52_is_pending_when_empty_progress() {
        let band = heatmap_bands()
            .into_iter()
            .find(|b| b.contains(52))
            .expect("decade containing micro-step 52");
        assert_eq!(band, HeatmapBand { lo: 51, hi: 60 });
        let cell = heatmap_cell(2, band, &[]);
        assert_eq!(cell.state, HeatmapCellState::Pending);
        assert_eq!(cell.state.as_str(), "pending");
        assert!(cell.total > 0);
        assert_eq!(cell.done, 0);
        let name = cell.accessible_name();
        assert!(name.contains("51"), "{name}");
        assert!(name.contains("60"), "{name}");
        assert!(name.contains(&format!("0/{}", cell.total)), "{name}");
        assert_eq!(heatmap_click_target(2, band, &[]), Some(52));
    }

    #[test]
    fn wave_d_synthetic_completed_moves_cell_state() {
        let band = heatmap_bands()[0];
        let pending = heatmap_cell(1, band, &[]);
        assert_eq!(pending.state, HeatmapCellState::Pending);
        assert!(pending.total >= 2, "need multiple P1 drills in 1..=10");
        let first = heatmap_click_target(1, band, &[]).expect("non-empty decade");
        let partial = heatmap_cell(1, band, &[first]);
        assert_eq!(partial.state, HeatmapCellState::Partial);
        assert_eq!(partial.done, 1);
        let next = heatmap_click_target(1, band, &[first]).expect("still pending drills");
        assert_ne!(next, first);

        let all: Vec<i32> = drills_for_partition(1)
            .into_iter()
            .filter(|&n| band.contains(n))
            .collect();
        let done_cell = heatmap_cell(1, band, &all);
        assert_eq!(done_cell.state, HeatmapCellState::Done);
        assert_eq!(heatmap_click_target(1, band, &all), Some(all[0]));

        let empty_band = heatmap_bands()
            .into_iter()
            .find(|b| heatmap_cell(5, *b, &[]).state == HeatmapCellState::Empty)
            .expect("P5 map_only has empty decades");
        assert_eq!(heatmap_click_target(5, empty_band, &[]), None);
        let empty_cell = heatmap_cell(5, empty_band, &[]);
        assert_eq!(empty_cell.done, 0);
        assert_eq!(empty_cell.total, 0);
        assert!(empty_cell.accessible_name().contains("0/0"));
    }

    #[test]
    fn wave_d1_decade_drills_are_partition_filtered() {
        let band = HeatmapBand { lo: 1, hi: 10 };
        let drills = heatmap_decade_drills(1, band);
        assert!(!drills.is_empty());
        assert!(drills.len() <= 10);
        for n in &drills {
            assert!(band.contains(*n), "{n} outside {band:?}");
            assert!(
                partitions_for_micro_step(*n).contains(&1),
                "{n} is not tagged with partition 1"
            );
        }
        assert!(drills.contains(&1));
        assert!(drills.contains(&10));
        assert!(!drills.contains(&3), "micro 3 is partition 4");
        assert!(!drills.contains(&4), "micro 4 is partition 3");

        let empty_band = heatmap_bands()
            .into_iter()
            .find(|b| heatmap_cell(5, *b, &[]).state == HeatmapCellState::Empty)
            .expect("P5 map_only has empty decades");
        assert!(heatmap_decade_drills(5, empty_band).is_empty());
    }

    #[test]
    fn facet_append_on_p1_keeps_related_list_drill() {
        let filter = ConceptFacetFilter {
            extra_partitions: vec![],
            query: "append".into(),
        };
        let drills = filtered_drills_for_partition(1, &filter);
        assert!(drills.contains(&20), "py-20-list-change is python-lists P1");
        assert!(!drills.contains(&1), "micro 1 is not an append hit");
        assert!(!drills.is_empty());
        let unfiltered = drills_for_partition(1);
        assert!(drills.len() < unfiltered.len());
        let cells = heatmap_cells_for_drills(&drills, &[]);
        let hits = cells.iter().filter(|c| c.total > 0).count();
        let unfiltered_hits = heatmap_cells(1, &[]).iter().filter(|c| c.total > 0).count();
        assert!(hits > 0 && hits < unfiltered_hits);
        let decade_11 = heatmap_bands()[1];
        assert!(heatmap_decade_drills_in(&drills, decade_11).contains(&20));
    }

    #[test]
    fn facet_recursion_and_dfs_on_p3_keeps_graph_dfs() {
        let filter = ConceptFacetFilter {
            extra_partitions: vec![],
            query: "recursion dfs".into(),
        };
        let drills = filtered_drills_for_partition(3, &filter);
        assert!(drills.contains(&109), "pattern-dfs related_step is 109");
        assert!(
            !drills.contains(&133),
            "permutations matches recursion but not dfs"
        );
    }

    #[test]
    fn facet_extra_partition_is_and_with_active_tab() {
        let filter = ConceptFacetFilter {
            extra_partitions: vec![3],
            query: String::new(),
        };
        let drills = filtered_drills_for_partition(1, &filter);
        assert!(!drills.is_empty());
        assert!(!drills.contains(&20), "micro 20 is only partition 1");
        assert!(drills.contains(&133), "micro 133 is tagged 1,2,3");
        for n in &drills {
            let tags = partitions_for_micro_step(*n);
            assert!(
                tags.contains(&1) && tags.contains(&3),
                "{n} tags {tags:?} must include 1 and 3"
            );
        }
    }

    /// C6 baseline @ `87a5334`: Wave D must not retag `1..=1000`.
    #[test]
    fn wave_d_freeze_1_to_1000_matches_c6_baseline() {
        let current = STEP_PARTITIONS
            .iter()
            .filter(|(n, _)| (1..=1000).contains(n))
            .map(|(n, tags)| {
                format!(
                    "{n}:{}",
                    tags.iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(",")
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let expected = include_str!("wave_d_freeze_1_1000.txt").trim_end();
        assert_eq!(current, expected);
    }
}
