# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Version bumping automation for the agents-govern framework.

Updates VERSION, CHANGELOG.md, and agent framework_version fields
in a single command. Does NOT create commits, branches, or tags —
those remain human decisions.

Usage:
    uv run agov-bump minor          # 0.27.0 → 0.28.0
    uv run agov-bump patch          # 0.27.0 → 0.27.1
    uv run agov-bump major          # 0.27.0 → 1.0.0
    uv run agov-bump 0.30.0         # explicit target version
"""

from __future__ import annotations

import re
import sys
from datetime import date
from enum import Enum
from pathlib import Path

from agents_govern._files import find_agents, repo_root
from agents_govern._output import check_fail, check_ok

__all__ = [
    "BumpType",
    "bump_semver",
    "update_version_file",
    "update_changelog",
    "update_readme_citation",
    "update_citation_cff",
    "update_agent_framework_versions",
    "main",
]


class BumpType(Enum):
    MAJOR = "major"
    MINOR = "minor"
    PATCH = "patch"


_SEMVER_RE = re.compile(r"^(\d+)\.(\d+)\.(\d+)$")


def bump_semver(
    current: str,
    bump_type: BumpType | None,
    *,
    explicit: str | None = None,
) -> str:
    """Compute the next version string.

    Either bump_type or explicit must be provided.  Each semver
    component is an independent integer — no carry between components.
    """
    current = current.strip()
    m = _SEMVER_RE.match(current)
    if not m:
        raise ValueError(f"current version {current!r} must be MAJOR.MINOR.PATCH")

    if explicit is not None:
        explicit = explicit.strip()
        if not _SEMVER_RE.match(explicit):
            raise ValueError(f"explicit version {explicit!r} must be MAJOR.MINOR.PATCH")
        if explicit == current:
            raise ValueError(f"explicit version {explicit!r} is same as current")
        return explicit

    major, minor, patch = int(m[1]), int(m[2]), int(m[3])
    match bump_type:
        case BumpType.MAJOR:
            return f"{major + 1}.0.0"
        case BumpType.MINOR:
            return f"{major}.{minor + 1}.0"
        case BumpType.PATCH:
            return f"{major}.{minor}.{patch + 1}"
        case _:
            raise ValueError("either bump_type or explicit must be provided")


def update_version_file(version_file: Path, new_version: str) -> None:
    """Rewrite the VERSION file with the new version."""
    version_file.write_text(f"{new_version}\n", encoding="utf-8")


def update_changelog(
    changelog: Path,
    new_version: str,
    today: str | None = None,
) -> None:
    """Insert a new version section into CHANGELOG.md.

    Inserts after the first ``---`` separator, before existing entries.
    """
    if today is None:
        today = date.today().isoformat()

    content = changelog.read_text(encoding="utf-8")
    new_section = (
        f"\n## v{new_version} ({today})\n"
        f"\n<!-- TODO: fill in release notes -->\n"
    )

    # Insert after the first "---" line
    sep_pattern = re.compile(r"^---\s*$", re.MULTILINE)
    m = sep_pattern.search(content)
    if m:
        insert_pos = m.end()
        content = content[:insert_pos] + "\n" + new_section + content[insert_pos:]
    else:
        # No separator found — append at end
        content = content.rstrip() + "\n\n---\n" + new_section

    changelog.write_text(content, encoding="utf-8")


def update_readme_citation(readme: Path, new_version: str) -> bool:
    """Update the citation version in README.md.

    Uses major.minor for academic citations (e.g., "Version 0.28"),
    not the full semver. Returns True if any replacement was made.
    """
    m = _SEMVER_RE.match(new_version.strip())
    if not m:
        return False
    major_minor = f"{m[1]}.{m[2]}"

    text = readme.read_text(encoding="utf-8")
    # Match "(Version X.Y)" in APA and "{Version X.Y}" in BibTeX
    pattern = re.compile(r"((?:\(|\{)Version\s+)\d+\.\d+((?:\)|\}))")
    new_text, n = pattern.subn(rf"\g<1>{major_minor}\2", text)
    if n > 0:
        readme.write_text(new_text, encoding="utf-8")
        return True
    return False


def update_citation_cff(cff_file: Path, new_version: str) -> bool:
    """Update the version field in CITATION.cff.

    Uses major.minor for academic citations (e.g., "1.1"),
    matching the granularity of the README excerpt. Returns True
    if the version field was found and updated, False if no
    matching `version: "X.Y"` line existed.
    """
    m = _SEMVER_RE.match(new_version.strip())
    if not m:
        return False
    major_minor = f"{m[1]}.{m[2]}"

    text = cff_file.read_text(encoding="utf-8")
    pattern = re.compile(r'^(version:\s*")\d+\.\d+(")', re.MULTILINE)
    new_text, n = pattern.subn(rf"\g<1>{major_minor}\2", text)
    if n > 0:
        cff_file.write_text(new_text, encoding="utf-8")
        return True
    return False


def update_agent_framework_versions(
    agent_files: list[Path] | tuple[Path, ...],
    old_version: str,
    new_version: str,
) -> int:
    """Update framework_version in agent YAML files.

    Uses text replacement (not YAML round-trip) to preserve comments
    and formatting. Only replaces lines that match the old version.

    Returns the number of files updated.
    """
    # Match: framework_version: "0.27.0"  with optional trailing comment
    pattern = re.compile(
        rf'(framework_version:\s*"){re.escape(old_version)}(")'
    )
    updated = 0
    for path in agent_files:
        text = path.read_text(encoding="utf-8")
        new_text, n = pattern.subn(rf"\g<1>{new_version}\2", text)
        if n > 0:
            path.write_text(new_text, encoding="utf-8")
            updated += 1
    return updated


def main() -> None:
    """CLI entry point for agov-bump."""
    if len(sys.argv) < 2:
        print("Usage: agov-bump <major|minor|patch|X.Y.Z>", file=sys.stderr)
        sys.exit(1)

    arg = sys.argv[1].lower()
    root = repo_root()
    version_file = root / "VERSION"
    changelog = root / "CHANGELOG.md"

    current = version_file.read_text(encoding="utf-8").strip()

    # Determine new version
    bump_type: BumpType | None = None
    explicit: str | None = None
    try:
        bump_type = BumpType(arg)
    except ValueError:
        explicit = arg

    try:
        new_version = bump_semver(current, bump_type, explicit=explicit)
    except ValueError as exc:
        check_fail(str(exc))
        sys.exit(1)

    # Apply changes
    print(f"Bumping {current} → {new_version}\n")

    update_version_file(version_file, new_version)
    check_ok(f"VERSION: {current} → {new_version}")

    update_changelog(changelog, new_version)
    check_ok(f"CHANGELOG.md: added ## v{new_version} section")

    readme = root / "README.md"
    if readme.exists() and update_readme_citation(readme, new_version):
        m = _SEMVER_RE.match(new_version)
        check_ok(f"README.md: citation updated to Version {m[1]}.{m[2]}")
    elif readme.exists():
        check_ok("README.md: no citation block found (skipped)")

    cff = root / "CITATION.cff"
    if cff.exists() and update_citation_cff(cff, new_version):
        m = _SEMVER_RE.match(new_version)
        check_ok(f"CITATION.cff: version updated to {m[1]}.{m[2]}")
    elif cff.exists():
        check_ok("CITATION.cff: no version field found (skipped)")

    agent_files = find_agents(root)
    count = update_agent_framework_versions(agent_files, current, new_version)
    check_ok(f"agents: updated framework_version in {count}/{len(agent_files)} files")

    print(f"\nDone. Review changes, then commit and run: uv run agov-pre-tag-check v{new_version}")
