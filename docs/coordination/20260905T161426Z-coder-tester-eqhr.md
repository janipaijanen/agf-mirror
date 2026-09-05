---
artifacts_owned: []
from: coder-tester
handoff:
  do_not_touch:
  - docs/reference/tooling-token-roi.md
  must_read:
  - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2383
  - docs/plans/20260905T144657Z-roi-workflow-baseline-repair.md
  next_agent: governor-reviewer
  status: ready
  task:
  - Review MR !2383 at exact head c922e682544965cd3e9cc4077a763486f7414340; merge only after canonical pipeline 2822977825 succeeds.
id: 20260905T161426Z-coder-tester-eqhr
immutable: true
kind: handoff
model: codex-gpt-5
references-to:
- 20260905T144657Z-planner-architect-2s3a.md
requires-ack: false
timestamp: 2026-09-05T16:14:26Z
to: governor-reviewer
topic: roi-workflow-baseline-repair-gate2
---

# ROI workflow-baseline repair ready for Gate 2 review

MR !2383 implements the approved plan as a one-file change at exact head
`c922e682544965cd3e9cc4077a763486f7414340`.

## Implemented

- `cold-start` and non-empty `session-resume:<role>` workflows now receive
  the documented 3,000–5,000 live baseline;
- non-empty `next-handoff:<role>` workflows receive the documented
  1,000–3,000 live baseline;
- all validation mappings retain their existing values and
  `#validation-baseline` source;
- unknown, missing-role, and empty-role workflow names remain unestimated;
- actual emitted CLI- and MCP-attributed records are tested to have identical
  baseline source and saved-token arithmetic.

No telemetry history is rewritten. No fields, network behavior, persistence
destination, or release metadata are changed.

## Evidence

- `cargo test --manifest-path tooling/rust/Cargo.toml -p agov-core`:
  35 passed;
- `cargo test --manifest-path tooling/rust/Cargo.toml --workspace` at the
  exact MR head: all non-ignored tests passed;
- `cargo fmt --manifest-path tooling/rust/Cargo.toml --package agov-core
  -- --check`: passed;
- `git diff --check`: passed.

Canonical exact-head MR pipeline `2822977825` is running on the private
runner and remains a binding merge precondition. Technical review may begin
now; do not merge until that pipeline succeeds and Gate 2 is explicitly
approved.
