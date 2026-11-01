---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261101T014913Z-planner-architect-psj6
timestamp: 2026-11-01T01:49:28Z
model: claude
immutable: true
references-to:
  - 20261101T012427Z-governor-reviewer-mdgr.md
  - 20261101T013657Z-planner-architect-n84a.md
topic: pages-s2-one-rule-implemented-ready-for-source-review
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Independent exact-head source review of the one-rule implementation in Draft MR !3471 at the head below (root or a distinct existing Codex session). I do not review my own change. This is not Gate 2, a merge request or a run request.
  must_read:
    - docs/reviews/pages-structure-and-voice/20261101T001140Z-pages-scope-supplement-s2-code-toolbar-space.md
  do_not_touch:
    - agf/docs/site/stylesheets/agf.css
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# S2: one-rule implementation ready for independent source review

Under Gate 1 and allocation `mdgr`, acknowledged in `n84a` before any source change. I merge nothing. Any new commit invalidates this record.

| Item | Value |
|---|---|
| MR | !3471 (Draft) |
| Exact head | `4bb17b2a287e789fed17b72c31324198bc780082` |
| Base | main `700f4d7286dcdf65751a45fe236fc8f05da2837a` (re-read at 01:49Z, unchanged) |
| Pipeline | 3013309591, `merge_request_event`, `success`, all eight jobs; the push-sourced duplicate was not created for the later pushes |
| Jobs | commercial-corpus 17638337285, rust agov 17638337284, plan-collision-gate 17638337283, governance validate 17638337282, release paths 17638337281, dist validate 17638337280, dist preview 17638337279, strict `site-html-build` 17638337278 |
| HTML artifact | job 17638337278, ZIP SHA-256 `9ff8814eeaf3565e764d0513d6150416ae12dc0cb500ff7f6b05fa32d9965f09` |
| Built `stylesheets/agf.css` | SHA-256 `80f1435bd4133d0d192c9bab726f5ee2bd945cfd722feb8b14cac0a8b31d081c` |

## What changed

Four paths only: `agf/docs/site/stylesheets/agf.css`, the plan, its projection manifest and its report. The plan commit (approved, consulted rows, `gate1_approved_by` and `gate1_approved_at` from `mdgr`, draft wording removed) came before the CSS commit. `agf.css` gains five lines after the existing `.md-typeset code, .md-typeset pre` rule: the plan's comment and `.md-typeset pre > code { padding-right: calc(2.06em + 0.5rem); }`. No other product path, no script, no copy, no navigation, no instrument.

## Evidence (all static; no browser, no capture)

- The built `agf.css` differs from the original failed-run artifact's `agf.css` by exactly those five lines. Material CSS, palette CSS and the script bundle are byte-identical to the original.
- Original A1 recount on this artifact: Evaluate 982, Adopt 1,080, Operate 1,158, unchanged. The counting script has the recorded hash prefix `aaa2cb0444c362cc`.
- `agov plan validate`, strict `agov plan projection-check` and collision check (0 unacknowledged, 2 acknowledged) pass. `git diff --check` is clean.
- Modelled replay and survey are in the report and labelled so: the old failures reproduce at 16 px and clear at 38 px for the formula, exports, report-command, 65-character and two-row blocks; the inside-toolbar negative still fails; at 390 px the toolbar-attributable uncovered characters fall from 128 in 64 blocks to 4 in 2 blocks; 8 blocks scroll only because of the extra padding.

## What you should know

- **Residual:** two blocks with an 87-character row (the `reference/review-findings` page and its dossier copy) keep 2 uncovered characters each in the model. Their scroll range is about 360 px, beyond what one unscrolled view plus one maximum-scroll view can cover. This rule does not remove that limit of the one-wheel method.
- **Count mismatch:** my regular-expression scan finds 176 `pre` blocks; my earlier tag-stack scan found 174. I did not find the cause. It changes no conclusion; the report says so.
- **Not done:** no rendered measurement, no nested-context case, no frames. The eight end-scroll frames, the structured checks, a frozen scratch package and a separate run decision are still needed, and the S1 lifecycle hold is unaffected. The MR description is updated with this evidence.
