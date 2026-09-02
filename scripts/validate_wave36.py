"""Validate Wave 36 generation, execution, integration, safety, and originality."""

from contextlib import redirect_stdout
from io import StringIO
from pathlib import Path
import os
import re
import tempfile

import gen_wave26, gen_wave27, gen_wave28, gen_wave29, gen_wave30
import gen_wave31, gen_wave32, gen_wave33, gen_wave34, gen_wave35, gen_wave36

ROOT = Path(__file__).resolve().parents[1]
CURRICULUM = ROOT / "web/src/curriculum.rs"
CONCEPTS = ROOT / "web/src/concepts/mod.rs"
E2E = (
    ROOT / "web/e2e/tests/journey.auth-hub.spec.ts",
    ROOT / "web/e2e/tests/progress.check.spec.ts",
    ROOT / "web/e2e/tests/session.navigation.spec.ts",
)
FORBIDDEN = (
    "input(", "threading", "multiprocessing", "concurrent", "subprocess",
    "socket", "requests", "urllib", "asyncio", "http://", "https://",
    "time.", "datetime", "random", "sleep(", "hash(",
)


def execute_test(step):
    with tempfile.TemporaryDirectory() as directory:
        workdir = Path(directory)
        (workdir / "solution.py").write_text(step["solution"], encoding="utf-8")
        namespace = {}
        exec(compile(step["pytest"], f"pytest-{step['num']}", "exec"), namespace)
        tests = [value for key, value in namespace.items() if key.startswith("test_")]
        assert len(tests) == 1, f"expected one real test for {step['num']}"
        output = StringIO()

        class Capture:
            def readouterr(self):
                return type("CaptureResult", (), {"out": output.getvalue()})()

        previous = Path.cwd()
        try:
            os.chdir(workdir)
            with redirect_stdout(output):
                tests[0](Capture())
        finally:
            os.chdir(previous)


def main():
    steps = gen_wave36.build_raw()
    assert [step["num"] for step in steps] == list(range(3101, 3161))
    assert len({step["slug"] for step in steps}) == 60
    families = {name: sum(step["family"] == name for step in steps) for name in {s["family"] for s in steps}}
    assert len(families) == 10 and set(families.values()) == {6}, families
    old = (
        gen_wave26.build_raw(gen_wave26.RAW) + gen_wave27.build_steps()
        + gen_wave28.build_steps() + gen_wave29.build_raw(gen_wave29.RAW)
        + gen_wave30.build_raw(gen_wave30.RAW) + gen_wave31.build_raw(gen_wave31.RAW)
        + gen_wave32.build_raw(gen_wave32.RAW) + gen_wave33.build_raw(gen_wave33.RAW)
        + gen_wave34.build_raw(gen_wave34.RAW) + gen_wave35.build_raw(gen_wave35.RAW)
    )
    old_signatures = {(s["slug"], s["prompt"], s["solution"]) for s in old}
    assert not old_signatures.intersection(
        (s["slug"], s["prompt"], s["solution"]) for s in steps
    ), "Wave 36 duplicates a complete prior pedagogical signature"
    delivery_count = 0
    markers = (
        "version", "release", "revision", "artefactos", "compatible", "soporte",
        "cohorte", "porcentaje", "canary", "migracion", "nuevo", "gates",
        "requeridos", "restaurar", "previo", "ambientes", "promover",
    )
    for step in steps:
        teaching = "\n".join((step["prompt"], step["solution"], step["pytest"]))
        assert not any(token in teaching for token in FORBIDDEN), f"unsafe token in {step['num']}"
        assert "..." not in teaching and "Objective for step" not in teaching, f"placeholder in {step['num']}"
        compile(step["solution"], f"solution-{step['num']}", "exec")
        execute_test(step)
        if any(word in step["solution"] for word in markers):
            delivery_count += 1
    assert delivery_count >= 30, "insufficient safe-delivery coverage"

    source = CURRICULUM.read_text(encoding="utf-8")
    catalog = [int(value) for value in re.findall(r"micro_step:\s*(\d+)", source)]
    assert catalog == list(range(1, 3161)), "catalog must be exact and ordered 1..=3160"
    for step in steps:
        constant = f'PY{step["num"]}_{step["slug"].upper().replace("-", "_")}'
        assert source.count(f"pub const {constant}:") == 1
        assert source.count(f"    &{constant},") == 1
    assert 'next: Some("py-3101-release-manifiesto"), show_type_chips: false, micro_step: 3100' in source
    assert re.search(r'next: None, show_type_chips: false, micro_step: 3160,', source)

    concepts = CONCEPTS.read_text(encoding="utf-8")
    numbers = [int(value) for value in re.findall(r"^    \((\d+), &\[", concepts, re.MULTILINE)]
    active = numbers[:numbers.index(3160) + 1]
    assert active[-60:] == list(range(3101, 3161))
    assert active == sorted(set(active))
    for path in E2E:
        assert path.read_text(encoding="utf-8").count("toHaveCount(3160)") == 1
    print(f"Wave 36 contract OK: 60 solutions passed; 10x6 families; {delivery_count} safe-delivery exercises; catalog exact 1..=3160")


if __name__ == "__main__":
    main()
