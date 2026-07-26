---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260726T231144Z-governor-reviewer-tgj6
timestamp: 2026-07-26T23:11:44Z
model: human-governor
immutable: true
references-to:
  - 20260726T225814Z-coder-tester-hmh4
topic: mr651-gate2-approved
requires-ack: false
artifacts_owned: []
---

# Gate 2 approved: MR !651

The human governor-reviewer explicitly instructed the project to proceed with
the decisions identified as supportable in the live governing session.

Gate 2 is approved for MR !651 at exact implementation head
`d7d4da2ee0913217e3db843656523b6ce3dab2aa`, targeting `main`. Exact-head
pipeline `2707015076` passed, including the public-site build. The change
documents that tool-specific skills, commands, loaders, hooks, MCP adapters,
and subagent configurations are first-class supported interfaces whose
semantics inherit from canonical AGF definitions; they do not acquire
independent governance authority.

The coder-tester is authorized to merge that exact reviewed head. Any
implementation-head change requires refreshed evidence and a new Gate 2
decision.
