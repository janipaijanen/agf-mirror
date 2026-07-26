# SPDX-License-Identifier: MIT
"""Focused regression tests for release/docs-build provenance derivation.

Covers the three cases required by
docs/plans/public-version-and-docs-provenance.md's acceptance evidence list:
exact-tag build, later-`main` build, and unavailable-tag build.
"""

from __future__ import annotations

import importlib.util
import subprocess
import tempfile
import unittest
from pathlib import Path


HOOK_PATH = Path(__file__).with_name("release_info.py")
SPEC = importlib.util.spec_from_file_location("release_info", HOOK_PATH)
assert SPEC and SPEC.loader
HOOK = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(HOOK)


def _git(root: Path, *args: str) -> str:
    return subprocess.check_output(
        ["git", *args], cwd=root, text=True, stderr=subprocess.DEVNULL
    ).strip()


class ResolveReleaseTest(unittest.TestCase):
    def setUp(self) -> None:
        self._tmp = tempfile.TemporaryDirectory()
        self.repo = Path(self._tmp.name)
        _git(self.repo, "init", "-q")
        _git(self.repo, "config", "user.email", "test@example.invalid")
        _git(self.repo, "config", "user.name", "Test")
        (self.repo / "VERSION").write_text("2.4.3\n", encoding="utf-8")
        _git(self.repo, "add", "VERSION")
        _git(self.repo, "commit", "-q", "-m", "release commit")
        self.release_commit = _git(self.repo, "rev-parse", "HEAD")
        _git(self.repo, "tag", "-a", "v2.4.3", "-m", "v2.4.3")

    def tearDown(self) -> None:
        self._tmp.cleanup()

    def test_exact_tag_build_reports_release_documentation(self) -> None:
        release = HOOK._resolve_release(self.repo, self.release_commit, "2.4.3")

        self.assertEqual(release["tag"], "v2.4.3")
        self.assertEqual(release["commit"], self.release_commit)
        self.assertTrue(release["exact"])
        self.assertEqual(release["status"], "release documentation")

    def test_later_main_build_reports_development_documentation(self) -> None:
        (self.repo / "NOTES").write_text("later work\n", encoding="utf-8")
        _git(self.repo, "add", "NOTES")
        _git(self.repo, "commit", "-q", "-m", "later commit")
        later_commit = _git(self.repo, "rev-parse", "HEAD")

        release = HOOK._resolve_release(self.repo, later_commit, "2.4.3")

        self.assertEqual(release["tag"], "v2.4.3")
        self.assertEqual(release["commit"], self.release_commit)
        self.assertFalse(release["exact"])
        self.assertEqual(release["status"], "development documentation after v2.4.3")

    def test_no_tags_reports_unknown_without_fabricating_a_release(self) -> None:
        # Overwrite with a fresh repo that never received the tag.
        untagged = Path(tempfile.mkdtemp())
        try:
            _git(untagged, "init", "-q")
            _git(untagged, "config", "user.email", "test@example.invalid")
            _git(untagged, "config", "user.name", "Test")
            (untagged / "VERSION").write_text("2.4.3\n", encoding="utf-8")
            _git(untagged, "add", "VERSION")
            _git(untagged, "commit", "-q", "-m", "no tags here")
            commit = _git(untagged, "rev-parse", "HEAD")

            release = HOOK._resolve_release(untagged, commit, "2.4.3")
        finally:
            subprocess.run(["rm", "-rf", str(untagged)], check=False)

        self.assertEqual(release["tag"], "unknown")
        self.assertEqual(release["commit"], "unknown")
        self.assertFalse(release["exact"])
        self.assertEqual(release["status"], "unknown")

    def test_tag_not_matching_version_reports_unconfirmed_identity(self) -> None:
        # agf/VERSION on this checkout ("2.9.9") does not match the highest
        # resolvable tag (v2.4.3) -- must not silently present v2.4.3 as if
        # it were the current release.
        release = HOOK._resolve_release(self.repo, self.release_commit, "2.9.9")

        self.assertEqual(release["tag"], "v2.4.3")
        self.assertFalse(release["exact"])
        self.assertIn("release identity unconfirmed", release["status"])
        self.assertIn("v2.4.3", release["status"])
        self.assertIn("v2.9.9", release["status"])

    def test_diverged_build_is_not_claimed_as_after_the_release(self) -> None:
        # A build commit that is not a descendant of the release commit (e.g.
        # a branch that diverged before the tag) must not be labeled "after".
        _git(self.repo, "checkout", "-q", "-b", "diverged", self.release_commit)
        _git(self.repo, "reset", "-q", "--hard", "HEAD~0")
        # Create a divergent commit that shares no descent from the tag by
        # amending on an orphan branch history point instead: simplest way is
        # an unrelated orphan branch.
        _git(self.repo, "checkout", "-q", "--orphan", "orphan-branch")
        _git(self.repo, "rm", "-rq", "--cached", ".")
        (self.repo / "OTHER").write_text("unrelated history\n", encoding="utf-8")
        _git(self.repo, "add", "OTHER")
        _git(self.repo, "commit", "-q", "-m", "unrelated root commit")
        diverged_commit = _git(self.repo, "rev-parse", "HEAD")

        release = HOOK._resolve_release(self.repo, diverged_commit, "2.4.3")

        self.assertEqual(release["tag"], "v2.4.3")
        self.assertFalse(release["exact"])
        self.assertNotIn("after v2.4.3", release["status"])
        self.assertIn("not confirmed to be after release v2.4.3", release["status"])


if __name__ == "__main__":
    unittest.main()
