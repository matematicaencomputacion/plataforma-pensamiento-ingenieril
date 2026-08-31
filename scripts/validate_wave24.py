"""Validate the active Wave 24 contract (micro-steps 2381-2440)."""

from pathlib import Path
import re


ROOT = Path(__file__).resolve().parents[1]
CURRICULUM = ROOT / "web" / "src" / "curriculum.rs"
CONCEPTS = ROOT / "web" / "src" / "concepts" / "mod.rs"


def micro_steps(source: str) -> set[int]:
    return {int(value) for value in re.findall(r"micro_step:\s*(\d+)", source)}


def main() -> None:
    curriculum = CURRICULUM.read_text(encoding="utf-8")
    concepts = CONCEPTS.read_text(encoding="utf-8")
    steps = micro_steps(curriculum)

    expected = set(range(2381, 2441))
    assert expected <= steps, f"missing Wave 24 steps: {sorted(expected - steps)}"
    assert 'micro_step: 2380' in curriculum
    assert 'next: Some("py-2381-map-lambda")' in curriculum
    assert 'micro_step: 2440' in curriculum
    assert 'next: Some("py-2441-map-lambda")' in curriculum
    assert '(2381, &[3])' in concepts
    assert '(2440, &[5, 3])' in concepts
    print("Wave 24 contract OK: 2381..=2440")


if __name__ == "__main__":
    main()
