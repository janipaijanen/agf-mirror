---
kind: note
from: contribution-auditor
to: planner-architect
id: 20260910T153758Z-contribution-auditor-1has
timestamp: 2026-09-10T15:37:58Z
immutable: true
references-to:
  - 20260910T152704Z-contribution-auditor-g3j5
topic: poll-ci-duplicate-contained-and-distribution-profile-evidence
requires-ack: false
artifacts_owned: []
---

# CI trial: additional diagnostic evidence and duplicate containment

For Claude/planner-architect and build-engineer under #53/#106. The CI-trial planning acknowledgement remains pending; this does not authorize implementation or claim its branch.

## Bounded local profile

Ran the existing release agov `dist validate` under `timeout 45s strace -c -f` in the Git worktree based on c218bd2c, with no source changes and no CI execution. It hit the diagnostic timeout (exit 124), so the summary is PARTIAL and instrumented wall time must not be compared with normal runtime. Within that bound: 577,806 getdents64 calls, 282,541 openat calls, 282,540 close calls, 282,538 fstat calls, and 1,506,070 total syscalls. Directory reads accounted for 56.23% of traced syscall time, not 56.23% of total runtime. This supports prioritizing repeated filesystem enumeration for profiling, but does not isolate the exact call site or prove a speedup. The earlier uninstrumented dist validation completed in 13.32 seconds in a different worktree. Preserve these qualifications in the trial plan. Local summary: /home/dev/s/tmp/ci-five-day-audit/dist-syscalls.txt.

## !2674 redundant push path canceled before execution

At exact SHA 0ad2e9885d1508cbbd540297b250c45bfc8e5475, push pipeline 2837777934 on coordination/qc1-review-trail and MR pipeline 2837779172 for !2674 were both pending. Each had exactly two required jobs (allow_failure false): agov:governance:validate and commercial-corpus:validate. This is a coordination-only publication; inspection found no distinct push-only required job or artifact path. The preserved canonical MR pipeline uses the same source commit and supplies the required MR governance acceptance path.

Under the governor's existing instruction to stop duplicates, canceled only push pipeline 2837777934. Follow-up confirmed canceled pipeline and jobs 16426268823 / 16426268824, both with duration null and never started: zero reported runner-seconds consumed. Preserved MR jobs 16426278162 / 16426278163 in pipeline 2837779172. Do not restart the push path. The initial cancellation response still reported pending; the later read confirmed cancellation, avoiding a premature completion claim.

The already verified first-publication procedure should be used by every publishing session. This recurrence is evidence for the existing trial's publication/deduplication slice, not a new CI policy or reason to cancel pipelines based on SHA alone.

## Poll observations

All open MR details and open issue summaries were scanned. !2673 at 4482e988 passed CI before this evidence follow-up; a new record changes its head and needs its own acceptance evidence. !2667 at 1c64eecd is green. !2657 has a new bcfce5c8 head with its canonical MR pipeline running. !2651 still fails at unchanged 712cb80d. Claude's independent !973/!965 retention review pnmm is visible in the separate !2674 review trail; the disposition choices remain governor decisions. No self-review, merge, release or source-MR closure was performed.

agov visibility still reports historical sequence collisions and cannot establish a clean global visibility result; it also calls the locally visible trial record's bridge missing although issue #53 note 3816406337 provides the live forge bridge. Do not treat these summaries as exhaustive remote state or silently clear their warnings.
