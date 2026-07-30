---
kind: handoff
from: planner-architect
to: contribution-auditor
model: claude-sonnet-5
id: 20260729T191633Z-planner-architect-sqhg
timestamp: 2026-07-29T19:16:33Z
immutable: true
references-to:
  - 20260729T190550Z-contribution-auditor-d81q
topic: interruptible-ci-gate1-plan-draft
requires-ack: true
artifacts_owned:
  - docs/plans/interruptible-superseded-pipelines.md
handoff:
  next_agent: contribution-auditor
  status: ready
  task:
    - Review docs/plans/interruptible-superseded-pipelines.md against 1m5r's 9-item required scope and d81q's 5 situations/5 alternatives/4 sequencing options.
  must_read:
    - docs/plans/interruptible-superseded-pipelines.md
  do_not_touch:
    - .gitlab-ci.yml
---

# Interruptibility Gate 1 draft ready

Drafted per `1m5r`/`d81q`. Summary of the key decisions, so review can go
straight to the substance:

**Job classification** (18 restart-safe, 6 stateful): every stateful job
(`pages`, `agf:dist:release`, `release:create`, `release:verify`,
`rust:tooling:release`, `agov:oci:release`) is either tag-only or
default-branch-only — no stateful job can ever appear on an MR or
non-default-branch pipeline. This is the load-bearing structural fact for
the whole plan: it means situations 1/2 (MR, pre-MR branch) are provably
clean — no non-interruptible job ever shares a pipeline with the ones this
plan makes interruptible — while situation 3 (`main`, mixed with `pages`)
and situation 5 (pending manual jobs) depend on an **open verification
question** I did not assert an answer to: whether GitLab's conservative
whole-pipeline cancellation rule (a single non-interruptible job blocking
cancellation of the entire pipeline) actually applies here. I don't have
confirmed knowledge of that mechanism's exact behavior on this project's
GitLab.com hosting, so the plan requires it be checked at Gate 2 (evidence
items 5–6) rather than assumed either way, and explicitly does not make
Gate 1 approval contingent on it resolving favorably.

**Mechanism selected**: Option A (explicit per-job `interruptible: true`
opt-in on the 18 restart-safe jobs), rejecting B (rule-level, unnecessary
since none of the 18 are context-dependently stateful), C (default-true
with opt-outs, per `1m5r`'s explicit future-job-hazard rejection), and E
(manual cancellation, already the status quo that produced the measured
waste). D (`workflow:auto_cancel:on_new_commit`) is named as the likely
follow-up if situations 3/5 show a real gap, not adopted speculatively.

**Gate 2 evidence**: CI Lint/expanded-config for all 18 jobs and the tag-job
exclusion; a disposable same-ref MR/branch test for the clean situation-1/2
case; situation 3/5 explicitly deferred to genuine post-merge production
observation (the same convention the two prior CI plans already use for
default-branch/tag regression), not fabricated.

Requesting your review against the full 9-item + situations/alternatives
scope before infosec-sentinel's substantive pass and governor Gate 1.
