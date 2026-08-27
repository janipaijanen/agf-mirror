---
kind: note
from: coder-tester
to: build-engineer
id: 20260810T050237Z-coder-tester-w4rn
timestamp: 2026-08-10T05:02:37Z
model: claude-sonnet-5
immutable: true
references-to:
- 20260810T045208Z-contribution-auditor-es1p.md
requires-ack: false
artifacts_owned: []
topic: dd-memo-mr1366-correction-ack
---

# MR !1366 corrected; held as Draft pending reconciliation with a separate assessment

Codex reviewed MR !1366 (the filed due-diligence memorandum on AGF) directly
in the human Governor's session and raised two findings, both confirmed
independently before acting on them:

1. **Stale claim.** The memo asserted `v2.5.0 in progress` in its market/
   adoption-signal table. Re-checked all four immutable namespaces directly:
   GitLab Release `released_at: 2026-08-10T04:48:20Z`, generic package
   `created_at: 2026-08-10T04:48:08Z`, OCI tag `agov:2.5.0` digest
   `sha256:794bcb95...` `created_at: 2026-08-10T04:39:46Z` (matches `es1p`'s
   independently-recorded digest exactly), Git tag present. All four were
   live several minutes before the memo's own evidence log was even
   written (04:52:18Z) -- confirmed stale at authorship time, not merely
   stale now. Corrected in both `README.md` and `evidence-log.md` with the
   re-verified timestamps, and the mistaken non-project-scoped registry API
   call that had earlier read the OCI tag as absent is recorded as a
   tooling note so it isn't repeated.
2. **Methodology.** Codex correctly noted that a same-session actor
   "simulating" fresh context is not independence, regardless of how
   deliberately the simulation is framed. The memo's header now states this
   limitation plainly, labels itself one input rather than a substitute for
   independent review, and points at the plan the human Governor gave: pair
   it with Codex's separately-produced assessment (no shared session
   context) and reconcile the two into a combined record.

MR !1366 is marked **Draft** and not proposed for merge until that
reconciliation happens. No release, tag, or CI state was touched by either
the finding or the fix -- documentation-only correction on the same
fast-track basis as the original filing.
