"""Apply ordered Wave 28 to the exact 2620-step baseline."""

from pathlib import Path
import gen_wave28

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
        (2621, 2626, (3, 1)), (2627, 2632, (3, 5)),
        (2633, 2638, (3, 1)), (2639, 2644, (3, 2)),
        (2645, 2650, (3, 1)), (2651, 2656, (3, 1)),
        (2657, 2662, (3, 1)), (2663, 2668, (3, 5)),
        (2669, 2674, (3, 4)), (2675, 2680, (3, 5)),
    )
    return "\n".join(
        f"    ({number}, &[{', '.join(map(str, tags))}]),"
        for start, end, tags in families for number in range(start, end + 1)
    )


def transformed_files():
    steps = gen_wave28.build_steps()
    curriculum = CURRICULUM.read_text(encoding="utf-8")
    curriculum = replace_once(
        curriculum,
        '    next: None, show_type_chips: false, micro_step: 2620,\n};\npub const CODING_STEPS:',
        '    next: Some("py-2621-chunk-tamano"), show_type_chips: false, micro_step: 2620,\n};\n'
        + gen_wave28.emit_rust(steps) + '\npub const CODING_STEPS:',
        "step 2620 boundary",
    )
    curriculum = replace_once(
        curriculum, "    &PY2620_OLA27_SUITE,\n\n];",
        "    &PY2620_OLA27_SUITE,\n" + gen_wave28.emit_refs(steps) + "\n\n];",
        "catalog references",
    )
    for old, new, label in (
        ("CODING_STEPS.len(), 2620", "CODING_STEPS.len(), 2680", "Rust count"),
        ("catalog must contain 2620 steps", "catalog must contain 2680 steps", "count message"),
        ("step.micro_step <= 2620", "step.micro_step <= 2680", "step ceiling"),
        ("(1..=2620).collect()", "(1..=2680).collect()", "Rust exact range"),
        ('1..=2620"', '1..=2680"', "range message"),
        ('assert_eq!(step.next, None, "step 2620 is the end of the rail");',
         'assert_eq!(step.next, Some("py-2621-chunk-tamano"), "step 2620 chains to Wave 28");',
         "Wave 27 boundary test"),
    ):
        curriculum = replace_once(curriculum, old, new, label)
    test_anchor = "    #[test]\n    fn micro_step_unlocked_uses_cursor() {"
    wave_test = '''    #[test]
    fn py2621_to_py2680_parallel_reduction_chain() {
        for n in 2621..=2680 {
            let step = coding_step_by_micro_step(n).expect("wave28 chain step");
            assert_eq!(step.micro_step, n);
            assert!(step.id.starts_with(&format!("py-{n}-")));
            if n < 2680 {
                let next_step = coding_step_by_micro_step(n + 1).expect("next wave28 step");
                assert_eq!(step.next, Some(next_step.id));
            } else {
                assert_eq!(step.next, None, "step 2680 is the end of the rail");
            }
        }
    }

'''
    curriculum = replace_once(curriculum, test_anchor, wave_test + test_anchor, "Wave 28 Rust test")

    rows = partition_rows()
    concepts = CONCEPTS.read_text(encoding="utf-8")
    concepts = replace_once(concepts, "    (2620, &[3, 5]),\n];", "    (2620, &[3, 5]),\n" + rows + "\n];", "partition tail")
    start = concepts.index("    const WAVE27_FROZEN_BEYOND_2560:")
    end = concepts.index("\n    ];", start) + len("\n    ];")
    replacement = "    const WAVE28_FROZEN_BEYOND_2620: &[(i32, &[u8])] = &[\n" + rows + "\n    ];"
    concepts = concepts[:start] + replacement + concepts[end:]
    concepts = replace_once(concepts, "fn wave27_freeze_rows_beyond_2560()", "fn wave28_freeze_rows_beyond_2620()", "freeze test")
    concepts = replace_once(concepts, ".filter(|(n, _)| *n > 2560)", ".filter(|(n, _)| *n > 2620)", "freeze filter")
    concepts = replace_once(concepts, "WAVE27_FROZEN_BEYOND_2560", "WAVE28_FROZEN_BEYOND_2620", "freeze expected")
    concepts = replace_once(concepts, '"do not add or remove rows > 2560"', '"do not add or remove rows > 2620"', "freeze message")

    outputs = {CURRICULUM: curriculum, CONCEPTS: concepts}
    for path in E2E:
        source = path.read_text(encoding="utf-8")
        outputs[path] = replace_once(source, "toHaveCount(2620)", "toHaveCount(2680)", str(path))
    return outputs


if __name__ == "__main__":
    outputs = transformed_files()  # verify every anchor before the first write
    for path, source in outputs.items():
        path.write_text(source, encoding="utf-8")
    print("Applied ordered Wave 28: 2621..=2680")
