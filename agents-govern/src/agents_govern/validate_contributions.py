# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Adversarial pattern detection for governance contributions.

Mechanically flags contributions that exhibit patterns correlating with
adversarial steering, unsound guidance, or false precedent. Every finding
requires human review — this tool detects patterns, not intent.

Usage:
    uv run agov-validate-contributions [FILES...]

Without arguments, scans learnings/*.yaml for adversarial patterns.
"""

from __future__ import annotations

import re
import sys
from pathlib import Path
from typing import Any

from agents_govern._files import find_learnings, repo_root
from agents_govern._output import ok, warn, fail
from agents_govern._yaml import load_yaml, extract_freetext

__all__ = [
    "check_absence_bias",
    "check_governance_relaxation",
    "check_evidence_quality",
    "audit_learning",
    "validate_contributions",
    "main",
]


# ============================================================================
# Absence-of-incident bias detection
# ============================================================================

_ABSENCE_PATTERNS: list[re.Pattern[str]] = [
    re.compile(p, re.IGNORECASE) for p in [
        r"no\s+incidents?\b",
        r"no\s+issues?\b",
        r"no\s+problems?\b",
        r"no\s+complaints?\b",
        r"no\s+failures?\b",
        r"no\s+bugs?\b",
        r"no\s+one\s+complained",
        r"without\s+(?:any\s+)?issues?\b",
        r"without\s+(?:any\s+)?problems?\b",
        r"without\s+(?:any\s+)?incidents?\b",
        r"without\s+consequences\b",
        r"saw\s+no\b",
        r"had\s+no\s+(?:issues?|problems?|incidents?)\b",
        r"worked\s+(?:fine|for\s+us)\b",
    ]
]


def check_absence_bias(text: str) -> list[str]:
    """Detect absence-of-incident reasoning in text.

    Returns a list of finding descriptions. Empty list means no flags.
    """
    findings: list[str] = []
    for pattern in _ABSENCE_PATTERNS:
        m = pattern.search(text)
        if m:
            findings.append(
                f"Absence bias: '{m.group()}' — absence of reported incidents "
                f"is not evidence of safety"
            )
    return findings


# ============================================================================
# Governance relaxation detection
# ============================================================================

_RELAXATION_PATTERNS: list[re.Pattern[str]] = [
    re.compile(p, re.IGNORECASE) for p in [
        r"\b(?:remove|drop|eliminate|disable)\b.{0,40}\b(?:review|check|gate|rule|boundary|escalation|validation)\b",
        r"\b(?:skip|bypass|avoid)\b.{0,40}\b(?:review|check|gate|rule|boundary|escalation|validation)\b",
        r"\b(?:skipping)\b.{0,40}\b(?:review|check|gate|rule|boundary|escalation|validation)\b",
        r"\b(?:relax|loosen|weaken|soften|relaxed|loosened|weakened|softened)\b.{0,40}\b(?:boundary|rule|constraint|requirement|enforcement)\b",
        r"\bunnecessary\s+(?:overhead|ceremony|friction|process|bureaucracy)\b",
        r"\b(?:overhead|ceremony|friction)\b.{0,20}\b(?:unnecessary|excessive|too\s+much)\b",
    ]
]


def check_governance_relaxation(text: str) -> list[str]:
    """Detect recommendations to relax governance mechanisms.

    Returns a list of finding descriptions. Empty list means no flags.
    """
    findings: list[str] = []
    for pattern in _RELAXATION_PATTERNS:
        m = pattern.search(text)
        if m:
            findings.append(
                f"Governance relaxation: '{m.group()}' — proposals to weaken "
                f"governance require explicit evidence and human approval"
            )
    return findings


# ============================================================================
# Evidence quality detection
# ============================================================================

_WEAK_EVIDENCE_PATTERNS: list[re.Pattern[str]] = [
    re.compile(p, re.IGNORECASE) for p in [
        r"\ball\s+projects?\s+should\b",
        r"\balways\s+works?\b",
        r"\bnever\s+fails?\b",
        r"\beveryone\s+should\b",
        r"\bregardless\s+of\b.{0,20}\b(?:project|size|context|domain)\b",
        r"\buniversally\s+(?:applicable|true|valid)\b",
    ]
]


def check_evidence_quality(text: str) -> list[str]:
    """Detect weak or overly universal evidence claims.

    Returns a list of finding descriptions. Empty list means no flags.
    """
    findings: list[str] = []
    for pattern in _WEAK_EVIDENCE_PATTERNS:
        m = pattern.search(text)
        if m:
            findings.append(
                f"Evidence quality: '{m.group()}' — universal claims require "
                f"corroboration across multiple projects"
            )
    return findings


# ============================================================================
# Learning record audit (integration)
# ============================================================================

def audit_learning(data: dict[str, Any]) -> list[str]:
    """Audit a single learning record for adversarial patterns.

    Extracts free-text fields and runs all checks. Returns combined findings.
    """
    texts = extract_freetext(data)
    if not texts:
        return []

    combined = " ".join(texts)
    findings: list[str] = []
    findings.extend(check_absence_bias(combined))
    findings.extend(check_governance_relaxation(combined))
    findings.extend(check_evidence_quality(combined))
    return findings


# ============================================================================
# CLI
# ============================================================================

def validate_contributions(files: tuple[Path, ...] | None = None) -> int:
    """Validate contribution files for adversarial patterns. Returns exit code."""
    root = repo_root()
    if files is None:
        files = find_learnings(root)

    if not files:
        print("No contribution files found.")
        return 0

    print(f"Contribution audit: {len(files)} file(s)")
    print()

    total_findings = 0

    for path in files:
        data = load_yaml(path)
        if not data:
            continue

        # Audit each learning entry
        learnings = data.get("learnings", [])
        if not isinstance(learnings, list):
            learnings = [data]  # single-entry format

        file_findings: list[str] = []
        for entry in learnings:
            if isinstance(entry, dict):
                file_findings.extend(audit_learning(entry))

        if file_findings:
            warn(f"{path.name}: {len(file_findings)} finding(s)")
            for f in file_findings:
                print(f"       {f}")
            total_findings += len(file_findings)
        else:
            ok(path.name)

    print()
    if total_findings > 0:
        print(f"\033[0;33m{total_findings} finding(s) flagged for human review.\033[0m")
        print("These are patterns, not verdicts — review each finding before acting.")
        return 1
    print(f"\033[0;32mAll {len(files)} contribution(s) passed audit.\033[0m")
    return 0


def main() -> None:
    """CLI entry point for validate-contributions."""
    files: tuple[Path, ...] | None = None
    if len(sys.argv) > 1:
        files = tuple(Path(a) for a in sys.argv[1:])
    sys.exit(validate_contributions(files))


if __name__ == "__main__":
    main()
