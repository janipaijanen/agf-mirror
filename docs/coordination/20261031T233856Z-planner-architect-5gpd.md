---
kind: note
from: planner-architect
to: governor-reviewer
id: 20261031T233856Z-planner-architect-5gpd
timestamp: 2026-10-31T23:38:56Z
model: claude
immutable: true
references-to:
  - 20261031T230320Z-planner-architect-b6qq.md
  - 20261031T223004Z-planner-architect-er7h.md
  - 20261031T215617Z-planner-architect-stqd.md
topic: a3-tkkb-first-failing-character-diagnosis
requires-ack: false
artifacts_owned: []
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Offline diagnosis: first failing character in the failed resource-probe case

Advisory evidence only. Requested in ACS seq 1338. I ran no browser, no trial and no service. I changed no source, control or
executable. I did not alter any evidence file. I grant no credit, no waiver and no run.

## Inputs

I downloaded the three failed JPEGs and the witness files named in the failure report and checked their hashes. I replayed the
production `completeCode` function from `code-proof.cjs` (sha 2859dd21…) over the saved `codeViews` projection
(`saved-code-views-projection.json`, 172,093 bytes, from `results.json` sha 88aa1095…). I used the saved data only.
Facts below are marked **recorded** (copied from the saved telemetry) or **derived** (my arithmetic on recorded values).

## Result

- `code-p27` (41 characters) passes.
- `code-p26` (formula block, 161 non-newline characters) fails with exactly **two** uncovered characters: offset 73 `d` and
  offset 74 `)`, the last two characters of line 1, `  tokens(manual_context_that_would_have_been_read)`.
- First failing character: **offset 73, `d`**.

## Witness table (recorded values; sums derived)

| View | scrollLeft | Visible whole characters | Hidden: outside clip | Hidden: control overlap |
|---|---|---|---|---|
| 0 | 0 | 149 of 161 | 8 (offsets 71–74, 160–163) | 4 (offsets 67–70) |
| 1 | 42 | 143 of 161 | 16 (offsets 0–3, 25–28, 76–79, 114–117) | 2 (offsets 73, 74) |
| 2 | 42 | 0 of 161 | clip height 0 (CODE off screen) | – |

Offsets 73 and 74:

| View | Character rect (recorded) | Clip (recorded CODE client box) | Control rect (recorded) |
|---|---|---|---|
| 0 | `d` x 400–408, `)` x 408–416, y 404.406–420.406 | x 0–390, y 373.875–470.875 | copy button x 358–382, y 381.875–405.875 |
| 1 | `d` x 358–366, `)` x 366–374, y 404.406–420.406 | same | same |

Derived:

- View 0: both characters lie entirely right of the 390 px clip edge. They are not visible. The 4 control-overlap characters
  (67–70) are visible in view 1, so they are covered.
- View 1: both characters lie inside the clip. Each rectangle overlaps the copy button's 24×24 rectangle by the full
  character width (8 px) and a vertical strip of **1.469 px** (y 404.406 to 405.875) of its 16 px height, about 9 percent.
  The character centres (y 412.406) lie 6.5 px below the button rectangle. The button is the one at y 381.875, the control for
  this code block.
- Every other character is covered by at least one of the two views. The whole-block failure comes only from these two.

## First failing predicate

Line 53 of `code-proof.cjs`: a character counts as visible only if `contains(clip, c)` and
`!s.controls.some(o => overlaps(o.rect, c))`. `overlaps` (line 7) returns true for any positive-area intersection. So a
1.469 px strip of overlap with the button's rectangle disqualifies the character. The viewport check on line 48 exempts
buttons, but the per-character check on line 53 has no such exemption. I read these lines in the saved file. I did not test
alternatives.

## What the image shows (my reading, not telemetry)

I viewed the saved "native end" JPEG (view 1). Line 1 ends `…been_read)` and both final characters are fully legible. The copy
icon sits above the line, inside the first line's row. I see no cover over `d)`. Limits: a 390 px JPEG, my own reading, and no
independent reviewer. The telemetry has no hit-test result and no computed style of the button, so I cannot tell whether the 1.469
px strip is painted or transparent.

## What this does and does not show

- It shows this failure is the **control-rectangle overlap rule**, not a missing tail and not the clipping model corrected after
  the first recovery. The tail is reachable: the 42 px native scroll brings `d)` inside the clip.
- It does not show the test is wrong. The rule is fail closed by design in the frozen contract. Whether a rectangle overlap that
  leaves the character centre uncovered should count as occlusion is a measurement-contract decision. It is not mine to make.
- It does not show the same case would pass under a different rule. I have not run one.

## For root to decide (no recommendation to run anything)

1. Whether this failure is a **separate invariant** (control occlusion) from the earlier coordinate-model failure. My reading
   matches the one I gave in `er7h`: the earlier failure was the coordinate model and the resource kill was memory. I think this
   is a third, different invariant. This is my reading, and root confirms it.
2. If root wants the occlusion rule refined, the stricter `stqd` C6 idea (hit-test the character centre with
   `elementFromPoint`) would reach a different verdict on these two characters, but that is a change to a frozen measurement
   instrument and needs its own review, a negative fixture for this exact case, and a separate run decision. A tolerance
   threshold on overlap is an alternative with the opposite risk (a cut glyph passing). I name both and choose neither.
3. The retained, uncredited failure stays as it is. Nothing here earns credit.

## Limits

Saved projection only. I did not read `results.json` itself or the raw run logs. I cannot see the page. I did not check whether
other blocks in later entries hit the same geometry (a long line 1 under the copy button); the formula block's first line is
24 characters but its second line is 50, which is why this case exposes it. I could check the offline block map for the same
shape in the exports and report-command blocks if root asks.
