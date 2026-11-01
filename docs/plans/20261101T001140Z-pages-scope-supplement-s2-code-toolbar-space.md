---
{
  "id": "20261101T001140Z-pages-scope-supplement-s2-code-toolbar-space",
  "status": "approved",
  "topic": "pages-scope-supplement-code-toolbar-space",
  "affected_paths": [
    "agf/docs/site/stylesheets/agf.css",
    "docs/plans/20261101T001140Z-pages-scope-supplement-s2-code-toolbar-space.md",
    "docs/reviews/rule-projection/20261101T001140Z-pages-scope-supplement-s2-code-toolbar-space.yaml",
    "docs/reviews/pages-structure-and-voice/20261101T001140Z-pages-scope-supplement-s2-code-toolbar-space.md"
  ],
  "depends_on": [
    "20261030T163253Z-pages-structure-and-voice",
    "20261027T052118Z-gitlab-pages-homepage-modernization"
  ],
  "advisory_consultation": [
    {
      "role": "infosec-sentinel",
      "status": "consulted",
      "reason": "Independent plan review recorded CLEAR for the four-path scope in governor review e5b2 (host report SHA-256 32a1d288729b6327d7dae71f434b2b27182967baf88b2d5654cb7fd1b2e226eb). This revision keeps the scope and trust boundaries unchanged: one stylesheet rule, no script, link, data flow or credential change. The clearance carries only while that holds."
    },
    {
      "role": "contribution-auditor",
      "status": "consulted",
      "reason": "Plan BLOCK in e5b2 for the first draft (host report SHA-256 c9fc6bc16430e2e2f8f6ad5220cd332122515cd4fbc5051f907027e635843eb6) was answered by this revision. Plan delta CLEAR is recorded in governor decision mdgr and host report tmp/codex-s2-9c92-delta-review.md SHA-256 244247c1cd02d6bcff71591081708e86812f455aced1d3798e7f75761a3f2911."
    },
    {
      "role": "web-ux-reviewer",
      "status": "consulted",
      "reason": "Plan BLOCK in e5b2 for the first draft was answered by this revision. Plan delta CLEAR is recorded in governor decision mdgr and the same host report (SHA-256 244247c1cd02d6bcff71591081708e86812f455aced1d3798e7f75761a3f2911). This is a plan consultation only. Rendered review in four modes is a separate later requirement and is not supplied by this plan."
    }
  ],
  "gate1_approved_by": "20261101T012427Z-governor-reviewer-mdgr",
  "gate1_approved_at": "2026-11-01T01:24:27Z"
}
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Pages scope supplement S2: space for the code toolbar on narrow displays

Governance mode: STRICT, because one stylesheet rule changes how every code block on every public page renders.
Status `approved`. Gate 1 approved by governor decision `20261101T012427Z-governor-reviewer-mdgr` (2026-11-01T01:24:27Z) at !3471 head `9c92f545c95a1abbd3830d40040fb7e4afdf2eaf`, under delegation `jda1`. That decision also names the sole coder-tester for the one-rule implementation. It authorizes no capture, no browser or service run, no Gate 2, merge or release and no A3 credit or reset. It follows root's planning disposition `rjb6` (Option B) and the evidence in `shrn` and `pm5g`.

Lifecycle: supplements `20261030T163253Z-pages-structure-and-voice`; the parent file is not edited. `agf.css` is also named in the approved plan `20261027T052118Z-gitlab-pages-homepage-modernization`; this plan acknowledges that overlap and changes none of that plan's rules. Collision check against freshly fetched target main (`700f4d72`, after the learning merge): the only live plan on main that declares `agf.css` in `affected_paths` is that one. Three older plans name the file in prose only and declare no `affected_paths`.

## Why

The Material code toolbar (`nav.md-code__nav`) is absolute at the top right of each `pre`, 36 px wide on a 390 px display including its offset. The code text can scroll to within 16 px of the block's right edge. So at maximum horizontal scroll the end of a long first or second row sits under the toolbar, and no native view makes those characters clear of it. Static evidence (CSS and script bytes identical in the original failed-run artifact and the later S1 artifact):

- The derived geometry reproduces the recorded failure exactly: formula block, row 1, offsets 73 and 74 (`shrn`, section 2).
- The same model predicts one uncovered character on the exports block and two on the report-command block, and two on two other blocks of that page (`shrn`, section 3). The toolbar is translucent, not opaque, so legibility under it is not decidable from rectangles.
- Readers on narrow screens meet the same overlap on any page whose code has a long first or second row.

## Exact work

One rule in `agf/docs/site/stylesheets/agf.css`, using the existing selector and cascade conventions of the file:

```css
/* Reserve the width of the code toolbar so the end of a long row can scroll clear of it. */
.md-typeset pre > code {
  padding-right: calc(2.06em + 0.5rem);
}
```

### Why one rule holds in every context (derived from the original-artifact CSS; not rendered)

The toolbar sizes scale with the font size `E` of the enclosing `pre`: it is `1.5E` wide, with padding `2 × 0.2rem`, and it sits `0.25E` from the right edge. So the space it needs on the right of the text is `1.75E + 0.4rem`. The code font size is `0.85E` (`.md-typeset code{font-size:.85em}`), so `1em` in the rule is `0.85E`, and the rule gives `2.06 × 0.85E + 0.5rem = 1.751E + 0.5rem`. That is at least `1.75E + 0.4rem` for every `E`: a fixed margin of `0.1rem` (2 px at 1 rem = 20 px) plus `0.001E`. A fixed pixel or code-em value cannot do this, because `E` changes by context. The first draft's `2.75em` was a mobile-only value. At nested `E = 12.8 px` it gave only 29.9 px against a need of 30.4 px, which was the e5b2 finding.

