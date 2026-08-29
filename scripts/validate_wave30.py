"""Valida los 60 micro-steps (2741-2800) de la Ola 30 ejecutando sus pytest en Python.

Reusa la definición de pasos de `gen_wave30.py` (sin input, stdlib-only, determinista).
"""

import tempfile
import os
import sys

sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import gen_wave30  # noqa: E402


def run_step(step_d):
    d = tempfile.mkdtemp()
    solpath = os.path.join(d, "solution.py")
    with open(solpath, "w", encoding="utf-8") as fh:
        fh.write(step_d["solution"])
    # constraint: solution must not call input()
    assert "input(" not in step_d["solution"], f"step {step_d['num']} uses input()"
    pytest_code = step_d["pytest"]
    try:
        exec(compile(pytest_code, "<pytest>", "exec"), {})
    except AssertionError as exc:
        return False, "pytest assert failed: %r" % exc
    except Exception as exc:  # noqa: BLE001
        return False, "pytest raised: %r" % exc
    return True, "ok"


def main():
    steps = gen_wave30.build_raw(gen_wave30.RAW)
    assert len(steps) == 60, f"expected 60 steps, got {len(steps)}"
    failed = []
    for s in steps:
        ok, msg = run_step(s)
        status = "OK" if ok else "FAIL"
        print(f"[{status}] py-{s['num']}-{s['slug']}: {msg}")
        if not ok:
            failed.append((s["num"], s["slug"], msg))
    print("-----")
    if failed:
        print(f"{len(failed)} FAILURES:")
        for f in failed:
            print("  ", f)
        sys.exit(1)
    print("All 60 wave-30 steps pass.")


if __name__ == "__main__":
    main()