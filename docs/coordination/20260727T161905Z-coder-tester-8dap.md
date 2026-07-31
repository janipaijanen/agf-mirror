---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260727T161905Z-coder-tester-8dap
timestamp: 2026-07-27T16:19:05Z
model: codex-gpt-5
immutable: true
references-to: []
topic: coord-visibility-performance-next-release
requires-ack: false
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Draft a Gate 1 plan for improving agov coord visibility performance in the next release.
    - Include assistant-facing guidance for fetched remote bases and targeted branch scans.
    - Preserve deterministic output, exit-code semantics, and token-ROI telemetry.
  must_read:
    - docs/reference/coord-visibility.md
    - docs/reference/tooling-token-roi.md
    - tooling/rust/crates/agov/src/main.rs
  do_not_touch:
    - tooling/elixir
    - agf/VERSION
    - any release branch or tag
---

# Plan `agov coord visibility` scaling work for the next release

The governor requested this improvement for the next release after a live
assistant poll repeatedly appeared to hang. A bounded trace established that
the command was making progress locally, not waiting on GitLab, but scales
poorly because it loads coordination records with individual `git show`
subprocesses and repeats branch inspection across a branch-heavy checkout.

Observed on this checkout (2026-07-27 UTC):

- default all-local-branch scan exceeded 15 seconds and was still processing;
- the 15-second trace recorded 787 attempted Git executions;
- checkout scale was roughly 497 local+remote refs and 794 coordination files;
- a targeted scan against stale local `main` took 6.1 seconds and emitted
  168 KB of misleading branch-local findings;
- the same targeted branch against fetched `origin/main` took 5.6 seconds and
  emitted 2.3 KB of useful deterministic output.

The plan should evaluate batching object reads (for example `git cat-file
--batch`), avoiding repeated base-message parsing, and bounding default branch
selection without losing hidden-obligation coverage. Add regression/performance
fixtures representative of hundreds of messages and branches.

Documentation in the same change should tell assistants to fetch first, use
`origin/main`, and pass explicit `--branch` values for routine polling; reserve
the unbounded scan for maintenance/audit runs. Non-zero exit codes for real
obligations or historical sequence collisions must remain distinct from
timeouts or operational failures.

This handoff authorizes planning only. Implementation begins after Gate 1
approval; no version, tag, or release action is authorized here.
