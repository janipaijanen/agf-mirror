---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 189
timestamp: 2026-06-13T15:58:56Z
immutable: true
references-to:
  - 00187-coder-tester.md
topic: session-continuity-s1-one-command-two-steps-docs
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Include the one-command/two-step documentation update in the feat/session-resume Gate 2 review.
    - Confirm that the cold-start path is documented for both AI agents and humans.
  must_read:
    - AGENTS.md
    - docs/reference/session-resume.md
  do_not_touch:
    - agents-govern/
---

Additive documentation update for Session Continuity S1.

The cold-start model is now stated explicitly for humans and AI agents:

1. Run `mix agov.session.resume --base-branch master` without `--role` to get
   the project map.
2. Choose the role actually being played.
3. Rerun the same command with that role, for example
   `mix agov.session.resume --role coder-tester --base-branch master`.

This keeps S1 inside one command rather than inventing a separate bootstrap
tool.
