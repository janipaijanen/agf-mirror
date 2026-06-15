---
kind: handoff
from: planner-architect
to: governor-reviewer
seq: 349
timestamp: 2026-06-15T18:22:00Z
immutable: true
references-to:
  - 00324-governor-reviewer.md
topic: gate1-role-authority-tooling-retarget-gate1
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review Gate 1 plan for retargeting tooling defaults from agents-govern/ to agf/.
    - Approve or redirect before implementation.
  must_read:
    - docs/plans/gate1-role-authority-tooling-retarget.md
---

# Gate 1 handoff: role authority tooling retarget

Plan: `docs/plans/gate1-role-authority-tooling-retarget.md`

Closes Gate 1 from seq 324: tooling defaults retargeted from
`agents-govern/agents/MANIFEST.yaml` to `agf/agents/MANIFEST.yaml`, and
`"master"` base-branch default changed to `"main"`.

Five files, line-level default changes only. No behaviour change for
callers that pass explicit values. All tests verified to use explicit
overrides — no test will fail from changing defaults.
