---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 261
timestamp: 2026-06-13T21:28:00Z
immutable: true
references-to:
  - docs/plans/token-roi-telemetry-poc.md
  - docs/reference/tooling-token-roi.md
topic: token-roi-telemetry-poc-gate2
requires-ack: true
artifacts_owned:
  - AGENTS.md
  - docs/plans/token-roi-telemetry-poc.md
  - docs/reference/tooling-token-roi.md
  - tooling/elixir/lib/agov/token_roi.ex
  - tooling/elixir/lib/mix/tasks/agov.token.roi.ex
  - tooling/elixir/lib/mix/tasks/agov.session.resume.ex
  - tooling/elixir/lib/mix/tasks/agov.handoff.next.ex
  - tooling/elixir/test/agov/token_roi_test.exs
model: codex
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review `feat/token-roi-poc` for Gate 2.
    - Decide whether the local JSONL token ROI telemetry POC is acceptable.
    - Confirm whether automatic collection should be extended to more deterministic commands in a follow-up.
  must_read:
    - docs/plans/token-roi-telemetry-poc.md
    - docs/reference/tooling-token-roi.md
    - tooling/elixir/lib/agov/token_roi.ex
    - tooling/elixir/lib/mix/tasks/agov.token.roi.ex
    - tooling/elixir/test/agov/token_roi_test.exs
  do_not_touch:
    - agents-govern/
    - agents-govern/agents/MANIFEST.yaml
    - agents-govern/learnings/foundry.yaml
---

# Gate 2 Handoff: token ROI telemetry POC

Branch: `feat/token-roi-poc`

Summary:

- Adds `Agov.TokenRoi`, a local JSONL telemetry helper for deterministic
  tool-output character counts and approximate token estimates.
- Adds `mix agov.token.roi report` and `mix agov.token.roi record`.
- Automatically records successful output sizes for:
  - `mix agov.session.resume`
  - `mix agov.handoff.next`
- Adds `docs/reference/tooling-token-roi.md` to current master-lineage branch
  and links it from `AGENTS.md`.

Privacy/safety boundary:

- Default log path is `../../tmp/agov-token-roi.jsonl`, covered by the existing
  root `tmp/` ignore rule.
- `AGOV_TOKEN_ROI_LOG` can override the path.
- `AGOV_TOKEN_ROI=0` disables automatic writes.
- Records store command name, arguments, workflow label, character count,
  approximate token count, and timestamp. They do not store prompts, command
  output bodies, model responses, secrets, billing data, or remote telemetry.

Validation:

```bash
cd tooling/elixir
mise exec -- mix test test/agov/token_roi_test.exs
mise exec -- mix compile --warnings-as-errors
mise exec -- mix test
mise exec -- mix agov.plan.validate ../../docs/plans/token-roi-telemetry-poc.md
mise exec -- mix agov.coord.validate ../../docs/coordination/00261-coder-tester.md
git diff --check
```

Manual smoke:

```bash
mix agov.token.roi record --log <tmp-log> --command manual --chars 16 --manual-chars 80 --workflow smoke
AGOV_TOKEN_ROI_LOG=<tmp-log> mix agov.session.resume --base-branch master
mix agov.token.roi report --log <tmp-log>
```

Results: OK. Full test suite passed with 135 tests after clearing stale
`/tmp/agov_session_resume_*` test fixture directories. The first full-suite run
hit that pre-existing temp-dir collision in `SessionResumeTest`; no product
code change was needed for the rerun.