| Context (source of `E`) | `E` | Toolbar space needed | Padding from the rule | Margin |
|---|---|---|---|---|
| Normal content, under 76.25 em wide (`.md-typeset{font-size:.8rem}`) | 16 px | 36.0 px | 38.0 px | 2.0 px |
| Normal content, 76.25 em and wider (`.md-typeset{font-size:.7rem}`) | 14 px | 32.5 px | 34.5 px | 2.0 px |
| Inside `.admonition` or `details` (`font-size:.64rem`) | 12.8 px | 30.4 px | 32.4 px | 2.0 px |
| Print (`.md-typeset{font-size:.68rem}`) | 13.6 px | 31.8 px | 33.8 px | 2.0 px |

These are derived from the CSS of the original failed-run artifact (Material CSS, palette CSS, script bundle and `agf.css` byte-identical to the later S1 artifact) at a 20 px root font size. The 1 rem = 20 px assumption holds below 100 em width (`html{font-size:125%}`); the `137.5%` and `150%` steps at 100 em and 125 em scale `rem` and `E` together, and the margin stays `0.1rem`. They are not rendered measurements, and the plan does not claim otherwise. The table's rows are checked at implementation by the rendered review below.

Nested contexts: my scan of the S1 artifact HTML (174 `pre` blocks in 52 pages, tag stack for `div.admonition` and `details`) found no code block inside an admonition or `details`. Root's finding that nested blocks exist may rest on source or markup I did not scan. The rule must hold for them anyway, so the table includes that row and the rendered review includes one nested case if the actual site has one, or a synthetic nested page in the scratch capture if it has none.

- Not changed: code text, copy payload and copy behaviour, native scrolling, the toolbar position or colours, any script, any link, navigation, Home text, the original reading counter, any role or rule, and the conservative whole-character completeness method.
- The rule has the same specificity as Material's `.md-typeset pre>code` and loads after it (`extra_css`).
- The A3 instrument stays as it is. This plan changes no control file and grants no rerun.

## Allocation

Author of this plan: planner-architect (Claude). The implementer of the one CSS rule is named in a separate published governor allocation; it is not a reviewer of this change. Independent source review by a session distinct from the author and the implementer. Rendered review by the distinct-model web-UX reviewer. The capture operator is neither the author nor the web-UX reviewer.

## Acceptance checks (before and after)

1. The diff of `agf.css` against main is exactly the one rule and its comment. No other path changes except this plan, its manifest and its report.
2. Offline replay (modelled; assumptions stated: 8 px character advance at the 13.6 px code size as recorded, left padding as recorded, one unscrolled view and one maximum-scroll view, the existing button-rectangle predicate) on the saved `code-p26` telemetry and all 11 blocks of the original page at 390 px with the new padding: no uncovered character. The same replay at the old padding reproduces the recorded offsets 73 and 74. A synthetic row ending 1 px inside the toolbar must still fail. This check was run for the plan: with 38 px the replay leaves no uncovered character on the formula, exports, report-command, 65-character and two-row blocks, and reproduces the old failures at 16 px.
3. **Site-wide survey, labelled by method.** (a) *Modelled*: from the CI HTML artifact, for each code block, the longest row against the block width at 390 px and at 1440 px under the stated assumptions (character advance 0.588 of the code font size, left padding `1.1764705882em`, the table above). The report lists blocks that newly need a horizontal scroll bar and states that these are derived geometry. (b) *Measured*: the rendered review below reports computed styles and actual client and scroll widths for the checked blocks. The report never presents (a) as a viewport measurement.
4. Strict build and every hook pass on the exact head. `agov plan projection-check` (strict) passes on the manifest.
5. Original A1 recount on the CI artifact stays at 982, 1,080 and 1,158 (or the figures of the base then current). CSS cannot change the counted text, and the recount is evidence of that.
6. **Rendered review, eight viewport frames.** Two pages (the telemetry page, and one page whose longest code row is also long at 1440 px) in four modes (1440×900 and 390×844, light and dark). Each frame is the **end-scroll** state: the code block scrolled to its maximum by one native wheel gesture, with the toolbar and the end of the longest rows in view. Structured assertions, not images, separately cover: the initial state (`scrollLeft` 0, geometry recorded before the gesture); the full character union by the existing conservative whole-character method over the initial and end-scroll views; the exact copied text (a pointer click on the copy control, the copied string equal to the block's `textContent`); keyboard reachability of the copy control (Tab focus) and its pointer reachability; and computed `padding-right`, `font-size` and toolbar rectangle for the block. If the web-UX reviewer needs an image of the initial state too, the ceiling becomes 16 frames; the plan names that alternative instead of omitting the evidence. Root decides whether these frames count against the A3 ceiling.
7. Independent source review and the web-UX review. Binding Gate 2 is a separate decision. No release, version or tag.

## Effects on A3, for root

A3's retained observations are bound to source `6d47f8db` and artifact base `5a610485`. This change produces a new artifact. Whether any retained observation stays credited, and how the failed entries re-run, is root's decision and needs a fresh authority record. This plan does not decide it.

## Risks and rollback

Risk: 20 px less width for very long rows on every code block, so a block that barely fitted may now scroll horizontally. Check 3 lists them. Risk: the 8 px character advance is recorded only for the page's font; the survey and replay are modelled and labelled so. Rollback: revert the single hunk through a governed revert.

## Not in scope

Home, navigation, the A3 telemetry text and controls, any script, any toolbar redesign or hiding, a legibility or paint classifier, an overlap tolerance, role or rule changes, and any release or deployment.
