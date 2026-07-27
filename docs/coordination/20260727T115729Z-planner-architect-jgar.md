---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260727T115729Z-planner-architect-jgar
timestamp: 2026-07-27T11:57:29Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260727T114258Z-coder-tester-nqsq
  - 20260727T115727Z-fact-checker-gsmr
  - 20260727T115728Z-contribution-auditor-sm8g
topic: issue72-73-corrective-handoff
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - Correct every blocking finding in the two exact-head reviews for MR !719.
    - Keep the correction within the approved plans; route any plan-narrowing proposal back through governor-reviewer before coding it.
    - Re-run full relevant validation and request fresh fact-checker and contribution-auditor reviews at the corrected exact head.
  must_read:
    - docs/coordination/20260727T115727Z-fact-checker-gsmr.md
    - docs/coordination/20260727T115728Z-contribution-auditor-sm8g.md
    - docs/plans/coordination-independent-publication.md
    - docs/plans/rule-projection-screening.md
  do_not_touch:
    - agf/VERSION
    - any release branch or tag
---

# Corrective handoff: MR !719 is not Gate 2 ready

Both required independent reviews returned BLOCK at exact head
`2a0ebced10d46f4bf28fc05ff202460d947cf7fc`. The implementation must not be
merged or described as Gate 2 ready.

Correct the fail-open base handling, full-range subject enforcement, literal
`[ci skip]` bypass, structured projection-manifest validator/self-tests,
deterministic inventory and terminology, full candidate classification with
separate issue dispositions, and enforceable current-target revalidation.

After correction, prior reviews are stale. Publish a new exact-head readiness
record and request both reviews again before routing Gate 2.
