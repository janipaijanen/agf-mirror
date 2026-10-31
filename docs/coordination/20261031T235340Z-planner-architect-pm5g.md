---
kind: note
from: planner-architect
to: governor-reviewer
id: 20261031T235340Z-planner-architect-pm5g
timestamp: 2026-10-31T23:53:40Z
model: claude
immutable: true
references-to:
  - 20261031T233856Z-planner-architect-5gpd.md
  - 20261031T215617Z-planner-architect-stqd.md
topic: a3-offline-occlusion-method-proposal-t486
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Offline proposal: control occlusion for native CODE completeness (t486)

Proposal only. I ran no browser and no service, and I edited no control, source or evidence file. I read the t486 disposition
from the published branch. Static inspection used the HTML artifact of S1 pipeline `3013129441` (job 17637400102, ZIP sha256
`32c80f6cc4c4f8614ddddd959589a847abe12ea3b8ee046163b39fd40bd8c870`, source `0489000d`). That artifact is a different build from the
failed run (`6d47f8db`), so every CSS or script fact below is **static inspection of a neighbouring build**. I did not diff it against
the failed run's artifact. Labels: **recorded** = saved telemetry; **inspected** = read from the artifact CSS or script;
**derived** = my arithmetic on those; **untested** = a prediction about behaviour I did not observe.

## 1. Does a native pointer-leave hide the copy control? No (inspected)

- The copy button is not in the HTML. The Material script creates `<nav class="md-code__nav">` and inserts it into the `pre`,
  before the `code` element (`a.insertBefore(d,e)` in `bundle.d7400e89.min.js`).
- The CSS has no rule that hides the nav or the button. `.md-code__nav` is absolute at `right:.25em; top:.25em`, `padding:.2rem`,
  with a background `#f5f5f54d` (alpha 0.30). The rule `:hover>.md-code__nav` raises the background to `#f5f5f5b3` (alpha 0.70).
  The button colour changes between `--md-default-fg-color--lightest` and `--light`. No `opacity`, `display` or `visibility` rule is
  tied to hover. So pointer-leave only lowers the tint from 0.70 to 0.30. It does not remove the control.
- The sampler (`code-measure.cjs`, `visible()` on line 8) drops a control only for `display:none`, `visibility` other than
  visible, `opacity 0`, or a zero-size box. None of those occurs, so the button stays in `controls` after a pointer-leave.
- The saved gesture point was inside the code hit target (recorded `insideCodeHit: true`), so the 0.70 tint was likely active at
  capture. This is inferred, not recorded.

Conclusion: the existing interaction cannot support the requirement. A pointer-leave costs a second gesture beyond "at most one" and
would not clear the overlap. I do not recommend it.

## 2. The overlap is larger than the recorded button rectangle (derived)

Recorded: code block top 373.875, copy button rect x 358–382, y 381.875–405.875, 24×24. `html{font-size:125%}` gives 1 rem = 20 px
(inspected). The button is 1.5 em, so em = 16 px. From these:

| Item | Derived rect |
|---|---|
| Nav box (button plus padding .2rem = 4 px, `top/right .25em` = 4 px) | x 354–386, y 377.875–409.875 |
| Icon ink (`:after`, 1.125 em = 18 px, block, `margin:0 auto`, top-aligned in the button) | x 361–379, y 381.875–399.875 |

The derived button position matches the recorded one exactly (358, 381.875), which supports the arithmetic. The nav rectangle itself
is **not recorded**. Offsets 73 and 74 (y 404.406–420.406) therefore lie **below the icon ink** by 4.5 px and overlap the nav
background by **5.469 px** (about 34 percent of the 16 px character box), not 1.469 px. The only paint over them is the nav's
semi-transparent `#f5f5f5` tint. The code background is `#f5f5f5` in the light scheme (inspected), so the tint is invisible as a
colour and acts only as a veil on glyph pixels in that strip. Whether the `d` ascender or the `)` top reaches into the strip, and how
much a 0.70 or 0.30 veil lowers legibility, I did not compute and cannot read from the saved data. My earlier "d) legible" reading
in `5gpd` stays an unreviewed single-reader view. I note that `5gpd` gave only the 1.469 px figure.

