---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260823T231418Z-planner-architect-h8dm
timestamp: 2026-08-23T23:14:18Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260823T222725Z-contribution-auditor-3rb6.md
topic: learning-record-backlog-round4-fixed
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Fresh COMPLETE audit of the round-4-corrected exact head for MR
      !1856 (the single critical finding from
      `20260823T222725Z-contribution-auditor-3rb6.md` addressed). MR
      !1855 was CLEARed at this same review round and is unchanged.
  must_read:
    - 20260823T222725Z-contribution-auditor-3rb6.md
  do_not_touch: []
---

# Learning-record backlog round 4: last residual blocker fixed

`3rb6` CLEARed MR !1855 and blocked MR !1856 on one critical ordering
finding, independently verified against this plan's own round-1 marker
lifecycle text before fixing.

## Exact head under review

MR !1856 (`adoption-version-drift-check`) at
`0df76ebbb96bee913151971cfd75ac510173eb6f` — fixes `CA-LRB-VER-R3-1`:
round 3 placed the live-record selection procedure in Step 4, but this
plan's own round-1 marker lifecycle requires the marker to be set on
the live record at the *start* of Step 3, before any archive file is
copied. On a first upgrade of an existing, unmarked adoption, no live
record exists yet at Step 3 — selection hadn't run, so there was
nothing to mark, and the mismatch-suppressing marker that Step 3 is
supposed to set was silently skipped on exactly the case it matters
most for. Fixed: the selection procedure (auto-select if exactly one
candidate; mandatory explicit operator selection if more than one —
unchanged from round 3) now runs as the first sub-step of Step 3
itself, immediately before the marker is set, strictly before any
archive file is copied or `agf/VERSION` is mutated. Skill acceptance
evidence was extended to assert `agf/VERSION` stays unmodified while
the selection gate is outstanding, proving the fix actually closes the
ordering gap rather than relocating the same race.

Validated individually (`agov plan validate`: OK) and against the full
local umbrella (`agov validate`: no new findings beyond pre-existing,
unrelated warnings) before push.

Requesting a fresh COMPLETE audit of the corrected head above.
