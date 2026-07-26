# SPDX-License-Identifier: MIT
"""Append the full public docs corpus to the AI evaluation dossier page."""

from __future__ import annotations

import re
from pathlib import Path, PurePosixPath
from urllib.parse import urlsplit


DOSSIER_PAGE = "ai-evaluation-dossier.md"
APPENDIX_MARKER = "<!-- AGF_DOSSIER_APPENDIX -->"
REPOSITORY_REFERENCES_MARKER = "<!-- AGF_DOSSIER_REPOSITORY_REFERENCES -->"
RELEASE_TUPLE_MARKER = "<!-- AGF_DOSSIER_RELEASE_TUPLE -->"
REPOSITORY_URL = "https://gitlab.com/janipaijanen/agents-govern-framework"
DOSSIER_TOP_ANCHOR = "#ai-evaluation-dossier"

REPOSITORY_REFERENCES = [
    ("Active role authority", "agf/agents/MANIFEST.yaml", False, "Which foundry roles are active, suspended, or skipped"),
    ("Role contracts", "agf/agents/", True, "Role jurisdiction and restrictions"),
    ("Binding rule library", "agf/rules/", True, "Named behavioral obligations and tool authority"),
    ("Schemas", "agf/schemas/", True, "Machine-readable artifact shapes only"),
    ("Gate 1 plans", "docs/plans/", True, "Approved intent and implementation boundaries"),
    ("Coordination trail", "docs/coordination/", True, "Claims, reviews, handoffs, and binding decisions"),
    ("Active learning register", "agf/learnings/foundry.yaml", False, "Project learning evidence and recommendations"),
    ("Supported tooling source", "tooling/rust/", True, "Implemented CLI and MCP behavior, subject to tests"),
    ("CI configuration", ".gitlab-ci.yml", False, "Checks and event sources configured for this repository"),
    ("Release manifest", "agf/dist/release-manifest.yaml", False, "Files included in the framework distribution"),
]

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

LINKED_IMAGE_RE = re.compile(r"(\[!\[[^\]]*\]\([^)]+\)\]\()([^)\s]+)(\))")
MARKDOWN_LINK_RE = re.compile(r"(!?\[[^\]]*\]\()([^)\s]+)(\))")
HTML_HREF_RE = re.compile(r'(<a\s+[^>]*href=")([^"]+)(")')
HEADING_RE = re.compile(r"^(#{1,6})\s+(.+?)\s*$")
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


def _section_anchor(page_anchor: str, section_anchor: str) -> str:
    return f"{page_anchor}--{section_anchor}"


def _slugify_heading(heading: str) -> str:
    heading = re.sub(r"<[^>]+>", "", heading)
    heading = re.sub(r"\[([^\]]+)\]\([^)]+\)", r"\1", heading)
    heading = heading.replace("`", "").replace("*", "").replace("_", "")
    heading = heading.lower()
    heading = re.sub(r"[^a-z0-9]+", "-", heading)
    return heading.strip("-")


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
    if normalized_path == DOSSIER_PAGE:
        return anchor or DOSSIER_TOP_ANCHOR

    suffix = PurePosixPath(normalized_path).suffix.lower()
    if is_image or suffix in IMAGE_EXTENSIONS:
        return normalized_path + query + anchor

    doc_path = _doc_target(normalized_path, page_anchor_by_path)
    if doc_path is not None:
        page_anchor = page_anchor_by_path[doc_path]
        if anchor:
            return f"#{_section_anchor(page_anchor, anchor.removeprefix('#'))}"
        return f"#{page_anchor}"

    return normalized_path + query + anchor


def _repository_reference_table(config) -> str:
    """Build link-only repository references for the exact docs source commit."""
    release = (config.get("extra") or {}).get("agf_release") or {}
    commit = str(release.get("commit") or "unknown")
    pinned = re.fullmatch(r"[0-9a-fA-F]{40}", commit) is not None

    if pinned:
        status = (
            f"Repository links below are pinned to this documentation build's "
            f"exact source commit `{commit}`."
        )
    else:
        status = (
            "The exact source commit is unresolved/unpinned. Repository paths "
            "fall back to the canonical repository root; do not treat them as "
            "exact-build evidence."
        )

    rows = []
    for category, path, is_directory, claim_boundary in REPOSITORY_REFERENCES:
        if pinned:
            route = "tree" if is_directory else "blob"
            url = f"{REPOSITORY_URL}/-/{route}/{commit}/{path.rstrip('/')}"
        else:
            url = REPOSITORY_URL
        rows.append(
            f"| {category} | [`{path}`]({url}) | {claim_boundary} | Record whether opened |"
        )

    rows.append(
        "| Published releases | "
        f"[GitLab Releases]({REPOSITORY_URL}/-/releases) | "
        "Currently downloadable release artifacts | Record release and files opened |"
    )

    return "\n".join(
        [
            status,
            "",
            "| Evidence category | Authoritative route | Claim boundary | Review record |",
            "|---|---|---|---|",
            *rows,
        ]
    )


