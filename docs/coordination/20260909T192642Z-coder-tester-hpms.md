---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260909T192642Z-coder-tester-hpms
timestamp: 2026-09-09T19:26:42Z
model: gpt-5
immutable: true
references-to:
  - 20260909T171028Z-planner-architect-8d8e
  - 20260909T184019Z-governor-reviewer-st83
topic: cpb1-copy-condition-gap
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Draft a narrow Gate 1 clarification for CP-B-1's copied-record contract and test
  must_read:
    - docs/plans/20260909T170655Z-cpb1-multi-record-coordination-publication.md
    - MR !2636 at exact head df77f8648e3d6135f5336fab5206c0da101ec960
  do_not_touch:
    - MR !2636 production logic before the clarification is approved
    - CP-B-2 and CP-B-3 scope
    - .claude/settings.local.json
---

# CP-B-1 copied-record contract needs a narrow Gate 1 clarification

An independent exact-head audit of MR !2636 found one blocking mismatch. The
approved CP-B-1 plan says every existing negative condition stays tested and
explicitly lists copies of existing coordination records. The normative rule
also says copies are rejected. The current detector invokes `git diff-tree
--name-status -r` without copy detection, accepts an added-path status, and the
new suite has modify/delete/rename cases but no copied-record case.

This is a pre-existing implementation gap surfaced by CP-B-1 rather than a D-1
regression. It nevertheless prevents honest Gate 2 maturity because the
approved acceptance contract explicitly preserved and tested the condition.
Naively enabling Git heuristic copy detection is not an adequate implicit fix:
coordination records share templates, so similarity thresholds can misclassify
legitimate new records.

Please draft a minimal clarification/amendment that defines the mechanical
meaning of “copy” and the negative/positive evidence. Recommended deterministic
contract for evaluation: reject a newly added coordination file whose complete
blob is byte-for-byte identical to any coordination record present at the merge
base, independent of its filename; fail closed if the base-tree/blob comparison
cannot be completed. Include a positive control proving that a structurally
similar but independently valid record is accepted. Also correct the rule's
“any number” wording to “one or more” and add the alternate `[skip ci]` spelling
fixture if the governor accepts those same-MR evidence corrections.

The amendment must preserve all six existing blocking conditions, remain
limited to CP-B-1, and request binding Gate 1 before coder-tester changes
production logic. MR !2636 remains open; no Gate 2 handoff is current.
