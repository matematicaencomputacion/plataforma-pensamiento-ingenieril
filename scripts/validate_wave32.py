"""Validate Wave 32 generation, execution, integration, safety, and originality."""

from contextlib import redirect_stdout
from io import StringIO
from pathlib import Path
import os
import re
import tempfile

import gen_wave26
import gen_wave27
import gen_wave28
import gen_wave29
import gen_wave30
import gen_wave31
import gen_wave32

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
    steps = gen_wave32.build_raw(gen_wave32.RAW)
    assert [step["num"] for step in steps] == list(range(2861, 2921))
    assert len({step["slug"] for step in steps}) == 60
    families = {name: sum(step["family"] == name for step in steps) for name in {s["family"] for s in steps}}
    assert len(families) == 10 and set(families.values()) == {6}, families
    old = (
        gen_wave26.build_raw(gen_wave26.RAW)
        + gen_wave27.build_steps()
        + gen_wave28.build_steps()
        + gen_wave29.build_raw(gen_wave29.RAW)
        + gen_wave30.build_raw(gen_wave30.RAW)
        + gen_wave31.build_raw(gen_wave31.RAW)
    )
    old_signatures = {(s["slug"], s["prompt"], s["solution"]) for s in old}
    assert not old_signatures.intersection(
        (s["slug"], s["prompt"], s["solution"]) for s in steps
    ), "Wave 32 duplicates a complete prior pedagogical signature"
    aggregate_count = 0
    for step in steps:
        teaching = "\n".join((step["prompt"], step["solution"], step["pytest"]))
        assert not any(token in teaching for token in FORBIDDEN), f"unsafe token in {step['num']}"
        assert "..." not in teaching and "Objective for step" not in teaching, f"placeholder in {step['num']}"
        compile(step["solution"], f"solution-{step['num']}", "exec")
        execute_test(step)
        if any(word in step["solution"] for word in ("shard", "parcial", "total", "ventana", "partes")):
            aggregate_count += 1
    assert aggregate_count >= 28, "insufficient distributed aggregation coverage"

    source = CURRICULUM.read_text(encoding="utf-8")
    catalog = [int(value) for value in re.findall(r"micro_step:\s*(\d+)", source)]
    assert catalog in (
        list(range(1, 2921)), list(range(1, 2981)), list(range(1, 3041)),
        list(range(1, 3101)),
    ), "catalog must end at a verified cumulative ceiling from Wave 32 through Wave 35"
    for step in steps:
        constant = f'PY{step["num"]}_{step["slug"].upper().replace("-", "_")}'
        assert source.count(f"pub const {constant}:") == 1
        assert source.count(f"    &{constant},") == 1
    assert 'next: Some("py-2861-particionar-paridad"), show_type_chips: false, micro_step: 2860' in source
    if catalog[-1] == 2920:
        assert re.search(r'next: None, show_type_chips: false, micro_step: 2920,', source)
    else:
        assert 'next: Some("py-2921-offset-siguiente"), show_type_chips: false, micro_step: 2920' in source

    concepts = CONCEPTS.read_text(encoding="utf-8")
    numbers = [int(value) for value in re.findall(r"^    \((\d+), &\[", concepts, re.MULTILINE)]
    active = numbers[:numbers.index(2920) + 1]
    assert active[-60:] == list(range(2861, 2921))
    assert active == sorted(set(active))
    for path in E2E:
        assert path.read_text(encoding="utf-8").count(f"toHaveCount({catalog[-1]})") == 1
    print(f"Wave 32 cumulative contract OK: 60 solutions passed; 10x6 families; {aggregate_count} distributed/aggregation exercises; catalog ends at {catalog[-1]}")


if __name__ == "__main__":
    main()
