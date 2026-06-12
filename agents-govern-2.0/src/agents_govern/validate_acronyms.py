# SPDX-License-Identifier: MIT
# Copyright (c) 2026 Jani Päijänen
"""Validate that markdown acronyms are expanded on first use per file.

Enforces .claude/rules/docs/abbreviation-rule.md: every acronym in a
markdown file must be expanded once via `Phrase (ACR)` form or via a
markdown link to the glossary. Non-acronym tokens (filenames, RFC 2119
keywords, section markers, date placeholders, project-specific names)
are exempt; SPDX header lines, fenced code blocks, inline code, and
YAML frontmatter are skipped before extraction.
"""

from __future__ import annotations

import re
import sys
from dataclasses import dataclass
from pathlib import Path

from agents_govern._files import repo_root
from agents_govern._output import ok

__all__ = [
    "Violation",
    "extract_acronyms",
    "is_expanded",
    "check_file",
    "validate_acronyms",
    "main",
]


# Tokens that look like acronyms but aren't. The lint exempts these
# unconditionally. Add to this list only when reviewer discretion agrees
# a token is genuinely not an acronym (filename, marker, etc.).
_NON_ACRONYMS: frozenset[str] = frozenset({
    # Filenames and file conventions
    "README", "CHANGELOG", "LICENSE", "CONTRIBUTING", "MANIFEST",
    "NOTICE", "KEYS", "AUTHORS", "GOVERNANCE", "SECURITY",
    "ADOPTING", "DISCLAIMER", "LICENSING", "VERSION", "CITATION",
    # RFC 2119 normative keywords
    "MUST", "SHOULD", "MAY", "SHALL", "NOT", "REQUIRED", "RECOMMENDED",
    "OPTIONAL",
    # Tiny grammatical words that grep catches
    "AND", "OR", "THE", "WHO", "WHY", "HOW", "FOR", "ARE", "WAS",
    "HAS", "HAD", "BUT", "ITS", "OUR", "OWN", "OUT", "OFF", "OLD",
    "NEW", "NOW", "ONE", "TWO", "TEN", "ALL", "ANY", "USE", "WAY",
    "GET", "PUT", "RUN", "SEE", "SET", "TRY", "WHEN", "WHAT", "WHERE",
    "ALSO", "AGAIN", "MORE", "MOST", "LESS", "LEAST", "SAME", "STILL",
    # Section and convention markers
    "TODO", "FIXME", "NOTE", "WARNING", "CRITICAL", "DRAFT",
    "EOF", "BOF", "BREAKING", "BASELINE", "TESTING",
    "BEFORE", "AFTER", "AWAY", "ACT", "ADD", "CLOSED", "CHANGES",
    "CORRECT", "CROSS", "APPROVAL", "ACTIONS",
    # English verbs shouted in caps (often in agent prompt role text)
    "READ", "STOP", "OWNS", "OWN", "WRITE", "DELETE", "UPDATE",
    "CREATE", "MODIFY", "EXECUTE", "BLOCK", "MUST", "NEVER",
    # English emphasis / section markers (caught false-positive in v1.1.5 sweep)
    "FEEDBACK", "LOOPS", "START", "HERE", "UNTESTED", "WARN",
    # Conformance-audit status labels used in ADOPTING.md
    "GAP", "GAPS", "PARTIAL", "PARTIALS", "CONFORMANT",
    # Git ref names rendered in caps in prose (always treat as code-token)
    "HEAD",
    # Date and template placeholders
    "YYYY", "YYYYMMDD", "YEAR", "MONTH", "DATE", "TIME",
    # Project-specific names rendered in caps for emphasis (defined as
    # terms elsewhere in the framework's own docs/agents)
    "BRIEF", "WATCH", "BLOCK", "SKIP", "RAPID", "DARE",
    "AGENTS", "CLAUDE", "SKILL", "SKILLS", "REVIEWER", "AUDITOR",
    "GOVERNOR", "ARCHITECT", "PLANNER", "CODER", "TESTER",
    "BUILD", "RELEASE", "ENGINEER", "SENTINEL", "GUARDIAN",
    # Common English words shouted in caps that clear naive grep
    "WORKFLOW", "WORKFLOWS", "LEARNED", "LESSONS", "NEVER", "ALWAYS",
    "PARTIAL", "FULL", "EMPTY", "MISSING", "PRESENT", "ABSENT",
    "ARTICLE", "SECTION", "CHAPTER", "PAGE", "PAGES",
    "CONTEXT", "PURPOSE", "REASON", "BECAUSE",
    "FROM", "INTO", "ONTO", "OVER", "UNDER", "ACROSS", "BETWEEN",
    "WITHIN", "WITHOUT", "AGAINST", "TOWARD", "BEYOND",
    "ENABLE", "DISABLE", "ENABLED", "DISABLED",
    # Self-referential — this lint's own vocabulary
    "ABBREVIATIONS", "ACRONYMS", "EXPANSION", "ALLOWLIST",
})


