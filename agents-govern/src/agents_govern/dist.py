# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Build distribution archive for downstream project adoption."""

from __future__ import annotations

import re
import shutil
import subprocess
import sys
import tarfile
import zipfile
from datetime import datetime, timezone
from pathlib import Path

from agents_govern._files import repo_root

__all__ = ["build_dist", "main"]

# Files and directories to copy into the distribution (immutable)
_COPY_FILES: tuple[str, ...] = (
    "VERSION",
    "framework.yaml",
    "framework.md",
    "ADOPTING.md",
    "GOVERNANCE.md",
    "LEARNINGS.md",
    "LESSONS-LEARNED.md",
    "README.md",
    "CONTRIBUTING.md",
    "CODE_OF_CONDUCT.md",
    "CHANGELOG.md",
    "SECURITY.md",
    "LICENSE",
    "DISCLAIMER.md",
    ".pre-commit-config.yaml",
)

_COPY_DIRS: tuple[str, ...] = (
    "rules-template",
    "skills",
    "project-specific-examples",
    "schemas",
)

_COPY_SCRIPT_FILES: tuple[str, ...] = (
    "scripts/generate-schemas.py",
)

_COPY_DOC_FILES: tuple[str, ...] = (
    "docs/binding-contract.md",
    "docs/integration-contract.md",
    "docs/contract-stability.md",
    "docs/adopt/compliance-profile.md",
)

_OPTIONAL_DIRS: tuple[str, ...] = (
    ".gitlab/issue_templates",
    ".gitlab/merge_request_templates",
    "docs/adr",
)


def _git(*args: str) -> str:
    """Run a git command and return stdout."""
    result = subprocess.run(
        ("git", *args),
        capture_output=True,
        text=True,
    )
    return result.stdout.strip()


def _human_size(path: Path) -> str:
    """Return human-readable file size."""
    size = path.stat().st_size
    for unit in ("B", "K", "M", "G"):
        if size < 1024:
            return f"{size:.0f}{unit}" if unit == "B" else f"{size:.1f}{unit}"
        size /= 1024
    return f"{size:.1f}T"


def _public_repo_url(url: str) -> str:
    """Return a public HTTPS repository URL for MANIFEST provenance."""
    url = url.strip()

    scp_like = re.match(r"^git@(?P<host>[^:]+):(?P<path>.+?)(?:\.git)?$", url)
    if scp_like:
        return f"https://{scp_like['host']}/{scp_like['path']}"

    ssh_url = re.match(r"^ssh://git@(?P<host>[^/]+)/(?P<path>.+?)(?:\.git)?$", url)
    if ssh_url:
        return f"https://{ssh_url['host']}/{ssh_url['path']}"

    url = re.sub(r"^(https://)[^/@]+@", r"\1", url)
    return re.sub(r"\.git$", "", url)


