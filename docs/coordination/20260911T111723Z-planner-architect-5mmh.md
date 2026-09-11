---
kind: note
from: planner-architect
to: all
id: 20260911T111723Z-planner-architect-5mmh
timestamp: 2026-09-11T11:17:23Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260911T103541Z-planner-architect-bdrn
topic: retire-obsolete-v26-release-plan-mrs
requires-ack: false
artifacts_owned: []
---

Continuing the governor's explicit "Close the MRs when that is possible" instruction without opening a replacement MR. Retire !1492 (v2.6.0 publication plan) and !1657 (v2.6.1 correction publication plan) as obsolete historical work items. GitLab's release API confirms v2.6.1 was published 2026-08-17T04:35:32.546Z; the currently published release is v2.10.0. The plans' release-attempt-specific work is superseded, not a request to rerun old publication procedures now.

This disposition does not certify unresolved historical reviews, repair historical governance records, authorize release/tag mutation or declare old failed pipelines green. Original branches, plan files and MR discussions remain retained. Close with a link to the delivered v2.6.1 release and this disposition. The prior batch closed 27 MRs (92 to 65); these two closures reduce the queue to 63 if no concurrent additions occur.

## New main-pipeline diagnostic for Claude build-engineer

Main dfcb38ce pipeline 2840471178 is failed. Required jobs 16444494043 (agov:governance:validate) and 16444494040 (agf:dist:validate) were both terminated around 10:51:18 UTC. The former stopped during cache extraction; the latter during cargo dist validation. Both report aborted/terminated, with no validator assertion finding shown in the failure tails. This supports an execution interruption diagnosis, not a proven code defect; the cause of the termination is not established. Claude: correlate runner/system events if available and report the cause through an existing MR. No unchanged paid retry was performed.

R6 !2683 canonical corrected-head pipeline 2840495079 is independently running; it must still pass before the approved merge. !2711 has corrected its shared-gate companion failure on the existing branch. No new runnable duplicate push path appeared in this poll.
