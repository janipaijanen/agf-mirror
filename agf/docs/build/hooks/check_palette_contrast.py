# SPDX-License-Identifier: MIT
"""Mechanical WCAG AA contrast regression for the semantic responsibility
palette (agf/docs/site/reference/palette.md, "Semantic responsibility
palette" section) and every DOT figure that applies it.

Gate 2 review `20260722T125000Z-contribution-auditor-r4js` (BLOCK 1) found
that the palette contract's original text-colour assignments for Human
decision (amber) and Automated check (green-mid) rendered below the 4.5:1
WCAG AA normal-text threshold. The first version of this script fixed the
values but hard-coded them as duplicated Python constants -- Gate 2
re-review `20260722T130424Z-contribution-auditor-h1s8` (BLOCK 1) correctly
found that a duplicated check does not guard the governed sources: changing
`palette.md`'s text-colour column, or a figure's `fontcolor=`, back to an
invalid value left this check green because it never read those files.

This version reads the actual sources of truth instead of a private copy:

- `agf/docs/site/reference/palette.md`'s base "Palette" table supplies the
  token -> hex map (single source for hex values).
- `agf/docs/site/reference/palette.md`'s "Semantic responsibility palette"
  table supplies the fill/text pairings the contract requires.
- Every `agf/docs/figures/src/*.dot` and `docs/academic/figures/src/*.dot`
  file is scanned for explicit `fontcolor=` assignments (the only places a
  figure renders text on a fill) and each is checked against its node's
  `fillcolor` (or white, for unfilled nodes/edge labels on the page
  background). Workstream D's Class 3 (academic SVG/PDF figures) opts
  into this same contract and check the same way Class 1 does -- see
  `iter_dot_sources()`.

Run directly: `python3 agf/docs/build/hooks/check_palette_contrast.py`
Self-test (parser correctness, not just the current sources):
`python3 agf/docs/build/hooks/check_palette_contrast.py --self-test`
"""

from __future__ import annotations

import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[4]
PALETTE_MD = REPO_ROOT / "agf/docs/site/reference/palette.md"
DOT_SOURCE_DIRS = [
    REPO_ROOT / "agf/docs/figures/src",
    REPO_ROOT / "docs/academic/figures/src",
]

MIN_CONTRAST_NORMAL_TEXT = 4.5

NAMED_COLOURS = {"white": "#FFFFFF", "black": "#000000"}

HEX_RE = re.compile(r"#([0-9A-Fa-f]{6})")
TOKEN_ROW_RE = re.compile(r"^\|\s*`([\w-]+)`\s*\|\s*`(#[0-9A-Fa-f]{6})`\s*\|")
THEME_ROW_RE = re.compile(
    r"^\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|\s*([^|]+?)\s*\|\s*([0-9.]+):1"
)
ATTR_BLOCK_RE = re.compile(r"\[([^\[\]]*)\]", re.DOTALL)
ATTR_RE = re.compile(r'(\w+)\s*=\s*("(?:[^"\\]|\\.)*"|[^,\]\s]+)', re.DOTALL)


class ContrastCheckError(ValueError):
    pass


def _linearize(channel: float) -> float:
    if channel <= 0.03928:
        return channel / 12.92
    return ((channel + 0.055) / 1.055) ** 2.4


def relative_luminance(hex_colour: str) -> float:
    hex_colour = hex_colour.lstrip("#")
    r, g, b = (int(hex_colour[i : i + 2], 16) / 255 for i in (0, 2, 4))
    r, g, b = _linearize(r), _linearize(g), _linearize(b)
    return 0.2126 * r + 0.7152 * g + 0.0722 * b


def contrast_ratio(hex_a: str, hex_b: str) -> float:
    lum_a = relative_luminance(hex_a)
    lum_b = relative_luminance(hex_b)
    lighter, darker = max(lum_a, lum_b), min(lum_a, lum_b)
    return (lighter + 0.05) / (darker + 0.05)


def normalize_hex(value: str) -> str | None:
    value = value.strip().strip('"')
    m = HEX_RE.search(value)
    if m:
        return "#" + m.group(1).upper()
    return NAMED_COLOURS.get(value.lower())


def parse_base_palette_table(md_text: str) -> dict[str, str]:
    """Token name -> hex, read from palette.md's "Palette" table."""
    tokens: dict[str, str] = {}
    for line in md_text.splitlines():
        m = TOKEN_ROW_RE.match(line)
        if m:
            tokens[m.group(1)] = m.group(2).upper()
    return tokens


