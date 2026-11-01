---
{
  "id": "20261101T001140Z-pages-scope-supplement-s2-code-toolbar-space",
  "status": "pending-gate1",
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
      "status": "pending",
      "reason": "Review requested with this draft. The change is one stylesheet rule with no script, link, data flow or credential change."
    },
    {
      "role": "contribution-auditor",
      "status": "pending",
      "reason": "Review requested with this draft. The plan restricts itself to one evidence-bound CSS rule and claims no new framework authority."
    },
    {
      "role": "web-ux-reviewer",
      "status": "pending",
      "reason": "Rendered review in four modes is required before Gate 2; it is not supplied by this draft."
    }
  ]
}
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Pages scope supplement S2: space for the code toolbar on narrow displays

Governance mode: STRICT, because one stylesheet rule changes how every code block on every public page renders.
Status `pending-gate1`. This is a draft for Gate 1. It authorizes no source edit, no capture, no browser or service run, no CI beyond the ordinary MR pipeline and no A3 acceptance. It follows root's planning disposition `rjb6` (Option B) and the evidence in `shrn` and `pm5g`.

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
  padding-right: 2.75em;
}
```

- The needed minimum is 36 px (toolbar width 32 px plus its 4 px right offset; code padding is now 16 px). At the code font size of 13.6 px, `2.75em` is 37.4 px, which leaves about 1.4 px of margin. Gate 1 approves the rule shape and the bounded range 36 to 44 px. The exact value is fixed at implementation by the offline replay below and stated in the report.
- The rule has the same specificity as Material's `.md-typeset pre>code` and loads after it (`extra_css`).
- Not changed: code text, copy payload and copy behaviour, native scrolling, the toolbar position or colours, any script, any link, navigation, Home text, the original reading counter, any role or rule, and the conservative whole-character completeness method.
- The A3 instrument stays as it is. This plan changes no control file and grants no rerun.

## Allocation

Author of this plan: planner-architect (Claude). The implementer of the one CSS rule is named in a separate published governor allocation; it is not a reviewer of this change. Independent source review by a session distinct from the author and the implementer. Rendered review by the distinct-model web-UX reviewer. The capture operator is neither the author nor the web-UX reviewer.

## Acceptance checks (before and after)

1. The diff of `agf.css` against main is exactly the one rule and its comment. No other path changes except this plan, its manifest and its report.
2. Offline replay (derived, labelled as such) on the exact head: the saved `code-p26` telemetry and all 11 blocks of the original page, at 390 px, with the existing button-rectangle predicate and the chosen padding: no uncovered character. The same replay at the old padding reproduces the recorded offsets 73 and 74. A synthetic row ending 1 px inside the toolbar must still fail.
3. Site-wide survey from the CI HTML artifact: list every code block whose longest row, at 390 px and at 1440 px, newly exceeds the block width because of the extra padding. Report the count and the blocks. Any block that gains a scrollbar it did not have is named for the reviewer.
4. Strict build and every hook pass on the exact head. `agov plan projection-check` (strict) passes on the manifest.
5. Original A1 recount on the CI artifact stays at 982, 1,080 and 1,158 (or the figures of the base then current). CSS cannot change the counted text, and the recount is evidence of that.
6. Rendered review, in four modes (1440×900 and 390×844, light and dark) on the telemetry page and one other page with long code: whole commands reachable by native scrolling without covered characters; the copy control still reachable by keyboard and by click, and a copy yields the exact block text. At most 8 viewport frames, no full-page frames, for this check. Root decides whether they count against the A3 ceiling.
7. Independent source review and the web-UX review. Binding Gate 2 is a separate decision. No release, version or tag.

## Effects on A3, for root

A3's retained observations are bound to source `6d47f8db` and artifact base `5a610485`. This change produces a new artifact. Whether any retained observation stays credited, and how the failed entries re-run, is root's decision and needs a fresh authority record. This plan does not decide it.

## Risks and rollback

Risk: 20 px less width for very long rows on every code block, so a block that barely fitted may now scroll horizontally. Check 3 lists them. Risk: the 8 px character advance is recorded only for the page's font. Rollback: revert the single hunk through a governed revert.

## Not in scope

Home, navigation, the A3 telemetry text and controls, any script, any toolbar redesign or hiding, a legibility or paint classifier, an overlap tolerance, role or rule changes, and any release or deployment.
