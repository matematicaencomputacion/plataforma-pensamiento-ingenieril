"""Apply consistency-recovery Wave 34 to the exact 2980-step baseline."""

from pathlib import Path
import gen_wave34

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
        (2981, 2986, (5, 1)), (2987, 2992, (5, 3)),
        (2993, 2998, (5, 3)), (2999, 3004, (5, 3)),
        (3005, 3010, (5, 1)), (3011, 3016, (5, 3)),
        (3017, 3022, (5, 1)), (3023, 3028, (5, 3)),
        (3029, 3034, (5, 3)), (3035, 3040, (5, 3)),
    )
    return "\n".join(
        f"    ({number}, &[{', '.join(map(str, tags))}]),"
        for start, end, tags in families for number in range(start, end + 1)
    )


def transformed_files():
    steps = gen_wave34.build_raw()
    curriculum = CURRICULUM.read_text(encoding="utf-8")
    if "micro_step: 3040," in curriculum:
        return {}
    curriculum = replace_once(
        curriculum,
        '    next: None, show_type_chips: false, micro_step: 2980,\n};\npub const CODING_STEPS:',
        '    next: Some("py-2981-esquema-campos"), show_type_chips: false, micro_step: 2980,\n};\n'
        + gen_wave34.emit_rust(steps) + '\npub const CODING_STEPS:',
        "step 2980 boundary",
    )
    curriculum = replace_once(
        curriculum, "    &PY2980_OLA33_SUITE,\n\n];",
        "    &PY2980_OLA33_SUITE,\n" + gen_wave34.emit_refs(steps) + "\n\n];",
        "catalog references",
    )
    for old, new, label in (
        ("CODING_STEPS.len(), 2980", "CODING_STEPS.len(), 3040", "Rust count"),
        ("catalog must contain 2980 steps", "catalog must contain 3040 steps", "count message"),
        ("step.micro_step <= 2980", "step.micro_step <= 3040", "step ceiling"),
        ("(1..=2980).collect()", "(1..=3040).collect()", "Rust exact range"),
        ('1..=2980"', '1..=3040"', "range message"),
        ('assert_eq!(step.next, None, "step 2980 is the end of the rail");',
         'assert_eq!(step.next, Some("py-2981-esquema-campos"), "step 2980 chains to Wave 34");',
         "Wave 33 boundary test"),
    ):
        curriculum = replace_once(curriculum, old, new, label)
    test_anchor = "    #[test]\n    fn micro_step_unlocked_uses_cursor() {"
    wave_test = '''    #[test]
    fn py2981_to_py3040_consistency_recovery_chain() {
        for n in 2981..=3040 {
            let step = coding_step_by_micro_step(n).expect("wave34 chain step");
            assert_eq!(step.micro_step, n);
            assert!(step.id.starts_with(&format!("py-{n}-")));
            if n < 3040 {
                let next_step = coding_step_by_micro_step(n + 1).expect("next wave34 step");
                assert_eq!(step.next, Some(next_step.id));
            } else {
                assert_eq!(step.next, None, "step 3040 is the end of the rail");
            }
        }
    }

'''
    curriculum = replace_once(curriculum, test_anchor, wave_test + test_anchor, "Wave 34 Rust test")

    rows = partition_rows()
    concepts = CONCEPTS.read_text(encoding="utf-8")
    concepts = replace_once(concepts, "    (2980, &[5, 3]),\n];", "    (2980, &[5, 3]),\n" + rows + "\n];", "partition tail")
    start = concepts.index("    const WAVE33_FROZEN_BEYOND_2920:")
    end = concepts.index("\n    ];", start) + len("\n    ];")
    replacement = "    const WAVE34_FROZEN_BEYOND_2980: &[(i32, &[u8])] = &[\n" + rows + "\n    ];"
    concepts = concepts[:start] + replacement + concepts[end:]
    concepts = replace_once(concepts, "fn wave33_freeze_rows_beyond_2920()", "fn wave34_freeze_rows_beyond_2980()", "freeze test")
    concepts = replace_once(concepts, ".filter(|(n, _)| *n > 2920)", ".filter(|(n, _)| *n > 2980)", "freeze filter")
    concepts = replace_once(concepts, "WAVE33_FROZEN_BEYOND_2920", "WAVE34_FROZEN_BEYOND_2980", "freeze expected")
    concepts = replace_once(concepts, '"do not add or remove rows > 2920"', '"do not add or remove rows > 2980"', "freeze message")
    concepts = replace_once(concepts, "micro_step > 2920` (Wave 32 ceiling)", "micro_step > 2980` (Wave 33 ceiling)", "freeze comment")

    outputs = {CURRICULUM: curriculum, CONCEPTS: concepts}
    for path in E2E:
        source = path.read_text(encoding="utf-8")
        outputs[path] = replace_once(source, "toHaveCount(2980)", "toHaveCount(3040)", str(path))
    return outputs


if __name__ == "__main__":
    outputs = transformed_files()
    for path, source in outputs.items():
        path.write_text(source, encoding="utf-8")
    print("Applied consistency-recovery Wave 34: 2981..=3040" if outputs else "Wave 34 already applied; no changes")
