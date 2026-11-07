# SPDX-License-Identifier: MIT
"""Tests for the build-time changelog page hook.

Positive: the generated page carries the source changelog verbatim.
Negative: the build aborts when the source changelog is missing.
"""

from __future__ import annotations

import importlib.util
import tempfile
import unittest
from pathlib import Path
from types import SimpleNamespace

HOOK_PATH = Path(__file__).with_name("changelog_page.py")
SPEC = importlib.util.spec_from_file_location("changelog_page", HOOK_PATH)
assert SPEC and SPEC.loader
HOOK = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(HOOK)


class ChangelogPageTest(unittest.TestCase):
    def setUp(self) -> None:
        self._tmp = tempfile.TemporaryDirectory()
        root = Path(self._tmp.name)
        (root / "docs" / "build").mkdir(parents=True)
        (root / "docs" / "site").mkdir(parents=True)
        self.changelog = root / "CHANGELOG.md"
        self.config = SimpleNamespace(
            config_file_path=str(root / "docs" / "build" / "mkdocs.yml"),
            docs_dir=str(root / "docs" / "site"),
        )
        self.page = root / "docs" / "site" / "reference" / "changelog.md"

    def tearDown(self) -> None:
        self._tmp.cleanup()

    def test_generated_page_contains_source_changelog_verbatim(self) -> None:
        body = "# Changelog\n\n## [9.9.9] - 2026-01-01\n\n- a change\n"
        self.changelog.write_text(body, encoding="utf-8")

        HOOK.on_pre_build(self.config)

        text = self.page.read_text(encoding="utf-8")
        self.assertTrue(text.startswith("<!-- Generated at build time"))
        self.assertTrue(text.endswith(body))

    def test_regeneration_replaces_a_stale_page(self) -> None:
        self.changelog.write_text("# Changelog\n\nnew\n", encoding="utf-8")
        self.page.parent.mkdir(parents=True)
        self.page.write_text("stale", encoding="utf-8")

        HOOK.on_pre_build(self.config)

        self.assertNotIn("stale", self.page.read_text(encoding="utf-8"))

    def test_repo_relative_site_links_are_rewritten_to_page_relative_links(self) -> None:
        # agf/CHANGELOG.md links into the site as docs/site/<path>, relative to agf/.
        self.changelog.write_text(
            "See [Review](docs/site/docs/operate/review-and-release.md#anchor) "
            "and [ext](https://example.invalid/x).\n",
            encoding="utf-8",
        )

        HOOK.on_pre_build(self.config)

        text = self.page.read_text(encoding="utf-8")
        self.assertIn("](../docs/operate/review-and-release.md#anchor)", text)
        self.assertNotIn("docs/site/", text)
        self.assertIn("](https://example.invalid/x)", text)

    def test_missing_source_aborts_the_build_and_writes_no_page(self) -> None:
        with self.assertRaises(FileNotFoundError) as ctx:
            HOOK.on_pre_build(self.config)

        self.assertIn("source file not found", str(ctx.exception))
        self.assertFalse(self.page.exists())


if __name__ == "__main__":
    unittest.main()
