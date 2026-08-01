# SPDX-License-Identifier: MIT
"""Tests for public figure accessibility and AI disclosure checks."""

from __future__ import annotations

import importlib.util
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch


HOOK_PATH = Path(__file__).with_name("figure_accessibility.py")
SPEC = importlib.util.spec_from_file_location("figure_accessibility", HOOK_PATH)
assert SPEC and SPEC.loader
HOOK = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(HOOK)


class FigureAccessibilityTest(unittest.TestCase):
    def _validate(self, content: str) -> list[str]:
        with tempfile.TemporaryDirectory() as temp:
            path = Path(temp) / "page.md"
            path.write_text(content, encoding="utf-8")
            with patch.object(HOOK, "REPO_ROOT", Path(temp)):
                return HOOK.validate_figure_accessibility([path])

    def test_descriptive_alt_and_visible_disclosure_pass(self) -> None:
        failures = self._validate(
            "![Plan passing through two human gates to merge](figure.svg)\n\n"
            "*AI-generated figure. Publication remains the responsibility of "
            "the human maintainer.*\n"
        )
        self.assertEqual(failures, [])

    def test_generic_alt_fails(self) -> None:
        failures = self._validate(
            "![Gate pipeline](figure.svg)\n\n"
            "*AI-generated figure. Publication remains the responsibility of "
            "the human maintainer.*\n"
        )
        self.assertTrue(any("alt text" in failure for failure in failures))

    def test_missing_disclosure_fails(self) -> None:
        failures = self._validate(
            "![Plan passing through two human gates to merge](figure.svg)\n"
        )
        self.assertTrue(any("disclosure" in failure for failure in failures))

    def test_code_examples_are_not_treated_as_embeds(self) -> None:
        failures = self._validate(
            "Use `![Alt text](figures/name.svg)` to embed a figure.\n\n"
            "```markdown\n![Example](figure.svg)\n```\n"
        )
        self.assertEqual(failures, [])


if __name__ == "__main__":
    unittest.main()

