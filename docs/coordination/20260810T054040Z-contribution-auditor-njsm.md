---
kind: handoff
from: contribution-auditor
to: coder-tester
id: 20260810T054040Z-contribution-auditor-njsm
timestamp: 2026-08-10T05:40:40Z
model: Codex GPT-5
immutable: true
references-to:
  - "20260810T053101Z-contribution-auditor-rktm"
  - "20260810T051437Z-contribution-auditor-5krd"
topic: v250-forward-sync-noncircular-review-publication
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "Recreate the corrected forward-sync from current main after rktm, then hold target main unchanged through exact-head review and Governor Gate 2."
    - "Use visible but initially unmerged coordination MRs for the exact-head review and Gate 2 decision; merge those records to main immediately after the forward-sync lands."
  must_read:
    - "docs/coordination/20260810T053101Z-contribution-auditor-rktm.md"
    - "docs/plans/20260809T194404Z-v250-final-release-publication.md"
  do_not_touch:
    - ".claude/settings.local.json"
    - "refs/tags/v2.5.0"
    - "release/2.5"
    - "release-2.5.0"
---

# Resolve the target-advance publication cycle

MR !1377 is already obsolete: its first parent is `6d148f7e...`, while
round-1 review record `rktm` was subsequently merged through MR !1376 and
advanced `main` to `1e75b8f4ea0eee7f0ede8c7097b5780f31e9a6be`.

The approved plan requires any target advance to invalidate the candidate.
At the same time, merging each exact-head review/decision coordination record
to `main` before the forward-sync would itself advance the target and create an
infinite recreation loop.

The root visibility rule provides the non-circular path: a coordination message
is visible when it is committed and pushed to a shared branch and presented in
an MR/bridge note. It need not merge to `main` before the action it governs.

# Required sequence

1. Close/supersede MR !1377 and recreate the corrected no-FF merge from the
   then-current `main`, which must contain `rktm` and this handoff if this
   handoff has landed by branch-cut time.
2. Preserve the round-1 content corrections: all 14 live role versions,
   accurate roadmap/release-artifact truth, exact changed-path count, accurate
   non-changelog characterization, historical fixtures unchanged, and the
   exact adopter-doc parity outcome.
3. Run one canonical successful exact-head pipeline and freeze source and
   target SHAs.
4. Contribution-auditor commits the exact-head review on a fresh coordination
   branch from the frozen target, pushes it, opens a complete coordination MR,
   and bridges it to the forward-sync MR. **Do not merge that review MR yet.**
5. If CLEAR, governor-reviewer records the binding exact-head Gate 2 decision
   the same way on a pushed coordination branch/MR and bridges it. **Do not
   merge that decision MR yet.**
6. With target `main` still unchanged, merge the exact reviewed/approved
   forward-sync head according to the approved plan.
7. Immediately merge the already-visible review and Governor decision
   coordination MRs to the now-forward-synced `main`, preserving their
   immutable commits. Resolve only if a genuine conflict appears; do not alter
   the record bodies.
8. Publish the final v2.5.0 completion record and then release the diligence
   Gate 1 planning precondition.

Any unrelated `main` merge during steps 3–6 invalidates the candidate and
restarts the sequence. Hold unrelated work during this narrow window. This
ordering changes no Gate authority and grants no exception to exact-head review;
it only uses the existing visibility rule to avoid self-invalidating evidence.

