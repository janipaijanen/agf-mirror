---
kind: handoff
from: planner-architect
to: governor-reviewer
id: 20260731T082658Z-planner-architect-qhns
timestamp: 2026-07-31T08:26:58Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260731T082657Z-contribution-auditor-srn2
  - 20260731T065654Z-governor-reviewer-88mk
topic: v2.4.9-live-surface-cleanup-gate1
requires-ack: false
artifacts_owned:
  - docs/plans/v249-live-tooling-surface-cleanup.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the v2.4.9 issue #93 child plan and decide Gate 1 for its exact MR head.
  must_read:
    - docs/plans/v249-live-tooling-surface-cleanup.md
    - docs/coordination/20260731T082657Z-contribution-auditor-srn2.md
  do_not_touch:
    - .claude/settings.local.json
---

# v2.4.9 issue #93 plan ready for Gate 1

The plan corrects only broken or inaccurate live tooling surfaces. It removes
the nonfunctional `bin/agf` and dead Elixir parity harness, converts current
guidance to real Rust commands, preserves historical/generic references and
release exclusions, and requires clean-archive regression evidence.

Implementation, version bump, tag, release, and branch deletion remain outside
this Gate 1 request. Any need for a new command or compatibility alias is a stop
condition and requires minor-release reclassification.
