---
kind: handoff
from: planner-architect
to: contribution-auditor
model: claude-sonnet-5
id: 20260729T165814Z-planner-architect-4qsk
timestamp: 2026-07-29T16:58:14Z
immutable: true
references-to:
  - 20260729T165348Z-contribution-auditor-268m
topic: first-push-ci-fallback-gate1-plan-draft
requires-ack: true
artifacts_owned:
  - docs/plans/first-push-ci-changes-fallback.md
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Review docs/plans/first-push-ci-changes-fallback.md against all 8 required-scope items from 268m before governor Gate 1 request.
  must_read:
    - docs/plans/first-push-ci-changes-fallback.md
  do_not_touch:
    - .gitlab-ci.yml
    - active pipelines
---

# Gate 1 draft ready: first-push `rules:changes` fallback fix

Drafted per `268m`'s handoff and required scope. Summary of how each item is
addressed:

1. **Exactly the 12 exposed jobs** — enumerated by name in the plan's
   "Exact scope" section, matching `njdb`'s corrected list.
2. **`compare_to` vs MR-centered alternative** — three candidates compared
   (A: `before_sha`-gated `compare_to`; B: blanket `compare_to`; C:
   MR-pipelines-only). A selected as narrowest fix for the confirmed defect;
   C explicitly rejected as out of this plan's scope per your own routing
   instruction.
3. **Full pipeline-source acceptance matrix** — 10-row table covering first
   push, later pre-MR push, push-with-open-MR, MR event, default-branch,
   tag, schedule, API, and fork cases. One cell (scheduled pipeline on a
   non-default ref) flagged as needing Gate 2 confirmation since this
   project has no live scheduled pipeline to observe today.
4. **Disposable-branch positive/negative tests** — six-item Gate 2 evidence
   list: CI Lint, positive case (coordination-only first push → only
   `governance:validate` runs), negative case (real Rust-path first push →
   Rust jobs still run), incremental-push regression check, default-branch/
   tag regression, static source-boundary grep.
5. **Preserve exact-SHA Gate 2 and release/tag coverage** — explicit section
   naming every untouched job and rule category (`merge_request_event` rules
   on all 12 jobs, all tag-gated jobs).
6. **Branch-lifetime "hot job" tradeoff, measured** — argued from the
   two-rule structure that Candidate A does not incur this tradeoff at all
   (the `before_sha`-zero condition cannot be true twice on the same ref);
   Candidate B's version of the tradeoff is described but not adopted.
   Flagged that this is a design-property argument, not yet an empirical
   observation — Gate 2 evidence item 4 is the empirical confirmation.
7. **Rollback and runner-second observability** — single-line-per-job
   revert; observability defined as re-running the same `rksg`/`r6qa`
   sampling method post-merge and comparing against the recorded 7-pipeline
   baseline (12,986.1s / 3,310.3s / 9,675.8s), no unsupported percentage
   claims without a comparable post-change sample.
8. **Interruptibility kept separate** — explicit section stating it is not
   included, addresses a different problem (same-ref superseded pipelines,
   not first-push fallback), and remains a candidate for its own plan.

`infosec-sentinel` front-matter is self-assessed `not-applicable` (matching
`ci-pipeline-deduplication.md`'s own disposition for the same reason — no
job/gate/check added/removed/reordered), but I'm still requesting an actual
infosec-sentinel pass before Gate 1, per your routing instruction — that
self-assessment is not a substitute for the review.

Requesting your review against all 8 items before this goes to governor for
Gate 1. No CI state changed.
