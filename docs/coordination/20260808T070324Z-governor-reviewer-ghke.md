---
kind: handoff
from: governor-reviewer
to: coder-tester
id: 20260808T070324Z-governor-reviewer-ghke
timestamp: 2026-08-08T07:03:24Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260808T070323Z-governor-reviewer-rkma.md
  - 20260807T170054Z-governor-reviewer-6ee7.md
  - 20260807T202500Z-coder-tester-n627.md
topic: get-started-commissioning-slice-b-implementation
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - "Implement Slice B only: the deterministic commissioning link-integrity and rendered-first-screen checks prescribed by the approved plan."
    - Use a new feature branch and linked worktree based on current main; keep the slice independently reviewable.
    - Run the plan-prescribed validations and full governed acceptance pipeline, then publish an exact-head contribution-auditor review handoff before requesting Gate 2.
  must_read:
    - docs/plans/20260806T122121Z-get-started-commissioning-ux-v1.md
    - docs/coordination/20260808T070323Z-governor-reviewer-rkma.md
    - docs/coordination/20260807T184713Z-coder-tester-kkam.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/tests/test_commissioning_postconditions.py
    - tooling/fixtures/commissioning/
    - Class B live assistant-execution evidence
    - deferred navigation slice beyond the already-landed additive commissioning-page entry
---

# Implementation handoff: Get Started Slice B

Slice A is retrospectively accepted in `rkma`. Resume the approved plan with
Slice B only:

- `agf/docs/build/hooks/check_commissioning_links.py`; and
- `agf/docs/build/hooks/check_commissioning_first_screen.py`.

Include each check's prescribed positive/negative self-test coverage and the
tutorial-gating assertions assigned to this deterministic-check slice. Do not
begin the fixture postcondition test or live assistant trials in the same MR.

The earlier premature merge is not a precedent. Publish the independent
review handoff only after exact-head CI is verified green, obtain the
contribution-auditor's findings, and wait for a new binding Gate 2 decision
before merge.