def build_dist() -> int:
    """Build the distribution archive. Returns exit code."""
    root = repo_root()

    # Read version
    version_file = root / "VERSION"
    if not version_file.exists():
        print("ERROR: VERSION file not found", file=sys.stderr)
        return 1
    version = version_file.read_text(encoding="utf-8").strip()
    if not version:
        print("ERROR: VERSION file is empty", file=sys.stderr)
        return 1

    commit = _git("rev-parse", "--short", "HEAD") or "unknown"
    commit_full = _git("rev-parse", "HEAD") or "unknown"
    repo_url = _git("remote", "get-url", "origin") or "https://gitlab.com/janipaijanen/agent-governance"
    repo_url = _public_repo_url(repo_url)
    date = datetime.now(timezone.utc).strftime("%Y-%m-%d")

    dist_name = f"agents-govern-{version}"
    dist_dir = root / "dist" / dist_name

    print("=== agents-govern distribution builder ===")
    print(f"Version:  {version}")
    print(f"Commit:   {commit_full}")
    print(f"Date:     {date}")
    print()

    # Clean previous build
    dist_parent = root / "dist"
    if dist_parent.exists():
        shutil.rmtree(dist_parent)
    dist_dir.mkdir(parents=True)

    # Copy single files
    for filename in _COPY_FILES:
        src = root / filename
        if src.exists():
            shutil.copy2(src, dist_dir / filename)

    # Copy directories
    for dirname in _COPY_DIRS:
        src = root / dirname
        if src.is_dir():
            shutil.copytree(src, dist_dir / dirname)

    # Targeted scripts used by shipped schema artefacts
    for filename in _COPY_SCRIPT_FILES:
        src = root / filename
        if src.exists():
            dst = dist_dir / filename
            dst.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(src, dst)

    # Agent definitions
    agents_dst = dist_dir / "agents"
    agents_dst.mkdir()
    for yaml_file in sorted((root / "agents").glob("*.yaml")):
        shutil.copy2(yaml_file, agents_dst / yaml_file.name)

    # Learnings template and adopter mapping
    learnings_dst = dist_dir / "learnings"
    learnings_dst.mkdir()
    template = root / "learnings" / "_template.yaml"
    if template.exists():
        shutil.copy2(template, learnings_dst / "_template.yaml")
    adopter_map = root / "learnings" / ".adopter-mapping.example"
    if adopter_map.exists():
        shutil.copy2(adopter_map, learnings_dst / ".adopter-mapping.example")

    # Docs
    docs_dst = dist_dir / "docs"
    docs_dst.mkdir(exist_ok=True)
    for filename in _COPY_DOC_FILES:
        src = root / filename
        if src.exists():
            dst = dist_dir / filename
            dst.parent.mkdir(parents=True, exist_ok=True)
            shutil.copy2(src, dst)
    anon_notice = root / "docs" / "ANONYMIZATION-NOTICE.md"
    if anon_notice.exists():
        shutil.copy2(anon_notice, docs_dst / "ANONYMIZATION-NOTICE.md")
    # Communication graphs (generated by pre-tag-check at release time)
    for graph_file in sorted((root / "docs").glob("agent-communication-*")):
        if graph_file.suffix in (".md", ".svg", ".png", ".pdf", ".dot"):
            shutil.copy2(graph_file, docs_dst / graph_file.name)

    # Python package and pyproject.toml for uv-based invocation
    # All CLI tools are invoked via `uv run agov-*` — no shell wrappers needed.
    pyproject_src = root / "pyproject.toml"
    if pyproject_src.exists():
        shutil.copy2(pyproject_src, dist_dir / "pyproject.toml")
    src_dir = root / "src"
    if src_dir.is_dir():
        shutil.copytree(
            src_dir,
            dist_dir / "src",
            ignore=shutil.ignore_patterns("__pycache__", "*.pyc"),
        )

    # Optional directories
    for dirname in _OPTIONAL_DIRS:
        src = root / dirname
        if src.is_dir():
            dst = dist_dir / dirname
            dst.mkdir(parents=True, exist_ok=True)
            for md_file in sorted(src.glob("*.md")):
                shutil.copy2(md_file, dst / md_file.name)

    # Generate MANIFEST.md
    manifest = dist_dir / "MANIFEST.md"
    manifest.write_text(
        f"""# agents-govern {version}

## Provenance

| Field | Value |
|-------|-------|
| Framework | agents-govern |
| Version | {version} |
| Commit | {commit_full} |
| Date | {date} |
| Repository | {repo_url} |
| License | CC-BY-SA-4.0 |

## Contents

This distribution contains the adoptable files from the agents-govern
framework. See [ADOPTING.md](ADOPTING.md) for the step-by-step adoption guide.

| Directory / File | Purpose | Action |
|------------------|---------|--------|
| `agents/` | Agent definitions (Phase 1 + Phase 3) | Copy to `governance/agents/`, customize |
| `rules-template/` | Governance rules | Copy to `.claude/rules/`, customize |
| `skills/` | AI assistant workflows (upgrade, learning, infosec) | Copy to `.claude/skills/`, customize (optional) |
| `project-specific-examples/` | Stack-specific rule examples | Copy if stack matches |
| `schemas/` | Strict-contract JSON Schemas and invariants | Use for validation and integration checks |
| `scripts/generate-schemas.py` | Schema regeneration helper | Use when maintaining schema artefacts |
| `learnings/_template.yaml` | Learning record template | Copy to `learnings/<codename>.yaml` |
| `learnings/.adopter-mapping.example` | Codename mapping format | Keep private (see anonymization policy) |
| `framework.yaml` | Machine-readable framework declaration | Copy and tune for adopter compliance profile |
| `framework.md` | Framework theory & reference | Read-only reference |
| `ADOPTING.md` | Adoption guide | Follow step by step |
| `GOVERNANCE.md` | Governance template | Basis for project-specific governance |
| `LEARNINGS.md` | Learnings guide & privacy policy | Read-only reference |
| `LESSONS-LEARNED.md` | Narrative lessons from adopters | Read-only reference |
| `README.md` | Overview and quick start | Read-only reference |
| `docs/binding-contract.md` | Agent-output binding contract | Read before enabling full compliance |
| `docs/integration-contract.md` | Validator and integration contract | Read before integrating tooling |
| `docs/contract-stability.md` | Contract versioning and stability promise | Read before upgrading |
| `docs/adopt/compliance-profile.md` | Compliance target definitions | Use to choose adoption depth |
| `docs/ANONYMIZATION-NOTICE.md` | Anonymization policy details | Read before submitting learnings |
| `docs/adr/` | Architecture decision records | Informational |
| `pyproject.toml` + `src/agents_govern/` | CLI tools (`uv run agov-*`) | Validation, bumping, health check |
| `.pre-commit-config.yaml` | Pre-commit hook config | Optional: `pre-commit install` |
| `CONTRIBUTING.md` | Contribution guidelines | Read before submitting |
| `CODE_OF_CONDUCT.md` | Community standards | Read-only reference |
| `.gitlab/issue_templates/` | Issue templates (Learning, Rule Proposal, Bug) | Use when filing issues |
| `LICENSE` | CC BY-SA 4.0 | Include in project |

## Quick start

```bash
# 1. Copy agent definitions to your project
mkdir -p governance/agents
cp agents/*.yaml governance/agents/

# 2. Copy governance rules
cp -r rules-template/ .claude/rules/

# 2b. (Optional) Copy AI assistant skills
cp -r skills/ .claude/skills/

# 3. Create your project learning record (use a codename, not the real name)
mkdir -p learnings
cp learnings/_template.yaml learnings/<codename>.yaml

# 4. Copy license
cp LICENSE .

# 5. Customize
#    - governance/agents/*.yaml — set jurisdiction to your domain
#    - .claude/rules/ — search for <!-- CUSTOMIZE --> comments
#    - learnings/<codename>.yaml — fill in project context and assessments

# 6. Write your GOVERNANCE.md (use the included one as a starting point)
cp GOVERNANCE.md .
#    Edit to match your project's roles, pipeline, and gate rules.

# 7. (Optional) Copy stack-specific examples if applicable
#    cp -r project-specific-examples/python-flask/ .claude/rules/
```

For the full 7-step adoption process, see [ADOPTING.md](ADOPTING.md).

## Upgrading

When upgrading from a previous version, record the upgrade in your
`learnings/<project>.yaml` adoption section:

```yaml
upgrade_history:
  - date: "{date}"
    from_version: "<previous>"
    to_version: "{version}"
    agent_changes: []
    notes: ""
```
""",
        encoding="utf-8",
    )

    # Apply .distignore exclusions
    distignore = root / ".distignore"
    if distignore.exists():
        excluded = 0
        for line in distignore.read_text(encoding="utf-8").splitlines():
            pattern = line.strip()
            if not pattern or pattern.startswith("#"):
                continue
            # Safety: reject patterns that could escape the dist directory
            if pattern.startswith("/") or ".." in pattern:
                print(f"  SKIPPED (unsafe pattern): {pattern}")
                continue
            try:
                matches = sorted(dist_dir.glob(pattern))
            except (NotImplementedError, ValueError):
                print(f"  SKIPPED (invalid pattern): {pattern}")
                continue
            for match in matches:
                # Verify the match is actually inside dist_dir (defense in depth)
                try:
                    match.resolve().relative_to(dist_dir.resolve())
                except ValueError:
                    print(f"  SKIPPED (outside dist): {match}")
                    continue
                rel = match.relative_to(dist_dir)
                if match.is_dir():
                    shutil.rmtree(match)
                elif match.is_symlink() or match.is_file():
                    match.unlink()
                print(f"  excluded: {rel}")
                excluded += 1
        if excluded > 0:
            print(f"Excluded {excluded} path(s) via .distignore")
        else:
            print(".distignore: no exclusions matched (file is empty or patterns are unused)")
        print()

    # Create archives
    tar_path = root / "dist" / f"{dist_name}.tar.gz"
    with tarfile.open(tar_path, "w:gz") as tar:
        tar.add(dist_dir, arcname=dist_name)

    zip_path = root / "dist" / f"{dist_name}.zip"
    with zipfile.ZipFile(zip_path, "w", zipfile.ZIP_DEFLATED) as zf:
        for file_path in sorted(dist_dir.rglob("*")):
            if file_path.is_file():
                arcname = str(Path(dist_name) / file_path.relative_to(dist_dir))
                zf.write(file_path, arcname)

    print("Distribution built:")
    print(f"  dist/{dist_name}/          (directory)")
    print(f"  dist/{dist_name}.tar.gz    ({_human_size(tar_path)})")
    print(f"  dist/{dist_name}.zip       ({_human_size(zip_path)})")
    print()
    print(f"MANIFEST: dist/{dist_name}/MANIFEST.md")

    # Check for stale remote branches
    print()
    branches_raw = _git("branch", "-r")
    stale = [
        line.strip().removeprefix("origin/")
        for line in branches_raw.splitlines()
        if line.strip()
        and not line.strip().endswith("origin/main")
        and "origin/HEAD" not in line
        and not line.strip() == "origin/main"
    ]
    if stale:
        print("\u26a0  STALE BRANCH WARNING \u2014 review before release:")
        print("   Stale branches may contain pre-anonymization data.")
        print()
        for branch in stale:
            print(f"   - origin/{branch}")
        print()
        print("   Delete with: git push origin --delete <branch>")
    else:
        print("\u2713  No stale remote branches found.")

    return 0


_HELP = """\
agov-dist — Build distribution archive for downstream adoption

Usage:
  uv run agov-dist

Builds a release archive containing only adoptable files:
  - Agent definitions (agents/*.yaml, excluding agents/local/)
  - Rule templates, skills, project-specific examples
  - Framework documentation (framework.md, ADOPTING.md, etc.)
  - Validation tools (Python package + shell wrappers)
  - Communication graphs (if generated by pre-tag-check)

Output:
  dist/<name>/          directory
  dist/<name>.tar.gz    compressed archive
  dist/<name>.zip       compressed archive
  dist/<name>/MANIFEST.md  provenance metadata

Applies .distignore patterns to exclude repo-only files.
Checks for stale remote branches that may contain sensitive data.

No arguments — reads VERSION from the repo root.

Exit codes: 0 = success, 1 = VERSION file missing.

Full documentation: ADOPTING.md "Getting started".
"""


def main() -> None:
    if len(sys.argv) > 1 and sys.argv[1] in ("--help", "-h"):
        print(_HELP)
        return
    sys.exit(build_dist())


if __name__ == "__main__":
    main()
