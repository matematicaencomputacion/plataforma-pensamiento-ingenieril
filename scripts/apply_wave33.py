"""Apply coordinated-pipelines Wave 33 to the exact 2920-step baseline."""

from pathlib import Path
import gen_wave33

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
        (2921, 2926, (5, 1)), (2927, 2932, (5, 3)),
        (2933, 2938, (5, 3)), (2939, 2944, (5, 1)),
        (2945, 2950, (5, 3)), (2951, 2956, (5, 3)),
        (2957, 2962, (5, 1)), (2963, 2968, (5, 3)),
        (2969, 2974, (5, 3)), (2975, 2980, (5, 3)),
    )
    return "\n".join(
        f"    ({number}, &[{', '.join(map(str, tags))}]),"
        for start, end, tags in families for number in range(start, end + 1)
    )


def transformed_files():
    steps = gen_wave33.build_raw()
    curriculum = CURRICULUM.read_text(encoding="utf-8")
    if "micro_step: 2980," in curriculum:
        return {}
    curriculum = replace_once(
        curriculum,
        '    next: None, show_type_chips: false, micro_step: 2920,\n};\npub const CODING_STEPS:',
        '    next: Some("py-2921-offset-siguiente"), show_type_chips: false, micro_step: 2920,\n};\n'
        + gen_wave33.emit_rust(steps) + '\npub const CODING_STEPS:',
        "step 2920 boundary",
    )
    curriculum = replace_once(
        curriculum, "    &PY2920_OLA32_SUITE,\n\n];",
        "    &PY2920_OLA32_SUITE,\n" + gen_wave33.emit_refs(steps) + "\n\n];",
        "catalog references",
    )
    for old, new, label in (
        ("CODING_STEPS.len(), 2920", "CODING_STEPS.len(), 2980", "Rust count"),
        ("catalog must contain 2920 steps", "catalog must contain 2980 steps", "count message"),
        ("step.micro_step <= 2920", "step.micro_step <= 2980", "step ceiling"),
        ("(1..=2920).collect()", "(1..=2980).collect()", "Rust exact range"),
        ('1..=2920"', '1..=2980"', "range message"),
        ('assert_eq!(step.next, None, "step 2920 is the end of the rail");',
         'assert_eq!(step.next, Some("py-2921-offset-siguiente"), "step 2920 chains to Wave 33");',
         "Wave 32 boundary test"),
    ):
        curriculum = replace_once(curriculum, old, new, label)
    test_anchor = "    #[test]\n    fn micro_step_unlocked_uses_cursor() {"
    wave_test = '''    #[test]
    fn py2921_to_py2980_coordinated_pipeline_chain() {
        for n in 2921..=2980 {
            let step = coding_step_by_micro_step(n).expect("wave33 chain step");
            assert_eq!(step.micro_step, n);
            assert!(step.id.starts_with(&format!("py-{n}-")));
            if n < 2980 {
                let next_step = coding_step_by_micro_step(n + 1).expect("next wave33 step");
                assert_eq!(step.next, Some(next_step.id));
            } else {
                assert_eq!(step.next, None, "step 2980 is the end of the rail");
            }
        }
    }

'''
    curriculum = replace_once(curriculum, test_anchor, wave_test + test_anchor, "Wave 33 Rust test")

    rows = partition_rows()
    concepts = CONCEPTS.read_text(encoding="utf-8")
    concepts = replace_once(concepts, "    (2920, &[5, 3]),\n];", "    (2920, &[5, 3]),\n" + rows + "\n];", "partition tail")
    start = concepts.index("    const WAVE32_FROZEN_BEYOND_2860:")
    end = concepts.index("\n    ];", start) + len("\n    ];")
    replacement = "    const WAVE33_FROZEN_BEYOND_2920: &[(i32, &[u8])] = &[\n" + rows + "\n    ];"
    concepts = concepts[:start] + replacement + concepts[end:]
    concepts = replace_once(concepts, "fn wave32_freeze_rows_beyond_2860()", "fn wave33_freeze_rows_beyond_2920()", "freeze test")
    concepts = replace_once(concepts, ".filter(|(n, _)| *n > 2860)", ".filter(|(n, _)| *n > 2920)", "freeze filter")
    concepts = replace_once(concepts, "WAVE32_FROZEN_BEYOND_2860", "WAVE33_FROZEN_BEYOND_2920", "freeze expected")
    concepts = replace_once(concepts, '"do not add or remove rows > 2860"', '"do not add or remove rows > 2920"', "freeze message")
    concepts = replace_once(concepts, "micro_step > 2860` (Wave 31 ceiling)", "micro_step > 2920` (Wave 32 ceiling)", "freeze comment")

    outputs = {CURRICULUM: curriculum, CONCEPTS: concepts}
    for path in E2E:
        source = path.read_text(encoding="utf-8")
        outputs[path] = replace_once(source, "toHaveCount(2920)", "toHaveCount(2980)", str(path))
    return outputs


if __name__ == "__main__":
    outputs = transformed_files()
    for path, source in outputs.items():
        path.write_text(source, encoding="utf-8")
    print("Applied coordinated-pipelines Wave 33: 2921..=2980" if outputs else "Wave 33 already applied; no changes")
