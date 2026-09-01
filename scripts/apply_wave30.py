"""Apply ordered Wave 30 to the exact 2740-step baseline."""

from pathlib import Path
import gen_wave30

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
        (2741, 2746, (3, 1)), (2747, 2752, (3, 1)),
        (2753, 2758, (5, 1)), (2759, 2764, (3, 1)),
        (2765, 2770, (3, 1)), (2771, 2776, (3, 4)),
        (2777, 2782, (5, 3)), (2783, 2788, (5, 1)),
        (2789, 2794, (5, 3)), (2795, 2800, (5, 3)),
    )
    return "\n".join(
        f"    ({number}, &[{', '.join(map(str, tags))}]),"
        for start, end, tags in families for number in range(start, end + 1)
    )


def transformed_files():
    steps = gen_wave30.build_raw(gen_wave30.RAW)
    curriculum = CURRICULUM.read_text(encoding="utf-8")
    curriculum = replace_once(
        curriculum,
        '    next: None, show_type_chips: false, micro_step: 2740,\n};\npub const CODING_STEPS:',
        '    next: Some("py-2741-merge-dos-listas"), show_type_chips: false, micro_step: 2740,\n};\n'
        + gen_wave30.emit_rust(steps) + '\npub const CODING_STEPS:',
        "step 2740 boundary",
    )
    curriculum = replace_once(
        curriculum, "    &PY2740_OLA29_SUITE,\n\n];",
        "    &PY2740_OLA29_SUITE,\n" + gen_wave30.emit_refs(steps) + "\n\n];",
        "catalog references",
    )
    for old, new, label in (
        ("CODING_STEPS.len(), 2740", "CODING_STEPS.len(), 2800", "Rust count"),
        ("catalog must contain 2740 steps", "catalog must contain 2800 steps", "count message"),
        ("step.micro_step <= 2740", "step.micro_step <= 2800", "step ceiling"),
        ("(1..=2740).collect()", "(1..=2800).collect()", "Rust exact range"),
        ('1..=2740"', '1..=2800"', "range message"),
        ('assert_eq!(step.next, None, "step 2740 is the end of the rail");',
         'assert_eq!(step.next, Some("py-2741-merge-dos-listas"), "step 2740 chains to Wave 30");',
         "Wave 29 boundary test"),
    ):
        curriculum = replace_once(curriculum, old, new, label)
    test_anchor = "    #[test]\n    fn micro_step_unlocked_uses_cursor() {"
    wave_test = '''    #[test]
    fn py2741_to_py2800_reconciliation_chain() {
        for n in 2741..=2800 {
            let step = coding_step_by_micro_step(n).expect("wave30 chain step");
            assert_eq!(step.micro_step, n);
            assert!(step.id.starts_with(&format!("py-{n}-")));
            if n < 2800 {
                let next_step = coding_step_by_micro_step(n + 1).expect("next wave30 step");
                assert_eq!(step.next, Some(next_step.id));
            } else {
                assert_eq!(step.next, None, "step 2800 is the end of the rail");
            }
        }
    }

'''
    curriculum = replace_once(curriculum, test_anchor, wave_test + test_anchor, "Wave 30 Rust test")

    rows = partition_rows()
    concepts = CONCEPTS.read_text(encoding="utf-8")
    concepts = replace_once(concepts, "    (2740, &[5, 3]),\n];", "    (2740, &[5, 3]),\n" + rows + "\n];", "partition tail")
    start = concepts.index("    const WAVE29_FROZEN_BEYOND_2680:")
    end = concepts.index("\n    ];", start) + len("\n    ];")
    replacement = "    const WAVE30_FROZEN_BEYOND_2740: &[(i32, &[u8])] = &[\n" + rows + "\n    ];"
    concepts = concepts[:start] + replacement + concepts[end:]
    concepts = replace_once(concepts, "fn wave29_freeze_rows_beyond_2680()", "fn wave30_freeze_rows_beyond_2740()", "freeze test")
    concepts = replace_once(concepts, ".filter(|(n, _)| *n > 2680)", ".filter(|(n, _)| *n > 2740)", "freeze filter")
    concepts = replace_once(concepts, "WAVE29_FROZEN_BEYOND_2680", "WAVE30_FROZEN_BEYOND_2740", "freeze expected")
    concepts = replace_once(concepts, '"do not add or remove rows > 2680"', '"do not add or remove rows > 2740"', "freeze message")

    outputs = {CURRICULUM: curriculum, CONCEPTS: concepts}
    for path in E2E:
        source = path.read_text(encoding="utf-8")
        outputs[path] = replace_once(source, "toHaveCount(2740)", "toHaveCount(2800)", str(path))
    return outputs


if __name__ == "__main__":
    outputs = transformed_files()
    for path, source in outputs.items():
        path.write_text(source, encoding="utf-8")
    print("Applied ordered Wave 30: 2741..=2800")
