"""Apply ordered Wave 27 to the exact 2560-step baseline."""

from pathlib import Path
import gen_wave27

ROOT = Path(__file__).resolve().parents[1]
CURRICULUM = ROOT / "web/src/curriculum.rs"
CONCEPTS = ROOT / "web/src/concepts/mod.rs"
E2E = (
    ROOT / "web/e2e/tests/journey.auth-hub.spec.ts",
    ROOT / "web/e2e/tests/progress.check.spec.ts",
    ROOT / "web/e2e/tests/session.navigation.spec.ts",
)


def replace_once(source, old, new, label):
    count = source.count(old)
    if count != 1:
        raise RuntimeError(f"{label}: expected one exact anchor, found {count}")
    return source.replace(old, new, 1)


def partition_rows():
    families = (
        (2561, 2566, (3, 2)), (2567, 2572, (3, 1)),
        (2573, 2578, (3, 2)), (2579, 2584, (3, 1)),
        (2585, 2590, (3, 1)), (2591, 2596, (3,)),
        (2597, 2602, (3,)), (2603, 2608, (3, 1)),
        (2609, 2614, (3, 1)), (2615, 2620, (3, 5)),
    )
    return "\n".join(
        f"    ({number}, &[{', '.join(map(str, tags))}]),"
        for start, end, tags in families for number in range(start, end + 1)
    )


def transformed_files():
    steps = gen_wave27.build_steps()
    curriculum = CURRICULUM.read_text(encoding="utf-8")
    curriculum = replace_once(
        curriculum,
        '    next: None, show_type_chips: false, micro_step: 2560,\n};\npub const CODING_STEPS:',
        '    next: Some("py-2561-base-cero"), show_type_chips: false, micro_step: 2560,\n};\n'
        + gen_wave27.emit_rust(steps) + '\npub const CODING_STEPS:',
        "step 2560 boundary",
    )
    curriculum = replace_once(
        curriculum, "    &PY2560_SCORE_CHECK,\n\n];",
        "    &PY2560_SCORE_CHECK,\n" + gen_wave27.emit_refs(steps) + "\n\n];",
        "catalog references",
    )
    for old, new, label in (
        ("CODING_STEPS.len(), 2560", "CODING_STEPS.len(), 2620", "Rust count"),
        ("catalog must contain 2560 steps", "catalog must contain 2620 steps", "count message"),
        ("step.micro_step <= 2560", "step.micro_step <= 2620", "step ceiling"),
        ("(1..=2560).collect()", "(1..=2620).collect()", "Rust exact range"),
        ('1..=2560"', '1..=2620"', "range message"),
        ('assert_eq!(step.next, None, "step 2560 is the end of the rail");',
         'assert_eq!(step.next, Some("py-2561-base-cero"), "step 2560 chains to Wave 27");',
         "Wave 26 boundary test"),
    ):
        curriculum = replace_once(curriculum, old, new, label)
    test_anchor = "    #[test]\n    fn micro_step_unlocked_uses_cursor() {"
    wave_test = '''    #[test]
    fn py2561_to_py2620_recursion_comprehension_chain() {
        for n in 2561..=2620 {
            let step = coding_step_by_micro_step(n).expect("wave27 chain step");
            assert_eq!(step.micro_step, n);
            assert!(step.id.starts_with(&format!("py-{n}-")));
            if n < 2620 {
                let next_step = coding_step_by_micro_step(n + 1).expect("next wave27 step");
                assert_eq!(step.next, Some(next_step.id));
            } else {
                assert_eq!(step.next, None, "step 2620 is the end of the rail");
            }
        }
    }

'''
    curriculum = replace_once(curriculum, test_anchor, wave_test + test_anchor, "Wave 27 Rust test")

    rows = partition_rows()
    concepts = CONCEPTS.read_text(encoding="utf-8")
    concepts = replace_once(concepts, "    (2560, &[5, 3]),\n];", "    (2560, &[5, 3]),\n" + rows + "\n];", "partition tail")
    start = concepts.index("    const WAVE26_FROZEN_BEYOND_2500:")
    end = concepts.index("\n    ];", start) + len("\n    ];")
    replacement = "    const WAVE27_FROZEN_BEYOND_2560: &[(i32, &[u8])] = &[\n" + rows + "\n    ];"
    concepts = concepts[:start] + replacement + concepts[end:]
    concepts = replace_once(concepts, "fn wave26_freeze_rows_beyond_2500()", "fn wave27_freeze_rows_beyond_2560()", "freeze test")
    concepts = replace_once(concepts, ".filter(|(n, _)| *n > 2500)", ".filter(|(n, _)| *n > 2560)", "freeze filter")
    concepts = replace_once(concepts, "WAVE26_FROZEN_BEYOND_2500", "WAVE27_FROZEN_BEYOND_2560", "freeze expected")
    concepts = replace_once(concepts, '"do not add or remove rows > 2500"', '"do not add or remove rows > 2560"', "freeze message")

    outputs = {CURRICULUM: curriculum, CONCEPTS: concepts}
    for path in E2E:
        source = path.read_text(encoding="utf-8")
        outputs[path] = replace_once(source, "toHaveCount(2560)", "toHaveCount(2620)", str(path))
    return outputs


if __name__ == "__main__":
    outputs = transformed_files()  # all anchors are checked before the first write
    for path, source in outputs.items():
        path.write_text(source, encoding="utf-8")
    print("Applied ordered Wave 27: 2561..=2620")
