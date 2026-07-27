---
id: 20260727T152325Z-coder-tester-rrtp
kind: decision-record
immutable: true
from: coder-tester
to: all
timestamp: 2026-07-27T15:23:25Z
model: claude-sonnet-5
references-to:
  - 20260727T133607Z-coder-tester-6n4a
  - 20260727T124630Z-contribution-auditor-mthb
  - 20260727T124631Z-planner-architect-mqsp
topic: issue72-73-governor-decisions-recorded
requires-ack: false
artifacts_owned: []
---

# Governor decisions recorded for MR !719

Recording the two decisions requested in
`20260727T133607Z-coder-tester-6n4a` (the governor-decisions handoff filed
against `docs/coordination/20260727T124630Z-contribution-auditor-mthb.md`'s
blockers 2 and 4).

## Decision 1 — A5, current-target enforcement

**Implement enforcement.** The approved plan
(`docs/plans/coordination-independent-publication.md`) requires mechanical
current-target revalidation, not documentation of the current absence of
`only_allow_merge_if_pipeline_succeeds`/merge trains. No plan amendment is
authorized. Coder-tester is to build the enforcement.

## Decision 2 — issue #74 disposition

**Pull into this MR's scope.** The full per-occurrence reconciliation of
the 17-file/53-matching-line baseline inventory is not deferred; it is
required before #72/#73 are considered done. Issue #74 should be closed (or
repurposed) once the reconciliation lands in this MR rather than carrying a
separate non-blocking priority label.

## What follows

Coder-tester will implement both, produce a real green exact-head CI run,
and request fresh exact-head fact-checker and contribution-auditor reviews
per the second corrective handoff
(`20260727T124631Z-planner-architect-mqsp`). This record does not itself
claim Gate 2 readiness.
