---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 267
timestamp: 2026-06-14T01:32:00+03:00
immutable: true
references-to:
  - docs/plans/token-roi-live-savings.md
  - docs/reference/tooling-token-roi.md
topic: token-roi-live-savings-gate2
requires-ack: true
artifacts_owned:
  - docs/plans/token-roi-live-savings.md
  - docs/reference/tooling-token-roi.md
  - tooling/elixir/lib/agov/token_roi.ex
  - tooling/elixir/test/agov/token_roi_test.exs
model: codex
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review branch `feat/token-roi-live-savings`.
    - Decide Gate 2 for automatic potential saved-token estimates in token ROI telemetry.
    - Confirm whether the documented baseline ranges are acceptable for live estimates.
  must_read:
    - docs/plans/token-roi-live-savings.md
    - docs/reference/tooling-token-roi.md
    - tooling/elixir/lib/agov/token_roi.ex
    - tooling/elixir/test/agov/token_roi_test.exs
  do_not_touch:
    - agents-govern/
    - agents-govern/agents/MANIFEST.yaml
    - agents-govern/learnings/foundry.yaml
---

# Gate 2 Handoff: token ROI live savings estimates

Branch: `feat/token-roi-live-savings`

Summary:

- Live `record_tool_output/4` records now include:
  - prompt-overhead token estimate;
  - documented manual-baseline range for known workflows;
  - `approx_saved_min_tokens`;
  - `approx_saved_max_tokens`.
- Known automatic baselines:
  - `cold-start` and `session-resume:<role>` use 3,000-5,000 manual tokens;
  - `next-handoff:<role>` uses 1,000-3,000 manual tokens.
- `mix agov.token.roi report` rolls up potential saved-token ranges by command
  and workflow.
- Manual `record --manual-chars` measurements still work and now also produce
  saved-token fields.

Safety boundary:

- No prompts, command output bodies, model responses, billing data, secrets, or
  remote telemetry are captured.
- Unknown workflows still record usage and output size without inventing a
  savings estimate.

Validation:

```bash
cd tooling/elixir
mise exec -- mix test test/agov/token_roi_test.exs
mise exec -- mix compile --warnings-as-errors
mise exec -- mix test
git diff --check
```

Manual smoke:

```bash
tmp_log=$(mktemp)
AGOV_TOKEN_ROI_LOG="$tmp_log" mix agov.session.resume --base-branch master >/dev/null
mix agov.token.roi report --log "$tmp_log"
```

Observed report included a live `cold-start` savings range:

```text
potential saved: 2430-4430
```

Results: OK. Full suite passed with 136 tests.
