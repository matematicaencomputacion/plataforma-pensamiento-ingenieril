"""Apply ordered Wave 29 to the exact 2680-step baseline."""

from pathlib import Path
import gen_wave29

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
        (2681, 2686, (3,)), (2687, 2692, (3, 2)),
        (2693, 2698, (3, 4)), (2699, 2704, (3, 1)),
        (2705, 2710, (5, 1)), (2711, 2716, (3, 2)),
        (2717, 2722, (5, 3)), (2723, 2728, (3, 4)),
        (2729, 2734, (3, 1)), (2735, 2740, (5, 3)),
    )
    return "\n".join(
        f"    ({number}, &[{', '.join(map(str, tags))}]),"
        for start, end, tags in families for number in range(start, end + 1)
    )


def transformed_files():
    steps = gen_wave29.build_raw(gen_wave29.RAW)
    curriculum = CURRICULUM.read_text(encoding="utf-8")
    curriculum = replace_once(
        curriculum,
        '    next: None, show_type_chips: false, micro_step: 2680,\n};\npub const CODING_STEPS:',
        '    next: Some("py-2681-hof-normalizar"), show_type_chips: false, micro_step: 2680,\n};\n'
        + gen_wave29.emit_rust(steps) + '\npub const CODING_STEPS:',
        "step 2680 boundary",
    )
    curriculum = replace_once(
        curriculum, "    &PY2680_OLA28_SUITE,\n\n];",
        "    &PY2680_OLA28_SUITE,\n" + gen_wave29.emit_refs(steps) + "\n\n];",
        "catalog references",
    )
    for old, new, label in (
        ("CODING_STEPS.len(), 2680", "CODING_STEPS.len(), 2740", "Rust count"),
        ("catalog must contain 2680 steps", "catalog must contain 2740 steps", "count message"),
        ("step.micro_step <= 2680", "step.micro_step <= 2740", "step ceiling"),
        ("(1..=2680).collect()", "(1..=2740).collect()", "Rust exact range"),
        ('1..=2680"', '1..=2740"', "range message"),
        ('assert_eq!(step.next, None, "step 2680 is the end of the rail");',
         'assert_eq!(step.next, Some("py-2681-hof-normalizar"), "step 2680 chains to Wave 29");',
         "Wave 28 boundary test"),
    ):
        curriculum = replace_once(curriculum, old, new, label)
    test_anchor = "    #[test]\n    fn micro_step_unlocked_uses_cursor() {"
    wave_test = '''    #[test]
    fn py2681_to_py2740_lazy_streaming_chain() {
        for n in 2681..=2740 {
            let step = coding_step_by_micro_step(n).expect("wave29 chain step");
            assert_eq!(step.micro_step, n);
            assert!(step.id.starts_with(&format!("py-{n}-")));
            if n < 2740 {
                let next_step = coding_step_by_micro_step(n + 1).expect("next wave29 step");
                assert_eq!(step.next, Some(next_step.id));
            } else {
                assert_eq!(step.next, None, "step 2740 is the end of the rail");
            }
        }
    }

'''
    curriculum = replace_once(curriculum, test_anchor, wave_test + test_anchor, "Wave 29 Rust test")

    rows = partition_rows()
    concepts = CONCEPTS.read_text(encoding="utf-8")
    concepts = replace_once(concepts, "    (2680, &[3, 5]),\n];", "    (2680, &[3, 5]),\n" + rows + "\n];", "partition tail")
    start = concepts.index("    const WAVE28_FROZEN_BEYOND_2620:")
    end = concepts.index("\n    ];", start) + len("\n    ];")
    replacement = "    const WAVE29_FROZEN_BEYOND_2680: &[(i32, &[u8])] = &[\n" + rows + "\n    ];"
    concepts = concepts[:start] + replacement + concepts[end:]
    concepts = replace_once(concepts, "fn wave28_freeze_rows_beyond_2620()", "fn wave29_freeze_rows_beyond_2680()", "freeze test")
    concepts = replace_once(concepts, ".filter(|(n, _)| *n > 2620)", ".filter(|(n, _)| *n > 2680)", "freeze filter")
    concepts = replace_once(concepts, "WAVE28_FROZEN_BEYOND_2620", "WAVE29_FROZEN_BEYOND_2680", "freeze expected")
    concepts = replace_once(concepts, '"do not add or remove rows > 2620"', '"do not add or remove rows > 2680"', "freeze message")

    outputs = {CURRICULUM: curriculum, CONCEPTS: concepts}
    for path in E2E:
        source = path.read_text(encoding="utf-8")
        outputs[path] = replace_once(source, "toHaveCount(2680)", "toHaveCount(2740)", str(path))
    return outputs


if __name__ == "__main__":
    outputs = transformed_files()
    for path, source in outputs.items():
        path.write_text(source, encoding="utf-8")
    print("Applied ordered Wave 29: 2681..=2740")