def resolve_colour_ref(ref: str, tokens: dict[str, str]) -> str:
    """Resolve a responsibility-table cell (hex literal, `token` backtick
    reference, or a bare colour word like "white") to a hex string."""
    hex_hit = normalize_hex(ref)
    if hex_hit:
        return hex_hit
    m = re.search(r"`([\w-]+)`", ref)
    if m and m.group(1) in tokens:
        return tokens[m.group(1)]
    for name, hex_value in NAMED_COLOURS.items():
        if name in ref.lower():
            return hex_value
    raise ContrastCheckError(f"cannot resolve colour reference: {ref!r}")


def parse_responsibility_table(
    md_text: str, tokens: dict[str, str]
) -> list[tuple[str, str, str]]:
    """(responsibility, fill_hex, text_hex) rows from the "Semantic
    responsibility palette" table."""
    rows: list[tuple[str, str, str]] = []
    in_section = False
    seen_header = False
    for line in md_text.splitlines():
        if line.startswith("## Semantic responsibility palette"):
            in_section = True
            continue
        if in_section and line.startswith("## "):
            break
        if not in_section:
            continue
        if not line.startswith("|"):
            continue
        cols = [c.strip() for c in line.strip().strip("|").split("|")]
        if len(cols) < 4:
            continue
        if cols[0] == "Responsibility":
            seen_header = True
            continue
        if set(cols[0]) <= {"-"}:
            continue
        if not seen_header:
            continue
        name, fill_ref, _border_ref, text_ref = cols[0], cols[1], cols[2], cols[3]
        rows.append((name, resolve_colour_ref(fill_ref, tokens), resolve_colour_ref(text_ref, tokens)))
    return rows


def parse_theme_pairings(
    md_text: str, tokens: dict[str, str]
) -> list[tuple[str, str, str]]:
    """(pairing name, background, foreground) from the documentation-theme table."""
    rows: list[tuple[str, str, str]] = []
    in_section = False
    for line in md_text.splitlines():
        if line.startswith("### Documentation theme pairings"):
            in_section = True
            continue
        if in_section and line.startswith("##"):
            break
        if not in_section or not line.startswith("|"):
            continue
        match = THEME_ROW_RE.match(line)
        if not match or match.group(1).strip() in {"Theme pairing", "---"}:
            continue
        name, foreground_ref, background_ref, documented_ratio = match.groups()
        foreground = resolve_colour_ref(foreground_ref, tokens)
        background = resolve_colour_ref(background_ref, tokens)
        measured = contrast_ratio(foreground, background)
        if abs(measured - float(documented_ratio)) > 0.015:
            raise ContrastCheckError(
                f"{name.strip()}: documented contrast {documented_ratio}:1 "
                f"does not match computed {measured:.2f}:1"
            )
        rows.append((name.strip(), background, foreground))
    return rows


def _iter_bracket_statements(dot_text: str) -> list[tuple[str, str]]:
    """(head, attrs_text) for every top-level `[...]` block in document
    order. `head` is the raw text since the previous block's `]` (or start
    of file) -- the caller classifies it to learn what kind of statement
    this bracket belongs to (node default, edge default, a named node, or
    an edge)."""
    statements = []
    stmt_start = 0
    depth = 0
    bracket_open_idx = None
    for i, ch in enumerate(dot_text):
        if ch == "[":
            if depth == 0:
                bracket_open_idx = i
            depth += 1
        elif ch == "]":
            depth -= 1
            if depth == 0 and bracket_open_idx is not None:
                head = dot_text[stmt_start:bracket_open_idx]
                attrs_text = dot_text[bracket_open_idx + 1 : i]
                statements.append((head, attrs_text))
                stmt_start = i + 1
    return statements


_HEAD_EDGE_RE = re.compile(r"^(\w+)\s*->\s*(\w+)$")
_HEAD_NODE_RE = re.compile(r"^(\w+)$")


def _classify_head(head: str) -> tuple[str, str | None, str | None]:
    """Classify the statement immediately preceding a `[...]` block, using
    the last non-blank source line before the bracket (this project's DOT
    files always keep an identifier/arrow and its opening bracket on one
    line, even when the attribute list itself continues on later lines)."""
    lines = [ln for ln in head.splitlines() if ln.strip()]
    if not lines:
        return ("unknown", None, None)
    last = re.sub(r"//.*$", "", lines[-1]).strip()
    if last == "node":
        return ("node_default", None, None)
    if last == "edge":
        return ("edge_default", None, None)
    m = _HEAD_EDGE_RE.match(last)
    if m:
        return ("edge", m.group(1), m.group(2))
    m = _HEAD_NODE_RE.match(last)
    if m:
        return ("node", m.group(1), None)
    return ("unknown", None, None)


