# SPDX-License-Identifier: MIT
"""Generate build provenance for the public MkDocs site.

Two identities are derived and kept separately labeled (plan:
docs/plans/public-version-and-docs-provenance.md):

- **release provenance** — the latest immutable Git release tag (expected
  `v${agf/VERSION}`) and the exact commit that tag points at. Resolved from
  local Git tag refs only; CI jobs that publish documentation must make tag
  data explicit first (see `.gitlab-ci.yml`'s `pages`/`site-html-build` jobs)
  because a shallow checkout does not fetch tags by default.
- **docs build provenance** — the exact commit/branch this documentation was
  built from, which may be later than (or diverge from) the release commit.

`agf/VERSION` remains the only authored semantic-version scalar; the release
tag/commit are read from Git, never duplicated by hand.
"""

from __future__ import annotations

import os
import re
import subprocess
from datetime import datetime, timezone
from pathlib import Path

_VERSION_TAG_RE = re.compile(r"^v(\d+)\.(\d+)\.(\d+)$")


def _run(root: Path, *args: str) -> str:
    try:
        return subprocess.check_output(
            ["git", *args],
            cwd=root,
            text=True,
            stderr=subprocess.DEVNULL,
        ).strip()
    except (OSError, subprocess.CalledProcessError):
        return "unknown"


def _is_ancestor(root: Path, ancestor: str, descendant: str) -> bool | None:
    """Return True if `ancestor` is an ancestor of `descendant`, False if
    definitively not, or None if the relationship cannot be determined (e.g.
    a shallow checkout without enough history). Never guess.
    """
    try:
        result = subprocess.run(
            ["git", "merge-base", "--is-ancestor", ancestor, descendant],
            cwd=root,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
        )
    except OSError:
        return None
    if result.returncode == 0:
        return True
    if result.returncode == 1:
        return False
    return None


def _resolve_release(repo_root_path: Path, build_commit: str, version: str) -> dict[str, object]:
    """Resolve the latest `vX.Y.Z` release tag and its peeled commit.

    Returns `{"tag": "unknown", "commit": "unknown", "exact": False,
    "status": "unknown"}` when no version tag can be resolved (e.g. tags were
    never fetched into a shallow checkout) — this function never fabricates a
    release identity. If the highest resolvable tag does not match
    `v{agf/VERSION}`, the status explicitly flags the mismatch instead of
    silently presenting a possibly-contradictory release identity.
    """
    unresolved = {
        "tag": "unknown",
        "commit": "unknown",
        "short_commit": "unknown",
        "exact": False,
        "status": "unknown",
    }

    raw_tags = _run(repo_root_path, "tag", "--list", "v*")
    candidates = []
    for line in raw_tags.splitlines():
        line = line.strip()
        match = _VERSION_TAG_RE.match(line)
        if match:
            candidates.append((tuple(int(part) for part in match.groups()), line))

    if not candidates:
        return unresolved

    candidates.sort(reverse=True)
    tag = candidates[0][1]
    expected_tag = f"v{version}"

    release_commit = _run(repo_root_path, "rev-parse", f"{tag}^{{commit}}")
    if release_commit == "unknown":
        return {**unresolved, "tag": tag}

    release_short_commit = _run(repo_root_path, "rev-parse", "--short=12", release_commit)

    if tag != expected_tag:
        return {
            "tag": tag,
            "commit": release_commit,
            "short_commit": release_short_commit,
            "exact": False,
            "status": (
                f"release identity unconfirmed: latest resolvable tag {tag} "
                f"does not match agf/VERSION ({expected_tag})"
            ),
        }

    exact = build_commit != "unknown" and build_commit == release_commit
    if exact:
        status = "release documentation"
    else:
        relation = (
            _is_ancestor(repo_root_path, release_commit, build_commit)
            if build_commit != "unknown"
            else None
        )
        if relation is True:
            status = f"development documentation after {tag}"
        else:
            status = (
                f"documentation not confirmed to be after release {tag} "
                "(diverged history or ancestry unavailable)"
            )

    return {
        "tag": tag,
        "commit": release_commit,
        "short_commit": release_short_commit,
        "exact": exact,
        "status": status,
    }


def _resolve_branch(repo_root_path: Path) -> str:
    """Resolve the live docs-build branch name for the build provenance row.

    `git branch --show-current` is empty on any detached-HEAD checkout, which
    includes both tag pipelines *and* GitLab merge-request pipelines (an MR
    pipeline checks out a detached merge commit, not the source branch).
    Falls back to GitLab CI predefined variables in that order: a branch
    pipeline sets `CI_COMMIT_BRANCH`; a merge-request pipeline sets
    `CI_MERGE_REQUEST_SOURCE_BRANCH_NAME` (not `CI_COMMIT_BRANCH`); a tag
    pipeline sets `CI_COMMIT_TAG`. Never fabricates a branch name.
    """
    branch = _run(repo_root_path, "branch", "--show-current")
    if branch and branch != "unknown":
        return branch
    return (
        os.environ.get("CI_COMMIT_BRANCH")
        or os.environ.get("CI_MERGE_REQUEST_SOURCE_BRANCH_NAME")
        or os.environ.get("CI_COMMIT_TAG")
        or "unknown"
    )


