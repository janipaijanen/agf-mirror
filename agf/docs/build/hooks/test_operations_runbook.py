# SPDX-License-Identifier: MIT
"""Contract checks for the public operations and continuity runbook."""

from pathlib import Path
import unittest


SITE = Path(__file__).parents[2] / "site"
OPERATE = SITE / "docs" / "operate"

RECOVERY_CASES = {
    "no checkpoint": ("unknown", "publish a recovery note"),
    "dirty tree": ("local", "identify changed/untracked paths"),
    "local-only commit": ("local", "push only if already authorized"),
    "pushed branch without handoff": ("shared", "publish a validated handoff"),
    "missing network/base": ("unknown", "shared-state comparison is incomplete"),
    "fresh clone": ("unknown", "Assume no local commits"),
}


class OperationsRunbookTests(unittest.TestCase):
    def test_runbook_routes_all_continuity_and_operating_situations(self) -> None:
        text = (OPERATE / "runbook.md").read_text(encoding="utf-8")
        for target in (
            "session-continuity.md#start-or-resume",
            "session-continuity.md#graceful-checkpoint",
            "session-continuity.md#abrupt-loss-recovery",
            "poll-and-coordinate.md",
            "day-to-day.md",
            "review-and-release.md",
            "../adopt/upgrade-from-v1.md",
            "../adopt/exit-and-decommission.md",
        ):
            self.assertIn(target, text)

    def test_abrupt_recovery_scenarios_preserve_uncertainty(self) -> None:
        text = (OPERATE / "session-continuity.md").read_text(encoding="utf-8")
        for scenario in (
            "Dirty persistent checkout",
            "Local-only commit not pushed",
            "Pushed branch without handoff",
            "Lost or unsaved IDE buffer",
            "Base branch missing or network unavailable",
            "Fresh clone on another machine",
        ):
            self.assertIn(scenario, text)
        self.assertIn("unknown", text)
        self.assertIn("do not claim the missing work existed", text)

    def test_recovery_inputs_have_expected_classification_and_action(self) -> None:
        """Tabletop fixture: each input maps to an evidence class and action."""
        text = (OPERATE / "session-continuity.md").read_text(encoding="utf-8")
        normalized = text.lower()
        for scenario, (classification, expected_action) in RECOVERY_CASES.items():
            with self.subTest(scenario=scenario):
                self.assertIn(classification.lower(), normalized)
                self.assertIn(expected_action.lower(), normalized)

        # Authority/security ambiguity must be escalated rather than inferred.
        self.assertIn("Escalate unclear approval, ownership, security", text)

    def test_poll_includes_every_shared_activity_surface(self) -> None:
        text = (OPERATE / "poll-and-coordinate.md").read_text(encoding="utf-8")
        for phrase in (
            "coordination",
            "discussions",
            "comments/notes",
            "tickets/issues",
            "issue comments",
            "pipelines/jobs",
        ):
            self.assertIn(phrase, text)

    def test_adapter_sections_do_not_define_core_durability(self) -> None:
        runbook = (OPERATE / "runbook.md").read_text(encoding="utf-8")
        core, adapters = runbook.split("## Adapter boundaries", maxsplit=1)
        self.assertIn("Durable-state rule", core)
        self.assertNotIn("glab", core)
        self.assertNotIn("/compact-agf", core)
        self.assertIn("GitLab adapter", adapters)
        self.assertIn("Claude adapter", adapters)

    def test_other_surfaces_point_to_canonical_pages_without_competing_contracts(self) -> None:
        """Cross-surface invariant: adapters/references name the canonical source."""
        root = Path(__file__).parents[4]
        pointers = {
            root / ".claude" / "commands" / "compact-agf.md": "session-continuity.md",
            root / "docs" / "reference" / "session-resume.md": "session-continuity.md",
            root / "docs" / "reference" / "agent-cold-start-habits.md": "runbook.md",
            root / "agf" / "templates" / "AGENTS.md": "operate/runbook.md",
        }
        for path, canonical_target in pointers.items():
            with self.subTest(path=path):
                self.assertIn(canonical_target, path.read_text(encoding="utf-8"))

        # The public map alone owns the complete nine-route decision table.
        route_heading = "## Choose the situation"
        self.assertIn(route_heading, (OPERATE / "runbook.md").read_text(encoding="utf-8"))
        for path in pointers:
            self.assertNotIn(route_heading, path.read_text(encoding="utf-8"))

    def test_shipped_template_uses_supported_graph_index_syntax(self) -> None:
        root = Path(__file__).parents[4]
        template = (root / "agf" / "templates" / "AGENTS.md").read_text(encoding="utf-8")
        self.assertNotIn("agov coord graph-index docs/coordination/", template)
        self.assertGreaterEqual(template.count("agov coord graph-index"), 2)


if __name__ == "__main__":
    unittest.main()
