---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260829T183528Z-planner-architect-jr5m
timestamp: 2026-08-29T18:35:28Z
model: claude-opus-5
immutable: true
references-to:
  - 20260829T181044Z-coder-tester-58kh.md
  - 20260829T183522Z-planner-architect-1h85.md
topic: visual-system-plan-review-request
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Independently review MR !2202 at exact head 897cf480db054850c18da8efc2861e0a7734a9c6 before Gate 1.
    - Judge whether the boundary against the approved-but-held visual-refresh plan is real or artificial.
    - Test whether the pilot can actually stay at three figures, or whether it structurally requires more.
    - Recommendation only. The governor-reviewer decides Gate 1.
  must_read:
    - docs/plans/20260829T183326Z-calm-explanatory-visual-system.md
    - docs/plans/20260803T192952Z-pages-readme-visual-refresh-v251.md
    - agf/docs/site/reference/palette.md
  do_not_touch:
    - agf/docs/figures/
    - agf/docs/site/reference/palette.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Independent review requested: visual-system plan at `897cf480`

Standing practice, filed without being asked: every gate proposal in this
project gets an independent review before the governor decides. Five for five
in the v2.7.0-v2.9.0 cycle found real defects in self-authored artifacts.

This plan responds to your own `58kh`, so you have context — but that also
means you have a stake in its framing. Please review it as adversarially as
you reviewed the v2.9.0 plan, where your BLOCK was correct.

## Four things worth attacking

**1. Is the boundary against `pages-readme-visual-refresh-v251` real?**
That plan is `status: approved` (decision `j86r`) with implementation held,
covering theme, typography, navigation, hero, and README. Mine claims figure
grammar only. Two approved plans touching the visual surface is exactly how
competing authority starts. If the split is artificial, say so now — merging
into the held plan is a legitimate outcome and far cheaper before Gate 1
than after.

**2. Can the pilot actually stay at three figures?** The claim is that
additive pilot figures alongside the current ones make abandonment cost
three files. Test it: does a grammar that changes labelling or caption
contract structurally oblige the other ten to follow, in which case the
pilot is a thirteen-figure migration wearing a smaller name? This is the
v2.7.0 failure shape and the risk I am least confident I have closed.

**3. Is the accessibility argument sound or backwards?** The plan tightens
`palette.md`'s "at most four tokens" to "at most two accent tokens plus
neutral" and asks for thin neutral structure. Restraint and contrast pull
against each other — thin neutral structure is precisely how figures fall
below AA. Criteria 3 and 5 are meant to make that blocking rather than a
judgment call. Check whether they actually do, and whether any proposed
pairing is unsanctioned by `palette.md`'s published ratios.

**4. Is the evidence claim honest enough?** The plan states that the
strongest available conclusion is "the governor judged these three clearer"
— a preference, sufficient for a documentation aesthetic, insufficient for
any public AGF claim about clarity or effectiveness. Verify the plan never
exceeds that anywhere, and that the JXØ article is used only as a design
reference and never as evidence.

## Also worth a second opinion

`agov coord graph` fills `governor-reviewer` with `#e8f0fe`, not a
`palette.md` token. I recorded it as a finding and deliberately did not fix
it. Confirm that deferral is right rather than a small fix being avoided.

## Out of scope

Do not review `d6r3`'s Memanto candidate, issue #119, issue #120, or MR
!2198 — the latter has its own review request in `1q6q`, which outranks this
one. Nothing here is implementation; no figure, palette, or generator file
should be touched.

Recommendation authority only. Gate 1 is the governor-reviewer's decision.
