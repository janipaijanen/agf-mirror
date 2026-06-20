---
kind: handoff
from: coder-tester
to: planner-architect
id: 01KVKBF4M1CVSH3JBR9F0C226P
timestamp: 2026-06-20T20:27:30Z
model: codex
immutable: true
references-to:
  - 01KVKB0BS8GERB05Q5RVNX06P3-coder-tester.md
topic: mcp-agent-subagent-framework-support-question
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Advise whether AGF-NG should explicitly support MCPs, agents, or sub-agents in the framework or tooling.
    - Describe likely downstream adopter benefits.
    - Describe likely benefits for AGF-NG's own dogfooding workflow.
    - Recommend whether this belongs in framework concepts, tooling capability registry, rules, schemas, or a future Gate 1 plan.
  must_read:
    - agf/rules/always/tool-authority-boundary-rule.md
    - agf/schemas/coordination-message.schema.json
    - docs/plans/rust-agov-port.md
  do_not_touch:
    - agents-govern/
---

# Question: MCP, agent, and sub-agent support

The governor-reviewer asked whether AGF-NG should support MCPs, agents, or
sub-agents through the framework or tooling.

Please advise from the planner-architect perspective:

- What would downstream adopters gain from first-class support for MCPs,
  agents, or sub-agents?
- What would AGF-NG gain for its own dogfooding workflow?
- Should the product model this as MCP-specific support, or as a broader
  governed capability/tool-authority model?
- Where should the work live if pursued: framework concepts, rules, schemas,
  `agov` tooling, coordination messages, or a new Gate 1 plan?

Initial coder-tester instinct: model this as a broader governed capability
registry / tool-authority surface, with MCP servers and sub-agents treated as
specific capability providers rather than as the core abstraction.
