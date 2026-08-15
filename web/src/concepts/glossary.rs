//! In-memory conceptual glossary (Wave B). Search is WASM-only (ADR 002).
//!
//! Partition ids map 1:1 onto [`super::ConceptPartition::id`] (`1..=5`).
//! Drill tags stay in `STEP_PARTITIONS`; this module is the search corpus.

use crate::curriculum::coding_step_by_id;

/// Canonical partition id for glossary lenses. Maps onto `ConceptPartition.id`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PartitionId {
    P1MemoryData,
    P2ScopeControl,
    P3Paradigms,
    P4Ecosystem,
    P5Domains,
}

impl PartitionId {
    pub const ALL: [PartitionId; 5] = [
        PartitionId::P1MemoryData,
        PartitionId::P2ScopeControl,
        PartitionId::P3Paradigms,
        PartitionId::P4Ecosystem,
        PartitionId::P5Domains,
    ];

    pub fn label(self) -> &'static str {
        match self {
            PartitionId::P1MemoryData => "Azul Memoria",
            PartitionId::P2ScopeControl => "Violeta Ámbitos",
            PartitionId::P3Paradigms => "Ámbar Paradigmas",
            PartitionId::P4Ecosystem => "Verde Ecosistema",
            PartitionId::P5Domains => "Magenta Dominios",
        }
    }

    /// CSS class (`badge-lens-pN`) — not a Tailwind utility.
    pub fn color_badge(self) -> &'static str {
        match self {
            PartitionId::P1MemoryData => "badge-lens-p1",
            PartitionId::P2ScopeControl => "badge-lens-p2",
            PartitionId::P3Paradigms => "badge-lens-p3",
            PartitionId::P4Ecosystem => "badge-lens-p4",
            PartitionId::P5Domains => "badge-lens-p5",
        }
    }

    pub fn as_u8(self) -> u8 {
        match self {
            PartitionId::P1MemoryData => 1,
            PartitionId::P2ScopeControl => 2,
            PartitionId::P3Paradigms => 3,
            PartitionId::P4Ecosystem => 4,
            PartitionId::P5Domains => 5,
        }
    }

    pub fn from_u8(id: u8) -> Option<Self> {
        match id {
            1 => Some(PartitionId::P1MemoryData),
            2 => Some(PartitionId::P2ScopeControl),
            3 => Some(PartitionId::P3Paradigms),
            4 => Some(PartitionId::P4Ecosystem),
            5 => Some(PartitionId::P5Domains),
            _ => None,
        }
    }
}

/// One partition-colored facet of a glossary term.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConceptLens {
    pub partition: PartitionId,
    pub headline: &'static str,
    pub tldr: &'static str,
    pub code_example: &'static str,
    pub related_step_id: Option<&'static str>,
}

/// Search corpus entry. Intent is derived from `id` prefix, not a stored field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GlossaryEntry {
    pub id: &'static str,
    pub title: &'static str,
    pub keywords: &'static [&'static str],
    pub lenses: &'static [ConceptLens],
    pub common_pitfall: Option<&'static str>,
}

/// Four search intents (grouping only — not a struct field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SearchIntent {
    Model,
    Syntax,
    Pattern,
    Trap,
}

impl SearchIntent {
    pub fn label(self) -> &'static str {
        match self {
            SearchIntent::Model => "Modelos mentales",
            SearchIntent::Syntax => "Sintaxis y stdlib",
            SearchIntent::Pattern => "Patrones y algoritmos",
            SearchIntent::Trap => "Trampas y errores",
        }
    }
}

/// Unprefixed canonical ids (e.g. `python-lists`) count as mental models.
pub fn search_intent(id: &str) -> SearchIntent {
    if id.starts_with("syntax-") {
        SearchIntent::Syntax
    } else if id.starts_with("pattern-") {
        SearchIntent::Pattern
    } else if id.starts_with("trap-") {
        SearchIntent::Trap
    } else {
        SearchIntent::Model
    }
}

