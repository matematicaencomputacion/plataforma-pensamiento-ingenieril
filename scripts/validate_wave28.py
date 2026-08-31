"""Validate Wave 28 generation, execution, integration, safety, and originality."""

from contextlib import redirect_stdout
from io import StringIO
from pathlib import Path
import os
import re
import tempfile

import gen_wave26
import gen_wave27
import gen_wave28

ROOT = Path(__file__).resolve().parents[1]
CURRICULUM = ROOT / "web/src/curriculum.rs"
CONCEPTS = ROOT / "web/src/concepts/mod.rs"
E2E = (
    ROOT / "web/e2e/tests/journey.auth-hub.spec.ts",
    ROOT / "web/e2e/tests/progress.check.spec.ts",
    ROOT / "web/e2e/tests/session.navigation.spec.ts",
)
FORBIDDEN = ("input(", "threading", "multiprocessing", "concurrent", "subprocess", "socket", "requests", "urllib", "asyncio")


def execute_test(step):
    with tempfile.TemporaryDirectory() as directory:
        workdir = Path(directory)
        (workdir / "solution.py").write_text(step["solution"], encoding="utf-8")
        namespace = {}
        exec(compile(step["pytest"], f"pytest-{step['num']}", "exec"), namespace)
        test = next(value for key, value in namespace.items() if key.startswith("test_"))
        output = StringIO()

        class Capture:
            def readouterr(self):
                return type("CaptureResult", (), {"out": output.getvalue()})()

        previous = Path.cwd()
        try:
            os.chdir(workdir)
            with redirect_stdout(output):
                test(Capture())
        finally:
            os.chdir(previous)


def main():
    steps = gen_wave28.build_steps()
    assert [step["num"] for step in steps] == list(range(2621, 2681))
    assert len({step["slug"] for step in steps}) == 60
    old = gen_wave26.build_raw(gen_wave26.RAW) + gen_wave27.build_steps()
    old_signatures = {(s["slug"], s["prompt"], s["solution"]) for s in old}
    assert not old_signatures.intersection(
        (s["slug"], s["prompt"], s["solution"]) for s in steps
    ), "Wave 28 duplicates a complete prior pedagogical signature"
    equivalence_count = 0
    for step in steps:
        text = "\n".join((step["prompt"], step["solution"], step["pytest"]))
        assert not any(token in text for token in FORBIDDEN), f"unsafe token in {step['num']}"
        assert "..." not in text and "Objective for step" not in text, f"placeholder in {step['num']}"
        compile(step["solution"], f"solution-{step['num']}", "exec")
        execute_test(step)
        if step["equivalence"]:
            equivalence_count += 1
            assert "directo" in step["solution"] and "directo" in step["pytest"]
    assert equivalence_count >= 10, "insufficient direct-vs-partial equivalence coverage"

    source = CURRICULUM.read_text(encoding="utf-8")
    catalog = [int(value) for value in re.findall(r"micro_step:\s*(\d+)", source)]
    assert catalog == list(range(1, 2681)), "catalog must be exact and ordered 1..=2680"
    for step in steps:
        constant = f'PY{step["num"]}_{step["slug"].upper().replace("-", "_")}'
        assert source.count(f"pub const {constant}:") == 1
        assert source.count(f"    &{constant},") == 1
    assert 'next: Some("py-2621-chunk-tamano"), show_type_chips: false, micro_step: 2620' in source
    assert re.search(r'next: None, show_type_chips: false, micro_step: 2680,', source)

    concepts = CONCEPTS.read_text(encoding="utf-8")
    numbers = [int(value) for value in re.findall(r"^    \((\d+), &\[", concepts, re.MULTILINE)]
    active = numbers[:numbers.index(2680) + 1]
    assert active[-60:] == list(range(2621, 2681))
    assert active == sorted(set(active))
    for path in E2E:
        assert path.read_text(encoding="utf-8").count("toHaveCount(2680)") == 1
    print(f"Wave 28 contract OK: 60 solutions passed; {equivalence_count} equivalence tests; catalog exact 1..=2680")


if __name__ == "__main__":
    main()
