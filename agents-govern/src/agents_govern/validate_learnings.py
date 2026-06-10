# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Validate learning records against the anonymization policy."""

from __future__ import annotations

import re
import sys
from dataclasses import dataclass
from pathlib import Path
from typing import Any

from agents_govern._files import find_learnings, repo_root
from agents_govern._output import error, ok, warn
from agents_govern._yaml import load_yaml

__all__ = ["validate_learnings", "main", "_check_learnings_array"]

# L2: confidence ordinal enum (issue #62)
_CONFIDENCE_LEVELS: frozenset[str] = frozenset({
    "single-adopter",
    "multi-adopter",
    "longitudinal",
    "industry-corroborated",
})

# L2: levels that require corroboration (evidence_count >= 2 OR len(evidence_refs) >= 2)
_CONFIDENCE_REQUIRES_CORROBORATION: frozenset[str] = frozenset({
    "multi-adopter",
    "longitudinal",
})

# Heuristic patterns for PII detection (immutable)
_URL_RE = re.compile(r"https?://[^\s\"]+")
_EMAIL_RE = re.compile(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}")
_COMPANY_RE = re.compile(r"\b(Inc|Corp|Ltd|GmbH|LLC|Oy|Oyj|Pty|plc)\b")
_HOSTNAME_RE = re.compile(
    r"\b[a-zA-Z0-9-]+\.(internal|local|corp|cloud|aws|azure|gcp)\b"
)


@dataclass(frozen=True, slots=True)
class _FileResult:
    name: str
    errors: int
    warnings: int


def _scan_content(
    name: str, text: str
) -> tuple[int, int]:
    """Scan file text for PII heuristics. Returns (errors, warnings)."""
    warnings = 0

    # Strip comment lines for scanning
    lines = [
        line for line in text.splitlines()
        if not line.strip().startswith("#")
    ]
    content = "\n".join(lines)

    # URLs
    matches = _URL_RE.findall(content)
    # Exclude framework_commit lines
    matches = [m for m in matches if "framework_commit" not in content.split(m)[0].rsplit("\n", 1)[-1]]
    if matches:
        warn(f"{name}: contains URL(s) \u2014 verify no internal hostnames are exposed")
        for m in matches[:3]:
            print(f"         {m}")
        warnings += 1

    # Email addresses
    if _EMAIL_RE.search(content):
        matches = _EMAIL_RE.findall(content)
        warn(f"{name}: contains email address(es)")
        for m in matches[:3]:
            print(f"         {m}")
        warnings += 1

    # Company suffixes (skip SPDX lines)
    non_spdx = "\n".join(l for l in lines if "SPDX" not in l)
    if _COMPANY_RE.search(non_spdx):
        matches = _COMPANY_RE.findall(non_spdx)
        warn(f"{name}: contains company suffix \u2014 likely an organisation name")
        warnings += 1

    # Hostnames
    if _HOSTNAME_RE.search(content):
        matches = _HOSTNAME_RE.findall(content)
        warn(f"{name}: contains hostname pattern \u2014 verify no infrastructure identifiers")
        warnings += 1

    return 0, warnings


def _check_structure(
    name: str, data: dict[str, Any]
) -> int:
    """Check structural requirements. Returns error count."""
    errors = 0

    project = data.get("project")
    if not isinstance(project, dict):
        error(f"{name}: missing project section")
        return 1

    codename = project.get("codename")
    if codename is None:
        # Check for old project.name field
        if "name" in project:
            error(f"{name}: uses old project.name field \u2014 rename to project.codename")
        else:
            error(f"{name}: project.codename field is missing (old format?)")
        errors += 1
    elif not codename or not str(codename).strip():
        error(f"{name}: project.codename is empty")
        errors += 1

    return errors


def _check_superseded_by(
    name: str, idx: int, entry: dict[str, Any], titles_by_index: list[str]
) -> int:
    """Check superseded_by field on a single entry. Returns error count."""
    sup = entry.get("superseded_by")
    if sup is None:
        return 0  # Optional field

    if not isinstance(sup, str) or not sup.strip():
        error(f"{name}: entry #{idx + 1}: superseded_by must be a non-empty string")
        return 1

    # Self-reference
    own_title = entry.get("title", "")
    if isinstance(own_title, str) and sup == own_title:
        error(f"{name}: entry #{idx + 1}: superseded_by cannot reference the entry's own title")
        return 1

    # Find matching entry by title; must be at a later index (append-only invariant)
    later_match = False
    earlier_match = False
    for j, t in enumerate(titles_by_index):
        if j == idx or t != sup:
            continue
        if j > idx:
            later_match = True
        else:
            earlier_match = True

    if not later_match and not earlier_match:
        error(
            f"{name}: entry #{idx + 1}: superseded_by '{sup}' does not match any "
            f"other entry's title in this file"
        )
        return 1
    if not later_match and earlier_match:
        error(
            f"{name}: entry #{idx + 1}: superseded_by '{sup}' references an "
            f"earlier entry; supersession must point forward (append-only invariant)"
        )
        return 1

    return 0


