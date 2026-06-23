---
kind: handoff
from: coder-tester
to: planner-architect
id: 20260623T212241Z-coder-tester-kdmr
timestamp: 2026-06-23T21:22:41Z
immutable: true
model: codex-gpt-5
references-to:
  - 20260623T210847Z-coder-tester-1enr.md
topic: token-roi-agent-log-split
requires-ack: false
artifacts_owned:
  - AGENTS.md
  - agf/docs/site/docs/adopt/tooling-and-tokens.md
  - agf/docs/site/reference/environment-variables.md
  - docs/coordination/20260623T212241Z-coder-tester-kdmr.md
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Configure Claude-run shells to use `AGOV_TOKEN_ROI_LOG=$PWD/tmp/claude-agov-token-roi.jsonl`.
    - Keep Codex-run shells on `AGOV_TOKEN_ROI_LOG=$PWD/tmp/codex-agov-token-roi.jsonl`.
    - Avoid writing either assistant's live ROI telemetry to the old shared default `tmp/agov-token-roi.jsonl`.
  must_read:
    - AGENTS.md
    - agf/docs/site/reference/environment-variables.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Handoff: split token ROI logs by assistant

Codex renamed the existing shared live ROI log from
`tmp/agov-token-roi.jsonl` to `tmp/codex-agov-token-roi.jsonl` in the working
checkout. That file remains ignored under `tmp/`; it is the Codex continuation
point, not a committed fixture.

Please configure Claude sessions to write their live ROI telemetry to:

```bash
export AGOV_TOKEN_ROI_LOG="$PWD/tmp/claude-agov-token-roi.jsonl"
```

Codex sessions should continue with:

```bash
export AGOV_TOKEN_ROI_LOG="$PWD/tmp/codex-agov-token-roi.jsonl"
```

I added persistent guidance to `AGENTS.md`, the adopter tooling page, and the
environment variable reference so future sessions on another machine know which
file belongs to which assistant. This avoids conflicting writes while keeping
stable continuation paths for later ROI analysis.
