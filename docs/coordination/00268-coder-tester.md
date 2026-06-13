---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 268
timestamp: 2026-06-14T01:36:00+03:00
immutable: true
references-to:
  - docs/coordination/00266-coder-tester.md
topic: token-roi-live-savings-gate2-bridge
requires-ack: true
artifacts_owned:
  - .worktrees/token-roi-live-savings
model: codex
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review branch `feat/token-roi-live-savings`.
    - Decide Gate 2 for automatic potential saved-token estimates in token ROI telemetry.
    - Confirm whether the documented baseline ranges may be used for live estimates.
  must_read:
    - .worktrees/token-roi-live-savings/docs/plans/token-roi-live-savings.md
    - .worktrees/token-roi-live-savings/docs/coordination/00267-coder-tester.md
    - .worktrees/token-roi-live-savings/docs/reference/tooling-token-roi.md
    - .worktrees/token-roi-live-savings/tooling/elixir/lib/agov/token_roi.ex
    - .worktrees/token-roi-live-savings/tooling/elixir/test/agov/token_roi_test.exs
  do_not_touch:
    - agents-govern/
    - agents-govern/agents/MANIFEST.yaml
    - agents-govern/learnings/foundry.yaml
---

Visibility bridge for token ROI live savings estimates.

Branch: `feat/token-roi-live-savings`

Branch commit: `abc0cd9 feat(tooling): estimate live token ROI savings`

Branch-local record:

- `00267-coder-tester.md` is the detailed Gate 2 handoff.

Summary:

The initial token ROI POC logged live tool usage and output size, but it only
computed saved-token estimates when a manual baseline was recorded. This branch
extends live telemetry so known workflows also record potential savings using
documented baseline ranges:

- `cold-start` and `session-resume:<role>`: 3,000-5,000 manual tokens;
- `next-handoff:<role>`: 1,000-3,000 manual tokens.

Each matching live record gets baseline range, prompt-overhead estimate, and
`approx_saved_min_tokens` / `approx_saved_max_tokens`. Reports roll up saved
ranges by command and workflow.

Validation on the branch:

```bash
cd tooling/elixir
mise exec -- mix test test/agov/token_roi_test.exs
mise exec -- mix compile --warnings-as-errors
mise exec -- mix test
mise exec -- mix agov.plan.validate ../../docs/plans/token-roi-live-savings.md
mise exec -- mix agov.coord.validate ../../docs/coordination/00267-coder-tester.md
git diff --check
```

Results: OK. Full suite passed with 136 tests. Smoke report showed
`potential saved: 2430-4430` for one live `session.resume` cold-start record.
