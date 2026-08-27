---
kind: handoff
from: governor-reviewer
to: build-engineer
id: 20260729T180503Z-governor-reviewer-hrrm
timestamp: 2026-07-29T18:05:03Z
model: codex
immutable: true
references-to:
  - 20260729T175843Z-governor-reviewer-m15q
topic: first-push-ci-fallback-implementation
requires-ack: true
artifacts_owned:
  - .gitlab-ci.yml
handoff:
  next_agent: build-engineer
  status: ready
  task:
    - Implement the approved first-push rules:changes fallback correction for exactly the 12 named jobs.
    - Produce every disposable-branch and regression evidence item required by the approved plan before Gate 2 handoff.
  must_read:
    - docs/plans/first-push-ci-changes-fallback.md
    - 20260729T175843Z-governor-reviewer-m15q
  do_not_touch:
    - release/tag-only job rules
    - merge_request_event semantics
    - interruptible settings
---

# Implement approved first-push CI fallback correction

MR !790 merged the approved plan into `main` at `19e7e88`. Gate 1 decision
`20260729T175843Z-governor-reviewer-m15q` authorizes implementation by the
active build-engineer role.

Implement only the selected three-rule, push-source-bounded mechanism for the
12 listed jobs. Preserve each job's exact existing fallback condition, use the
inline path-list anchor form, and execute the complete Gate 2 evidence matrix:
CI Lint/expanded configuration, coordination-only first-push positive case,
Rust-path first-push negative case with exact should/must-not-run sets,
incremental second-push behavior, default-branch/tag regression boundaries,
and structural scope verification.

Do not combine interruptibility, MR-only workflow redesign, or broader issue
#53 changes. Submit implementation on a feature branch and request exact-head
contribution-auditor review before the governor-reviewer's Gate 2 decision.
