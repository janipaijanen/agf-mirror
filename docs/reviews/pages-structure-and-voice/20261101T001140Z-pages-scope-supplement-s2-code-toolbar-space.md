<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# S2 acceptance report: space for the code toolbar

Implements Gate 1 approved plan `20261101T001140Z-pages-scope-supplement-s2-code-toolbar-space` (governor decision `mdgr`). Labels: **modelled** = my arithmetic on recorded values and original-artifact CSS; **measured** = a rendered measurement (none exists yet); **recorded** = saved A3 telemetry. Nothing below is a rendered result, and nothing below is independent review.

## The change

One rule in `agf/docs/site/stylesheets/agf.css`, with its comment, after the existing `.md-typeset code, .md-typeset pre` rule:

```css
/* Reserve the width of the code toolbar so the end of a long row can scroll clear of it. */
.md-typeset pre > code {
  padding-right: calc(2.06em + 0.5rem);
}
```

The diff against main is these five added lines. No other product path changes. The padding scales with the toolbar in each context (plan table: needed 36.0, 32.5, 30.4 and 31.8 px against 38.0, 34.5, 32.4 and 33.8 px; margin 2 px).

## Offline replay (modelled)

Method: the existing button-rectangle predicate; recorded 8 px character advance, 16 px left padding and 19.031 px row pitch at the 13.6 px code size; one unscrolled view and one view at maximum scroll; block width 390 px. The model reproduces the recorded failure for the formula block (row 1, offsets 73 and 74) at the old padding. The script was `s2model.py` in the author's scratch directory (not shipped); its output is summarized here.

Blocks of the original failed page (`docs/adopt/tooling-and-tokens/`), uncovered characters at old padding (16 px) then new padding (38.02 px):

| Rows (characters) | Max scroll old / new (px) | Uncovered old → new |
|---|---|---|
| 17, 54 | 74 / 96 | 2 → 0 |
| 72, 78 | 266 / 288 | 2 → 0 |
| 65 | 162 / 184 | 2 → 0 |
| 24, 50, 37, 50 (formula) | 42 / 64 | 2 → 0 |
| 63, 29, 64 (exports) | 154 / 176 | 1 → 0 |
| 58 (report command) | 106 / 128 | 2 → 0 |
| 14, 41, 36, 23 | 0 / 0 | 0 → 0 |
| a 63-row listing (up to 137 characters) | 738 / 760 | 894 → 906 (length, not the toolbar) |

Negative: a row whose last character ends 1 px inside the button box (padding 30.99 px) still leaves 1 uncovered character, so the check still fails on known-bad input.

## Whole-site scroll survey (modelled)

Source: the HTML of the S1 CI artifact (`0489000d`; code blocks do not depend on the stylesheet). 176 `pre` blocks in 25 pages with an `index.html` (the scan counts 174 in 52 routes elsewhere because that counted per file and excluded some copies; this scan counts every `pre`).

- At 390 px, 154 blocks can be covered by one unscrolled view and one maximum-scroll view; 22 are longer than that and are not decided by this rule.
- Of the 154, toolbar-attributable uncovered characters: **128 in 64 blocks at the old padding; 4 in 2 blocks at the new padding.** The two residual blocks have an 87-character row (maximum scroll about 360 px): the characters hidden under the toolbar in the unscrolled view fall left of the clip in the end view. They are the same row on the `reference/review-findings` page and in the dossier copy of it. This is a limit of the one-wheel, two-view method for very long rows, which this plan does not claim to remove.
- Blocks that scroll horizontally **only because** of the new padding at 390 px: 8 (rows of 42 to 44 characters, on `docs/adopt/mcp-server`, `docs/adopt/install-and-layouts`, `reference/environment-variables`, `reference/release-artifacts`, and four in the dossier). The new need is about 22 px more width.
- At 1440 px the block width is not recorded. With a modelled 11.9 px code font, 0.588 em advance and block widths of 650 to 800 px, 0 to 2 blocks newly scroll (0 at 700 and 750 px). The window is about 20.5 px wide. This is an assumption range, not a measurement; the rendered review resolves it.

## Not done, and limits

- No rendered measurement, no browser, no capture. The eight end-scroll frames and the structured checks (initial geometry, full character union, copied text, keyboard and pointer reachability, computed padding, font size and toolbar rectangle) remain prospective and need a frozen scratch package and a separate run decision.
- Nested contexts (`admonition`, `details`): the artifact has none; the plan's table covers them by derivation, and the rendered review needs one synthetic nested case.
- Pipeline-bound evidence (exact head, strict build, projection check, artifact hash, the original A1 recount on the artifact, and the presence of the rule in the built `agf.css`) is recorded in the MR description and the handoff, not in this file, to avoid a second pipeline.
- Independent exact-head source review is required and is not supplied by me.
