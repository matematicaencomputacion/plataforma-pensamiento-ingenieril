"""Validate Wave 27 generation, execution, integration, and originality."""

from contextlib import redirect_stdout
from io import StringIO
from pathlib import Path
import os
import re
import tempfile

import gen_wave26
import gen_wave27

ROOT = Path(__file__).resolve().parents[1]
CURRICULUM = ROOT / "web/src/curriculum.rs"
CONCEPTS = ROOT / "web/src/concepts/mod.rs"
E2E = (
    ROOT / "web/e2e/tests/journey.auth-hub.spec.ts",
    ROOT / "web/e2e/tests/progress.check.spec.ts",
    ROOT / "web/e2e/tests/session.navigation.spec.ts",
)


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
    steps = gen_wave27.build_steps()
    assert [step["num"] for step in steps] == list(range(2561, 2621))
    assert len({step["slug"] for step in steps}) == 60
    old = gen_wave26.build_raw(gen_wave26.RAW)
    old_signatures = {(s["slug"], s["prompt"], s["solution"]) for s in old}
    assert not old_signatures.intersection(
        (s["slug"], s["prompt"], s["solution"]) for s in steps
    ), "Wave 27 duplicates a complete Wave 26 pedagogical signature"
    for step in steps:
        assert "input(" not in step["solution"]
        assert not any(token in step["solution"] for token in ("requests", "threading", "socket"))
        compile(step["solution"], f"solution-{step['num']}", "exec")
        execute_test(step)

    source = CURRICULUM.read_text(encoding="utf-8")
    catalog = [int(value) for value in re.findall(r"micro_step:\s*(\d+)", source)]
    assert catalog == list(range(1, 2621)), "catalog must be exact and ordered 1..=2620"
    for step in steps:
        constant = f'PY{step["num"]}_{step["slug"].upper().replace("-", "_")}'
        assert source.count(f"pub const {constant}:") == 1
        assert source.count(f"    &{constant},") == 1
    assert 'next: Some("py-2561-base-cero"), show_type_chips: false, micro_step: 2560' in source
    assert re.search(r'next: None, show_type_chips: false, micro_step: 2620,', source)

    concepts = CONCEPTS.read_text(encoding="utf-8")
    numbers = [int(value) for value in re.findall(r"^    \((\d+), &\[", concepts, re.MULTILINE)]
    first_terminal = numbers.index(2620)
    active = numbers[:first_terminal + 1]
    assert active[-60:] == list(range(2561, 2621))
    assert active == sorted(set(active))
    for path in E2E:
        assert path.read_text(encoding="utf-8").count("toHaveCount(2620)") == 1
    print("Wave 27 contract OK: 60 solutions passed; catalog exact 1..=2620; content original")


if __name__ == "__main__":
    main()
