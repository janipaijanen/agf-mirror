# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Agent manifest — commissioning, suspension, and lifecycle management.

The manifest (agents/MANIFEST.yaml) declares which agents are active,
suspended, or skipped in a project. It is the single source of truth for
agent commissioning decisions.

Agents can recommend their own suspension (via activation hints), but
cannot execute it. Suspension requires human approval via a manifest
change through the governance gate.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
from typing import Any

from agents_govern._output import check_fail, check_ok, check_warn
from agents_govern._yaml import load_yaml

__all__ = [
    "AgentStatus",
    "VALID_STATUSES",
    "load_manifest",
    "validate_manifest",
]

VALID_STATUSES: frozenset[str] = frozenset({"active", "suspended", "skipped"})


@dataclass(frozen=True, slots=True)
class AgentStatus:
    """Status of a single agent in the manifest."""

    status: str
    played_by: str = ""
    reason: str = ""
    resume_when: str = ""

    @property
    def is_active(self) -> bool:
        return self.status == "active"

    @property
    def is_suspended(self) -> bool:
        return self.status == "suspended"

    @property
    def is_skipped(self) -> bool:
        return self.status == "skipped"


def load_manifest(manifest_path: Path) -> dict[str, AgentStatus]:
    """Load agents/MANIFEST.yaml and return agent statuses.

    Returns empty dict if file doesn't exist or can't be parsed.
    """
    data = load_yaml(manifest_path)
    if not data:
        return {}

    agents_section = data.get("agents")
    if not isinstance(agents_section, dict):
        return {}

    result: dict[str, AgentStatus] = {}
    for name, entry in agents_section.items():
        if not isinstance(entry, dict):
            continue
        result[str(name)] = AgentStatus(
            status=str(entry.get("status", "active")),
            played_by=str(entry.get("played_by", "")),
            reason=str(entry.get("reason", "")),
            resume_when=str(entry.get("resume_when", "")),
        )
    return result


def validate_manifest(root: Path) -> tuple[int, int]:
    """Validate manifest consistency against agent YAML files.

    Checks:
    - Every agent in manifest has a corresponding YAML file
    - Every agent YAML file is listed in the manifest
    - Status values are valid
    - Suspended/skipped agents have a reason

    Returns (errors, warnings).
    """
    errors = 0
    warnings = 0

    agents_dir = root / "agents"
    manifest_path = agents_dir / "MANIFEST.yaml"

    # Collect agent YAML files (excluding MANIFEST.yaml and local/)
    yaml_agents: set[str] = set()
    if agents_dir.is_dir():
        for f in agents_dir.glob("*.yaml"):
            if f.name == "MANIFEST.yaml":
                continue
            data = load_yaml(f)
            agent_sec = data.get("agent", {})
            if isinstance(agent_sec, dict) and "name" in agent_sec:
                yaml_agents.add(str(agent_sec["name"]))

    # Load manifest
    if not manifest_path.exists():
        if yaml_agents:
            check_warn(
                f"agents/MANIFEST.yaml: not found — {len(yaml_agents)} agent(s) "
                f"have no commissioning status"
            )
            warnings += 1
        return errors, warnings

    manifest = load_manifest(manifest_path)
    manifest_names = set(manifest.keys())

    # Check: every manifest entry has a YAML file
    for name in sorted(manifest_names - yaml_agents):
        check_fail(f"MANIFEST.yaml: '{name}' listed but no agents/{name}.yaml found")
        errors += 1

    # Check: every YAML file is in the manifest
    for name in sorted(yaml_agents - manifest_names):
        check_warn(f"MANIFEST.yaml: agents/{name}.yaml exists but not listed in manifest")
        warnings += 1

    # Check: valid statuses and required fields
    for name, status in sorted(manifest.items()):
        if status.status not in VALID_STATUSES:
            check_fail(
                f"MANIFEST.yaml: '{name}' has invalid status '{status.status}' "
                f"(expected: {', '.join(sorted(VALID_STATUSES))})"
            )
            errors += 1
            continue

        if status.is_suspended and not status.reason:
            check_fail(f"MANIFEST.yaml: '{name}' is suspended but has no reason")
            errors += 1

        if status.is_skipped and not status.reason:
            check_fail(f"MANIFEST.yaml: '{name}' is skipped but has no reason")
            errors += 1

    # Summary counts
    active = sum(1 for s in manifest.values() if s.is_active)
    suspended = sum(1 for s in manifest.values() if s.is_suspended)
    skipped = sum(1 for s in manifest.values() if s.is_skipped)

    if errors == 0:
        check_ok(
            f"MANIFEST.yaml: {active} active, {suspended} suspended, "
            f"{skipped} skipped"
        )

    return errors, warnings
