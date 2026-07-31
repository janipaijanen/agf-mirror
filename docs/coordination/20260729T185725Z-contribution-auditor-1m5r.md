---
kind: handoff
from: contribution-auditor
to: planner-architect
id: 20260729T185725Z-contribution-auditor-1m5r
timestamp: 2026-07-29T18:57:25Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260729T145459Z-contribution-auditor-spar
  - 20260729T185725Z-governor-reviewer-m5qp
topic: interruptible-ci-gate1-plan-draft
requires-ack: true
artifacts_owned:
  - docs/plans/interruptible-superseded-pipelines.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Draft a narrow Gate 1 plan for cancelling superseded same-ref CI work using interruptibility.
    - Compare explicit safe-job opt-in, default interruptible with stateful-job opt-out, and workflow auto-cancel modes before selecting a mechanism.
    - Route the plan through contribution-auditor and infosec-sentinel consultation before requesting Gate 1.
  must_read:
    - issue #53
    - 20260729T145459Z-contribution-auditor-spar
    - 20260729T185725Z-governor-reviewer-m5qp
    - docs/plans/first-push-ci-changes-fallback.md
    - .gitlab-ci.yml
  do_not_touch:
    - .gitlab-ci.yml
    - docs/plans/v246-release.md
    - agf/VERSION
    - release tags
---

# Draft the interruptibility Gate 1 plan while v2.4.6 release work is parked

## Accepted evidence and exact problem boundary

The project has `auto_cancel_pending_pipelines: enabled`, but no current job
declares `interruptible: true`. GitLab therefore retained obsolete MR pipelines
after a newer pipeline for the same MR ref began whenever a non-interruptible
job had already started.

The latest 100-pipeline window contains five measured same-ref overlap events.
Job-timeline reconstruction found at least 89.3 runner-minutes executed after
those older pipelines had been superseded; 53.6 minutes and rising came from
MR !792 pipeline `2716346277` after current-head pipeline `2716349170` was
created. At the namespace's Free-tier, Linux-small cost factor and purchased
minute price, that bounded window consumed at least 22.3% of the 400-minute
monthly allowance, with about EUR 0.78 replacement value at the 2026-07-27 ECB
reference rate. Treat the 100-pipeline result as a measured burst, not a
representative monthly forecast.

Interruptibility addresses only newer-pipeline supersession on the **same
ref**. It does not solve first-push branch/MR pairs, whose refs differ; MR !792
is the separate remedy for the first-push `rules:changes` fallback. Preserve
that distinction throughout the plan.

## Required Gate 1 scope

The plan must:

1. enumerate every job and classify it as pure/restart-safe or stateful/
   publication-sensitive, with evidence from its script, artifacts, `needs`,
   environment, registry, Pages, release, and tag behavior;
2. keep `release:create`, `release:verify`, immutable tag packaging,
   provenance publication, registry pushes, and any deployment/stateful job
   non-interruptible unless the plan proves cancellation is transactionally
   safe;
3. compare at least:
   - explicit `interruptible: true` opt-in on each restart-safe job;
   - `default: interruptible: true` with explicit stateful-job opt-outs,
     including the future-job inheritance risk; and
   - `workflow:auto_cancel:on_new_commit` modes combined with job-level
     interruptibility;
4. provide an acceptance matrix for pre-MR branch pushes, repeated MR pushes,
   default-branch pipelines, tags, manual jobs, scheduled/API pipelines,
   downstream jobs, artifacts, and release publication;
5. define a disposable same-ref test: start a deliberately long safe pipeline,
   push a second commit while it runs, prove the old pipeline and eligible jobs
   cancel, and prove the new exact-head pipeline completes;
6. prove a release/tag pipeline's stateful jobs remain non-interruptible in
   expanded CI configuration without creating a disposable release tag;
7. preserve complete exact-head Gate 2 evidence and require one final green
   implementation pipeline after the cancellation experiment;
8. state rollback as reverting the CI commit through an MR, plus post-merge
   observability based on post-supersession runner-seconds rather than pipeline
   counts alone; and
9. budget implementation evidence against the measured savings. The expected
   initial cost is roughly 65–80 compute minutes; the current evidence suggests
   payback after one or two avoided full stale pipelines, but the plan must
   label that as a bounded estimate rather than a guaranteed monthly saving.

Drafting is authorized. Implementation is not. Base the design on current
`main` plus MR !792's anticipated rule structure, but do not edit or stack on
MR !792; wait for its final Gate 2 disposition before implementation.
