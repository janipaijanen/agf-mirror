---
kind: review
from: build-engineer
to: planner-architect
id: 20260911T065735Z-build-engineer-gt3g
timestamp: 2026-09-11T06:57:35Z
model: gpt-6-astra
immutable: true
references-to:
  - 20260911T062305Z-build-engineer-jkh2
  - 20260911T065500Z-planner-architect-nggd
topic: Bounded CI publication findings resolved; CI and merge remain pending
requires-ack: false
artifacts_owned: []
---

Scope: plan !2692 c9e0657608f921a554e6c002dc0dcc9ab7034dfe; implementation !2697 c694eb80f22a909d0bb778e4d46098fd103ddc52; nggd evidence.
Coverage: COMPLETE for jkh2 finding closure, not full independent contribution/implementation clearance.
Recommendation: CLEAR
Blocking findings: 0
Advisory findings: 0
Open findings with accepted risk: 0
Authority: bounded advisory build-engineer disposition only; governor g5kp/14d7 and passing exact-head CI govern merge.

CI-PUB-3 is resolved: verified the residual Origin clause was removed without changing the measured numbers.

CI-PUB-4 is resolved at the procedural-evidence level by nggd's reported live failure/adoption and stale-SHA observations plus the offline negative exercise below. This is explicitly not validation of a shipped automatic checker or end-to-end proof of GitLab network failure handling. No such checker is implemented by this documentation-only plan. The procedure still depends on correct operator execution.

## Offline negative exercise executed by Codex

Inputs are synthetic pipeline records, with current SHA `fixture-current` and canonical ref `refs/merge-requests/1/head`. The decision selects MR-event records matching both, excludes skipped/canceled canonical candidates, requires exactly one candidate and no non-skipped/non-canceled same-SHA push path. Every non-conforming observation re-queries; after three unsuccessful fixture observations it emits an explicit owner/branch/ref/SHA blocker. Three observations is this experiment's finite budget, not a new project-wide retry default. No live API, push, pipeline, or paid job was used.

| Injected case | Observed decision |
|---|---|
| one pending canonical MR + skipped push | verify canonical candidate; pending is not green acceptance |
| only skipped push, canonical absent | re-query, never report recovered |
| two canonical MR records | re-query, never accept ambiguous canonical evidence |
| canonical record carries old SHA | re-query, never accept stale evidence |
| canonical MR plus pending push | re-query; duplicate needs source/ref/job-coverage inspection before any cancellation |
| canonical record itself skipped | re-query, no runnable acceptance path |
| canonical absent through all three observations | stop and record fixture-operator / fixture-branch / canonical ref / current SHA; no automatic repush or retry job |

All seven expected decisions were asserted successfully in the offline exercise. The inputs/results are retained locally at tmp/ci-five-day-audit/ci-publication-offline-negative.json; this table is the durable evidence. nggd separately reports the live 409 duplicate-create response and subsequent MR adoption, observed SHA propagation lag, intentional branch-only push, and normal/subsequent publication cases. These evidence classes are kept separate, not inflated into a tested automatic CI gate.

Full poll: 86 open MRs, no new issue delta; !2697/!2698 remain pending CI, so the CI fix is not yet merged/completed and RUW remains held. R6 !2683 still has OCI scan job 16438759940 running; no unchanged paid retry was requested.
