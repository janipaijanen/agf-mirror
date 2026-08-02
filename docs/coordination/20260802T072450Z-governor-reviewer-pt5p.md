---
artifacts_owned: []
from: governor-reviewer
id: 20260802T072450Z-governor-reviewer-pt5p
immutable: true
kind: decision-record
model: human
references-to:
- 20260802T063135Z-planner-architect-gkkm.md
requires-ack: false
timestamp: 2026-08-02T07:24:50Z
to: planner-architect
topic: v250-rc1-contract-core-gate1-approval
---

# Gate 1 approval: v2.5.0-rc.1 contract-core plan (issues #79, #80)

Transcribing the governor's exact statement: "I reviewed !913, it is
approved."

**Head correction, transcribed for the record.** At the moment this
statement was made, MR !913's exact head was `83e511a0` — but that head
had a failing pipeline (`agov:governance:validate`), caused by a commit
message mistake unrelated to plan content: the tip commit was titled with
the `coordination:` fast-path prefix on a branch that also carries
plan-file-modifying commits, correctly rejected by `coordination-
publication-rule.md`'s own mechanical range check. Fixed by renaming that
commit only — `git diff 83e511a0 50ce651a --stat` is empty, confirming
the plan content the governor reviewed is byte-identical to the new head.
This approval is recorded against the corrected exact head
`50ce651a51e6379c0c9de737826d51b280c9407d`, whose pipeline now passes.

## Decision

`docs/plans/v250-rc1-contract-core.md` is approved at Gate 1, as revised
per Codex's B1-B4 findings (MR !916): work packages 1 (#79) and 2 (#80)
with their concrete decisions, work packages 3 and 4 removed as already
shipped.

Implementation may proceed per the plan's own "Validation requirements"
and "Stop conditions" sections — up to two separate implementation
branches/MRs, one per work package, each requiring its own Gate 2.
