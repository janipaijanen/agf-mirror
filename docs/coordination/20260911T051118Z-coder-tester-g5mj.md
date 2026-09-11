---
kind: handoff
from: coder-tester
to: build-engineer
id: 20260911T051118Z-coder-tester-g5mj
timestamp: 2026-09-11T05:11:18Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260911T050735Z-coder-tester-d3rh
  - 20260910T152704Z-contribution-auditor-g3j5
topic: Governor requests CI duplicate-build investigation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: build-engineer
  status: ready
  task:
    - Investigate recurring first-push and MR-event duplicate pipelines, using exact evidence below; coordinate ownership with returning Claude.
    - Propose a bounded correction with trigger tests and preserved canonical MR acceptance, within normal Gate 1 requirements.
    - Inspect redundant rebuilds within jobs as a separate hypothesis; distinguish them from duplicate pipelines.
  must_read:
    - .gitlab-ci.yml
    - docs/coordination/20260910T152704Z-contribution-auditor-g3j5.md
  do_not_touch:
    - canonical MR acceptance pipelines
    - current S4/R6 implementation branches
---

Governor requests build-engineer review when current work is ready: duplicate builds keep starting. Read-only investigation now found six same-head push/MR-event pairs among 23 pipelines updated since 2026-09-10T21:00:00Z. Each pair has identical job-name coverage: agov:governance:validate and commercial-corpus:validate. Five push paths already completed; one was already canceled. No still-runnable redundant push path in this observed set remains to cancel.

| SHA | Push pipeline | Canonical MR pipeline | Push job seconds consumed | Push job IDs |
|---|---|---|---:|---|
| 9ec079fde1f9 | 2839644946 (canceled) | 2839645181 | 0.000 | 16438557613, 16438557612 |
| 1b83e8275317 | 2839623459 (success) | 2839623757 | 207.017 | 16438425188, 16438425187 |
| 19051adc428f | 2839612071 (success) | 2839612242 | 193.321 | 16438350141, 16438350140 |
| a2edf3acf346 | 2839581631 (success) | 2839582256 | 205.704 | 16438171589, 16438171588 |
| 722d74a03276 | 2839564814 (success) | 2839565011 | 197.313 | 16438077607, 16438077606 |
| 2863eb136cd6 | 2839559375 (success) | 2839559551 | 174.712 | 16438044357, 16438044356 |

Total completed redundant push job time: 978.068 seconds (16.30 runner-minutes), excluding queue wait. Canceled jobs have null duration, not proof of any measured consumption.

Affected MRs are !2684 through !2689. Pattern is consistent with the known branch-first/MR-later creation window; this is an inference to confirm against creation timestamps and workflow rules, not proof of an existing-MR push-guard defect. Initial implementation branch publication using `git push -o ci.skip` followed by MR creation previously avoided runnable push work; no commit-message skip markers. Prefer a reliable supported workflow/configuration correction over recurring manual cancellation, with explicit preservation of branch-only/tag/release cases where required.

Current heads !2678 250a426d, !2681 074293cd and !2683 07ffbeec each have one MR-event pipeline only (2839680767, 2839676496, 2839676639). Those are not duplicate pairs. Do not cancel them on the basis of this report. No CI changes, gate waiver, job retry or cancellation performed by this investigation.
