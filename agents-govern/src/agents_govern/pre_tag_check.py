# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Pre-tag validation: verify VERSION, CHANGELOG, and agents are aligned."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

from agents_govern._files import find_agents, repo_root
from agents_govern._output import check_fail, check_ok, check_warn
from agents_govern._yaml import load_yaml
from agents_govern.graph import generate_graph, has_dot_binary
from agents_govern.orchestrator import run_validator
from agents_govern.validate_agents import (
    _check_communication_symmetry,
    _check_ownership_overlaps,
    _extract_comm_agents,
    _generate_comm_graph,
    validate_agents,
)
from agents_govern.validate_infosec import validate_infosec
from agents_govern.validate_readme_stats import validate_readme_stats

__all__ = ["pre_tag_check", "main"]


def _git(*args: str) -> str:
    """Run a git command and return stdout."""
    result = subprocess.run(
        ("git", *args),
        capture_output=True,
        text=True,
    )
    return result.stdout.strip()


def _git_ok(*args: str) -> bool:
    """Run a git command and return True if it succeeds."""
    result = subprocess.run(
        ("git", *args),
        capture_output=True,
    )
    return result.returncode == 0


def pre_tag_check(tag: str, *, in_pre_push: bool = False) -> int:
    """Run all pre-tag checks. Returns exit code.

    When `in_pre_push=True` the check is being invoked from the
    `.git-hooks/pre-push` hook AFTER `git tag` has already created
    the tag locally. In that case the local-tag-existence check is
    inverted: the tag MUST exist locally (we are about to push it),
    and the remote is checked instead — failing only if the tag
    already exists on origin pointing to a different commit
    (a retag attempt).
    """
    version = tag.removeprefix("v")
    errors = 0
    root = repo_root()

    print(f"Pre-tag check for {tag} (version {version})")
    if in_pre_push:
        print("(running in pre-push mode — local tag may exist)")
    print()

    # 1. VERSION file matches
    version_file = root / "VERSION"
    file_version = version_file.read_text(encoding="utf-8").strip() if version_file.exists() else ""
    if file_version == version:
        check_ok(f"VERSION file: {file_version}")
    else:
        check_fail(f"VERSION file: {file_version} (expected {version})")
        errors += 1

    # 2. CHANGELOG has an entry for this version
    changelog = root / "CHANGELOG.md"
    if changelog.exists() and f"## v{version}" in changelog.read_text(encoding="utf-8"):
        check_ok(f"CHANGELOG.md: entry for v{version} found")
    else:
        check_fail(f"CHANGELOG.md: no entry for v{version}")
        errors += 1

    # 3. Agent definitions have matching framework_version
    agent_files = find_agents(root)
    agent_errors = 0
    for agent_path in agent_files:
        data = load_yaml(agent_path)
        agent_section = data.get("agent", {})
        fw_ver = str(agent_section.get("framework_version", "")) if isinstance(agent_section, dict) else ""
        if fw_ver != version:
            check_fail(
                f"{agent_path.name}: framework_version={fw_ver} (expected {version})"
            )
            agent_errors += 1

    if agent_errors == 0:
        check_ok(f"Agent definitions: all {len(agent_files)} at {version}")
    else:
        errors += agent_errors

    # 4. Tag existence \u2014 semantics depend on invocation context.
    #    Default mode: must NOT exist (we're about to create it).
    #    Pre-push mode: MUST exist locally (just created by git tag);
    #    must NOT exist on origin pointing to a different commit.
    local_exists = _git_ok("rev-parse", "--verify", f"refs/tags/{tag}")
    if in_pre_push:
        if not local_exists:
            check_fail(f"Tag {tag} does not exist locally (expected during pre-push)")
            errors += 1
        else:
            check_ok(f"Tag {tag} exists locally (expected during pre-push)")
            local_sha = _git("rev-parse", f"refs/tags/{tag}")
            remote_line = _git("ls-remote", "origin", f"refs/tags/{tag}")
            if remote_line:
                remote_sha = remote_line.split()[0]
                if remote_sha != local_sha:
                    check_fail(
                        f"Tag {tag} on origin points to {remote_sha[:8]}, "
                        f"local points to {local_sha[:8]} \u2014 refusing retag attempt"
                    )
                    errors += 1
                else:
                    check_ok(f"Tag {tag} on origin matches local")
            else:
                check_ok(f"Tag {tag} not yet on origin (first push)")
    else:
        if local_exists:
            check_fail(f"Tag {tag} already exists (tags are immutable \u2014 choose a new version)")
            errors += 1
        else:
            check_ok(f"Tag {tag} does not exist yet")

    # 5. Working tree is clean
    porcelain = _git("status", "--porcelain")
    if porcelain:
        check_fail("Working tree has uncommitted changes")
        errors += 1
    else:
        check_ok("Working tree is clean")

    # 6. Agent definitions pass structural validation
    agents_run = run_validator("validate-agents", validate_agents)
    if agents_run.exit_code == 0:
        check_ok(f"Agent definitions: all {len(agent_files)} structurally valid")
    else:
        check_fail("Agent definitions: structural validation failed")
        print("    Run: uv run agov-validate-agents")
        errors += 1

    # 7. InfoSec screening passes
    infosec_run = run_validator("validate-infosec", validate_infosec)
    if infosec_run.exit_code == 0:
        check_ok("InfoSec screening: all clear")
    else:
        check_fail("InfoSec screening: unacknowledged findings")
        print("    Run: uv run agov-validate-infosec")
        errors += 1

    # 8. README stats consistency (warn only, don't block)
    readme_run = run_validator("validate-readme-stats", validate_readme_stats)
    if readme_run.exit_code == 0:
        check_ok("README stats: consistent with repo")
    else:
        check_warn("README stats may be stale (non-blocking)")
        print("    Run: uv run agov-validate-readme-stats")

    # 8b. README citation version matches major.minor
    import re
    readme = root / "README.md"
    if readme.exists():
        readme_text = readme.read_text(encoding="utf-8")
        m = re.search(r"Version\s+(\d+\.\d+)", readme_text)
        major_minor = ".".join(version.split(".")[:2])
        if m and m[1] == major_minor:
            check_ok(f"README citation: Version {m[1]}")
        elif m:
            check_fail(f"README citation: Version {m[1]} (expected {major_minor})")
            print("    Run: uv run agov-bump to fix")
            errors += 1
        else:
            check_warn("README.md: no citation version found")

    # 9. Self-conformance: does the framework repo follow its own ADOPTING.md?
    self_conformance_items = {
        "agents/ directory with definitions": (root / "agents").is_dir() and any((root / "agents").glob("*.yaml")),
        ".claude/rules/ directory with rules": (root / ".claude" / "rules").is_dir() and any((root / ".claude" / "rules").rglob("*.md")),
        "GOVERNANCE.md exists": (root / "GOVERNANCE.md").exists(),
        "tests/ directory with tests": (root / "tests").is_dir() and any((root / "tests").glob("test_*.py")),
        ".pre-commit-config.yaml exists": (root / ".pre-commit-config.yaml").exists(),
    }
    conformance_failures = [item for item, ok_flag in self_conformance_items.items() if not ok_flag]
    if conformance_failures:
        check_fail("Self-conformance: framework repo does not follow its own ADOPTING.md")
        for item in conformance_failures:
            print(f"    Missing: {item}")
        errors += 1
    else:
        check_ok(f"Self-conformance: {len(self_conformance_items)} ADOPTING.md steps verified")

    # 10. Generate communication graph as release evidence
    all_agent_data: dict[str, dict] = {}
    for agent_path in agent_files:
        data = load_yaml(agent_path)
        agent_section = data.get("agent", {})
        if isinstance(agent_section, dict) and "name" in agent_section:
            all_agent_data[str(agent_section["name"])] = data

    if len(all_agent_data) >= 2:
        comm_warnings = _check_communication_symmetry(all_agent_data)
        own_warnings = _check_ownership_overlaps(all_agent_data)

        docs_dir = root / "docs"
        docs_dir.mkdir(parents=True, exist_ok=True)

        # Generate rendered graph (SVG + PNG if dot available, .dot if not)
        graph_stem = docs_dir / f"agent-communication-{version}"
        svg_path = generate_graph(
            all_agent_data, graph_stem, fmt="svg",
            title=f"Agent Communication — v{version}",
            source_path=str(root / "agents"),
        )
        png_path = generate_graph(
            all_agent_data, graph_stem, fmt="png",
            title=f"Agent Communication — v{version}",
            source_path=str(root / "agents"),
        )

        # Also generate markdown summary with validation results
        md_path = docs_dir / f"agent-communication-{version}.md"
        md_content = (
            f"# Agent Communication Graph — v{version}\n\n"
            f"Generated by `pre-tag-check` at release time.\n\n"
            f"## Graph\n\n"
            f"![Agent Communication](agent-communication-{version}.svg)\n\n"
            f"## Validation\n\n"
            f"- **Agents:** {len(all_agent_data)}\n"
            f"- **Communication asymmetries:** {len(comm_warnings)}\n"
            f"- **Ownership overlaps:** {len(own_warnings)}\n"
        )
        if comm_warnings:
            md_content += "\n### Communication asymmetries\n\n"
            for w in comm_warnings:
                md_content += f"- {w}\n"
        if own_warnings:
            md_content += "\n### Ownership overlaps\n\n"
            for w in own_warnings:
                md_content += f"- {w}\n"

        md_path.write_text(md_content, encoding="utf-8")

        if has_dot_binary():
            check_ok(f"Communication graph: {svg_path.name}, {png_path.name}, {md_path.name}")
        else:
            check_warn(f"Communication graph: .dot source saved (install graphviz for SVG/PNG)")


    # Summary
    print()
    if errors > 0:
        print(f"\033[0;31m{errors} check(s) failed.\033[0m Fix before tagging.")
        return 1
    else:
        print(f"\033[0;32mAll checks passed.\033[0m Safe to tag:")
        print(f"  git tag {tag} && git push origin {tag}")
        return 0


