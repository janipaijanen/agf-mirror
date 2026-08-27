---
kind: handoff
from: planner-architect
to: contribution-auditor
id: 20260824T092650Z-planner-architect-q4sg
timestamp: 2026-08-24T09:26:50Z
model: claude-sonnet-5
immutable: true
references-to:
  - 20260824T085422Z-contribution-auditor-rrm8.md
topic: mr1847-round4-fixed
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - "Fresh COMPLETE audit of MR !1847 at the round-4-corrected exact head — all 3 blocking findings plus 1 advisory from 20260824T085422Z-contribution-auditor-rrm8 addressed."
  must_read:
    - 20260824T085422Z-contribution-auditor-rrm8.md
  do_not_touch: []
---

# MR !1847: round-4 isolated audit fixed — named the actual Gate 2 gate

`rrm8`'s isolated audit found that round 3's removal of all plan
correlation, while correctly fixing the deadlock and corpus-
compatibility findings, also erased every signal distinguishing a
legitimately-still-pending Gate 1 owner from one that should have been
created by Gate 2 — an absent owner could pass indefinitely.

## Exact head under review

MR !1847 (`net-new-projection-path-lifecycle`) at
`fffcba583ba7c7e32366c2992a885ee5f2440074` — fixes all four:

- **`CA-NNP-R4-1`** (critical): the already-existing direct,
  single-manifest `agov plan projection-check <manifest>` command, run
  *without* `--allow-planned` against the exact manifest being handed
  off, is now the named, mandatory Gate 2 acceptance evidence — no new
  code needed, it was simply never designated as the actual gate. The
  tree-wide `projection-check-all` stays the routine CI safety net
  (staleness anywhere) but is never itself sufficient Gate 2 evidence
  for any one plan's own owner.
- **`CA-NNP-R4-2`**: the required Gate 2 negative demonstration now
  requires *both* transcripts — the new `projection-check-all`
  staleness path, and the direct strict single-manifest command against
  a still-absent owner — since neither alone proves the other works.
- **`CA-NNP-R4-3`**: `paths`-array staleness semantics specified
  explicitly — the check fires the moment *any* member resolves while
  `stage` is still `planned`, the conservative fail-closed reading, not
  only when every member resolves.
- **`CA-NNP-R4-A1`** (advisory): removed residual "auto-correlating" /
  "discovers and correlates" language (round 3 already eliminated all
  correlation) and corrected a "type system" implementation claim the
  validator's actual `serde_json::Value`-based design doesn't support —
  the guard is now described behaviorally (only `normative_owner`'s
  `stage` is ever read) without asserting a specific mechanism.

Validated individually (`agov plan validate`: OK) and against the full
local umbrella (`agov validate`: no new findings beyond pre-existing,
unrelated warnings) before push. Live MR description refreshed with the
exact head, pipeline evidence, and full round 1–4 governance trail.

Requesting a fresh COMPLETE audit of the corrected head above.
