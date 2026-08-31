"""Validate generated exercises and their Wave 26 catalog integration."""

from pathlib import Path
from contextlib import redirect_stdout
from io import StringIO
import re
import tempfile

import gen_wave26

ROOT = Path(__file__).resolve().parents[1]
CURRICULUM = ROOT / "web/src/curriculum.rs"
CONCEPTS = ROOT / "web/src/concepts/mod.rs"
E2E = (
    ROOT / "web/e2e/tests/journey.auth-hub.spec.ts",
    ROOT / "web/e2e/tests/progress.check.spec.ts",
    ROOT / "web/e2e/tests/session.navigation.spec.ts",
)


def main() -> None:
    steps = gen_wave26.build_raw(gen_wave26.RAW)
    numbers = [step["num"] for step in steps]
    assert numbers == list(range(2501, 2561)), "generator must emit exact ordered range"
    assert len({step["slug"] for step in steps}) == 60, "Wave 26 slugs must be unique"
    for step in steps:
        assert "input(" not in step["solution"], f"step {step['num']} uses input()"
        compile(step["solution"], f"solution-{step['num']}", "exec")
        compile(step["pytest"], f"pytest-{step['num']}", "exec")
        with tempfile.TemporaryDirectory() as directory:
            workdir = Path(directory)
            (workdir / "solution.py").write_text(step["solution"], encoding="utf-8")
            namespace: dict[str, object] = {}
            exec(compile(step["pytest"], f"pytest-{step['num']}", "exec"), namespace)
            test = next(value for key, value in namespace.items() if key.startswith("test_"))
            output = StringIO()

            class Capture:
                def readouterr(self):
                    return type("CaptureResult", (), {"out": output.getvalue()})()

            previous = Path.cwd()
            try:
                import os
                os.chdir(workdir)
                with redirect_stdout(output):
                    test(Capture())
            finally:
                os.chdir(previous)

    source = CURRICULUM.read_text(encoding="utf-8")
    catalog = [int(value) for value in re.findall(r"micro_step:\s*(\d+)", source)]
    assert catalog[:2560] == list(range(1, 2561)), "Wave 26 prefix must remain exact 1..=2560"
    for step in steps:
        constant = f'PY{step["num"]}_{step["slug"].upper().replace("-", "_")}'
        assert source.count(f"pub const {constant}:") == 1, f"bad definition count: {constant}"
        assert source.count(f"    &{constant},") == 1, f"bad catalog reference: {constant}"
    assert 'next: Some("py-2501-map-lambda"), show_type_chips: false, micro_step: 2500' in source
    assert 'next: Some("py-2561-base-cero"), show_type_chips: false, micro_step: 2560' in source

    concepts = CONCEPTS.read_text(encoding="utf-8")
    partition_numbers = [int(value) for value in re.findall(r"^    \((\d+), &\[", concepts, re.MULTILINE)]
    first_2560 = partition_numbers.index(2560)
    active = partition_numbers[: first_2560 + 1]
    assert active[-60:] == list(range(2501, 2561)), "Wave 26 partitions must cover exact range"
    assert active == sorted(set(active)), "active partitions must be unique and sorted"

    for path in E2E:
        assert "toHaveCount(2620)" in path.read_text(encoding="utf-8"), f"Wave 27 ceiling missing: {path}"
    print("Wave 26 cumulative contract OK: prefix and partitions 2501..=2560 preserved")


if __name__ == "__main__":
    main()
