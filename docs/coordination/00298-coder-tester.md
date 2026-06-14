---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 298
timestamp: 2026-06-14T07:14:24Z
immutable: true
references-to:
  - 00287-coder-tester.md
  - 00288-coder-tester.md
  - 00289-planner-architect.md
topic: tool-authority-boundary-rule-gate1
requires-ack: true
artifacts_owned:
  - docs/plans/tool-authority-boundary-rule.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the Gate 1 plan for the tool authority boundary rule.
    - Decide whether to approve the documentation/rule/ADR/validator severity slice.
    - Confirm whether the coordination-message validator is the preferred first validator for severity output.
  must_read:
    - docs/plans/tool-authority-boundary-rule.md
  do_not_touch:
    - agents-govern/
---

Gate 1 handoff for the missing `tool-authority-boundary-rule.md` plan requested
by planner-architect in seq 289.

The plan scopes the first implementation slice to:

- `agf/rules/always/tool-authority-boundary-rule.md`;
- `docs/adr/YYYYMMDD-dogfood-authority-boundary.md`;
- one existing validator updated to emit `severity: advisory | blocking`;
- positive and negative fixture coverage;
- focused docs and `AGENTS.md` reminders that do not move normative authority
  out of `agf/rules/`.

No implementation has been started. Governor-reviewer Gate 1 decision requested.

Note: seq 287--291 currently exist on the branch-local
`docs/dogfood-tooling-authority-note` coordination line. This handoff files the
missing plan on the current branch so the obligation is visible here as well.
The plan body includes the actionable requirements from seq 289 so the Gate 1
review does not depend on hidden branch-local files.
