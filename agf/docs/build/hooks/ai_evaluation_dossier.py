# SPDX-License-Identifier: MIT
"""Append the full public docs corpus to the AI evaluation dossier page."""

from __future__ import annotations

import re
from pathlib import Path, PurePosixPath
from urllib.parse import urlsplit


DOSSIER_PAGE = "ai-evaluation-dossier.md"
APPENDIX_MARKER = "<!-- AGF_DOSSIER_APPENDIX -->"

PAGE_ORDER = [
    "index.md",
    "use-cases.md",
    "evaluate.md",
    "docs/index.md",
    "docs/start/should-you-adopt.md",
    "docs/start/adopter-faq.md",
    "docs/concepts/position.md",
    "docs/concepts/roles-and-gates.md",
    "docs/concepts/coordination-protocol.md",
    "docs/concepts/evidence-and-learnings.md",
    "docs/adopt/downstream-impact.md",
    "docs/adopt/install-and-layouts.md",
    "docs/adopt/assistant-loader-configuration.md",
    "docs/start/first-governed-change.md",
    "docs/adopt/upgrade-from-v1.md",
    "docs/adopt/tooling-and-tokens.md",
    "docs/adopt/exit-and-decommission.md",
    "docs/operate/day-to-day.md",
    "docs/operate/run-agf-in-real-situations.md",
    "docs/operate/review-and-release.md",
    "docs/advanced/contracts-and-validators.md",
    "docs/advanced/customizing-the-framework.md",
    "reference/environment-variables.md",
    "reference/release-artifacts.md",
    "reference/maturity-vocabulary.md",
    "reference/coord-graph-index.md",
    "reference/tooling-conformance.md",
    "reference/release-info.md",
    "reference/disclaimer.md",
    "reference/figures.md",
    "reference/palette.md",
    "contribute.md",
    "future/evolution.md",
]

EXCLUDE = {
    DOSSIER_PAGE,
    "content-plan.md",
}

MARKDOWN_LINK_RE = re.compile(r"(!?\[[^\]]*\]\()([^)\s]+)(\))")
HTML_HREF_RE = re.compile(r'(<a\s+[^>]*href=")([^"]+)(")')
IMAGE_EXTENSIONS = {
    ".apng",
    ".avif",
    ".gif",
    ".jpg",
    ".jpeg",
    ".png",
    ".svg",
    ".webp",
}


def _is_external_target(target: str) -> bool:
    parsed = urlsplit(target)
    return bool(parsed.scheme) or target.startswith("#") or target.startswith("<")


def _source_anchor(src_path: str) -> str:
    stem = src_path.removesuffix(".md").lower()
    return "dossier-source-" + re.sub(r"[^a-z0-9]+", "-", stem).strip("-")


def _split_target(target: str) -> tuple[str, str, str]:
    anchor = ""
    path_part = target
    if "#" in path_part:
        path_part, anchor = path_part.split("#", 1)
        anchor = f"#{anchor}"

    query = ""
    if "?" in path_part:
        path_part, query = path_part.split("?", 1)
        query = f"?{query}"

    return path_part, query, anchor


def _normalize_relative_path(path_part: str, source_path: PurePosixPath) -> str:
    resolved = source_path.parent.joinpath(path_part)
    normalized = PurePosixPath(*resolved.parts)
    parts = []
    for part in normalized.parts:
        if part == ".":
            continue
        if part == "..":
            if parts:
                parts.pop()
            continue
        parts.append(part)

    return str(PurePosixPath(*parts))


def _doc_target(normalized_path: str, page_anchor_by_path: dict[str, str]) -> str | None:
    candidates = [normalized_path]
    if normalized_path.endswith("/"):
        candidates.append(normalized_path.rstrip("/") + ".md")
    elif not PurePosixPath(normalized_path).suffix:
        candidates.append(normalized_path + ".md")

    for candidate in candidates:
        if candidate in page_anchor_by_path:
            return candidate
    return None


def _rewrite_target(
    target: str,
    source_path: PurePosixPath,
    page_anchor_by_path: dict[str, str],
    *,
    is_image: bool = False,
) -> str:
    if _is_external_target(target):
        return target

    path_part, query, anchor = _split_target(target)
    if not path_part:
        return target

    normalized_path = _normalize_relative_path(path_part, source_path)
    suffix = PurePosixPath(normalized_path).suffix.lower()
    if is_image or suffix in IMAGE_EXTENSIONS:
        return normalized_path + query + anchor

    doc_path = _doc_target(normalized_path, page_anchor_by_path)
    if doc_path is not None:
        # Keep links inside the dossier. Page-fragment links resolve to the
        # source page root because appended headings are demoted and may collide
        # with curated front-matter headings.
        return f"#{page_anchor_by_path[doc_path]}"

    return normalized_path + query + anchor