const PYTHON_LISTS_LENSES: &[ConceptLens] = &[
    ConceptLens {
        partition: PartitionId::P1MemoryData,
        headline: "append muta; rebind crea otra lista",
        tldr: "xs.append cambia el objeto; xs = xs + […] crea uno nuevo.",
        code_example: "xs = [1]\nys = xs\nxs.append(2)  # ys → [1, 2]",
        related_step_id: Some("py-20-list-change"),
    },
    ConceptLens {
        partition: PartitionId::P2ScopeControl,
        headline: "Mutar la lista dentro de una función",
        tldr: "El parámetro apunta al mismo objeto: LEGB no copia la lista.",
        code_example: "def add(xs):\n    xs.append(1)\nadd(nums)  # nums cambia",
        related_step_id: Some("py-62-scope"),
    },
    ConceptLens {
        partition: PartitionId::P3Paradigms,
        headline: "Comprensión vs bucle",
        tldr: "Una comprensión construye una lista nueva; el bucle puede mutar.",
        code_example: "squares = [x * x for x in xs]",
        related_step_id: Some("py-24-list-comprehension"),
    },
    ConceptLens {
        partition: PartitionId::P4Ecosystem,
        headline: "deque para colas O(1)",
        tldr: "list.pop(0) es O(n); collections.deque es O(1) en ambos extremos.",
        code_example: "from collections import deque\nq = deque([1, 2])\nq.appendleft(0)",
        related_step_id: Some("py-21-list-add"),
    },
    ConceptLens {
        partition: PartitionId::P5Domains,
        headline: "Listas ↔ JSON",
        tldr: "json.dumps serializa listas a arrays; loads reconstruye list.",
        code_example: "import json\njson.dumps([1, 2])  # '[1, 2]'",
        related_step_id: Some("py-66-json"),
    },
];

