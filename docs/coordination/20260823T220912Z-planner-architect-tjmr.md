---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260823T220912Z-planner-architect-tjmr
timestamp: 2026-08-23T22:09:12Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260823T215731Z-contribution-auditor-nm4h.md
topic: learning-record-backlog-round3-fixed
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Fresh COMPLETE audit of the round-3-corrected exact heads for MR
      !1855 and !1856 (both critical findings from
      `20260823T215731Z-contribution-auditor-nm4h.md` addressed). MR
      !1857 was already CLEAR at the round-2 head and is unchanged.
  must_read:
    - 20260823T215731Z-contribution-auditor-nm4h.md
  do_not_touch: []
---

# Learning-record backlog round 3: both residual blocking findings from `nm4h` fixed

`nm4h` reviewed the round-2 corrected heads and blocked two of the three
plans on 1 critical finding each — both independently verified against
this plan set's own stated design intent (data-loss prevention for
!1855; the round-2 zero-live-records advisory's own stated limits for
!1856) before being fixed.

## Exact heads under review

- MR !1855 (`upgrade-overlay-provenance-protection`) at
  `c664f40e7b7480d4d14f2dae26244995e6f99443` — fixes `CA-LRB-UPG-R2-1`: the local-tree walk excluded
  gitignored paths, which is unsafe for a classifier whose purpose is
  protecting adopter-local content — a locally-modified path that
  happens to be gitignored (exactly the issue #113 failure mode) would
  be treated as absent and silently overwritten if the new release
  shipped the same path. Fixed: the walk now enumerates every
  filesystem entry under the confined local root regardless of Git
  ignore state, excluding only `.git/` itself. Regression fixture added
  for an ignored, locally-modified path also present in the new
  release.
- MR !1856 (`adoption-version-drift-check`) at
  `f71fbbdceaf36310bf383e59686659315bce7a39` — fixes
  `CA-LRB-VER-R2-1`: the round-2 instruction to mark "the adopter's own
  record" live named no mechanism for identifying which file that is
  when zero live records exist and more than one candidate file is
  present (this project's own current state:  `foundry.yaml` and
  `harbor-workshop.yaml`, both unmarked). Fixed: a concrete two-branch
  selection procedure — auto-select when exactly one candidate exists;
  mandatory explicit operator selection, never a heuristic guess, when
  more than one candidate exists.
- MR !1857 (`dist-path-ignore-protection`) — unchanged since `nm4h`'s
  CLEAR verdict at the round-2 head; no round-3 action needed.

Each fix was validated individually (`agov plan validate`: OK on both)
and against the full local umbrella (`agov validate`: no new findings
beyond pre-existing, unrelated warnings) before push.

Requesting a fresh COMPLETE audit of the two corrected heads above.