def _check_confidence(
    name: str, idx: int, entry: dict[str, Any]
) -> int:
    """Check confidence field on a single entry. Returns error count."""
    conf = entry.get("confidence")
    if conf is None:
        return 0  # Optional field

    if not isinstance(conf, dict):
        error(f"{name}: entry #{idx + 1}: confidence must be a mapping with level: field")
        return 1

    level = conf.get("level")
    if not isinstance(level, str) or not level.strip():
        error(f"{name}: entry #{idx + 1}: confidence.level is required and must be a string")
        return 1

    if level not in _CONFIDENCE_LEVELS:
        error(
            f"{name}: entry #{idx + 1}: confidence.level '{level}' must be one of "
            f"{sorted(_CONFIDENCE_LEVELS)}"
        )
        return 1

    errors = 0

    # evidence_count type check
    evidence_count = conf.get("evidence_count")
    if evidence_count is not None:
        if not isinstance(evidence_count, int) or isinstance(evidence_count, bool):
            error(f"{name}: entry #{idx + 1}: confidence.evidence_count must be an integer")
            errors += 1

    # evidence_refs type check
    evidence_refs = conf.get("evidence_refs")
    if evidence_refs is not None:
        if not isinstance(evidence_refs, list):
            error(f"{name}: entry #{idx + 1}: confidence.evidence_refs must be a list")
            errors += 1

    if errors > 0:
        return errors

    # Consistency rule: corroborated levels need evidence_count >= 2 OR len(evidence_refs) >= 2
    if level in _CONFIDENCE_REQUIRES_CORROBORATION:
        count_ok = isinstance(evidence_count, int) and not isinstance(evidence_count, bool) and evidence_count >= 2
        refs_ok = isinstance(evidence_refs, list) and len(evidence_refs) >= 2
        if not (count_ok or refs_ok):
            error(
                f"{name}: entry #{idx + 1}: confidence.level='{level}' requires "
                f"either evidence_count >= 2 or evidence_refs of length >= 2"
            )
            errors += 1

    return errors


def _check_learnings_array(name: str, data: dict[str, Any]) -> int:
    """Check optional per-entry fields in the learnings: array.

    Currently checks (issues L1, L2):
    - superseded_by: must reference a later-indexed entry's title in same file
    - confidence: optional dict with level (enum) + optional evidence_count/_refs

    Returns error count.
    """
    learnings = data.get("learnings", [])
    if not isinstance(learnings, list):
        return 0

    # Build title index for supersession lookup (empty string if title missing/non-string)
    titles_by_index: list[str] = []
    for entry in learnings:
        if isinstance(entry, dict):
            t = entry.get("title", "")
            titles_by_index.append(t if isinstance(t, str) else "")
        else:
            titles_by_index.append("")

    errors = 0
    for i, entry in enumerate(learnings):
        if not isinstance(entry, dict):
            continue
        errors += _check_superseded_by(name, i, entry, titles_by_index)
        errors += _check_confidence(name, i, entry)

    return errors


def validate_learnings(files: tuple[Path, ...] | None = None) -> int:
    """Validate learning record files. Returns exit code."""
    root = repo_root()
    if files is None:
        files = find_learnings(root)

    if not files:
        print("No learning files to validate.")
        return 0

    print(f"Validating {len(files)} learning record(s)...")
    print()

    total_errors = 0
    total_warnings = 0

    for path in files:
        name = path.name

        if not path.exists():
            error(f"{name}: file not found")
            total_errors += 1
            continue

        data = load_yaml(path)
        text = path.read_text(encoding="utf-8")

        errs = _check_structure(name, data)
        total_errors += errs

        if isinstance(data, dict):
            total_errors += _check_learnings_array(name, data)

        _, warns = _scan_content(name, text)
        total_warnings += warns

    print()
    print("--- Summary ---")
    print(f"Files checked: {len(files)}")
    print(f"Errors:   {total_errors}")
    print(f"Warnings: {total_warnings}")

    if total_errors > 0:
        print(f"\n\033[0;31mValidation failed.\033[0m Fix errors before submitting.")
        return 1
    elif total_warnings > 0:
        print(f"\n\033[0;33mPassed with warnings.\033[0m Review flagged items with the Sentinel.")
        return 0
    else:
        print(f"\n\033[0;32mAll checks passed.\033[0m")
        return 0


_HELP = """\
validate-learnings — Validate learning record structure and anonymization

Usage:
  uv run agov-validate-learnings [FILES...]

Checks learning YAML files for:
  - Required fields (project.codename)
  - PII patterns (emails, URLs, company suffixes, hostnames)
  - Structural integrity

If no FILES are given, validates all files in learnings/ (excluding template).

Exit codes: 0 = pass, 1 = validation errors found.

Full documentation: LEARNINGS.md.
"""


def main() -> None:
    if len(sys.argv) > 1 and sys.argv[1] in ("--help", "-h"):
        print(_HELP)
        return
    files: tuple[Path, ...] | None = None
    if len(sys.argv) > 1:
        files = tuple(Path(a) for a in sys.argv[1:])
    sys.exit(validate_learnings(files))


if __name__ == "__main__":
    main()