pub static GLOSSARY_ENTRIES: &[GlossaryEntry] = &[
    GlossaryEntry {
        id: "python-lists",
        title: "Listas de Python",
        keywords: &["append", "extend", "slice", "list", "deque", "json", "alias"],
        lenses: PYTHON_LISTS_LENSES,
        common_pitfall: Some(
            "Usar lista vacía como default de función (`def f(xs=[])`) comparte el mismo objeto.",
        ),
    },
    GlossaryEntry {
        id: "model-legb",
        title: "Orden LEGB",
        keywords: &["scope", "local", "global", "nonlocal", "legb"],
        lenses: &[ConceptLens {
            partition: PartitionId::P2ScopeControl,
            headline: "Local → Enclosing → Global → Built-in",
            tldr: "Python busca nombres en ese orden; no hay “salto” al azar.",
            code_example: "x = 1\ndef f():\n    print(x)  # lee Global",
            related_step_id: Some("py-62-scope"),
        }],
        common_pitfall: Some("Asignar a x dentro de f() la vuelve local en toda la función."),
    },
    GlossaryEntry {
        id: "model-mutability",
        title: "Mutabilidad y alias",
        keywords: &["mutable", "alias", "copy", "id", "is"],
        lenses: &[ConceptLens {
            partition: PartitionId::P1MemoryData,
            headline: "La variable es una etiqueta, no una caja",
            tldr: "Dos nombres pueden apuntar al mismo objeto mutable.",
            code_example: "a = [1]\nb = a\nb.append(2)\n# a es [1, 2]",
            related_step_id: Some("py-26-list-copy"),
        }],
        common_pitfall: Some("b = a no copia la lista; usá list(a) o a.copy()."),
    },
    GlossaryEntry {
        id: "model-recursion",
        title: "Recursión como diseño",
        keywords: &["recursion", "base case", "call stack"],
        lenses: &[ConceptLens {
            partition: PartitionId::P3Paradigms,
            headline: "Caso base + caso recursivo",
            tldr: "La recursión es un diseño, no un bucle disfrazado.",
            code_example: "def fact(n):\n    if n <= 1: return 1\n    return n * fact(n - 1)",
            related_step_id: Some("py-133-permutations"),
        }],
        common_pitfall: Some("Sin caso base el call stack explota (RecursionError)."),
    },
    GlossaryEntry {
        id: "model-comprehension",
        title: "Comprensiones",
        keywords: &["comprehension", "map", "filter"],
        lenses: &[ConceptLens {
            partition: PartitionId::P3Paradigms,
            headline: "Expresión que construye una colección",
            tldr: "La comprensión es funcional-lite: nueva lista, sin mutar la fuente.",
            code_example: "evens = [x for x in xs if x % 2 == 0]",
            related_step_id: Some("py-24-list-comprehension"),
        }],
        common_pitfall: None,
    },
    GlossaryEntry {
        id: "syntax-extend",
        title: "list.extend",
        keywords: &["extend", "concat", "iterable"],
        lenses: &[ConceptLens {
            partition: PartitionId::P1MemoryData,
            headline: "extend agrega ítems; append agrega un objeto",
            tldr: "xs.extend([2, 3]) aplana; xs.append([2, 3]) anida.",
            code_example: "xs = [1]\nxs.extend([2, 3])  # [1, 2, 3]",
            related_step_id: Some("py-21-list-add"),
        }],
        common_pitfall: Some("append(lista) mete la lista entera como un solo elemento."),
    },
    GlossaryEntry {
        id: "syntax-yield",
        title: "yield y generadores",
        keywords: &["yield", "generator", "iterator"],
        lenses: &[
            ConceptLens {
                partition: PartitionId::P2ScopeControl,
                headline: "El generador conserva el frame local",
        tldr: "yield pausa; las variables locales sobreviven entre next().",
        code_example: "def count():\n    n = 0\n    yield n",
                related_step_id: None,
            },
            ConceptLens {
                partition: PartitionId::P3Paradigms,
                headline: "Lazy vs materializar",
                tldr: "Un generador no es una lista: se consume una vez.",
                code_example: "g = (x * x for x in xs)\nlist(g)  # materializa",
                related_step_id: None,
            },
        ],
        common_pitfall: Some("Iterar dos veces el mismo generador deja la segunda pasada vacía."),
    },
    GlossaryEntry {
        id: "syntax-zip",
        title: "zip",
        keywords: &["zip", "enumerate", "parallel"],
        lenses: &[ConceptLens {
            partition: PartitionId::P4Ecosystem,
            headline: "Emparejar iterables de la stdlib",
            tldr: "zip corta al más corto; no rellena.",
            code_example: "for a, b in zip(xs, ys):\n    print(a, b)",
            related_step_id: Some("py-51-for"),
        }],
        common_pitfall: None,
    },
    GlossaryEntry {
        id: "syntax-lambda",
        title: "lambda",
        keywords: &["lambda", "anonymous", "key"],
        lenses: &[ConceptLens {
            partition: PartitionId::P3Paradigms,
            headline: "Función anónima de una expresión",
            tldr: "Útil como key=; no reemplaza un def con cuerpo.",
            code_example: "sorted(xs, key=lambda p: p[1])",
            related_step_id: Some("py-55-lambda"),
        }],
        common_pitfall: Some("Una lambda no puede tener statements (if/for/return)."),
    },
    GlossaryEntry {
        id: "pattern-two-pointers",
        title: "Two pointers",
        keywords: &["two pointers", "left", "right", "in-place"],
        lenses: &[ConceptLens {
            partition: PartitionId::P3Paradigms,
            headline: "Dos índices recorren la misma estructura",
            tldr: "Es una receta de índices, no un modelo de mutabilidad.",
            code_example: "i, j = 0, len(xs) - 1\nwhile i < j:\n    i += 1; j -= 1",
            related_step_id: None,
        }],
        common_pitfall: None,
    },
    GlossaryEntry {
        id: "pattern-sliding-window",
        title: "Sliding window",
        keywords: &["window", "subarray", "substring"],
        lenses: &[ConceptLens {
            partition: PartitionId::P3Paradigms,
            headline: "Ventana que crece y se contrae",
            tldr: "Mantené un invariante en [left, right) en O(n).",
            code_example: "left = 0\nfor right, x in enumerate(xs):\n    # encoger left si hace falta\n    pass",
            related_step_id: None,
        }],
        common_pitfall: None,
    },
    GlossaryEntry {
        id: "pattern-bfs",
        title: "BFS",
        keywords: &["bfs", "queue", "level order", "shortest"],
        lenses: &[
            ConceptLens {
                partition: PartitionId::P3Paradigms,
                headline: "Recorrido por niveles",
                tldr: "BFS usa cola; el primer hallazgo en grafo no ponderado es el más corto.",
                code_example: "from collections import deque\nq = deque([start])",
                related_step_id: Some("py-110-graph-bfs"),
            },
            ConceptLens {
                partition: PartitionId::P4Ecosystem,
                headline: "deque es la cola de la stdlib",
                tldr: "No uses list.pop(0) como cola de BFS.",
                code_example: "q.append(n)\nq.popleft()",
                related_step_id: Some("py-110-graph-bfs"),
            },
        ],
        common_pitfall: None,
    },
    GlossaryEntry {
        id: "pattern-dfs",
        title: "DFS",
        keywords: &["dfs", "stack", "recursion", "backtrack"],
        lenses: &[ConceptLens {
            partition: PartitionId::P3Paradigms,
            headline: "Profundizar antes de ramificar",
            tldr: "DFS es pila (recursiva o explícita), no cola.",
            code_example: "def dfs(u, seen):\n    seen.add(u)\n    for v in graph[u]: dfs(v, seen)",
            related_step_id: Some("py-109-graph-dfs"),
        }],
        common_pitfall: None,
    },
    GlossaryEntry {
        id: "trap-unboundlocal",
        title: "UnboundLocalError",
        keywords: &["unboundlocal", "assignment", "scope"],
        lenses: &[ConceptLens {
            partition: PartitionId::P2ScopeControl,
            headline: "Asignar hace local a toda la función",
            tldr: "Leer x y luego asignar x en el mismo def dispara UnboundLocalError.",
            code_example: "x = 1\ndef f():\n    print(x)\n    x = 2  # UnboundLocalError",
            related_step_id: Some("py-62-scope"),
        }],
        common_pitfall: Some("Usá nonlocal/global si realmente querés rebind del enclosing."),
    },
    GlossaryEntry {
        id: "trap-tuple-typeerror",
        title: "Tuple TypeError",
        keywords: &["tuple", "immutable", "typeerror"],
        lenses: &[ConceptLens {
            partition: PartitionId::P1MemoryData,
            headline: "Las tuplas no se mutan in-place",
            tldr: "t[0] = 1 lanza TypeError; reconstruí la tupla.",
            code_example: "t = (1, 2)\n# t[0] = 9  # TypeError\nt = (9,) + t[1:]",
            related_step_id: Some("py-30-tuple-update"),
        }],
        common_pitfall: Some("Una tupla puede contener una lista mutable: t[0].append(1) sí funciona."),
    },
    GlossaryEntry {
        id: "trap-aliasing",
        title: "Aliasing accidental",
        keywords: &["aliasing", "shared", "side effect"],
        lenses: &[ConceptLens {
            partition: PartitionId::P1MemoryData,
            headline: "Dos nombres, un objeto",
            tldr: "Mutar por un alias se ve por el otro.",
            code_example: "row = [0] * 3\ngrid = [row] * 3\ngrid[0][0] = 1  # las 3 filas cambian",
            related_step_id: Some("py-26-list-copy"),
        }],
        common_pitfall: Some("[row] * n no clona filas; usá [[0] * w for _ in range(h)]."),
    },
    GlossaryEntry {
        id: "trap-mutable-default",
        title: "Default argument mutable",
        keywords: &["default", "argument", "shared list"],
        lenses: &[ConceptLens {
            partition: PartitionId::P2ScopeControl,
            headline: "El default se evalúa una vez",
            tldr: "def f(xs=[]) comparte la misma lista entre llamadas.",
            code_example: "def f(xs=None):\n    if xs is None:\n        xs = []",
            related_step_id: Some("py-52-functions"),
        }],
        common_pitfall: Some("Nunca uses list/dict/set vacíos como default."),
    },
];

