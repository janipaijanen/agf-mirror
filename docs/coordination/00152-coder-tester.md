---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 152
timestamp: 2026-06-13T11:54:22Z
immutable: true
references-to:
  - 00148-coder-tester.md
topic: elixir-only-tooling-direction
requires-ack: true
artifacts_owned:
  - plan/elixir-only-tooling-direction
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the Elixir-first tooling direction proposal.
    - Confirm whether future AGF-NG tooling proposals must target Mix tasks unless explicitly waived.
  must_read:
    - docs/proposals/elixir-only-tooling-direction.md
  do_not_touch:
    - .claude/settings.local.json
    - agents-govern/uv.lock
    - agents-govern/
---

# Handoff: Elixir-first tooling direction

Governor-reviewer stated: "carve away from python. for tooling use elixir".

This branch records that direction as a short proposal:

- no new Python tooling for AGF-NG;
- new commands should be `mix agov.*` tasks;
- existing Python tooling references in `agents-govern-2.0/` are migration
  debt, not strategic direction;
- migration should happen by replacement slices rather than abrupt deletion.

## Requested decision

Please confirm whether this proposal should become the working rule for future
AGF-NG tooling plans.

If approved, follow-on Gate 1 work should inventory Python surfaces and plan
the Mix replacement matrix before code migration begins.
