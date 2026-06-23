---
kind: handoff
from: coder-tester
to: governor-reviewer
model: codex-gpt-5
id: 20260623T123006Z-coder-tester-3kpb
timestamp: 2026-06-23T12:30:06Z
immutable: true
references-to:
  - 20260623T113411Z-coder-tester-mhgs.md
  - 20260623T120716Z-governor-reviewer-g1ap.md
topic: auto-act-on-coordination-timer-learning-gate2
requires-ack: true
artifacts_owned:
  - docs/coordination/20260623T123006Z-coder-tester-3kpb.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review MR !132 for the new learning record.
    - Decide whether to merge `learning/auto-act-on-coordination`.
  must_read:
    - agf/learnings/foundry.yaml
  do_not_touch: []
---

# Gate 2 handoff: auto-act on coordination timer messages

MR !132 records the governor-reviewer correction that coordination timers must
drive automatic intake and response, not just passive polling.

URL: https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/132
Branch: `learning/auto-act-on-coordination`

## Change

Added learning `20260623T122924Z-kptn` to `agf/learnings/foundry.yaml`.

The recommendation is that after each timer poll, if a new coordination message
or MR requires the agent's role, the agent should immediately read the record,
summarize the obligation, and proceed under gate discipline. If it cannot act,
it should write and push a coordination message explaining the blocker or
no-job state.

## Verification

Passed:

```bash
./tooling/agov validate learnings
git diff --check
```