def on_pre_build(config, **kwargs) -> None:
    config_path = Path(config.config_file_path).resolve()
    docs_build_dir = config_path.parent
    dist_root = docs_build_dir.parent.parent
    repo_root = _run(dist_root, "rev-parse", "--show-toplevel")
    repo_root_path = Path(repo_root) if repo_root != "unknown" else dist_root.parent

    version_file = dist_root / "VERSION"
    version = version_file.read_text(encoding="utf-8").strip()

    # Fall back to GitLab CI env vars when git is unavailable (python:*-slim has no git).
    commit = _run(repo_root_path, "rev-parse", "HEAD")
    if commit == "unknown":
        commit = os.environ.get("CI_COMMIT_SHA", "unknown")

    short_commit = _run(repo_root_path, "rev-parse", "--short=12", "HEAD")
    if short_commit == "unknown":
        short_commit = os.environ.get("CI_COMMIT_SHORT_SHA", "unknown")

    branch = _resolve_branch(repo_root_path)

    timestamp = datetime.now(timezone.utc).replace(microsecond=0).isoformat()

    release = _resolve_release(repo_root_path, commit, version)

    extra = dict(config.get("extra") or {})
    extra["agf_release"] = {
        "version": version,
        "commit": commit,
        "short_commit": short_commit,
        "branch": branch,
        "timestamp": timestamp,
        "release_tag": release["tag"],
        "release_commit": release["commit"],
        "release_short_commit": release["short_commit"],
        "release_exact": release["exact"],
        "release_status": release["status"],
    }
    config["extra"] = extra

    target = Path(config.docs_dir).resolve() / "reference" / "release-info.md"
    target.parent.mkdir(parents=True, exist_ok=True)
    bibtex = (
        "@software{paijanen2026agf,\n"
        "  author  = {Päijänen, Jani},\n"
        "  title   = {agents-govern},\n"
        f"  year    = {{2026}},\n"
        f"  version = {{{version}}},\n"
        "  url     = {https://gitlab.com/janipaijanen/agents-govern-framework},\n"
        "  note    = {Governance for AI-assisted software development},\n"
        "  license = {CC-BY-SA-4.0 and MIT},\n"
        "}"
    )
    release_tag = release["tag"]
    release_commit = release["commit"]
    release_short_commit = release["short_commit"]
    release_status = release["status"]
    release_source_line = (
        f"tag `{release_tag}`, commit `{release_short_commit}`"
        if release_commit != "unknown"
        else "unresolved (release tag data was not available at build time)"
    )

    target.write_text(
        f"""# Release information

*Status: Generated at documentation build time.*

## Version and source

| Field | Value |
|---|---|
| Framework content version | `{version}` |
| Published docs | <https://janipaijanen.gitlab.io/agents-govern-framework/> |
| Latest release | `{release_tag}` |
| Release source | {release_source_line} |
| Live docs build | branch `{branch}`, commit `{commit}` |
| Documentation status | {release_status} |
| Short commit | `{short_commit}` |
| Build timestamp | `{timestamp}` |

## Versioning note

`agf/VERSION` is the single authored semantic-version scalar. The **latest
release** row above is the immutable Git release tag and the exact commit it
points at — the release source of truth. The **live docs build** row is the
exact commit and branch this documentation page was generated from, which may
be later than (or diverge from) the release commit; the **documentation
status** row states which case applies. Pages do not carry their own
version-numbered lifecycle labels; where a page cites a version, it is an
explicit historical statement (for example, *"introduced in v2.1"*), not a
claim about the current release.

## Downloads

Release archives and platform-specific Rust tooling binaries are published on
the [project GitLab Releases page](https://gitlab.com/janipaijanen/agents-govern-framework/-/releases).

## Acknowledgement of AI assistance

AI assistants have contributed to this project and its releases, including
planning, drafting, implementation, testing, documentation, and review. Their
work is subject to the project's governed review process. Release records
identify the reviewed revisions, validation evidence, and human gate
decisions; those records support reconstructibility but do not transfer
responsibility to an AI system.

AI assistants do not approve gates or releases and do not hold project
authority. The human maintainer and governor, Jani Päijänen, retains approval
authority, authorizes publication, and takes responsibility for the published
content and release decisions.

## How to cite

If you use agents-govern in research or a published project, please cite it as:

**APA 7th edition:**

> Päijänen, J. (2026). *agents-govern: Governance for AI-assisted
> software development* (Version {version}) [Computer software].
> https://gitlab.com/janipaijanen/agents-govern-framework

**BibTeX:**

```bibtex
{bibtex}
```

## Copyright

Copyright (c) 2026 Jani Päijänen.

## Licenses

| Material | License |
|---|---|
| Framework content | Creative Commons Attribution-ShareAlike 4.0 International (`CC-BY-SA-4.0`), with separate commercial licensing available for alternative terms |
| Tooling code | MIT License |

The distribution includes the complete license texts as `LICENSE` and
`LICENSE-MIT`.
""",
        encoding="utf-8",
    )
