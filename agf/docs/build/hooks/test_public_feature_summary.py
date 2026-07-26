# SPDX-License-Identifier: MIT
"""Regression tests for the governed public feature-summary placement."""

from __future__ import annotations

import unittest
from pathlib import Path


START = "<!-- public-feature-summary:start -->"
END = "<!-- public-feature-summary:end -->"
OPENING = "**Human plan and merge decisions.**"
REPOSITORY_ROOT = Path(__file__).resolve().parents[4]
README = REPOSITORY_ROOT / "README.md"
DOCS_LANDING = REPOSITORY_ROOT / "agf/docs/site/docs/index.md"
PUBLIC_MARKDOWN_ROOT = REPOSITORY_ROOT / "agf/docs/site"


def _block(path: Path) -> str:
    text = path.read_text(encoding="utf-8")
    before, separator, remainder = text.partition(START)
    if not separator:
        raise AssertionError(f"missing start marker in {path}")
    block, separator, after = remainder.partition(END)
    if not separator:
        raise AssertionError(f"missing end marker in {path}")
    if START in before or START in after or END in before or END in after:
        raise AssertionError(f"feature-summary markers are not unique in {path}")
    return block


class PublicFeatureSummaryTest(unittest.TestCase):
    def test_two_public_placements_are_byte_identical(self) -> None:
        self.assertEqual(_block(README), _block(DOCS_LANDING))

    def test_full_block_opener_occurs_on_exactly_two_public_pages(self) -> None:
        public_pages = [README, *PUBLIC_MARKDOWN_ROOT.rglob("*.md")]
        occurrences = [
            path.relative_to(REPOSITORY_ROOT)
            for path in public_pages
            if OPENING in path.read_text(encoding="utf-8")
        ]
        self.assertEqual(
            occurrences,
            [Path("README.md"), Path("agf/docs/site/docs/index.md")],
        )


if __name__ == "__main__":
    unittest.main()
