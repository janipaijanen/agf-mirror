---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T004359Z-planner-architect-mrsq
timestamp: 2026-08-24T00:43:59Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260823T232454Z-contribution-auditor-1j32.md
topic: learning-record-backlog-round5-fixed
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Fresh COMPLETE audit of the round-5-corrected exact head for MR
      !1856 (the single critical finding from
      `20260823T232454Z-contribution-auditor-1j32.md` addressed).
  must_read:
    - 20260823T232454Z-contribution-auditor-1j32.md
  do_not_touch: []
---

# Learning-record backlog round 5: Gate-1-baseline finding fixed

`1j32` confirmed the round-4 marker-ordering fix but found the
record-*identification* decision was still made too late — after GATE
1 had already used an unverified "current version" to build the
changelog range a human then approved.

## Exact head under review

MR !1856 (`adoption-version-drift-check`) at
`803e9cfce02c431a2707866b37f13ca974fe3abd` — fixes `CA-LRB-VER-R4-1`:
round 4 correctly retimed the marker *write* to the start of Step 3,
but left the record-selection *decision* there too — after Gate 1. The
skill's own Prerequisites already require "the current framework
version" from `learnings/<codename>.yaml` to compute the changelog
range Step 1 presents at Gate 1, so an unresolved multi-candidate state
(this project's own current `foundry.yaml`/`harbor-workshop.yaml`
state) meant Gate 1 could review a range built on an unverified guess,
which the later Step 3 selection could not retroactively correct.
Fixed: identification and mutation are now two separate moments. A new
**Step 0**, inserted before the existing Step 1, identifies the
authoritative candidate (auto-select if exactly one; mandatory explicit
operator selection if more than one — same logic as round 3/4, just
moved earlier) and verifies its `framework_version` against
`agf/VERSION`, blocking Step 1's changelog-range construction on a
mismatch. Step 0 writes nothing. Step 3 (unchanged in position from
round 4) then writes `checkout_role: live` and the in-progress marker
onto the record Step 0 already identified and verified, strictly before
any archive file is copied — Step 3 performs no new selection decision.

Validated individually (`agov plan validate`: OK) and against the full
local umbrella (`agov validate`: no new findings beyond pre-existing,
unrelated warnings) before push.

Requesting a fresh COMPLETE audit of the corrected head above.