# Acronym matcher: 3-8 chars, all uppercase, word-boundary-delimited.
# 2-char acronyms are excluded as too noisy (BY/SA/NC license codes,
# OS/DB/AI ambiguity, often false positives).
_ACRONYM_RE = re.compile(r"\b[A-Z]{3,8}\b")

# SPDX-License-Identifier header line (with or without HTML-comment wrap).
_SPDX_HEADER_RE = re.compile(r"SPDX-License-Identifier\s*:")

# Fenced code block markers — triple backtick or triple tilde.
_FENCE_RE = re.compile(r"^\s*(```|~~~)")

# Inline code spans within a single line.
_INLINE_CODE_RE = re.compile(r"`[^`\n]+`")

# YAML frontmatter delimiters.
_FRONTMATTER_RE = re.compile(r"^---\s*$")


def _strip_non_prose(text: str) -> str:
    """Remove SPDX headers, fenced code blocks, frontmatter, inline code.

    Returns text where non-prose regions are blanked out (replaced with
    blank lines) so that line numbers in the result still align with
    line numbers in the original.
    """
    lines = text.splitlines()
    out: list[str] = []
    in_fence = False
    in_frontmatter = False

    # Detect frontmatter only if first non-blank line is `---`.
    if lines and lines[0].strip() == "---":
        in_frontmatter = True
        out.append("")  # blank out opening delimiter
        for i in range(1, len(lines)):
            if lines[i].strip() == "---":
                out.append("")
                in_frontmatter = False
                # Continue processing the rest of the file from i+1
                start = i + 1
                break
            out.append("")
        else:
            # Unterminated frontmatter — treat the whole file as
            # frontmatter (degenerate case).
            return "\n".join("" for _ in lines)
        # Process remaining lines after frontmatter
        rest = lines[start:]
    else:
        rest = lines

    for line in rest:
        if _FENCE_RE.match(line):
            in_fence = not in_fence
            out.append("")  # blank fence marker line
            continue
        if in_fence:
            out.append("")
            continue
        if _SPDX_HEADER_RE.search(line):
            out.append("")
            continue
        # Strip inline code spans in prose lines.
        cleaned = _INLINE_CODE_RE.sub("", line)
        out.append(cleaned)

    return "\n".join(out)


def extract_acronyms(text: str) -> set[str]:
    """Return the set of acronym candidates found in `text`.

    Filters out non-prose regions, then matches `[A-Z]{3,8}`, then
    drops tokens in the non-acronym allowlist.
    """
    prose = _strip_non_prose(text)
    return {
        m.group(0)
        for m in _ACRONYM_RE.finditer(prose)
        if m.group(0) not in _NON_ACRONYMS
    }


def is_expanded(acronym: str, text: str) -> bool:
    """Return True if `acronym` is expanded somewhere in `text`.

    Recognises:
      1. Parenthetical form: `(ACR)`. Matches `Some Phrase (ACR)` etc.
      2. Markdown link with target containing 'glossary':
         `[ACR](anything-with-glossary-anywhere)`.
    """
    # Form 1: any "(ACR)" occurrence
    if re.search(rf"\({re.escape(acronym)}\)", text):
        return True
    # Form 2: markdown link where the target URL/anchor mentions 'glossary'
    link_pattern = rf"\[{re.escape(acronym)}\]\(([^)]*glossary[^)]*)\)"
    if re.search(link_pattern, text, re.IGNORECASE):
        return True
    return False


@dataclass(frozen=True, slots=True)
class Violation:
    path: Path
    line: int
    acronym: str


def check_file(path: Path) -> tuple[Violation, ...]:
    """Return a tuple of violations for `path`.

    A violation is one unexpanded acronym, reported at the line of its
    first occurrence in the file.
    """
    text = path.read_text(encoding="utf-8")
    prose = _strip_non_prose(text)

    candidates = extract_acronyms(text)
    if not candidates:
        return ()

    violations: list[Violation] = []
    for acronym in sorted(candidates):
        if is_expanded(acronym, text):
            continue
        # Find first-occurrence line in the prose-stripped text so that
        # SPDX-header occurrences don't falsely set line=1.
        first_line = _first_occurrence_line(acronym, prose)
        if first_line is None:
            # Shouldn't happen — the candidate was extracted from prose.
            continue
        violations.append(
            Violation(path=path, line=first_line, acronym=acronym)
        )
    return tuple(violations)