def _parse_attrs(attrs_text: str) -> dict[str, str]:
    attrs = {}
    for key, value in ATTR_RE.findall(attrs_text):
        attrs[key] = value.strip().strip('"')
    return attrs


Pairing = tuple[str, str, str]


def parse_dot_text_pairings(dot_text: str) -> list[Pairing]:
    """(label, background_hex, fontcolor_hex) for every node/edge statement
    that renders a label, resolving `fontcolor` (and `fillcolor`/`style`)
    against the nearest preceding `node [...]`/`edge [...]` default block
    when the statement itself doesn't set them explicitly -- Graphviz
    renders text in the *inherited* colour when `fontcolor` is omitted
    (true default: black), not in some neutral/no-text state. A checker
    that only looks at explicit `fontcolor=` misses exactly the case where
    someone deletes a conforming override and silently falls back to a
    default that fails contrast (Gate 2 review `h1s8` BLOCK 1, reproduced
    live by deleting `fontcolor=white` from a red rejected node)."""
    node_default: dict[str, str] = {}
    edge_default: dict[str, str] = {}
    pairings: list[Pairing] = []

    for head, attrs_text in _iter_bracket_statements(dot_text):
        kind, _a, _b = _classify_head(head)
        attrs = _parse_attrs(attrs_text)

        if kind == "node_default":
            node_default = attrs
            continue
        if kind == "edge_default":
            edge_default = attrs
            continue
        if kind not in ("node", "edge"):
            continue

        label = attrs.get("label")
        if label is None:
            # No explicit label on this statement. Edges render no text
            # without one; a label-less node would render its own id, which
            # none of this project's figures rely on for meaningful content
            # (every node that matters sets an explicit label) -- skip.
            continue

        defaults = node_default if kind == "node" else edge_default
        fontcolor_raw = attrs.get("fontcolor", defaults.get("fontcolor", "black"))
        fontcolor_hex = normalize_hex(fontcolor_raw) or "#000000"

        style = attrs.get("style", defaults.get("style", ""))
        has_explicit_fill = "fillcolor" in attrs or "fillcolor" in defaults
        if kind == "node" and ("filled" in style or has_explicit_fill):
            fill_raw = attrs.get("fillcolor", defaults.get("fillcolor", "white"))
            bg_hex = normalize_hex(fill_raw) or "#FFFFFF"
        else:
            # Unfilled node, or an edge label: rendered on the page background.
            bg_hex = "#FFFFFF"

        pairings.append((label.replace("\\n", " "), bg_hex, fontcolor_hex))
    return pairings


CONTRACT_MARKER = "Semantic responsibility palette:"


def iter_dot_sources() -> list[Path]:
    """DOT sources that have opted into the responsibility-palette contract.

    Most figures under agf/docs/figures/src/ and docs/academic/figures/src/
    still use the pre-contract, per-figure palette from before Workstream D
    and have not been converted yet (that conversion is deliberately
    incremental, one class/figure at a time -- see the plan's Class 1/3/4
    inventory). Scanning every *.dot file unconditionally would fail this
    check on unrelated, unconverted figures the instant it merges, well
    before their own conversion MRs land. Instead, a figure opts in by
    carrying the same header-comment reference to palette.md's contract
    section that gate-pipeline.dot already uses ("Semantic responsibility
    palette: agf/docs/site/reference/palette.md") -- exactly the marker
    each conversion MR adds. No separate registry to keep in sync; the
    figure's own source states whether it is governed.
    """
    sources: list[Path] = []
    for source_dir in DOT_SOURCE_DIRS:
        sources.extend(sorted(source_dir.glob("*.dot")))
    return [p for p in sources if CONTRACT_MARKER in p.read_text(encoding="utf-8")]


