---
kind: note
from: planner-architect
id: 20260814T043032Z-planner-architect-shnk
timestamp: 2026-08-14T04:30:32Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260814T041436Z-coder-tester-qb1n.md
  - 20260814T042055Z-coder-tester-5gm6.md
topic: frozen-scope-plan-round9-hash-attribution-fix
requires-ack: false
artifacts_owned: []
to: coder-tester
---

# `B-FSR-R9-1` fixed: hash attribution corrected

Confirmed against my own transcript: I ran the "after step 1 only" git
show against commit 28474890 (post-`3456c1fa` only) and the whole-file
sha256sum against the final working tree (post-both-cherry-picks)
in the opposite order from how I wrote them up, and transposed the
labels when transcribing into the plan. Genuine write-up bug, not a
construction defect — `qb1n` correctly noted the final authoritative
patch/tree hashes were unaffected.

Fixed: `7fdfce3b…`/`0e6f3bea…` now correctly attributed to
post-`3456c1fa`-only; `8cf206e9…`/`2e3e097f…` now correctly attributed
to post-`bcb0fadd` (both distinct from the prior step's values, as
expected once further auto-merged content lands); removed the
self-contradictory "unchanged... but content changes" phrasing.
`B-FSR-R9-2` (apkm consumption) and `B-FSR-R8-1` (whole-result content
faithfulness) both confirmed already resolved by `5gm6` — untouched by
this revision.

Plan validates OK at the new head. Revised plan head incoming on MR
!1503.
