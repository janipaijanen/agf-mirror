---
kind: decision-record
from: governor-reviewer
to: planner-architect,contribution-auditor,infosec-sentinel
id: 20261023T155713Z-governor-reviewer-ptds
timestamp: 2026-10-23T15:57:13Z
model: human-via-codex
immutable: true
references-to: []
topic: agov-strict-args-ref-list-scope-and-assignment
requires-ack: true
artifacts_owned:
  - docs/plans/20261023T155559Z-agov-strict-args-ref-list.md
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Governor decision: prepare the plan, then implement after Gate 1

The governor told Codex in the live session on 2026-10-23:
“proposal approved. plan first, and then implement. is adr needed?” This
record is filed by Codex on the governor's behalf under the governor's prior
explicit instruction to file these records.

The approved proposal scope is limited to the first two newly demonstrated
gaps:

1. make every `agov-mcp` tool fail closed on unknown argument keys; and
2. add a local, read-only Git-ref mode to `agov coord list`, including its
   MCP mapping and valid JSON for the empty-result edge.

Codex is temporarily assigned the planner-architect and subsequent
coder-tester work for this bounded task. This allocation resolves ownership
for this task only; it does not change the role manifest or grant authority
outside the approved scope.

“Plan first” is binding sequencing, not advance approval of plan text that did
not yet exist. The exact plan remains `pending-gate1` until its required
advisory rows are resolved, validation and collision checks pass, and a later
governor-reviewer record approves that exact plan head. Implementation may
then proceed; Gate 2 and merge remain separate decisions.

Codex assessed that no ADR is needed because the change extends existing CLI
and MCP adapter contracts without changing architecture, persistence,
authority, or protocol ownership. That engineering assessment is part of the
plan for review, not a new governance rule.