const SEARCH_CAP: usize = 8;

fn haystack(entry: &GlossaryEntry) -> impl Iterator<Item = &'static str> {
    std::iter::once(entry.title)
        .chain(std::iter::once(entry.id))
        .chain(entry.keywords.iter().copied())
}

fn match_rank(query: &str, entry: &GlossaryEntry) -> Option<u8> {
    let q = query.trim().to_ascii_lowercase();
    if q.is_empty() {
        return Some(3);
    }
    let mut best: Option<u8> = None;
    for field in haystack(entry) {
        let f = field.to_ascii_lowercase();
        let rank = if f == q {
            0
        } else if f.starts_with(&q) {
            1
        } else if f.contains(&q) {
            2
        } else {
            continue;
        };
        best = Some(best.map_or(rank, |b| b.min(rank)));
        if best == Some(0) {
            break;
        }
    }
    best
}

fn has_lens(entry: &GlossaryEntry, lens: PartitionId) -> bool {
    entry.lenses.iter().any(|l| l.partition == lens)
}

/// Case-insensitive search over title, id and keywords. Rank: exact → prefix → contains.
pub fn search_glossary(query: &str, lens: Option<PartitionId>) -> Vec<&'static GlossaryEntry> {
    let mut scored: Vec<(u8, usize, &'static GlossaryEntry)> = GLOSSARY_ENTRIES
        .iter()
        .enumerate()
        .filter(|(_, e)| match lens {
            Some(p) => has_lens(e, p),
            None => true,
        })
        .filter_map(|(i, e)| match_rank(query, e).map(|r| (r, i, e)))
        .collect();
    scored.sort_by_key(|(rank, idx, _)| (*rank, *idx));
    scored
        .into_iter()
        .take(SEARCH_CAP)
        .map(|(_, _, e)| e)
        .collect()
}

