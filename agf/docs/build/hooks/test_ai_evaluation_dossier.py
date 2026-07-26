# SPDX-License-Identifier: MIT
"""Focused regression tests for the AI evaluation dossier build hook."""

from __future__ import annotations

import importlib.util
import unittest
from pathlib import Path, PurePosixPath


HOOK_PATH = Path(__file__).with_name("ai_evaluation_dossier.py")
SPEC = importlib.util.spec_from_file_location("ai_evaluation_dossier", HOOK_PATH)
assert SPEC and SPEC.loader
HOOK = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(HOOK)


class RepositoryReferencesTest(unittest.TestCase):
    def test_references_are_pinned_to_exact_build_commit(self) -> None:
        commit = "0123456789abcdef0123456789abcdef01234567"
        config = {"extra": {"agf_release": {"commit": commit}}}

        table = HOOK._repository_reference_table(config)

        self.assertIn(f"/-/blob/{commit}/agf/agents/MANIFEST.yaml", table)
        self.assertIn(f"/-/tree/{commit}/agf/rules", table)
        self.assertIn(f"/-/blob/{commit}/agf/learnings/foundry.yaml", table)
        self.assertNotIn("/-/blob/main/", table)
        self.assertNotIn("/-/tree/main/", table)

    def test_unknown_commit_uses_explicit_repository_root_fallback(self) -> None:
        table = HOOK._repository_reference_table(
            {"extra": {"agf_release": {"commit": "unknown"}}}
        )

        self.assertIn("unresolved/unpinned", table)
        self.assertNotIn("/-/blob/unknown/", table)
        self.assertNotIn("/-/tree/unknown/", table)
        self.assertIn(HOOK.REPOSITORY_URL, table)

    def test_release_tuple_embeds_release_and_build_provenance_separately(self) -> None:
        config = {
            "extra": {
                "agf_release": {
                    "version": "2.4.3",
                    "release_tag": "v2.4.3",
                    "release_commit": "7a43ad687e45ae2a36f6c8c157f08d2127faaae7",
                    "branch": "main",
                    "commit": "ba2f29c2da760cf2d2ca8e4c23c65389a2d699e0",
                    "release_status": "development documentation after v2.4.3",
                }
            }
        }

        table = HOOK._release_tuple_table(config)

        self.assertIn("`2.4.3`", table)
        self.assertIn("`v2.4.3`", table)
        self.assertIn("`7a43ad687e45ae2a36f6c8c157f08d2127faaae7`", table)
        self.assertIn("`ba2f29c2da760cf2d2ca8e4c23c65389a2d699e0`", table)
        self.assertIn("development documentation after v2.4.3", table)

    def test_release_tuple_unknown_release_does_not_fabricate_a_commit(self) -> None:
        table = HOOK._release_tuple_table({"extra": {}})

        self.assertIn("| Latest release tag | `unknown` |", table)
        self.assertIn("| Release source commit | `unknown` |", table)
        self.assertIn("| Documentation status | unknown |", table)

    def test_copied_evaluate_link_targets_dossier_top(self) -> None:
        rewritten = HOOK._rewrite_target(
            "ai-evaluation-dossier.md",
            PurePosixPath("evaluate.md"),
            {"evaluate.md": "dossier-source-evaluate"},
        )

        self.assertEqual("#ai-evaluation-dossier", rewritten)

    def test_linked_image_rewrites_image_and_click_target(self) -> None:
        markdown = (
            "[![Framework map](../../reference/figures/framework-map.svg)]"
            "(../../reference/figures/framework-map.svg)"
        )

        rewritten = HOOK._rewrite_links(
            markdown,
            PurePosixPath("docs/start/framework-map.md"),
            {"docs/start/framework-map.md": "dossier-source-framework-map"},
        )

        expected = (
            "[![Framework map](reference/figures/framework-map.svg)]"
            "(reference/figures/framework-map.svg)"
        )
        self.assertEqual(expected, rewritten)


if __name__ == "__main__":
    unittest.main()
