"""Validate the active Wave 25 contract and the catalog ceiling at 2500."""

from pathlib import Path
import re


ROOT = Path(__file__).resolve().parents[1]
CURRICULUM = ROOT / "web" / "src" / "curriculum.rs"


def main() -> None:
    source = CURRICULUM.read_text(encoding="utf-8")
    steps = [int(value) for value in re.findall(r"micro_step:\s*(\d+)", source)]
    catalog = set(steps)

    expected = set(range(2441, 2501))
    assert expected <= catalog, f"missing Wave 25 steps: {sorted(expected - catalog)}"
    assert len(steps) == 2500, f"expected 2500 steps, got {len(steps)}"
    assert catalog == set(range(1, 2501)), "catalog must cover exactly 1..=2500"
    terminal = re.search(
        r'id: "py-2500-score-check".*?next: None,.*?micro_step: 2500,',
        source,
        flags=re.DOTALL,
    )
    assert terminal is not None, "micro-step 2500 must terminate the rail"
    print("Wave 25 contract OK: 2441..=2500; catalog ceiling 2500")


if __name__ == "__main__":
    main()
