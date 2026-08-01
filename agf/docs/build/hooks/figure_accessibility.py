# SPDX-License-Identifier: MIT
"""Require accessible alt text and visible AI disclosure for public figures."""

from __future__ import annotations

import re
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[4]
PUBLIC_MARKDOWN_ROOT = REPO_ROOT / "agf/docs/site"
README = REPO_ROOT / "README.md"

IMAGE_RE = re.compile(
    r"!\[([^\]]*)\]\([^\n)]*\.(?:svg|png|jpe?g|webp)(?:\?[^\n)]*)?\)",
    re.IGNORECASE,
)
INLINE_CODE_RE = re.compile(r"`[^`]*`")
DISCLOSURE_RE = re.compile(
    r"AI-generated figure[.;]\s+Publication remains the responsibility "
    r"of the human maintainer\.",
    re.IGNORECASE,
)
GENERIC_ALT = {
    "adoption and exit flow",
    "coordination channel",
    "day-to-day responsibility modes",
    "diagram",
    "example agf foundry coordination map",
    "figure",
    "gate pipeline",
    "governance loop",
    "image",
    "learning loop",
}


def _image_matches(lines: list[str]):
    """Yield (line number, alt text) outside fenced and inline code."""
    fenced = False
    for index, line in enumerate(lines):
        stripped = line.lstrip()
        if stripped.startswith(("```", "~~~")):
            fenced = not fenced
            continue
        if fenced:
            continue
        visible = INLINE_CODE_RE.sub("", line)
        for match in IMAGE_RE.finditer(visible):
            yield index, match.group(1).strip()


def validate_figure_accessibility(paths: list[Path]) -> list[str]:
    """Return actionable failures for figure embeds in ``paths``."""
    failures: list[str] = []
    for path in paths:
        lines = path.read_text(encoding="utf-8").splitlines()
        for index, alt in _image_matches(lines):
            location = f"{path.relative_to(REPO_ROOT)}:{index + 1}"
            words = re.findall(r"[\w-]+", alt, flags=re.UNICODE)
            if len(words) < 5 or alt.casefold() in GENERIC_ALT:
                failures.append(
                    f"{location}: figure alt text must describe what the figure shows"
                )

            nearby = " ".join(lines[index : index + 6])
            if not DISCLOSURE_RE.search(nearby):
                failures.append(
                    f"{location}: add the visible AI-generated figure disclosure "
                    "within five lines of the embed"
                )
    return failures


def _public_markdown_paths() -> list[Path]:
    return [README, *sorted(PUBLIC_MARKDOWN_ROOT.rglob("*.md"))]


def on_pre_build(*, config, **kwargs):  # noqa: ANN001, ARG001
    """Fail the strict MkDocs build when a public figure breaks the policy."""
    failures = validate_figure_accessibility(_public_markdown_paths())
    if failures:
        details = "\n".join(f"- {failure}" for failure in failures)
        raise RuntimeError(f"Figure accessibility/disclosure check failed:\n{details}")