_HELP = """\
pre-tag-check — Pre-release validation for agents-govern

Usage:
  uv run agov-pre-tag-check <tag>

Example:
  uv run agov-pre-tag-check v0.27.0

Checks before tagging a release:
  1. VERSION file matches the tag
  2. CHANGELOG.md has an entry for this version
  3. All agent definitions have matching framework_version
  4. Tag does not already exist (immutable tags)
  5. Working tree is clean
  6. Agent definitions pass structural validation
  7. InfoSec screening passes
  8. README stats are consistent
  9. Self-conformance: repo follows its own ADOPTING.md
 10. Generates communication graph (SVG + PNG) as release evidence

Requires: graphviz system binary for graph rendering.

Exit codes: 0 = safe to tag, 1 = fix issues first.

Full documentation: ADOPTING.md Step 7, CHANGELOG.md.
"""


def main() -> None:
    args = sys.argv[1:]
    if not args or args[0] in ("--help", "-h"):
        print(_HELP)
        return
    in_pre_push = False
    if "--in-pre-push" in args:
        in_pre_push = True
        args = [a for a in args if a != "--in-pre-push"]
    if len(args) != 1:
        print(f"Usage: {sys.argv[0]} [--in-pre-push] <tag>")
        print("Example: pre-tag-check v0.27.0")
        sys.exit(1)
    sys.exit(pre_tag_check(args[0], in_pre_push=in_pre_push))


if __name__ == "__main__":
    main()
