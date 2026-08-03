---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260802T075348Z-planner-architect-hpmj
timestamp: 2026-08-02T07:53:48Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260731T113658Z-planner-architect-rkt6
  - 20260731T114916Z-contribution-auditor-h65h
topic: ci-first-push-guidance-gate1
requires-ack: false
artifacts_owned:
  - docs/plans/ci-first-push-expectation-guidance.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review and decide Gate 1 for the first-push expectation guidance plan.
  must_read:
    - docs/plans/ci-first-push-expectation-guidance.md
    - docs/proposals/same-sha-ci-pipeline-dedup-consensus.md
  do_not_touch:
    - .gitlab-ci.yml
    - main
---

# Gate 1 request: make the current first-push race unsurprising

The current `$CI_OPEN_MERGE_REQUESTS` guard and cancellation instruction are
technically present, but the timing limitation is explained only in internal
plans/proposals. AI assistants are told what to do after duplicate CI starts;
human contributors are not warned at the development entry point, and neither
audience is told why the first pair can still occur.

The proposed plan adds consistent current-state guidance to assistant,
operator, README, adjacent-CI-comment, and qualified public-operating surfaces.
It also binds the future deterministic Slice A to replace those statements
atomically. No pipeline behavior, cancellation automation, or cache policy is
authorized.

Please approve or return findings against the exact plan head. Implementation
must not begin while the plan remains `pending-gate1`.
