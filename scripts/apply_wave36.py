"""Apply safe-delivery Wave 36 to the exact 3100-step baseline."""

from pathlib import Path
import gen_wave36

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
        (3101, 3106, (5, 3)), (3107, 3112, (5, 3)),
        (3113, 3118, (5, 3)), (3119, 3124, (5, 3)),
        (3125, 3130, (5, 3)), (3131, 3136, (5, 3)),
        (3137, 3142, (5, 3)), (3143, 3148, (5, 1)),
        (3149, 3154, (5, 3)), (3155, 3160, (5, 3)),
    )
    return "\n".join(
        f"    ({number}, &[{', '.join(map(str, tags))}]),"
        for start, end, tags in families for number in range(start, end + 1)
    )


def transformed_files():
    steps = gen_wave36.build_raw()
    curriculum = CURRICULUM.read_text(encoding="utf-8")
    if "micro_step: 3160," in curriculum:
        return {}
    curriculum = replace_once(
        curriculum,
        '    next: None, show_type_chips: false, micro_step: 3100,\n};\npub const CODING_STEPS:',
        '    next: Some("py-3101-release-manifiesto"), show_type_chips: false, micro_step: 3100,\n};\n'
        + gen_wave36.emit_rust(steps) + '\npub const CODING_STEPS:',
        "step 3100 boundary",
    )
    curriculum = replace_once(
        curriculum, "    &PY3100_OLA35_SUITE,\n\n];",
        "    &PY3100_OLA35_SUITE,\n" + gen_wave36.emit_refs(steps) + "\n\n];",
        "catalog references",
    )
    for old, new, label in (
        ("CODING_STEPS.len(), 3100", "CODING_STEPS.len(), 3160", "Rust count"),
        ("catalog must contain 3100 steps", "catalog must contain 3160 steps", "count message"),
        ("step.micro_step <= 3100", "step.micro_step <= 3160", "step ceiling"),
        ("(1..=3100).collect()", "(1..=3160).collect()", "Rust exact range"),
        ('1..=3100"', '1..=3160"', "range message"),
        ('assert_eq!(step.next, None, "step 3100 is the end of the rail");',
         'assert_eq!(step.next, Some("py-3101-release-manifiesto"), "step 3100 chains to Wave 36");',
         "Wave 35 boundary test"),
    ):
        curriculum = replace_once(curriculum, old, new, label)
    test_anchor = "    #[test]\n    fn micro_step_unlocked_uses_cursor() {"
    wave_test = '''    #[test]
    fn py3101_to_py3160_safe_delivery_chain() {
        for n in 3101..=3160 {
            let step = coding_step_by_micro_step(n).expect("wave36 chain step");
            assert_eq!(step.micro_step, n);
            assert!(step.id.starts_with(&format!("py-{n}-")));
            if n < 3160 {
                let next_step = coding_step_by_micro_step(n + 1).expect("next wave36 step");
                assert_eq!(step.next, Some(next_step.id));
            } else {
                assert_eq!(step.next, None, "step 3160 is the end of the rail");
            }
        }
    }

'''
    curriculum = replace_once(curriculum, test_anchor, wave_test + test_anchor, "Wave 36 Rust test")

    rows = partition_rows()
    concepts = CONCEPTS.read_text(encoding="utf-8")
    concepts = replace_once(concepts, "    (3100, &[5, 3]),\n];", "    (3100, &[5, 3]),\n" + rows + "\n];", "partition tail")
    start = concepts.index("    const WAVE35_FROZEN_BEYOND_3040:")
    end = concepts.index("\n    ];", start) + len("\n    ];")
    replacement = "    const WAVE36_FROZEN_BEYOND_3100: &[(i32, &[u8])] = &[\n" + rows + "\n    ];"
    concepts = concepts[:start] + replacement + concepts[end:]
    concepts = replace_once(concepts, "fn wave35_freeze_rows_beyond_3040()", "fn wave36_freeze_rows_beyond_3100()", "freeze test")
    concepts = replace_once(concepts, ".filter(|(n, _)| *n > 3040)", ".filter(|(n, _)| *n > 3100)", "freeze filter")
    concepts = replace_once(concepts, "WAVE35_FROZEN_BEYOND_3040", "WAVE36_FROZEN_BEYOND_3100", "freeze expected")
    concepts = replace_once(concepts, '"do not add or remove rows > 3040"', '"do not add or remove rows > 3100"', "freeze message")
    concepts = replace_once(concepts, "micro_step > 3040` (Wave 34 ceiling)", "micro_step > 3100` (Wave 35 ceiling)", "freeze comment")

    outputs = {CURRICULUM: curriculum, CONCEPTS: concepts}
    for path in E2E:
        source = path.read_text(encoding="utf-8")
        outputs[path] = replace_once(source, "toHaveCount(3100)", "toHaveCount(3160)", str(path))
    return outputs


if __name__ == "__main__":
    outputs = transformed_files()
    for path, source in outputs.items():
        path.write_text(source, encoding="utf-8")
    print("Applied safe-delivery Wave 36: 3101..=3160" if outputs else "Wave 36 already applied; no changes")
