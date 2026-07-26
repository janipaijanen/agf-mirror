# SPDX-License-Identifier: MIT
"""Reject current-looking versioned lifecycle/maturity labels in source docs.

docs/plans/public-version-and-docs-provenance.md scope item 3: "source
scanning rejects current-looking versioned lifecycle or maturity
labels -- including `Released (vX documentation)` and `Deployable (vX)` --
except allow-listed prose that explicitly labels the version as historical."

A label like `Deployable (v2.0)` or `Released (v2.1 documentation)` reads as
a current-release claim that goes stale the moment the framework version
moves on, duplicating `agf/VERSION` by hand in prose. The five-level
maturity words themselves (Deployable, Validated, Instantiated,
Surface-defined, Planned -- see
`.claude/rules/docs/implementation-honesty-rule.md`) are fine on their own;
only pairing one with a parenthetical version number is flagged, and only
when the *same parenthetical* explicitly marks the version historical (e.g.
"Deployable (v2.1, historical)"). Earlier revisions tried to recognize a
historical marker anywhere in the same clause or within some proximity of
the version number; three rounds of fact-check (8ka5, de5k, rjbm) each
found a same-line phrasing where that let an unrelated -- or contradicted --
"historical" exempt a current-looking label. Scoping the marker to the
matched parenthetical itself removes that entire bug class: the marker is
syntactically part of the label it qualifies, not merely nearby text.

Run directly: `python3 agf/docs/build/hooks/check_stale_version_labels.py`
Self-test: `python3 agf/docs/build/hooks/check_stale_version_labels.py --self-test`
"""

from __future__ import annotations

import re
import sys
import unittest
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[4]
# MkDocs docs_dir (agf/docs/build/mkdocs.yml: docs_dir: ../site) -- covers
# both top-level pages (index.md, evaluate.md, ...) and agf/docs/site/docs/.
DOCS_SOURCE_DIR = REPO_ROOT / "agf/docs/site"
# Generated at build time (release_info.py); not an authored source, and
# already scoped to never contain a stale versioned label.
GENERATED_EXCLUDES = {REPO_ROOT / "agf/docs/site/reference/release-info.md"}

MATURITY_WORDS = (
    "Deployable",
    "Validated",
    "Instantiated",
    "Surface-defined",
    "Planned",
    "Released",
)

STALE_LABEL_RE = re.compile(
    r"\b(?:" + "|".join(re.escape(word) for word in MATURITY_WORDS) + r")"
    r"\s*\(\s*v\d[\d.]*[^)]*\)",
    re.IGNORECASE,
)


class StaleVersionLabelError(ValueError):
    pass


def find_stale_labels(text: str) -> list[tuple[int, str]]:
    """Return (1-based line number, matched text) for each flagged label."""
    findings: list[tuple[int, str]] = []
    for lineno, line in enumerate(text.splitlines(), start=1):
        for match in STALE_LABEL_RE.finditer(line):
            if "historical" in match.group(0).lower():
                continue  # e.g. "Deployable (v2.0, historical)"
            findings.append((lineno, match.group(0)))
    return findings


def scan(docs_dir: Path) -> dict[Path, list[tuple[int, str]]]:
    results: dict[Path, list[tuple[int, str]]] = {}
    for path in sorted(docs_dir.rglob("*.md")):
        if path in GENERATED_EXCLUDES:
            continue
        findings = find_stale_labels(path.read_text(encoding="utf-8"))
        if findings:
            results[path] = findings
    return results


def main() -> int:
    results = scan(DOCS_SOURCE_DIR)
    if not results:
        print(f"OK: no stale versioned lifecycle labels under {DOCS_SOURCE_DIR}")
        return 0

    for path, findings in results.items():
        for lineno, matched in findings:
            print(f"{path}:{lineno}: stale versioned label {matched!r}", file=sys.stderr)
    print(
        "Use a non-versioned lifecycle status (e.g. 'Released documentation') "
        "or mark the version historical inside the same parenthetical "
        "(e.g. 'Deployable (v2.1, historical)').",
        file=sys.stderr,
    )
    return 1


class SelfTest(unittest.TestCase):
    def test_flags_current_looking_deployable_version(self) -> None:
        findings = find_stale_labels("*Status: Deployable (v2.0). Audience: ...*")
        self.assertEqual(len(findings), 1)

    def test_flags_released_documentation_convention(self) -> None:
        findings = find_stale_labels('Released (v2.x documentation) labels mark ...')
        self.assertEqual(len(findings), 1)

    def test_allows_explicit_historical_statement(self) -> None:
        findings = find_stale_labels("This feature was introduced in v2.1.")
        self.assertEqual(findings, [])

    def test_allows_maturity_word_without_version(self) -> None:
        findings = find_stale_labels("*Status: Deployable. Audience: ...*")
        self.assertEqual(findings, [])

    def test_allows_labeled_historical_marker(self) -> None:
        findings = find_stale_labels("Deployable (v2.0, historical) — superseded by v2.4.")
        self.assertEqual(findings, [])

    def test_rejects_unrelated_historical_marker_in_a_different_clause(self) -> None:
        # The "introduced in v1" marker qualifies v1, not the v2.0 label in
        # the second clause -- it must not exempt an unrelated stale label.
        findings = find_stale_labels("Introduced in v1; Deployable (v2.0)")
        self.assertEqual(len(findings), 1)

    def test_rejects_historical_word_qualifying_a_different_version_period(self) -> None:
        findings = find_stale_labels("Historical v1. Deployable (v2.0)")
        self.assertEqual(len(findings), 1)

    def test_rejects_as_of_qualifying_a_different_version_comma(self) -> None:
        findings = find_stale_labels("As of v1, Deployable (v2.0)")
        self.assertEqual(len(findings), 1)

    def test_rejects_historical_word_qualifying_a_different_version_dash(self) -> None:
        findings = find_stale_labels("Historical v1 — Deployable (v2.0)")
        self.assertEqual(len(findings), 1)

    def test_rejects_as_of_prefix_naming_the_same_version(self) -> None:
        # "as of" commonly introduces a current-state claim, not a
        # historical one -- must not exempt the match.
        findings = find_stale_labels("As of v2.0, Deployable (v2.0)")
        self.assertEqual(len(findings), 1)

    def test_rejects_historical_prose_before_a_contradicting_current_label(self) -> None:
        findings = find_stale_labels(
            "v2.0 feature is historical but current status: Deployable (v2.0)"
        )
        self.assertEqual(len(findings), 1)

    def test_rejects_historical_prose_after_the_label_same_version(self) -> None:
        findings = find_stale_labels(
            "v2.0 is a historical release discussed elsewhere. Deployable (v2.0)"
        )
        self.assertEqual(len(findings), 1)

    def test_rejects_historical_prose_following_the_label_same_version(self) -> None:
        findings = find_stale_labels(
            "Deployable (v2.0) — v2.0 historical migration notes follow."
        )
        self.assertEqual(len(findings), 1)

    def test_rejects_historical_prose_after_semicolon_same_version(self) -> None:
        findings = find_stale_labels("Deployable (v2.0); unrelated v2.0 is historical.")
        self.assertEqual(len(findings), 1)


if __name__ == "__main__":
    if "--self-test" in sys.argv:
        sys.argv.remove("--self-test")
        unittest.main()
    else:
        raise SystemExit(main())
