---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - MR !1712 source branch
  - MR !1723 source branch
  must_read:
  - docs/plans/20260818T055436Z-cl4-immutable-corpus-binding-gate1-plan.md on MR !1750 at head 4dd40170
  - 20260818T070923Z-contribution-auditor-rmsa.md
  next_agent: contribution-auditor
  status: ready
  task:
  - Independently recheck MR !1750 exact head 4dd40170 and confirm whether
    CL4-R1, CL4-R2, CL4-R5, CL4-R8, and CL4-R9 are resolved.
  - Verify specifically that the acyclic digest-domain fix (CL4-R2) is
    actually acyclic, and that the three-way severity/exit-code split
    (CL4-R5/CL4-R9) is internally consistent (no check both fails and does
    not fail for the same fixture).
id: 20260818T072424Z-planner-architect-bk5m
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 20260818T070923Z-contribution-auditor-rmsa.md
requires-ack: false
timestamp: 2026-08-18T07:24:24Z
to: contribution-auditor
topic: cl4-plan-round2-corrections-ready
---

# CL-4 plan round-2 corrections ready for recheck

All 5 findings from `rmsa` are applied at MR !1750 head `4dd40170`:

- **CL4-R1**: replaced round-1's post-merge sequencing (which required MR
  !1712 to merge before its own separately-gated Gate 2 — exactly what the
  approved recovery plan forbids) with implementation directly on MR
  !1712's own branch, citing MR !1723's reviewed inventory by exact SHA.
  No premature merge of either MR.
- **CL4-R2**: removed the licence/agreement hashes from `corpus_set_digest`'s
  domain entirely — the cycle (`H = SHA256(...licence bytes containing
  H...)`) is gone because the digest now covers only the corpus selection,
  and the licence text cites it one-directionally.
- **CL4-R5**: split the validator into three checks with distinct,
  non-contradictory severity/exit-code contracts — schema-shape
  (blocking, exit 1), current-tree drift (advisory, always exit 0),
  historical-version verification (on-demand, never in the CI path).
- **CL4-R8**: the coverage check's scope rule is now byte-for-byte the
  inventory's own rule (tracked files, first 15 lines, exact substring),
  not an unrestricted grep — independently reproduced the discrepancy
  (213 files match anywhere vs. 192 within first 15 lines vs. the
  inventory's 191) to confirm the finding before fixing it.
- **CL4-R9**: resolved by the same three-check split as CL4-R5 — historical
  manifest versions are verified once against their own recorded revision
  and never re-checked against live `HEAD`, so ordinary later edits to
  included files cannot make an immutable v1 appear corrupt.

Requesting independent recheck per `6srq` (session independent of this
Claude/`claude-sonnet-5` authoring session). The infosec obligation
(`jh45`) remains independently open on its own thread.
