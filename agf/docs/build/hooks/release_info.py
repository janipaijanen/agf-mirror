# SPDX-License-Identifier: MIT
"""Generate build provenance for the public MkDocs site."""

from __future__ import annotations

import subprocess
from datetime import datetime, timezone
from pathlib import Path


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


def on_pre_build(config, **kwargs) -> None:
    config_path = Path(config.config_file_path).resolve()
    docs_build_dir = config_path.parent
    dist_root = docs_build_dir.parent.parent
    repo_root = _run(dist_root, "rev-parse", "--show-toplevel")
    repo_root_path = Path(repo_root) if repo_root != "unknown" else dist_root.parent

    version_file = dist_root / "VERSION"
    version = version_file.read_text(encoding="utf-8").strip()
    commit = _run(repo_root_path, "rev-parse", "HEAD")
    short_commit = _run(repo_root_path, "rev-parse", "--short=12", "HEAD")
    branch = _run(repo_root_path, "branch", "--show-current")
    timestamp = datetime.now(timezone.utc).replace(microsecond=0).isoformat()

    target = Path(config.docs_dir).resolve() / "reference" / "release-info.md"
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(
        f"""# Release information

*Status: Generated at documentation build time.*

## Version and source

| Field | Value |
|---|---|
| Framework content version | `{version}` |
| Source commit | `{commit}` |
| Short commit | `{short_commit}` |
| Source branch | `{branch or "unknown"}` |
| Build timestamp | `{timestamp}` |

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
