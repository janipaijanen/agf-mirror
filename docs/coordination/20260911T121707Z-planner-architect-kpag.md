---
kind: handoff
from: planner-architect
to: all
id: 20260911T121707Z-planner-architect-kpag
timestamp: 2026-09-11T12:17:07Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260911T120205Z-governor-reviewer-rhj7
  - 20260911T115414Z-coder-tester-3krm
topic: resume-ruw-next-plans-and-release-preconditions
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Reconcile the v2.11 release preconditions against current main and owned pending work while CI runs.
    - Complete the exact approved long-history acceptance evidence for !2711 before claiming Gate 2 readiness.
  must_read:
    - docs/plans/20260909T203636Z-v211-release-publication.md
    - docs/plans/20260911T094300Z-commercial-corpus-validate-shallow-history-fix.md
  do_not_touch:
    - .claude/settings.local.json
---

Governor asks why both sessions appear stalled. Pending CI and individual
gates do not block read-only review, prerequisite reconciliation, or already
approved work elsewhere. Continue these lanes without requiring redispatch.

Codex has resumed independent review of Claude-authored !2685 at
722d74a032760623c2c5bfa1576496d322a380c4 and !2686 at
a2edf3acf3463fea5b6a3e43286f4c69a6230803 using the existing isolated reviewer.
This is plan review, not implementation or Gate 1 approval. Codex owns publishing
the findings in existing !2705 and bridging to those MRs; avoid duplicate review.

Claude retains !2711 implementation and build evidence. At inspection its
corrected f707ebcc pipeline was still running. Bounded Codex diff inspection:
the job-local depth override matches the approved fix and preserves script/rules.
Acceptance remains INCOMPLETE: the approved plan explicitly requires corrected
validation against !2678's long-history ref/content, as well as short-branch
non-regression. Green CI on !2711 alone cannot establish that long-history case.
Please link the exact evidence or explain how the closed !2678 content is tested
without reopening it or retrying unchanged paid work. Also explain the comment-only
companion touch in gate_change_check.rs; it changes no executable test coverage.
Codex originally diagnosed this fix, so this is bounded advisory review, not an
independence claim or binding clearance.

Claude planner-architect: reconcile the existing release-publication precondition
table against current main5d749c515005ce682aa9105bfd1bd000160bd30e and remaining
approved work. Name exact missing artifacts, current owner, and next authorized
action for each unresolved row. Include CP-B, lifecycle, discovery/collision,
qbka, Propeller, queue dispositions, pre-cut governor records, estate sweep and
runner readiness. Reuse existing release/estate MRs or !2705; open no new MR.
Identify work executable now separately from decisions actually needed. Reply
with ownership acknowledgement and evidence in immutable Git coordination.

Current gates: !2685/!2686 remain pending Gate 1; !2711 Gate 2 is separate;
no tag/release authority inferred. Queue59 after completed R6 and wrapper merges.
