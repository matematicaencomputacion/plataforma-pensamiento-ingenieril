"""Apply distributed-aggregation Wave 32 to the exact 2860-step baseline."""

from pathlib import Path
import gen_wave32

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
        (2861, 2866, (5, 1)), (2867, 2872, (5, 3)),
        (2873, 2878, (5, 3)), (2879, 2884, (5, 3)),
        (2885, 2890, (5, 3)), (2891, 2896, (5, 1)),
        (2897, 2902, (5, 3)), (2903, 2908, (5, 1)),
        (2909, 2914, (5, 3)), (2915, 2920, (5, 3)),
    )
    return "\n".join(
        f"    ({number}, &[{', '.join(map(str, tags))}]),"
        for start, end, tags in families for number in range(start, end + 1)
    )


def transformed_files():
    steps = gen_wave32.build_raw(gen_wave32.RAW)
    curriculum = CURRICULUM.read_text(encoding="utf-8")
    if "micro_step: 2920," in curriculum:
        return {}
    curriculum = replace_once(
        curriculum,
        '    next: None, show_type_chips: false, micro_step: 2860,\n};\npub const CODING_STEPS:',
        '    next: Some("py-2861-particionar-paridad"), show_type_chips: false, micro_step: 2860,\n};\n'
        + gen_wave32.emit_rust(steps) + '\npub const CODING_STEPS:',
        "step 2860 boundary",
    )
    curriculum = replace_once(
        curriculum, "    &PY2860_OLA31_SUITE,\n\n];",
        "    &PY2860_OLA31_SUITE,\n" + gen_wave32.emit_refs(steps) + "\n\n];",
        "catalog references",
    )
    for old, new, label in (
        ("CODING_STEPS.len(), 2860", "CODING_STEPS.len(), 2920", "Rust count"),
        ("catalog must contain 2860 steps", "catalog must contain 2920 steps", "count message"),
        ("step.micro_step <= 2860", "step.micro_step <= 2920", "step ceiling"),
        ("(1..=2860).collect()", "(1..=2920).collect()", "Rust exact range"),
        ('1..=2860"', '1..=2920"', "range message"),
        ('assert_eq!(step.next, None, "step 2860 is the end of the rail");',
         'assert_eq!(step.next, Some("py-2861-particionar-paridad"), "step 2860 chains to Wave 32");',
         "Wave 31 boundary test"),
    ):
        curriculum = replace_once(curriculum, old, new, label)
    test_anchor = "    #[test]\n    fn micro_step_unlocked_uses_cursor() {"
    wave_test = '''    #[test]
    fn py2861_to_py2920_distributed_aggregation_chain() {
        for n in 2861..=2920 {
            let step = coding_step_by_micro_step(n).expect("wave32 chain step");
            assert_eq!(step.micro_step, n);
            assert!(step.id.starts_with(&format!("py-{n}-")));
            if n < 2920 {
                let next_step = coding_step_by_micro_step(n + 1).expect("next wave32 step");
                assert_eq!(step.next, Some(next_step.id));
            } else {
                assert_eq!(step.next, None, "step 2920 is the end of the rail");
            }
        }
    }

'''
    curriculum = replace_once(curriculum, test_anchor, wave_test + test_anchor, "Wave 32 Rust test")

    rows = partition_rows()
    concepts = CONCEPTS.read_text(encoding="utf-8")
    concepts = replace_once(concepts, "    (2860, &[5, 3]),\n];", "    (2860, &[5, 3]),\n" + rows + "\n];", "partition tail")
    start = concepts.index("    const WAVE31_FROZEN_BEYOND_2800:")
    end = concepts.index("\n    ];", start) + len("\n    ];")
    replacement = "    const WAVE32_FROZEN_BEYOND_2860: &[(i32, &[u8])] = &[\n" + rows + "\n    ];"
    concepts = concepts[:start] + replacement + concepts[end:]
    concepts = replace_once(concepts, "fn wave31_freeze_rows_beyond_2800()", "fn wave32_freeze_rows_beyond_2860()", "freeze test")
    concepts = replace_once(concepts, ".filter(|(n, _)| *n > 2800)", ".filter(|(n, _)| *n > 2860)", "freeze filter")
    concepts = replace_once(concepts, "WAVE31_FROZEN_BEYOND_2800", "WAVE32_FROZEN_BEYOND_2860", "freeze expected")
    concepts = replace_once(concepts, '"do not add or remove rows > 2800"', '"do not add or remove rows > 2860"', "freeze message")
    concepts = replace_once(concepts, "micro_step > 2800` (Wave 30 ceiling)", "micro_step > 2860` (Wave 31 ceiling)", "freeze comment")

    outputs = {CURRICULUM: curriculum, CONCEPTS: concepts}
    for path in E2E:
        source = path.read_text(encoding="utf-8")
        outputs[path] = replace_once(source, "toHaveCount(2860)", "toHaveCount(2920)", str(path))
    return outputs


if __name__ == "__main__":
    outputs = transformed_files()
    for path, source in outputs.items():
        path.write_text(source, encoding="utf-8")
    print("Applied distributed-aggregation Wave 32: 2861..=2920" if outputs else "Wave 32 already applied; no changes")
