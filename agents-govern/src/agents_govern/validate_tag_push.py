# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Validate tag pushes via agov-pre-tag-check.

Designed to be invoked from a git pre-push hook (`.git-hooks/pre-push`).
Reads the standard git pre-push stdin format::

    <local_ref> <local_sha> <remote_ref> <remote_sha>

For each ref matching `refs/tags/vX.Y.Z`, runs `agov-pre-tag-check`
on the tag name. Aborts the push (exit 1) on any failure. Skips
non-tag refs and tag refs that don't match the semver pattern (e.g.,
release-candidate or pre-release tags) — the framework's release
convention is `vX.Y.Z` only.

This closes the gap documented in the v0.38.0 tag-drift incident
(see `reviews/2026-04-27_version-tag-drift-recurrence.md` and the
`learnings/iron-gate.yaml` entry "Version-tag drift recurs because
agov-pre-tag-check is opt-in, not gated"): pre-tag validation now
gates the push, not just an opt-in CLI step.
"""

from __future__ import annotations

import re
import subprocess
import sys
from collections.abc import Iterable

__all__ = ["TAG_RE", "extract_tags", "run_pre_tag_check", "main"]


TAG_RE = re.compile(r"^refs/tags/(v\d+\.\d+\.\d+)$")


def extract_tags(lines: Iterable[str]) -> list[str]:
    """Extract semver tag names from pre-push stdin lines.

    Each line is expected to be::

        <local_ref> <local_sha> <remote_ref> <remote_sha>

    Returns the list of tag names (e.g., "v0.41.0") for every line
    whose local_ref matches `refs/tags/vX.Y.Z`. Skips other refs and
    malformed lines silently.
    """
    tags: list[str] = []
    for raw in lines:
        parts = raw.strip().split()
        if not parts:
            continue
        local_ref = parts[0]
        m = TAG_RE.match(local_ref)
        if m:
            tags.append(m.group(1))
    return tags


def run_pre_tag_check(tag: str) -> bool:
    """Run `agov-pre-tag-check --in-pre-push <tag>` and return True on pass.

    The `--in-pre-push` flag tells pre-tag-check that the local tag
    is expected to exist (just created by `git tag`, about to be
    pushed). Without it, the existence check fails on the legitimate
    first-push of a freshly-created tag — the integration ordering
    bug observed in v0.41.0–v0.41.2 release work.

    Streams output to stderr so the user sees what failed when the
    push is aborted.
    """
    result = subprocess.run(
        ["uv", "run", "agov-pre-tag-check", "--in-pre-push", tag],
        check=False,
    )
    return result.returncode == 0


def main(argv: list[str] | None = None) -> int:
    """Entry point. Reads pre-push input from stdin; returns 0/1."""
    del argv  # unused; pre-push hooks don't pass args
    tags = extract_tags(sys.stdin)
    if not tags:
        return 0
    failed: list[str] = []
    for tag in tags:
        print(
            f"validate-tag-push: running agov-pre-tag-check {tag}",
            file=sys.stderr,
        )
        if not run_pre_tag_check(tag):
            failed.append(tag)
    if failed:
        print(
            f"\nvalidate-tag-push: pre-tag-check failed for: "
            f"{', '.join(failed)}",
            file=sys.stderr,
        )
        print(
            "Push aborted. Fix the discrepancy or use `git push --no-verify` "
            "to bypass (not recommended).",
            file=sys.stderr,
        )
        return 1
    return 0


if __name__ == "__main__":  # pragma: no cover
    sys.exit(main())