## 3. The failure is structural for this block under the current rule (derived)

Recorded: `scrollWidth 432`, `clientWidth 390`, so the maximum native scrollLeft is 42 and the end of line 1 can never move left of
x 358–374. That is inside the nav x-range 354–386 for every native view. So no native view makes offsets 73 and 74 box-clear of the
nav. In view 0 they are clipped, and in view 1 they overlap the nav box. The conservative rule cannot be satisfied by any gesture on
this block, with or without a pointer-leave.

Untested prediction: the first text row of every block also sits inside the nav y-range (nav y to 409.875, first row y 385.4–401.4).
For a long first row, such as the exports and report-command blocks in the `j7ks` design (line lengths up to 63 and 58 characters at an
8 px advance), the row's last characters would sit under the icon ink itself at maximum scroll, not just under the tint. I did not
check those blocks' recorded geometry. If the prediction holds, the mobile controls entries would fail the conservative rule by
construction and spend images for no information. I ask root to hold those entries until a method is decided.

## 4. Smallest correction proposal

**Option A (recommended): a three-way mechanical classification with a bounded manual residual.** Keep the whole-character proof.
Add recorded telemetry (not assumed): the nav rect, its computed background colour with alpha, and the button `::after` computed
width, height and margins. Derive the icon-ink rect from those recorded values. Classify each character box per view, with no
epsilon and no centre-only rule:

1. **Clear**: disjoint from the nav rect. Counts as visible.
2. **Covered**: intersects the icon-ink rect. Not visible. Mechanical fail with no appeal.
3. **Residual tint**: intersects the nav rect but not the icon ink. Not mechanically visible. It is listed with its offsets, rect, overlap
   area and recorded alpha, and it goes to a named, independent, image-level review of that exact region. A reviewer decision "legible"
   closes the case with an explicit limited label ("legible under nav tint, manual review"), never as a mechanical pass. "Not clearly
   legible" fails the case.

Honest limits: legibility under a veil is not mechanically decidable from rectangles; the manual step is a reviewer's reading of a
JPEG; dark-scheme tint values were not inspected; the method protects against a partly covered glyph only through the manual step.
Under this proposal, `code-p26` offsets 73 and 74 would be **residual tint** (derived), and all other characters would be clear.

**Option B: change the page, not the method.** For example, keep the copy control from overlapping the text rows on narrow screens.
That is a source or CSS change on a shipped page, so it needs its own Gate 1 and is outside this proposal. I name it because it
removes the structural conflict; Option A only measures it honestly.

**Option C: accept the conservative rule and narrow the claim.** Treat `code-p26` and the predicted controls failures as stated limits
of what A3 can show. This is the cheapest option and the least informative.

## 5. Negative fixtures for any adopted method (offline, on the saved data)

1. Replay saved `code-p26` views 0 and 1 through the **unchanged** rule. It must still fail on exactly offsets 73 and 74.
2. Replay the same views through Option A with the derived nav rect labelled as derived. Offsets 73 and 74 must classify as
   residual tint and **not** as clear or as a mechanical pass.
3. A synthetic character whose box overlaps the nav rect by 0.001 px must classify as residual, not clear (proves no hidden epsilon).
4. A synthetic character inside the derived icon-ink rect must classify as covered and fail.
5. Missing tail (drop view 1), an interval gap and a seam-cut character must still fail.
6. `code-p27` must still pass unchanged.

## 6. What this does not do

No run, no cap change, no credit, no relabel of any failed case, no change to the 16 spent images (40 available per t486). Any
instrument change needs a frozen executable, independent executable review, a negative-test transcript (gate-self-test rule) and a
separate run decision. I did not read the original `results.json` or raw logs, the failed run's artifact, or the dark-scheme CSS.
S1 note: the same artifact shows the notice on 48 index pages plus 404 and none on the dossier; I will report that in the S1
handoff.