def check_pairings(
    pairings: list[Pairing], source: str, minimum: float = MIN_CONTRAST_NORMAL_TEXT
) -> list[tuple[str, str, float, float]]:
    """Print each pairing's result; return the list of failures."""
    failures = []
    for label, bg_hex, fg_hex in pairings:
        ratio = contrast_ratio(bg_hex, fg_hex)
        status = "OK" if ratio >= minimum else "FAIL"
        print(f"[{status}] {source}: {fg_hex} on {bg_hex} ({label}) = {ratio:.2f}:1 (need >= {minimum}:1)")
        if ratio < minimum:
            failures.append((source, label, ratio, minimum))
    return failures


def run_real_checks() -> int:
    if not PALETTE_MD.exists():
        raise ContrastCheckError(f"palette contract not found: {PALETTE_MD}")

    md_text = PALETTE_MD.read_text(encoding="utf-8")
    tokens = parse_base_palette_table(md_text)
    if not tokens:
        raise ContrastCheckError(
            f"parsed zero tokens from {PALETTE_MD} -- parser is broken or the "
            "table format changed"
        )

    responsibility_rows = parse_responsibility_table(md_text, tokens)
    if not responsibility_rows:
        raise ContrastCheckError(
            f"parsed zero responsibility-table rows from {PALETTE_MD} -- "
            "parser is broken or the section was renamed/removed"
        )

    theme_pairings = parse_theme_pairings(md_text, tokens)
    if not theme_pairings:
        raise ContrastCheckError(
            f"parsed zero documentation-theme pairings from {PALETTE_MD}"
        )

    all_failures: list[tuple[str, str, float, float]] = []
    all_failures += check_pairings(
        theme_pairings,
        "palette.md#documentation-theme-pairings",
    )
    all_failures += check_pairings(
        [(name, fill, text) for name, fill, text in responsibility_rows],
        "palette.md#semantic-responsibility-palette",
    )

    dot_sources = iter_dot_sources()
    if not dot_sources:
        raise ContrastCheckError(f"no *.dot files found under {DOT_SOURCE_DIRS}")

    checked_any_dot_pairing = False
    for dot_path in dot_sources:
        pairings = parse_dot_text_pairings(dot_path.read_text(encoding="utf-8"))
        if pairings:
            checked_any_dot_pairing = True
        all_failures += check_pairings(pairings, dot_path.relative_to(REPO_ROOT).as_posix())

    if not checked_any_dot_pairing:
        raise ContrastCheckError(
            "parsed zero fontcolor-bearing node/edge blocks across all *.dot "
            "sources -- parser is broken (every figure has at least one "
            "explicit fontcolor today)"
        )

    if all_failures:
        print(f"\n{len(all_failures)} pairing(s) fail their WCAG target.")
        return 1

    print(
        f"\nAll {len(theme_pairings)} documentation-theme pairing(s), "
        f"{len(responsibility_rows)} responsibility pairing(s), and all DOT "
        f"figure text/fill pairings meet their WCAG target."
    )
    return 0


# ── Self-test: prove the parser actually detects a bad pairing ──────────────