/// Hits grouped in intent order (models, syntax, patterns, traps).
pub fn group_search_hits(
    hits: &[&'static GlossaryEntry],
) -> Vec<(SearchIntent, Vec<&'static GlossaryEntry>)> {
    let order = [
        SearchIntent::Model,
        SearchIntent::Syntax,
        SearchIntent::Pattern,
        SearchIntent::Trap,
    ];
    order
        .into_iter()
        .filter_map(|intent| {
            let group: Vec<_> = hits
                .iter()
                .copied()
                .filter(|e| search_intent(e.id) == intent)
                .collect();
            if group.is_empty() {
                None
            } else {
                Some((intent, group))
            }
        })
        .collect()
}

pub fn entry_by_id(id: &str) -> Option<&'static GlossaryEntry> {
    GLOSSARY_ENTRIES.iter().find(|e| e.id == id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::concepts::{partition_by_id, PARTITIONS};
    use std::time::Instant;

    #[test]
    fn from_u8_rejects_out_of_range_and_maps_p1() {
        assert!(PartitionId::from_u8(0).is_none());
        assert!(PartitionId::from_u8(6).is_none());
        assert_eq!(PartitionId::from_u8(1), Some(PartitionId::P1MemoryData));
        assert_eq!(PartitionId::from_u8(3).map(PartitionId::as_u8), Some(3));
        for id in 1u8..=5 {
            let p = PartitionId::from_u8(id).expect("1..=5");
            assert_eq!(p.as_u8(), id);
            assert_eq!(partition_by_id(id).map(|c| c.id), Some(id));
            assert_eq!(PARTITIONS[(id - 1) as usize].id, id);
        }
    }

    #[test]
    fn python_lists_has_five_distinct_lenses_and_keywords() {
        let entry = entry_by_id("python-lists").expect("python-lists seed");
        assert_eq!(entry.lenses.len(), 5);
        let mut seen = [false; 5];
        for lens in entry.lenses {
            let i = (lens.partition.as_u8() - 1) as usize;
            assert!(!seen[i], "duplicate partition {:?}", lens.partition);
            seen[i] = true;
            let lines = lens.code_example.lines().count();
            assert!(lines <= 3, "{} has {lines} snippet lines", lens.headline);
        }
        assert!(seen.iter().all(|&s| s));
        for kw in ["append", "extend", "slice"] {
            assert!(
                entry.keywords.iter().any(|k| k.eq_ignore_ascii_case(kw)),
                "missing keyword {kw}"
            );
        }
    }

    #[test]
    fn search_extend_finds_python_lists() {
        let hits = search_glossary("extend", None);
        assert!(
            hits.iter().any(|e| e.id == "python-lists"),
            "extend must hit python-lists, got {:?}",
            hits.iter().map(|e| e.id).collect::<Vec<_>>()
        );
        for q in ["append", "slice", "APPEND"] {
            assert!(
                search_glossary(q, None)
                    .iter()
                    .any(|e| e.id == "python-lists"),
                "query {q:?} missed python-lists"
            );
        }
    }

    #[test]
    fn four_intents_have_at_least_one_entry() {
        let mut found = [false; 4];
        for e in GLOSSARY_ENTRIES {
            match search_intent(e.id) {
                SearchIntent::Model => found[0] = true,
                SearchIntent::Syntax => found[1] = true,
                SearchIntent::Pattern => found[2] = true,
                SearchIntent::Trap => found[3] = true,
            }
        }
        assert!(found.iter().all(|&f| f), "missing intent in seed: {found:?}");
    }

    #[test]
    fn related_step_ids_exist_and_seed_size() {
        assert!(
            GLOSSARY_ENTRIES.len() >= 12 && GLOSSARY_ENTRIES.len() <= 32,
            "seed must be 12–20 (cap 32), got {}",
            GLOSSARY_ENTRIES.len()
        );
        for e in GLOSSARY_ENTRIES {
            assert!(!e.lenses.is_empty() && e.lenses.len() <= 5);
            let mut seen = Vec::new();
            for lens in e.lenses {
                assert!(
                    !seen.contains(&lens.partition),
                    "{} duplicates {:?}",
                    e.id,
                    lens.partition
                );
                seen.push(lens.partition);
                if let Some(step_id) = lens.related_step_id {
                    assert!(
                        coding_step_by_id(step_id).is_some(),
                        "{} related_step_id {step_id} not in catalog",
                        e.id
                    );
                }
            }
        }
    }

    #[test]
    fn lens_filter_keeps_only_matching_entries() {
        let hits = search_glossary("list", Some(PartitionId::P2ScopeControl));
        assert!(hits.iter().all(|e| has_lens(e, PartitionId::P2ScopeControl)));
        assert!(hits.iter().any(|e| e.id == "python-lists"));
    }

    #[test]
    fn color_badge_is_css_class_not_tailwind() {
        for p in PartitionId::ALL {
            let badge = p.color_badge();
            assert!(
                badge.starts_with("badge-lens-p"),
                "expected badge-lens-pN, got {badge}"
            );
            assert!(!badge.contains("bg-") && !badge.contains("text-"));
        }
    }

    #[test]
    fn search_budget_one_thousand_queries() {
        let queries = [
            "extend", "append", "slice", "legb", "zip", "bfs", "dfs", "yield", "lambda",
            "window", "alias", "tuple", "json", "deque", "xxxx-miss", "", "LIST",
        ];
        let start = Instant::now();
        for i in 0..1000 {
            let q = queries[i % queries.len()];
            let lens = if i % 6 == 0 {
                None
            } else {
                PartitionId::from_u8(((i % 5) + 1) as u8)
            };
            let _ = search_glossary(q, lens);
        }
        let elapsed = start.elapsed();
        assert!(
            elapsed.as_millis() < 50,
            "1000 searches took {elapsed:?} (CI budget 50ms)"
        );
    }
}
