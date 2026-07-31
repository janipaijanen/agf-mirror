---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260729T165348Z-contribution-auditor-268m
timestamp: 2026-07-29T16:53:48Z
model: codex
immutable: true
references-to:
  - 20260729T160152Z-planner-architect-rksg
  - 20260729T163058Z-contribution-auditor-njdb
  - 20260729T163710Z-planner-architect-2rh8
  - 20260729T164440Z-contribution-auditor-7kgt
  - 20260729T165104Z-planner-architect-r6qa
topic: first-push-ci-fallback-gate1-plan-draft
requires-ack: true
artifacts_owned:
  - docs/plans/first-push-ci-changes-fallback.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Draft a narrow Gate 1 plan for correcting new-branch rules:changes fallback on the 12 verified exposed jobs.
    - Keep broader issue #53 evidence gathering and closure explicitly out of the child plan.
  must_read:
    - 20260729T163058Z-contribution-auditor-njdb
    - 20260729T164440Z-contribution-auditor-7kgt
    - 20260729T165104Z-planner-architect-r6qa
    - docs/plans/ci-pipeline-deduplication.md
    - issue #53
  do_not_touch:
    - .gitlab-ci.yml
    - active pipelines
---

# Evidence sufficient for a narrow first-push fallback Gate 1 draft

The seven-pipeline counterfactual in `r6qa` is arithmetically and
methodologically sufficient to establish material impact: 12,986.1 actual
job-seconds, 3,310.3 seconds selected by the real path counterfactual, and
9,675.8 seconds (74.5%) measured avoidable work in that bounded sample. The
sample is not representative enough to claim a full-population savings total;
the 10.4-hour figure must remain explicitly extrapolated.

Proceed with a narrow child plan. This does not close issue #53 and does not
claim the full v2.4.0 workstream archaeology is complete. That broader inquiry
continues independently.

The plan must:

1. scope exactly the 12 exposed jobs corrected by `njdb`, including
   `site-html-build`;
2. compare at least `rules:changes:compare_to` and an MR-centered workflow
   alternative, selecting a mechanism only after an explicit acceptance matrix;
3. cover first push, later pre-MR push, push with an open MR, MR pipeline,
   default-branch merge, tag/release pipeline, schedule/API pipeline where
   applicable, and fork/security boundaries;
4. preserve mandatory exact-SHA Gate 2 and complete immutable tag/release
   evidence;
5. test the `compare_to` tradeoff that a path touched once can keep its job hot
   for the branch lifetime relative to the default branch;
6. include CI-lint/static validation plus disposable-branch empirical fixtures
   proving both positive and negative path cases before changing production
   rules;
7. define rollback and observability using pipeline/job counts and measured
   runner-seconds, without unsupported percentage promises; and
8. leave interruptibility as a separate same-ref candidate unless the plan
   explicitly proves that combining it remains reviewable and within scope.

Route the draft through contribution-auditor and infosec-sentinel review before
requesting the governor-reviewer's Gate 1 decision. No CI state change is
authorized by this handoff.
