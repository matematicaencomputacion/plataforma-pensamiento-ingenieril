"""Apply ordered Wave 26 to the current 2500-step baseline."""

from pathlib import Path

import gen_wave26

ROOT = Path(__file__).resolve().parents[1]
CURRICULUM = ROOT / "web/src/curriculum.rs"
CONCEPTS = ROOT / "web/src/concepts/mod.rs"
E2E = (
    ROOT / "web/e2e/tests/journey.auth-hub.spec.ts",
    ROOT / "web/e2e/tests/progress.check.spec.ts",
    ROOT / "web/e2e/tests/session.navigation.spec.ts",
)


def replace_once(source: str, old: str, new: str, label: str) -> str:
    count = source.count(old)
    if count != 1:
        raise RuntimeError(f"{label}: expected one exact anchor, found {count}")
    return source.replace(old, new, 1)


def partition_rows() -> str:
    families = (
        (2501, 2506, (3,)), (2507, 2512, (3, 2)),
        (2513, 2518, (3, 4)), (2519, 2524, (3, 1)),
        (2525, 2530, (5, 1)), (2531, 2536, (3, 2)),
        (2537, 2542, (5, 3)), (2543, 2548, (3, 4)),
        (2549, 2554, (3, 1)), (2555, 2560, (5, 3)),
    )
    return "\n".join(
        f"    ({number}, &[{', '.join(map(str, tags))}]),"
        for start, end, tags in families
        for number in range(start, end + 1)
    )


def apply_curriculum() -> None:
    source = CURRICULUM.read_text(encoding="utf-8")
    steps = gen_wave26.build_raw(gen_wave26.RAW)
    assert [step["num"] for step in steps] == list(range(2501, 2561))
    source = replace_once(
        source,
        '    next: None, show_type_chips: false, micro_step: 2500,\n};\npub const CODING_STEPS:',
        '    next: Some("py-2501-map-lambda"), show_type_chips: false, micro_step: 2500,\n};\n'
        + gen_wave26.emit_rust(steps) + '\npub const CODING_STEPS:',
        "step 2500 boundary",
    )
    source = replace_once(
        source, "    &PY2500_SCORE_CHECK,\n\n];",
        "    &PY2500_SCORE_CHECK,\n" + gen_wave26.emit_refs(steps) + "\n\n];",
        "catalog references",
    )
    source = replace_once(source, "CODING_STEPS.len(), 2500", "CODING_STEPS.len(), 2560", "Rust count")
    source = replace_once(source, "catalog must contain 2500 steps", "catalog must contain 2560 steps", "count message")
    source = replace_once(source, "step.micro_step <= 2500", "step.micro_step <= 2560", "step ceiling")
    source = replace_once(source, "(1..=2500).collect()", "(1..=2560).collect()", "Rust exact range")
    source = replace_once(source, "1..=2500\"", "1..=2560\"", "range message")
    CURRICULUM.write_text(source, encoding="utf-8")


def apply_concepts() -> None:
    source = CONCEPTS.read_text(encoding="utf-8")
    rows = partition_rows()
    source = replace_once(source, "    (2440, &[5, 3]),\n];", "    (2440, &[5, 3]),\n" + rows + "\n];", "partition tail")
    source = replace_once(
        source, "    const WAVE25_FROZEN_BEYOND_2500: &[(i32, &[u8])] = &[];",
        "    const WAVE26_FROZEN_BEYOND_2500: &[(i32, &[u8])] = &[\n" + rows + "\n    ];",
        "Wave 26 freeze",
    )
    source = replace_once(source, "fn wave24_freeze_rows_beyond_2440()", "fn wave26_freeze_rows_beyond_2500()", "freeze test")
    source = replace_once(source, ".filter(|(n, _)| *n > 2440)", ".filter(|(n, _)| *n > 2500)", "freeze filter")
    source = replace_once(source, "WAVE25_FROZEN_BEYOND_2500", "WAVE26_FROZEN_BEYOND_2500", "freeze expected")
    source = replace_once(source, '"do not add or remove rows > 2440"', '"do not add or remove rows > 2500"', "freeze message")
    CONCEPTS.write_text(source, encoding="utf-8")


def apply_e2e() -> None:
    for path in E2E:
        source = path.read_text(encoding="utf-8")
        source = replace_once(source, "toHaveCount(2500)", "toHaveCount(2560)", str(path))
        path.write_text(source, encoding="utf-8")


if __name__ == "__main__":
    apply_curriculum()
    apply_concepts()
    apply_e2e()
    print("Applied ordered Wave 26: 2501..=2560")
