# SPDX-License-Identifier: MIT
"""Foundry-only cross-surface checks for the public operations runbook.

This test intentionally reads repository-only assistant and coordination
fixtures. It lives outside ``agf/`` so release archives do not ship a test
whose inputs are absent from the release manifest.
"""

from pathlib import Path
import unittest


ROOT = Path(__file__).parents[2]
OPERATE = ROOT / "agf" / "docs" / "site" / "docs" / "operate"


class OperationsRunbookCrossSurfaceTests(unittest.TestCase):
    def test_other_surfaces_point_to_canonical_pages_without_competing_contracts(self) -> None:
        pointers = {
            ROOT / ".claude" / "commands" / "compact-agf.md": "session-continuity.md",
            ROOT / "docs" / "reference" / "session-resume.md": "session-continuity.md",
            ROOT / "docs" / "reference" / "agent-cold-start-habits.md": "runbook.md",
            ROOT / "agf" / "templates" / "AGENTS.md": "operate/runbook.md",
        }
        for path, canonical_target in pointers.items():
            with self.subTest(path=path):
                self.assertIn(canonical_target, path.read_text(encoding="utf-8"))

        route_heading = "## Choose the situation"
        self.assertIn(route_heading, (OPERATE / "runbook.md").read_text(encoding="utf-8"))
        for path in pointers:
            self.assertNotIn(route_heading, path.read_text(encoding="utf-8"))


if __name__ == "__main__":
    unittest.main()
