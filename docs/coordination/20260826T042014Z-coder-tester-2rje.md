---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260826T042014Z-coder-tester-2rje
timestamp: 2026-08-26T04:20:14Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260825T052314Z-coder-tester-ejg2.md
  - 20260825T052315Z-coder-tester-6djh.md
  - 20260825T203938Z-planner-architect-j3rm.md
  - 20260826T042010Z-governor-reviewer-4ssr.md
topic: shallow-history-ci-correction-gate1-planning
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Prepare and route a Gate 1 plan for the smallest durable correction to CI-HISTORY-1 under the governor commission in 4ssr.
    - Bind the correction to the affected acceptance work and its intended shipment rather than treating it as unowned maintenance.
  must_read:
    - docs/coordination/20260825T052314Z-coder-tester-ejg2.md
    - docs/coordination/20260825T052315Z-coder-tester-6djh.md
    - docs/coordination/20260826T042010Z-governor-reviewer-4ssr.md
    - .gitlab-ci.yml
    - tooling/rust/crates/agov/src/commercial_corpus.rs
  do_not_touch:
    - MR !2041 or MR !806 implementation scope before the correction plan is approved
    - Unchanged failed paid jobs
    - .claude/settings.local.json
---

# Gate 1 planning handoff: shared pinned-history CI correction

The correction is now commissioned, but implementation is not yet authorized.
Please prepare a bounded Gate 1 plan that diagnoses and resolves the standard
`rust:tooling:test` job's undeclared complete-history precondition.

The current evidence is deterministic: GitLab's depth-20 checkout cannot
resolve pinned corpus commit `48049759dfd9c681863d510ac6f7971235c5b0fb`, so
the real MR !1712 commercial-corpus acceptance test fails in otherwise-green
pipelines for both MRs !2041 and !806. The test passes in a full local clone;
139 other tests in the same CI job pass.

The plan should compare at least:

1. job-local complete history, such as `GIT_DEPTH: "0"` only for
   `rust:tooling:test`;
2. explicit acquisition and verification of the pinned revisions; and
3. a self-contained fixture that does not silently stop exercising the real
   historical-evidence path.

Recommended default: job-local complete history. It is simple, deterministic,
fork-compatible, and preserves the test's meaning, but its additional fetch
cost must be measured and constrained to the affected job. A fixed larger
shallow depth is not durable because repository growth will recreate the
failure.

Acceptance should include a fresh shallow-clone reproduction before the fix,
fork/MR and protected-ref reasoning, a check that both pinned revisions are
available, the real 142-included/50-excluded test, and new corrected exact-head
pipelines for the affected MRs after the correction reaches their base. No
unchanged failed job should be retried for cosmetic green status.