def _release_tuple_table(config) -> str:
    """Embed the release/docs-build provenance tuple directly in the dossier.

    `docs/plans/public-version-and-docs-provenance.md` requires the dossier
    to embed this tuple rather than only link to `reference/release-info.md`.
    """
    release = (config.get("extra") or {}).get("agf_release") or {}
    version = str(release.get("version") or "unknown")
    release_tag = str(release.get("release_tag") or "unknown")
    release_commit = str(release.get("release_commit") or "unknown")
    build_branch = str(release.get("branch") or "unknown")
    build_commit = str(release.get("commit") or "unknown")
    status = str(release.get("release_status") or "unknown")

    return "\n".join(
        [
            "| Field | Value |",
            "|---|---|",
            f"| Framework content version (`agf/VERSION`) | `{version}` |",
            f"| Latest release tag | `{release_tag}` |",
            f"| Release source commit | `{release_commit}` |",
            f"| Live docs build | branch `{build_branch}`, commit `{build_commit}` |",
            f"| Documentation status | {status} |",
        ]
    )


def _rewrite_links(
    markdown: str,
    source_path: PurePosixPath,
    page_anchor_by_path: dict[str, str],
) -> str:
    def replace(match: re.Match[str]) -> str:
        prefix, target, suffix = match.groups()
        return f"{prefix}{_rewrite_target(target, source_path, page_anchor_by_path, is_image=prefix.startswith('!'))}{suffix}"

    markdown = LINKED_IMAGE_RE.sub(replace, markdown)
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


def _demote_headings(
    markdown: str,
    *,
    page_anchor: str = "",
    section_anchors: set[str] | None = None,
    levels: int = 2,
) -> str:
    output = []
    in_fence = False
    fence_marker = ""
    section_anchors = section_anchors or set()

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

        if not in_fence:
            heading_match = HEADING_RE.match(line)
            if heading_match:
                hashes, rest = heading_match.groups()
                slug = _slugify_heading(rest)
                if page_anchor and slug in section_anchors:
                    output.append(f'<span id="{_section_anchor(page_anchor, slug)}"></span>')
                new_level = min(6, len(hashes) + levels)
                output.append("#" * new_level + " " + rest)
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


def _section_anchor_map(
    docs_dir: Path,
    pages: list[str],
    page_anchor_by_path: dict[str, str],
) -> dict[str, set[str]]:
    section_anchors = {src_path: set() for src_path in pages}
    for src_path in [*pages, DOSSIER_PAGE]:
        path = docs_dir / src_path
        if not path.exists():
            continue
        source = PurePosixPath(src_path)
        text = path.read_text(encoding="utf-8")
        for pattern in (LINKED_IMAGE_RE, MARKDOWN_LINK_RE, HTML_HREF_RE):
            for match in pattern.finditer(text):
                target = match.group(2)
                if _is_external_target(target):
                    continue
                path_part, _query, anchor = _split_target(target)
                if not path_part or not anchor:
                    continue
                normalized_path = _normalize_relative_path(path_part, source)
                doc_path = _doc_target(normalized_path, page_anchor_by_path)
                if doc_path is not None:
                    section_anchors.setdefault(doc_path, set()).add(anchor.removeprefix("#"))
    return section_anchors


def _appendix_for_page(
    docs_dir: Path,
    src_path: str,
    index: int,
    page_anchor_by_path: dict[str, str],
    section_anchor_by_path: dict[str, set[str]],
) -> str:
    source = PurePosixPath(src_path)
    path = docs_dir / src_path
    markdown = path.read_text(encoding="utf-8").strip()
    markdown = _rewrite_links(markdown, source, page_anchor_by_path)
    markdown = _rewrite_html_links(markdown, source, page_anchor_by_path)
    markdown = _demote_headings(
        markdown,
        page_anchor=page_anchor_by_path[src_path],
        section_anchors=section_anchor_by_path.get(src_path, set()),
    )

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

    if REPOSITORY_REFERENCES_MARKER not in markdown:
        raise ValueError(
            f"{DOSSIER_PAGE} is missing {REPOSITORY_REFERENCES_MARKER}"
        )
    markdown = markdown.replace(
        REPOSITORY_REFERENCES_MARKER,
        _repository_reference_table(config),
    )

    if RELEASE_TUPLE_MARKER not in markdown:
        raise ValueError(f"{DOSSIER_PAGE} is missing {RELEASE_TUPLE_MARKER}")
    markdown = markdown.replace(
        RELEASE_TUPLE_MARKER,
        _release_tuple_table(config),
    )

    docs_dir = Path(config.docs_dir).resolve()
    pages = _all_markdown_pages(docs_dir)
    page_anchor_by_path = {src_path: _source_anchor(src_path) for src_path in pages}
    section_anchor_by_path = _section_anchor_map(docs_dir, pages, page_anchor_by_path)
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
        _appendix_for_page(
            docs_dir,
            src_path,
            index,
            page_anchor_by_path,
            section_anchor_by_path,
        )
        for index, src_path in enumerate(pages, start=1)
    )

    generated = "\n\n".join(appendix)
    markdown = _rewrite_links(markdown, PurePosixPath(DOSSIER_PAGE), page_anchor_by_path)
    markdown = _rewrite_html_links(markdown, PurePosixPath(DOSSIER_PAGE), page_anchor_by_path)
    if APPENDIX_MARKER in markdown:
        return markdown.replace(APPENDIX_MARKER, generated)
    return markdown.rstrip() + "\n\n" + generated + "\n"
