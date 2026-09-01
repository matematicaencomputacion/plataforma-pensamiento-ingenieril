"""Apply resilient Wave 31 to the exact 2800-step baseline."""

from pathlib import Path
import gen_wave31

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
        (2801, 2806, (3, 1)), (2807, 2812, (3, 2)),
        (2813, 2818, (5, 1)), (2819, 2824, (5, 3)),
        (2825, 2830, (5, 3)), (2831, 2836, (5, 1)),
        (2837, 2842, (3, 4)), (2843, 2848, (5, 3)),
        (2849, 2854, (5, 3)), (2855, 2860, (5, 3)),
    )
    return "\n".join(
        f"    ({number}, &[{', '.join(map(str, tags))}]),"
        for start, end, tags in families for number in range(start, end + 1)
    )


def transformed_files():
    steps = gen_wave31.build_raw(gen_wave31.RAW)
    curriculum = CURRICULUM.read_text(encoding="utf-8")
    curriculum = replace_once(
        curriculum,
        '    next: None, show_type_chips: false, micro_step: 2800,\n};\npub const CODING_STEPS:',
        '    next: Some("py-2801-validar-requeridos"), show_type_chips: false, micro_step: 2800,\n};\n'
        + gen_wave31.emit_rust(steps) + '\npub const CODING_STEPS:',
        "step 2800 boundary",
    )
    curriculum = replace_once(
        curriculum, "    &PY2800_OLA30_SUITE,\n\n];",
        "    &PY2800_OLA30_SUITE,\n" + gen_wave31.emit_refs(steps) + "\n\n];",
        "catalog references",
    )
    for old, new, label in (
        ("CODING_STEPS.len(), 2800", "CODING_STEPS.len(), 2860", "Rust count"),
        ("catalog must contain 2800 steps", "catalog must contain 2860 steps", "count message"),
        ("step.micro_step <= 2800", "step.micro_step <= 2860", "step ceiling"),
        ("(1..=2800).collect()", "(1..=2860).collect()", "Rust exact range"),
        ('1..=2800"', '1..=2860"', "range message"),
        ('assert_eq!(step.next, None, "step 2800 is the end of the rail");',
         'assert_eq!(step.next, Some("py-2801-validar-requeridos"), "step 2800 chains to Wave 31");',
         "Wave 30 boundary test"),
    ):
        curriculum = replace_once(curriculum, old, new, label)
    test_anchor = "    #[test]\n    fn micro_step_unlocked_uses_cursor() {"
    wave_test = '''    #[test]
    fn py2801_to_py2860_resilient_pipeline_chain() {
        for n in 2801..=2860 {
            let step = coding_step_by_micro_step(n).expect("wave31 chain step");
            assert_eq!(step.micro_step, n);
            assert!(step.id.starts_with(&format!("py-{n}-")));
            if n < 2860 {
                let next_step = coding_step_by_micro_step(n + 1).expect("next wave31 step");
                assert_eq!(step.next, Some(next_step.id));
            } else {
                assert_eq!(step.next, None, "step 2860 is the end of the rail");
            }
        }
    }

'''
    curriculum = replace_once(curriculum, test_anchor, wave_test + test_anchor, "Wave 31 Rust test")

    rows = partition_rows()
    concepts = CONCEPTS.read_text(encoding="utf-8")
    concepts = replace_once(concepts, "    (2800, &[5, 3]),\n];", "    (2800, &[5, 3]),\n" + rows + "\n];", "partition tail")
    start = concepts.index("    const WAVE30_FROZEN_BEYOND_2740:")
    end = concepts.index("\n    ];", start) + len("\n    ];")
    replacement = "    const WAVE31_FROZEN_BEYOND_2800: &[(i32, &[u8])] = &[\n" + rows + "\n    ];"
    concepts = concepts[:start] + replacement + concepts[end:]
    concepts = replace_once(concepts, "fn wave30_freeze_rows_beyond_2740()", "fn wave31_freeze_rows_beyond_2800()", "freeze test")
    concepts = replace_once(concepts, ".filter(|(n, _)| *n > 2740)", ".filter(|(n, _)| *n > 2800)", "freeze filter")
    concepts = replace_once(concepts, "WAVE30_FROZEN_BEYOND_2740", "WAVE31_FROZEN_BEYOND_2800", "freeze expected")
    concepts = replace_once(concepts, '"do not add or remove rows > 2740"', '"do not add or remove rows > 2800"', "freeze message")
    concepts = replace_once(concepts, "micro_step > 2740` (Wave 29 ceiling)", "micro_step > 2800` (Wave 30 ceiling)", "freeze comment")

    outputs = {CURRICULUM: curriculum, CONCEPTS: concepts}
    for path in E2E:
        source = path.read_text(encoding="utf-8")
        outputs[path] = replace_once(source, "toHaveCount(2800)", "toHaveCount(2860)", str(path))
    return outputs


if __name__ == "__main__":
    outputs = transformed_files()
    for path, source in outputs.items():
        path.write_text(source, encoding="utf-8")
    print("Applied resilient Wave 31: 2801..=2860")