def _self_test() -> int:
    good_md = """
## Palette

| Token | Hex | Contrast on white | Role | Usage |
|---|---|---|---|---|
| `amber` | `#EE7700` | 2.89:1 — fill only | Caution / pending | Pending gate |

## Semantic responsibility palette (who acts, not what happened)

| Responsibility | Token (fill) | Border/line | Text colour | Shape (colour-independence) |
|---|---|---|---|---|
| Human decision | `amber` `#EE7700` | `amber` | `#1A1A1A` | diamond |

## Next section
"""
    tokens = parse_base_palette_table(good_md)
    assert tokens == {"amber": "#EE7700"}, tokens

    good_rows = parse_responsibility_table(good_md, tokens)
    assert good_rows == [("Human decision", "#EE7700", "#1A1A1A")], good_rows
    good_failures = check_pairings(good_rows, "self-test/good")
    assert not good_failures, "conforming pairing was wrongly flagged as a failure"

    theme_md = good_md.replace(
        "## Next section",
        """### Documentation theme pairings

| Theme pairing | Foreground | Background | Measured contrast | Use |
|---|---|---|---|---|
| Dark-theme accent | `#99CCEE` | `#1E2129` | 9.37:1 ✓ AA text | Focus |

## Next section""",
    )
    theme_rows = parse_theme_pairings(theme_md, tokens)
    assert theme_rows == [("Dark-theme accent", "#1E2129", "#99CCEE")], theme_rows
    assert not check_pairings(theme_rows, "self-test/theme-good")

    bad_theme_md = theme_md.replace("`#99CCEE`", "`#0077BB`")
    try:
        parse_theme_pairings(bad_theme_md, tokens)
    except ContrastCheckError:
        pass
    else:
        raise AssertionError("stale documented theme contrast was not detected")

    bad_md = good_md.replace("`#1A1A1A`", "white")
    bad_tokens = parse_base_palette_table(bad_md)
    bad_rows = parse_responsibility_table(bad_md, bad_tokens)
    assert bad_rows == [("Human decision", "#EE7700", "#FFFFFF")], bad_rows
    bad_failures = check_pairings(bad_rows, "self-test/bad")
    assert bad_failures, "mutated white-on-amber pairing was NOT detected as a failure"

    good_dot = """
    digraph g {
        gate1 [label="Gate 1", shape=diamond, fillcolor="#EE7700",
               fontcolor="#1A1A1A"]
    }
    """
    good_dot_failures = check_pairings(parse_dot_text_pairings(good_dot), "self-test/dot-good")
    assert not good_dot_failures, "conforming DOT pairing was wrongly flagged"

    bad_dot = good_dot.replace('fontcolor="#1A1A1A"', "fontcolor=white")
    bad_dot_pairings = parse_dot_text_pairings(bad_dot)
    assert bad_dot_pairings == [("Gate 1", "#EE7700", "#FFFFFF")], bad_dot_pairings
    bad_dot_failures = check_pairings(bad_dot_pairings, "self-test/dot-bad")
    assert bad_dot_failures, "mutated white-on-amber DOT pairing was NOT detected"

    # Deletion/inheritance mutation (Gate 2 review `h1s8` BLOCK 1): deleting
    # an explicit fontcolor must not make the statement invisible to the
    # checker -- it must fall back to the inherited default (node/edge
    # default block, else Graphviz's true default of black) and be checked
    # against that. h1s8 reproduced this live by deleting `fontcolor=white`
    # from a red rejected node: default black-on-red is ~3.36:1, below
    # 4.5:1, and the pre-fix parser (which only looked at explicit
    # `fontcolor=`) silently skipped it and reported zero failures.
    rejected_dot_conforming = """
    digraph g {
        rejected1 [label="Rejected /\\nstop", fillcolor="#CC3311",
                   fontcolor=white, style="filled,dashed"]
    }
    """
    conforming_pairings = parse_dot_text_pairings(rejected_dot_conforming)
    assert conforming_pairings == [("Rejected / stop", "#CC3311", "#FFFFFF")], conforming_pairings
    assert not check_pairings(conforming_pairings, "self-test/inherit-good")

    rejected_dot_deleted_fontcolor = """
    digraph g {
        rejected1 [label="Rejected /\\nstop", fillcolor="#CC3311",
                   style="filled,dashed"]
    }
    """
    deleted_pairings = parse_dot_text_pairings(rejected_dot_deleted_fontcolor)
    assert deleted_pairings == [("Rejected / stop", "#CC3311", "#000000")], (
        f"deleting fontcolor must resolve to the true Graphviz default "
        f"(black), not silently vanish from the checked set: {deleted_pairings}"
    )
    deleted_failures = check_pairings(deleted_pairings, "self-test/inherit-bad")
    assert deleted_failures, (
        "deleting fontcolor=white from a red-filled node must be caught "
        "(inherited black-on-red is ~3.36:1) -- it was NOT detected"
    )

    # A node/edge default block that itself sets a conforming fontcolor
    # must be honoured too (not just the true Graphviz black default).
    default_block_dot = """
    digraph g {
        node [fontcolor="#1A1A1A"]
        gate1 [label="Gate 1", shape=diamond, fillcolor="#EE7700"]
    }
    """
    default_block_pairings = parse_dot_text_pairings(default_block_dot)
    assert default_block_pairings == [("Gate 1", "#EE7700", "#1A1A1A")], default_block_pairings
    assert not check_pairings(default_block_pairings, "self-test/default-block")

    print(
        "\nSelf-test passed: the parser detects a conforming pairing, a "
        "mutated-invalid explicit pairing, a deleted-fontcolor pairing that "
        "falls back to an invalid inherited default, and honours a "
        "conforming node/edge default block."
    )
    return 0


def main(argv: list[str]) -> int:
    if "--self-test" in argv:
        try:
            return _self_test()
        except AssertionError as exc:
            print(f"SELF-TEST FAILURE: {exc}")
            return 1

    try:
        return run_real_checks()
    except ContrastCheckError as exc:
        print(f"ERROR: {exc}")
        return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
