"""Validate Wave 29 generation, execution, integration, safety, and originality."""

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
)
LAZY_MARKERS = ("yield", "next(", "islice", "takewhile", "dropwhile", "any(", "all(", "filter(", "map(")


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
    steps = gen_wave29.build_raw(gen_wave29.RAW)
    assert [step["num"] for step in steps] == list(range(2681, 2741))
    assert len({step["slug"] for step in steps}) == 60
    old = (
        gen_wave26.build_raw(gen_wave26.RAW)
        + gen_wave27.build_steps()
        + gen_wave28.build_steps()
    )
    old_signatures = {(s["slug"], s["prompt"], s["solution"]) for s in old}
    assert not old_signatures.intersection(
        (s["slug"], s["prompt"], s["solution"]) for s in steps
    ), "Wave 29 duplicates a complete prior pedagogical signature"
    lazy_count = 0
    for step in steps:
        teaching = "\n".join((step["prompt"], step["solution"]))
        assert not any(token in teaching for token in FORBIDDEN), f"unsafe token in {step['num']}"
        assert "..." not in teaching and "Objective for step" not in teaching, f"placeholder in {step['num']}"
        compile(step["solution"], f"solution-{step['num']}", "exec")
        execute_test(step)
        if any(marker in step["solution"] for marker in LAZY_MARKERS):
            lazy_count += 1
    assert lazy_count >= 24, "insufficient observable lazy/short-circuit coverage"

    source = CURRICULUM.read_text(encoding="utf-8")
    catalog = [int(value) for value in re.findall(r"micro_step:\s*(\d+)", source)]
    assert catalog in (
        list(range(1, 2741)), list(range(1, 2801)), list(range(1, 2861)),
        list(range(1, 2921)), list(range(1, 2981)), list(range(1, 3041)),
    ), "catalog must end at a verified cumulative ceiling from Wave 29 through Wave 34"
    for step in steps:
        constant = f'PY{step["num"]}_{step["slug"].upper().replace("-", "_")}'
        assert source.count(f"pub const {constant}:") == 1
        assert source.count(f"    &{constant},") == 1
    assert 'next: Some("py-2681-hof-normalizar"), show_type_chips: false, micro_step: 2680' in source
    if catalog[-1] == 2740:
        assert re.search(r'next: None, show_type_chips: false, micro_step: 2740,', source)
        expected_e2e = 2740
    else:
        assert 'next: Some("py-2741-merge-dos-listas"), show_type_chips: false, micro_step: 2740' in source
        expected_e2e = catalog[-1]

    concepts = CONCEPTS.read_text(encoding="utf-8")
    numbers = [int(value) for value in re.findall(r"^    \((\d+), &\[", concepts, re.MULTILINE)]
    active = numbers[:numbers.index(2740) + 1]
    assert active[-60:] == list(range(2681, 2741))
    assert active == sorted(set(active))
    for path in E2E:
        assert path.read_text(encoding="utf-8").count(f"toHaveCount({expected_e2e})") == 1
    print(f"Wave 29 contract OK: 60 solutions passed; {lazy_count} lazy/short-circuit exercises; cumulative catalog ends at {catalog[-1]}")


if __name__ == "__main__":
    main()
