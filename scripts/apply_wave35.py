"""Apply operational-resilience Wave 35 to the exact 3040-step baseline."""

from pathlib import Path
import gen_wave35

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
        (3041, 3046, (5, 3)), (3047, 3052, (5, 3)),
        (3053, 3058, (5, 3)), (3059, 3064, (5, 3)),
        (3065, 3070, (5, 1)), (3071, 3076, (5, 3)),
        (3077, 3082, (5, 3)), (3083, 3088, (5, 1)),
        (3089, 3094, (5, 3)), (3095, 3100, (5, 3)),
    )
    return "\n".join(
        f"    ({number}, &[{', '.join(map(str, tags))}]),"
        for start, end, tags in families for number in range(start, end + 1)
    )


def transformed_files():
    steps = gen_wave35.build_raw()
    curriculum = CURRICULUM.read_text(encoding="utf-8")
    if "micro_step: 3100," in curriculum:
        return {}
    curriculum = replace_once(
        curriculum,
        '    next: None, show_type_chips: false, micro_step: 3040,\n};\npub const CODING_STEPS:',
        '    next: Some("py-3041-telemetria-normalizar"), show_type_chips: false, micro_step: 3040,\n};\n'
        + gen_wave35.emit_rust(steps) + '\npub const CODING_STEPS:',
        "step 3040 boundary",
    )
    curriculum = replace_once(
        curriculum, "    &PY3040_OLA34_SUITE,\n\n];",
        "    &PY3040_OLA34_SUITE,\n" + gen_wave35.emit_refs(steps) + "\n\n];",
        "catalog references",
    )
    for old, new, label in (
        ("CODING_STEPS.len(), 3040", "CODING_STEPS.len(), 3100", "Rust count"),
        ("catalog must contain 3040 steps", "catalog must contain 3100 steps", "count message"),
        ("step.micro_step <= 3040", "step.micro_step <= 3100", "step ceiling"),
        ("(1..=3040).collect()", "(1..=3100).collect()", "Rust exact range"),
        ('1..=3040"', '1..=3100"', "range message"),
        ('assert_eq!(step.next, None, "step 3040 is the end of the rail");',
         'assert_eq!(step.next, Some("py-3041-telemetria-normalizar"), "step 3040 chains to Wave 35");',
         "Wave 34 boundary test"),
    ):
        curriculum = replace_once(curriculum, old, new, label)
    test_anchor = "    #[test]\n    fn micro_step_unlocked_uses_cursor() {"
    wave_test = '''    #[test]
    fn py3041_to_py3100_operational_resilience_chain() {
        for n in 3041..=3100 {
            let step = coding_step_by_micro_step(n).expect("wave35 chain step");
            assert_eq!(step.micro_step, n);
            assert!(step.id.starts_with(&format!("py-{n}-")));
            if n < 3100 {
                let next_step = coding_step_by_micro_step(n + 1).expect("next wave35 step");
                assert_eq!(step.next, Some(next_step.id));
            } else {
                assert_eq!(step.next, None, "step 3100 is the end of the rail");
            }
        }
    }

'''
    curriculum = replace_once(curriculum, test_anchor, wave_test + test_anchor, "Wave 35 Rust test")

    rows = partition_rows()
    concepts = CONCEPTS.read_text(encoding="utf-8")
    concepts = replace_once(concepts, "    (3040, &[5, 3]),\n];", "    (3040, &[5, 3]),\n" + rows + "\n];", "partition tail")
    start = concepts.index("    const WAVE34_FROZEN_BEYOND_2980:")
    end = concepts.index("\n    ];", start) + len("\n    ];")
    replacement = "    const WAVE35_FROZEN_BEYOND_3040: &[(i32, &[u8])] = &[\n" + rows + "\n    ];"
    concepts = concepts[:start] + replacement + concepts[end:]
    concepts = replace_once(concepts, "fn wave34_freeze_rows_beyond_2980()", "fn wave35_freeze_rows_beyond_3040()", "freeze test")
    concepts = replace_once(concepts, ".filter(|(n, _)| *n > 2980)", ".filter(|(n, _)| *n > 3040)", "freeze filter")
    concepts = replace_once(concepts, "WAVE34_FROZEN_BEYOND_2980", "WAVE35_FROZEN_BEYOND_3040", "freeze expected")
    concepts = replace_once(concepts, '"do not add or remove rows > 2980"', '"do not add or remove rows > 3040"', "freeze message")
    concepts = replace_once(concepts, "micro_step > 2980` (Wave 33 ceiling)", "micro_step > 3040` (Wave 34 ceiling)", "freeze comment")

    outputs = {CURRICULUM: curriculum, CONCEPTS: concepts}
    for path in E2E:
        source = path.read_text(encoding="utf-8")
        outputs[path] = replace_once(source, "toHaveCount(3040)", "toHaveCount(3100)", str(path))
    return outputs


if __name__ == "__main__":
    outputs = transformed_files()
    for path, source in outputs.items():
        path.write_text(source, encoding="utf-8")
    print("Applied operational-resilience Wave 35: 3041..=3100" if outputs else "Wave 35 already applied; no changes")
