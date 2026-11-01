---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20261101T011536Z-planner-architect-ated
timestamp: 2026-11-01T01:15:36Z
model: claude
immutable: true
references-to:
  - 20261101T005504Z-governor-reviewer-e5b2.md
  - 20261101T004300Z-planner-architect-bkp7.md
topic: pages-s2-plan-revised-for-e5b2
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Narrow contribution and web-UX delta review of the revised S2 plan in Draft MR !3471 at the exact head below, then Gate 1. This is a plan; no CSS edit, run or capture is requested. This record supersedes the head in bkp7.
  must_read:
    - docs/plans/20261101T001140Z-pages-scope-supplement-s2-code-toolbar-space.md
  do_not_touch:
    - agf/docs/site/stylesheets/agf.css
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# S2 plan revised for review e5b2

| Item | Value |
|---|---|
| MR | !3471 (Draft) |
| Exact head | `9c92f545c95a1abbd3830d40040fb7e4afdf2eaf` |
| Base | main `700f4d7286dcdf65751a45fe236fc8f05da2837a` (re-read at 01:14Z) |
| Pipeline | 3013269792 (`merge_request_event`), running when this was written; I report the result when it ends |
| Paths | the plan, its manifest and the report placeholder only (the manifest and the placeholder changed in one line, wording) |

## The three points of e5b2

1. **One rule across contexts.** The rule is now `padding-right: calc(2.06em + 0.5rem)`. The toolbar needs `1.75E + 0.4rem` where `E` is the font size of the enclosing `pre`; the rule gives `1.751E + 0.5rem`, so the margin is 2 px in every context. The plan's table gives normal narrow (need 36.0, have 38.0 px), wide screen (32.5, 34.5), nested admonition or details (30.4, 32.4) and print (31.8, 33.8). These are derived from the original-artifact CSS and are not rendered measurements. The first draft's `2.75em` gave 29.9 px in nested code, as you found. My HTML scan of the S1 artifact found no code block inside an admonition or `details` (174 `pre` blocks, 52 pages); the rule must hold for them anyway, and the plan requires one nested case in the rendered review (a synthetic page if the site has none). If your finding of real nested blocks rests on markup I did not scan, tell me where and I add it.
2. **Survey labelled.** The site-wide survey is split into a modelled part (assumptions listed) and a measured part (computed styles and actual client and scroll widths from the rendered review). The report may not present the modelled part as a viewport measurement.
3. **Eight frames.** Eight end-scroll frames: two pages × four modes. Structured assertions, not images, cover the initial state, the full character union (existing conservative method), exact copied text, and keyboard and pointer reachability of the copy control, with computed padding, font size and toolbar rectangle recorded. A 16-frame alternative (initial and end-scroll images) is named for your decision.

## Checks

`agov plan validate` OK. Strict `agov plan projection-check` OK. `agov plan collision-check --plans-only --changed-only --fail-on-unacknowledged-collision --base-ref origin/main`: 0 unacknowledged, 2 acknowledged. `git diff --check` clean. I re-ran my offline model with 38 px: no uncovered character on the formula, exports, report-command, 65-character and two-row blocks; the old failures reproduce at 16 px. The InfoSec row is now `consulted` on the strength of e5b2's CLEAR for the unchanged four-path scope; the contribution and web-UX rows stay `pending` until your delta review. The MR description is updated.

## Not covered

No browser or rendered measurement. The 8 px character advance is recorded only at the failed page's font size. The effect on A3's retained observations is still root's decision.
