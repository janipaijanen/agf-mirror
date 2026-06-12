# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Validate that a project's governance surface conforms to agents-govern.

Checks the framework-shaped artifacts mechanically:
- Step 1.3: agents/ exists with YAML agent definitions
- Mandatory agents (contribution-auditor, infosec-sentinel) present
- Step 2: GOVERNANCE.md exists
- License headers (SPDX) on every agent definition
- agents/MANIFEST.yaml present and conformant

Project-tooling artifacts (.claude/rules/, tests/, .pre-commit-config.yaml,
rules-template sync) are intentionally out of scope: under Layout B their
location depends on which directory hosts the framework, not on the
governance contract. Adopters who want those checks can wire them into
their own pre-commit. See the twin-doors learning record (2026-05-03).

Errors block the release. Warnings report but pass.
"""

from __future__ import annotations

import sys
from pathlib import Path

from agents_govern._files import repo_root
from agents_govern._output import check_fail, check_ok, check_warn
from agents_govern._yaml import load_yaml

__all__ = ["validate_adoption", "main"]

# Agents that MUST be present and active in every adopter project.
# These are not optional — they form the trust gate for contributions.
MANDATORY_AGENTS: frozenset[str] = frozenset({
    "contribution-auditor",
    "infosec-sentinel",
})


def validate_adoption(root: Path | None = None) -> tuple[int, int]:
    """Validate adoption conformance. Returns (errors, warnings)."""
    if root is None:
        root = repo_root()

    errors = 0
    warnings = 0

    # Step 1.3: agents/ with YAML definitions (ERROR if missing)
    agents_dir = root / "agents"
    agent_files = [
        f for f in agents_dir.glob("*.yaml")
        if f.name != "MANIFEST.yaml"
    ] if agents_dir.is_dir() else []
    if not agent_files:
        check_fail("agents/: no YAML agent definitions found (ADOPTING.md Step 1.3)")
        errors += 1
    else:
        check_ok(f"agents/: {len(agent_files)} agent definition(s)")

    # Mandatory agents (ERROR if missing)
    if agent_files:
        agent_names: set[str] = set()
        for af in agent_files:
            data = load_yaml(af)
            agent_sec = data.get("agent", {})
            if isinstance(agent_sec, dict) and "name" in agent_sec:
                agent_names.add(str(agent_sec["name"]))

        missing_mandatory = MANDATORY_AGENTS - agent_names
        if missing_mandatory:
            for name in sorted(missing_mandatory):
                check_fail(
                    f"Mandatory agent missing: {name} "
                    f"(copy from upstream agents/{name}.yaml)"
                )
                errors += 1
        else:
            check_ok(f"Mandatory agents: {', '.join(sorted(MANDATORY_AGENTS))}")

    # Step 2: GOVERNANCE.md (WARN if missing)
    governance = root / "GOVERNANCE.md"
    if not governance.exists():
        check_warn("GOVERNANCE.md: not found (ADOPTING.md Step 2)")
        warnings += 1
    else:
        check_ok("GOVERNANCE.md: present")

    # License headers on agent definitions (WARN if missing)
    if agent_files:
        agents_without_spdx = [
            f.name for f in agent_files
            if "SPDX-License-Identifier:" not in f.read_text(encoding="utf-8")
        ]
        if agents_without_spdx:
            check_warn(
                f"License headers: {len(agents_without_spdx)} agent(s) missing SPDX header: "
                + ", ".join(agents_without_spdx[:3])
                + ("..." if len(agents_without_spdx) > 3 else "")
            )
            warnings += 1
        else:
            check_ok(f"License headers: all {len(agent_files)} agent(s) have SPDX headers")

    # Agent manifest (WARN if missing)
    manifest_file = root / "agents" / "MANIFEST.yaml"
    if manifest_file.exists():
        from agents_govern.manifest import validate_manifest
        m_errors, m_warnings = validate_manifest(root)
        errors += m_errors
        warnings += m_warnings
    elif (root / "agents").is_dir() and any((root / "agents").glob("*.yaml")):
        check_warn("agents/MANIFEST.yaml: not found — agents have no commissioning status")
        warnings += 1

    # Summary
    print()
    total = errors + warnings
    if errors > 0:
        print(f"\033[0;31m{errors} error(s), {warnings} warning(s).\033[0m")
    elif warnings > 0:
        print(f"\033[0;33mGovernance-surface conformance: {warnings} warning(s), 0 errors.\033[0m")
    else:
        print(f"\033[0;32mGovernance-surface conformance: all checks passed.\033[0m")

    return errors, warnings


_HELP = """\
validate-adoption — Check the agents-govern governance surface of a project

Usage:
  uv run agov-validate-adoption [ROOT]

Checks the framework-shaped artifacts mechanically:
  - agents/ exists with YAML agent definitions (ADOPTING.md Step 1.3) — ERROR if missing
  - Mandatory agents (contribution-auditor, infosec-sentinel) present — ERROR if missing
  - GOVERNANCE.md exists (ADOPTING.md Step 2) — WARN if missing
  - SPDX license headers on every agent definition — WARN if missing
  - agents/MANIFEST.yaml present and conformant — WARN if missing

Project-tooling artifacts (.claude/rules/, tests/, .pre-commit-config.yaml,
rules-template sync) are intentionally out of scope. Their location depends
on the adoption layout (Layout A vs. Layout B); enforcing them from a tool
that runs from the framework directory misfires under Layout B. Adopters
who want those checks can wire them into their own pre-commit.

If no ROOT is given, uses the current repository root.

Exit codes: 0 = pass (warnings only), 1 = errors found.

Full documentation: ADOPTING.md.
"""


def main() -> None:
    if len(sys.argv) > 1 and sys.argv[1] in ("--help", "-h"):
        print(_HELP)
        return

    root = None
    if len(sys.argv) > 1:
        root = Path(sys.argv[1])

    errors, _warnings = validate_adoption(root)
    sys.exit(1 if errors > 0 else 0)


if __name__ == "__main__":
    main()
