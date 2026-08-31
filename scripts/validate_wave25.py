"""Validate the Wave 25 range after later ordered waves extend the catalog."""

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
    assert catalog >= set(range(1, 2501)), "catalog must retain the exact Wave 25 prefix"
    boundary = re.search(
        r'id: "py-2500-score-check".*?next: Some\("py-2501-map-lambda"\),.*?micro_step: 2500,',
        source,
        flags=re.DOTALL,
    )
    assert boundary is not None, "micro-step 2500 must link to the ordered Wave 26 boundary"
    print("Wave 25 contract OK: 2441..=2500 retained; boundary links to 2501")


if __name__ == "__main__":
    main()
