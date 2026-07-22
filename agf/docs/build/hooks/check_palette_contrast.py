# SPDX-License-Identifier: MIT
"""Mechanical WCAG AA contrast regression for the semantic responsibility
palette (agf/docs/site/reference/palette.md, "Semantic responsibility
palette" section).

Gate 2 review `20260722T125000Z-contribution-auditor-r4js` (BLOCK 1) found
that the palette contract's original text-colour assignments for Human
decision (amber) and Automated check (green-mid) rendered below the 4.5:1
WCAG AA normal-text threshold at the node label sizes figures actually use
(2.89:1 and 3.72:1 respectively -- both fail; the contract had wrongly
compared them against the weaker 3:1 large-text/fill threshold instead).
This script encodes the corrected pairings as an executable check so a
future edit to palette.md or a figure cannot silently reintroduce an
invalid text/fill pairing without the docs build noticing.

Run directly: `python3 agf/docs/build/hooks/check_palette_contrast.py`
Exits non-zero (and prints every failing pairing) if any responsibility
pairing regresses below its documented WCAG target.
"""

from __future__ import annotations

import sys

# Hex tokens from the "Palette" table in palette.md.
TOKENS = {
    "blue-mid": "#0077BB",
    "blue-light": "#99CCEE",
    "green-mid": "#009944",
    "amber": "#EE7700",
    "red-mid": "#CC3311",
    "grey-mid": "#888888",
    "grey-light": "#CCCCCC",
    "white": "#FFFFFF",
    "near-black": "#1A1A1A",
}

# (responsibility, fill token, text token, minimum ratio, why)
# Every figure node in the inventory renders labels at normal text size
# (e.g. gate-pipeline's 11pt), never WCAG "large text" -- so every
# text-on-fill pairing here must clear the full 4.5:1 threshold, not the
# weaker 3:1 large-text/fill one.
RESPONSIBILITY_PAIRINGS = [
    ("Human decision", "amber", "near-black", 4.5),
    ("AI action", "blue-light", "near-black", 4.5),
    ("Automated check / validated outcome", "green-mid", "near-black", 4.5),
    ("Shared concept", "white", "blue-mid", 4.5),
    ("Reference / deferred content", "grey-light", "near-black", 4.5),
    ("Rejected / blocked outcome", "red-mid", "white", 4.5),
]


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


def main() -> int:
    failures = []
    for responsibility, fill_token, text_token, minimum in RESPONSIBILITY_PAIRINGS:
        ratio = contrast_ratio(TOKENS[fill_token], TOKENS[text_token])
        status = "OK" if ratio >= minimum else "FAIL"
        print(
            f"[{status}] {responsibility}: {text_token} on {fill_token} "
            f"= {ratio:.2f}:1 (need >= {minimum}:1)"
        )
        if ratio < minimum:
            failures.append((responsibility, fill_token, text_token, ratio, minimum))

    if failures:
        print(f"\n{len(failures)} palette pairing(s) fail their WCAG target.")
        return 1

    print(f"\nAll {len(RESPONSIBILITY_PAIRINGS)} palette pairings meet their WCAG target.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