def _rewrite_links(
    markdown: str,
    source_path: PurePosixPath,
    page_anchor_by_path: dict[str, str],
) -> str:
    def replace(match: re.Match[str]) -> str:
        prefix, target, suffix = match.groups()
        return f"{prefix}{_rewrite_target(target, source_path, page_anchor_by_path, is_image=prefix.startswith('!'))}{suffix}"

    return MARKDOWN_LINK_RE.sub(replace, markdown)


def _rewrite_html_links(
    markdown: str,
    source_path: PurePosixPath,
    page_anchor_by_path: dict[str, str],
) -> str:
    def replace(match: re.Match[str]) -> str:
        prefix, target, suffix = match.groups()
        return f"{prefix}{_rewrite_target(target, source_path, page_anchor_by_path)}{suffix}"

    return HTML_HREF_RE.sub(replace, markdown)


def _demote_headings(markdown: str, levels: int = 2) -> str:
    output = []
    in_fence = False
    fence_marker = ""

    for line in markdown.splitlines():
        stripped = line.lstrip()
        if stripped.startswith(("```", "~~~")):
            marker = stripped[:3]
            if not in_fence:
                in_fence = True
                fence_marker = marker
            elif marker == fence_marker:
                in_fence = False
                fence_marker = ""
            output.append(line)
            continue

        if not in_fence and line.startswith("#"):
            hashes, rest = line.split(" ", 1) if " " in line else (line, "")
            if set(hashes) == {"#"} and len(hashes) <= 6:
                new_level = min(6, len(hashes) + levels)
                output.append("#" * new_level + (" " + rest if rest else ""))
                continue

        output.append(line)

    return "\n".join(output).rstrip()


def _all_markdown_pages(docs_dir: Path) -> list[str]:
    discovered = {
        path.relative_to(docs_dir).as_posix()
        for path in docs_dir.rglob("*.md")
        if path.relative_to(docs_dir).as_posix() not in EXCLUDE
    }
    ordered = [page for page in PAGE_ORDER if page in discovered]
    extras = sorted(discovered.difference(ordered))
    return ordered + extras


def _appendix_for_page(
    docs_dir: Path,
    src_path: str,
    index: int,
    page_anchor_by_path: dict[str, str],
) -> str:
    source = PurePosixPath(src_path)
    path = docs_dir / src_path
    markdown = path.read_text(encoding="utf-8").strip()
    markdown = _rewrite_links(markdown, source, page_anchor_by_path)
    markdown = _rewrite_html_links(markdown, source, page_anchor_by_path)
    markdown = _demote_headings(markdown)

    title = src_path
    for line in markdown.splitlines():
        if line.startswith("### "):
            title = line[4:].strip()
            break

    return f"""
<div class="agf-dossier-page-break"></div>

<span id="{page_anchor_by_path[src_path]}"></span>

## Source Flow 07.{index:02d} - {title}

<p class="agf-dossier-marker"><strong>Flow 07.{index:02d}</strong> / appended
full source page / original source: <code>{src_path}</code></p>

{markdown}
""".rstrip()


def on_page_markdown(markdown, page, config, files):
    if page.file.src_path != DOSSIER_PAGE:
        return markdown

    docs_dir = Path(config.docs_dir).resolve()
    pages = _all_markdown_pages(docs_dir)
    page_anchor_by_path = {src_path: _source_anchor(src_path) for src_path in pages}
    appendix_index = "\n".join(
        f"| 07.{index:02d} | [`{src_path}`](#{page_anchor_by_path[src_path]}) |"
        for index, src_path in enumerate(pages, start=1)
    )
    appendix = [
        f"""
<p class="agf-dossier-marker"><strong>Flow 07</strong> / generated at docs
build time / source: all public Markdown pages except this dossier and
<code>content-plan.md</code></p>

This appendix appends the public documentation corpus into this single HTML
page so an AI evaluator can be pointed at one URL. Each appended page keeps an
explicit source-path marker. Internal documentation links are rewritten to
point to source-page anchors inside this dossier. Image paths are rewritten so
they still resolve from the dossier page.

| Flow | Original source page |
|---|---|
{appendix_index}
"""
        .strip(),
    ]
    appendix.extend(
        _appendix_for_page(docs_dir, src_path, index, page_anchor_by_path)
        for index, src_path in enumerate(pages, start=1)
    )

    generated = "\n\n".join(appendix)
    markdown = _rewrite_links(markdown, PurePosixPath(DOSSIER_PAGE), page_anchor_by_path)
    markdown = _rewrite_html_links(markdown, PurePosixPath(DOSSIER_PAGE), page_anchor_by_path)
    if APPENDIX_MARKER in markdown:
        return markdown.replace(APPENDIX_MARKER, generated)
    return markdown.rstrip() + "\n\n" + generated + "\n"
