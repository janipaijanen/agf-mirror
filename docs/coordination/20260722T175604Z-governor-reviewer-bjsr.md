---
kind: decision-record
from: governor-reviewer
to: build-engineer
id: 20260722T175604Z-governor-reviewer-bjsr
timestamp: 2026-07-22T17:56:04Z
model: human-governor
immutable: true
references-to:
  - 20260722T172225Z-contribution-auditor-a2hj
  - 20260722T165827Z-planner-architect-jarh
  - 20260722T164331Z-governor-reviewer-smmj
topic: ci-pipeline-deduplication-gate1-approved
requires-ack: true
artifacts_owned: []
---

# Governor decision: approve the CI-deduplication plan at Gate 1

The governor-reviewer approves the CI-pipeline-deduplication plan in MR !404 at
exact reviewed plan head `5c5aa43db6d95f2f24a9a12012f31577686cec0c`.
The plan status is changed from `pending-gate1` to `approved` by this decision.
The build-engineer consultation evidence is also moved out of the governed
`advisory_consultation` array into plan prose so `agov plan validate` accepts
the approved plan; this is a representation-only correction and does not
alter the reviewed mechanism or acceptance criteria.

Build-engineer is authorized to implement the plan on a separate branch after
this planning MR merges. Scope is strictly the single ordered
`workflow.rules` predicate approved in the plan; no job body, job-level rule,
stage, cache, release, or unrelated CI change is authorized.

Implementation evidence must follow the approved cost-aware sequence:

1. Prepare and validate the complete one-rule change locally.
2. Make the initial implementation-branch push with `ci.skip` and open its MR.
3. Make exactly one ordinary non-skipped acceptance push after the MR exists.
4. Prove that the acceptance SHA creates zero branch-push pipelines and one
   ordinary MR pipeline, while CI Lint and the static source-boundary checks
   pass.

This approval does not authorize a merge at Gate 2. The change should land
before !405's final acceptance push so the remaining release work benefits
from deduplication, but it must stay narrowly bounded and must not displace the
active ZIP `ETXTBSY` corrective remediation if complications arise. No version
bump, release branch, tag, or publication is authorized.
