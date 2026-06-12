# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Periodic governance health check.

Runs all validators in sequence and produces a summary. Designed to be
run periodically (pre-commit, weekly CI, or on-demand) to catch drift
before it accumulates.

Usage:
    uv run agov-health [ROOT]

Runs: validate-adoption, validate-agents (strict), validate-contributions,
      validate-infosec.
"""

from __future__ import annotations

import io
import sys
from contextlib import redirect_stdout
from pathlib import Path
from typing import Any

from agents_govern._files import repo_root
from agents_govern._output import check_fail, check_ok, check_warn

__all__ = ["health_check", "main"]


def health_check(root: Path | None = None) -> dict[str, Any]:
    """Run all governance validators and return a summary.

    Returns a dict with:
        passed: bool — True if zero errors
        errors: int — total error count
        warnings: int — total warning count
        sections: dict — per-section results
    """
    if root is None:
        root = repo_root()

    sections: dict[str, dict[str, Any]] = {}
    total_errors = 0
    total_warnings = 0

    print("=" * 60)
    print("  Governance Health Check")
    print("=" * 60)
    print()

    # 1. Adoption conformance
    print("--- Adoption conformance ---")
    from agents_govern.validate_adoption import validate_adoption
    a_errors, a_warnings = validate_adoption(root)
    sections["adoption"] = {"errors": a_errors, "warnings": a_warnings}
    total_errors += a_errors
    total_warnings += a_warnings
    print()

    # 2. Agent definitions (strict mode)
    print("--- Agent definitions (strict) ---")
    from agents_govern.validate_agents import validate_agents
    buf = io.StringIO()
    with redirect_stdout(buf):
        agents_rc = validate_agents(strict=True)
    agent_output = buf.getvalue()
    print(agent_output, end="")
    a_err = 1 if agents_rc != 0 else 0
    sections["agents"] = {"errors": a_err, "exit_code": agents_rc}
    total_errors += a_err
    print()

    # 3. Contribution audit (learnings)
    print("--- Contribution audit ---")
    from agents_govern.validate_contributions import validate_contributions
    buf = io.StringIO()
    with redirect_stdout(buf):
        contrib_rc = validate_contributions()
    contrib_output = buf.getvalue()
    print(contrib_output, end="")
    c_warnings = 1 if contrib_rc != 0 else 0
    sections["contributions"] = {"warnings": c_warnings, "exit_code": contrib_rc}
    total_warnings += c_warnings
    print()

    # 4. InfoSec screening
    print("--- InfoSec screening ---")
    from agents_govern.validate_infosec import validate_infosec
    buf = io.StringIO()
    with redirect_stdout(buf):
        infosec_rc = validate_infosec()
    infosec_output = buf.getvalue()
    print(infosec_output, end="")
    i_err = 1 if infosec_rc != 0 else 0
    sections["infosec"] = {"errors": i_err, "exit_code": infosec_rc}
    total_errors += i_err

    # Summary
    print()
    print("=" * 60)
    passed = total_errors == 0
    if passed and total_warnings == 0:
        print(f"\033[0;32m  HEALTHY — all checks passed.\033[0m")
    elif passed:
        print(f"\033[0;33m  HEALTHY with {total_warnings} warning(s) — review flagged items.\033[0m")
    else:
        print(f"\033[0;31m  UNHEALTHY — {total_errors} error(s), {total_warnings} warning(s).\033[0m")
    print("=" * 60)

    return {
        "passed": passed,
        "errors": total_errors,
        "warnings": total_warnings,
        "sections": sections,
    }


def main() -> None:
    """CLI entry point for agov-health."""
    root = None
    if len(sys.argv) > 1 and sys.argv[1] not in ("--help", "-h"):
        root = Path(sys.argv[1])
    elif len(sys.argv) > 1:
        print("Usage: agov-health [ROOT]")
        print()
        print("Runs all governance validators and produces a summary.")
        print("Designed for periodic use: pre-commit, weekly CI, or on-demand.")
        print()
        print("Checks run:")
        print("  1. Adoption conformance (validate-adoption)")
        print("  2. Agent definitions, strict mode (validate-agents --strict)")
        print("  3. Contribution audit (validate-contributions)")
        print("  4. InfoSec screening (validate-infosec)")
        return

    result = health_check(root)
    sys.exit(0 if result["passed"] else 1)


if __name__ == "__main__":
    main()
