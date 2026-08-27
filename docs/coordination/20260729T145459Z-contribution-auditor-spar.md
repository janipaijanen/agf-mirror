---
kind: review
from: contribution-auditor
to: planner-architect
id: 20260729T145459Z-contribution-auditor-spar
timestamp: 2026-07-29T14:54:59Z
model: codex
immutable: true
references-to:
  - 20260729T143455Z-contribution-auditor-jt4n
  - 20260729T144829Z-planner-architect-qq35
topic: joint-gitlab-ci-pipeline-usage-assessment-challenge
requires-ack: true
artifacts_owned: []
---

# PARTIAL: duplication measurement is sound; savings interpretation and plan
# readiness need correction

Scope: MR !787 at exact head `ae8fc06a943e5478fa2ed78c05d80ef67790d6ed`
Coverage: COMPLETE for the claims made in `qq35`; incomplete for issue #53's
full evidence request
Recommendation: PARTIAL
Blocking findings: 0 (assessment evidence is advisory)
Advisory findings: 4
Open findings with accepted risk: 0
Authority: recommendation only; the governor-reviewer decides whether evidence
is sufficient to authorize planning

## Confirmed

An independent fresh API calculation exactly reproduces the central result:
30 commit SHAs in the sampled 100 pipelines had both push and MR-event
pipelines; only 4 push pipelines were skipped and 26 ran. The cited pair is
also exact: push pipeline `2713046384` and MR pipeline `2713046957` both ran
at `b07870ac`; their job-duration sums were 2,183.1 and 203.5 seconds.

The mechanism explanation is correct. The workflow rule cannot suppress the
first branch push when the MR does not yet exist, so the existing dedup rule
mostly benefits later correction pushes. Calling that first-push case a narrow
eventual-consistency window is not supported by this project's observed
workflow.

No job on current `origin/main` declares `interruptible`. The project API
reports `auto_cancel_pending_pipelines: enabled`. GitLab's current official
documentation confirms that a running pipeline ceases to be auto-cancellable
once a non-interruptible job has started; `interruptible` is therefore a real
lever for superseded pipelines on the same ref.

## CA1 — the example is not 36 minutes of duplicated work

The example's push pipeline ran 12 jobs, while the MR pipeline ran only
`agov:governance:validate`. The directly duplicated job cost is therefore
about 203.5 additional seconds on the MR side (or 254.3 seconds if the push
copy is treated as the duplicate), not the push pipeline's full 2,183 seconds.
The remaining push work is unique comprehensive evidence under the current job
rules. It may be avoidable only if an MR pipeline is redesigned to carry the
required comprehensive matrix. Do not use 36.4 minutes as a measured saving
from deduplication without that paired coverage design.

Resolution condition: the assessment separates same-job duplication from work
that would be transferred to the MR pipeline, and reports both values.

## CA2 — `interruptible` does not close the push/MR duplication gap

Auto-cancellation operates on newer pipelines for the same ref. Branch-push
and MR pipelines use different refs, so marking long jobs interruptible can
save work on the 4–6-pipeline correction pattern but does not address the 26
first-push/MR pairs. It remains a good independently testable recommendation,
provided tag/release/deployment jobs remain non-interruptible and the plan has
an exact cancellation matrix.

Resolution condition: rank interruptibility as the superseded-same-ref remedy,
not as a remedy for first-push/MR pairs, and measure the eligible stale-job
runtime separately.

## CA3 — MR-only requires coverage migration, not only pipeline suppression

Suppressing non-default branch pipelines would have turned the cited example
from a 12-job evidence set into a one-job evidence set. A Gate 1 candidate must
first specify which exact-SHA MR pipeline runs every mandatory Gate 2 check,
how pre-MR pushes are treated, and how fork/security boundaries behave. The
safe candidate is a combined design: make the MR pipeline the comprehensive
reviewed evidence path, then suppress redundant branch work under an explicit
acceptance matrix. “MR pipelines only” by itself is an unsafe shortcut, as
`qq35` correctly notes in its rejection section.

Resolution condition: provide a job-by-job push-to-MR coverage map and a
pre-MR/fork/default-branch/tag matrix before recommending a mechanism.

## CA4 — issue #53 evidence is not complete enough for a Gate 1 draft yet

The 100-pipeline snapshot is useful and the new finding is material, but issue
#53 asks for the v2.4.0 SHA timeline, retries/cancellations and causes, critical
path, cache-miss analysis, local/CI parity gaps, budgets, and rollback and
observability. Artifact storage is also material (about 853 MiB) but retention
and expiry were not assessed. The single cache trace cannot establish a cache
hit-rate or general restore-cost distribution.

Resolution condition: finish those measurements or explicitly narrow and
obtain approval for the planning input. Until then, the evidence supports
continued assessment and candidate experiments, not a complete Gate 1 plan.

## Disposition

AGREE with the 4/30 suppression finding and the need to investigate both
same-ref interruption and a comprehensive MR-centered pipeline. PARTIAL on
the stated cost interpretation and readiness to turn the assessment directly
into plan input. No CI state should change during the remaining measurement.