def _first_occurrence_line(acronym: str, prose: str) -> int | None:
    pattern = re.compile(rf"\b{re.escape(acronym)}\b")
    for i, line in enumerate(prose.splitlines(), start=1):
        if pattern.search(line):
            return i
    return None


# Directories to skip during repo walk: build outputs, vendored deps,
# generated content, and version control.
_SKIP_DIRS: frozenset[str] = frozenset({
    ".git", ".venv", "venv", "node_modules", "dist", "build",
    "public", "site", "_gen", "__pycache__", ".pytest_cache",
    ".mypy_cache", ".ruff_cache",
})

# Files exempt from the acronym lint. Reserved for append-only
# historical artefacts where the boy-scout rule does not apply, or
# for the definitional source files the lint relies on.
_EXEMPT_FILES: frozenset[str] = frozenset({
    "CHANGELOG.md",         # append-only release notes
    "LESSONS-LEARNED.md",   # append-only lessons log
    "abbreviations.md",     # definitional source for site-wide
    #                         hover tooltips — uses `*[TERM]: Definition`
    #                         syntax which the lint does not recognise
    #                         as expansion form
})


def _walk_markdown(root: Path) -> list[Path]:
    """Yield markdown files under root, skipping vendored/generated dirs."""
    found: list[Path] = []
    for path in root.rglob("*.md"):
        if any(part in _SKIP_DIRS for part in path.relative_to(root).parts):
            continue
        if path.name in _EXEMPT_FILES:
            continue
        found.append(path)
    return sorted(found)


def validate_acronyms(
    root: Path | None = None,
    files: tuple[Path, ...] | None = None,
) -> int:
    """Validate markdown files for unexpanded acronyms.

    If `files` is given, lint only those files; otherwise walk `root`
    (or the repo root) for every *.md file. The CI pipeline passes
    only files changed in the merge request, grandfathering pre-existing
    violations until those files are next touched.

    Returns 0 if all files conform, 1 if any violations found.
    """
    root = root or repo_root()
    if files is None:
        files = tuple(_walk_markdown(root))

    all_violations: list[Violation] = []
    for f in files:
        if not f.exists() or f.suffix != ".md":
            continue
        if f.name in _EXEMPT_FILES:
            continue
        all_violations.extend(check_file(f))

    if not all_violations:
        ok(f"All {len(files)} markdown files expand acronyms on first use.")
        return 0

    # Group by file for readable output.
    by_file: dict[Path, list[Violation]] = {}
    for v in all_violations:
        by_file.setdefault(v.path, []).append(v)

    print(f"\033[0;31mFAIL\033[0m {len(all_violations)} unexpanded acronym(s) "
          f"in {len(by_file)} file(s):")
    print()
    for path in sorted(by_file):
        rel = path.relative_to(root) if path.is_absolute() else path
        print(f"  {rel}")
        for v in sorted(by_file[path], key=lambda v: (v.line, v.acronym)):
            print(f"    line {v.line}: {v.acronym}")
        print()
    print("Fix: expand on first use as `Phrase (ACR)` or link to glossary.")
    print("See .claude/rules/docs/abbreviation-rule.md for the policy.")
    return 1


_HELP = """\
validate-acronyms — Lint markdown files for unexpanded acronyms

Usage:
  uv run agov-validate-acronyms                 # walk every *.md in the repo
  uv run agov-validate-acronyms PATH [PATH ...] # lint only the given files

Flags acronyms that are used without being expanded on first
occurrence (per file). With no arguments, walks every *.md in the
repository (use this to drain the backlog). With explicit file
arguments, lints only those — the merge-request CI job passes the
list of *.md files changed in the MR, grandfathering pre-existing
violations until those files are next touched.

An acronym is "expanded" when the file contains either:
  - A parenthetical form anywhere: `Phrase (ACR)`
  - A markdown link to a glossary entry: `[ACR](.../glossary.md#acr)`

Non-acronym tokens (filenames, RFC 2119 keywords, section markers,
date placeholders, project-specific names rendered in caps) are
exempt. SPDX-License-Identifier headers, fenced code blocks, inline
code, and YAML frontmatter are skipped before extraction.

Policy: .claude/rules/docs/abbreviation-rule.md
Source of truth for non-acronym list: this module's _NON_ACRONYMS set.

Exit codes: 0 = clean, 1 = violations found.
"""


def main() -> None:
    args = sys.argv[1:]
    if args and args[0] in ("--help", "-h"):
        print(_HELP)
        return
    files: tuple[Path, ...] | None = None
    if args:
        files = tuple(Path(a) for a in args)
    sys.exit(validate_acronyms(files=files))


if __name__ == "__main__":
    main()
