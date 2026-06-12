# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Validate that README.md stat claims match the actual repository state."""

from __future__ import annotations

import re
import sys
from dataclasses import dataclass
from pathlib import Path

from agents_govern._files import find_agents, find_learnings, find_rules, find_skills, repo_root
from agents_govern._output import ok, warn

__all__ = ["validate_readme_stats", "main"]


@dataclass(frozen=True, slots=True)
class StatComparison:
    label: str
    readme_value: int | None
    actual_value: int
    tolerance: int = 0

    @property
    def matches(self) -> bool:
        if self.readme_value is None:
            return False
        return abs(self.readme_value - self.actual_value) <= self.tolerance


def _extract_int(pattern: str, text: str) -> int | None:
    """Extract an integer from text using a regex pattern."""
    m = re.search(pattern, text)
    return int(m.group(1)) if m else None


def _count_learning_entries(files: tuple[Path, ...]) -> int:
    """Count total learning entries across all files."""
    count = 0
    for path in files:
        for line in path.read_text(encoding="utf-8").splitlines():
            if re.match(r"\s*- date:", line):
                count += 1
    return count


def validate_readme_stats() -> int:
    """Validate README stats. Returns exit code."""
    root = repo_root()
    readme_path = root / "README.md"

    if not readme_path.exists():
        print("\033[0;31mERROR\033[0m  README.md not found")
        return 1

    readme_text = readme_path.read_text(encoding="utf-8")

    # Count actuals
    learnings_files = find_learnings(root)
    actual_projects = len(learnings_files)
    actual_learnings = _count_learning_entries(learnings_files)
    actual_rules = len(find_rules(root))
    actual_agents = len(find_agents(root))
    actual_skills = len(find_skills(root))

    # Extract README claims
    stats: tuple[StatComparison, ...] = (
        StatComparison(
            label="Projects",
            readme_value=_extract_int(r"(\d+) projects have adopted", readme_text),
            actual_value=actual_projects,
        ),
        StatComparison(
            label="Learnings",
            readme_value=_extract_int(r"recorded ~(\d+) learnings", readme_text),
            actual_value=actual_learnings,
            tolerance=5,
        ),
        StatComparison(
            label="Rule templates",
            readme_value=_extract_int(r"(\d+) generic rule templates", readme_text),
            actual_value=actual_rules,
        ),
        StatComparison(
            label="Agent defs",
            readme_value=_extract_int(r"examples, (\d+) agent", readme_text),
            actual_value=actual_agents,
        ),
        StatComparison(
            label="Skills",
            readme_value=_extract_int(r"(\d+) reusable skills", readme_text),
            actual_value=actual_skills,
        ),
    )

    print("README stats validation")
    print()
    print(f"  {'Stat':<22s} {'README':<9s} Actual")
    print(f"  {'----':<22s} {'------':<9s} ------")

    warnings = 0
    for stat in stats:
        readme_str = str(stat.readme_value) if stat.readme_value is not None else "?"
        if stat.matches:
            ok(f"{stat.label:<22s} {readme_str:<9s} {stat.actual_value}")
        elif stat.readme_value is None:
            warn(f"{stat.label:<22s} {readme_str:<9s} {stat.actual_value} (not found in README)")
            warnings += 1
        else:
            diff = abs(stat.readme_value - stat.actual_value)
            warn(f"{stat.label:<22s} {readme_str:<9s} {stat.actual_value} (off by {diff})")
            warnings += 1

    print()
    if warnings > 0:
        print(f"\033[0;33m{warnings} stat(s) may be stale.\033[0m Consider updating README.md.")
        return 1

    print("\033[0;32mAll README stats match.\033[0m")
    return 0


_HELP = """\
validate-readme-stats — Verify README.md statistics match the repository

Usage:
  uv run agov-validate-readme-stats

Checks that quantitative claims in README.md (project count, learning
count, rule template count, agent definition count, skills count) match
the actual file counts in the repository.

No arguments — always validates against the current repo state.

Exit codes: 0 = all stats match, 1 = mismatch found.

Full documentation: agents/fact-checker.yaml.
"""


def main() -> None:
    if len(sys.argv) > 1 and sys.argv[1] in ("--help", "-h"):
        print(_HELP)
        return
    sys.exit(validate_readme_stats())


if __name__ == "__main__":
    main()
