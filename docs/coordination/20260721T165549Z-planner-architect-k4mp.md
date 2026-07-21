---
artifacts_owned: []
from: planner-architect
handoff:
  do_not_touch:
  - .claude/settings.local.json
  - tooling/elixir/mix.lock
  must_read:
  - agf/rules/always/tool-authority-boundary-rule.md
  - agf/rules/workflow/deputy-governor-rule.md
  - docs/plans/principal-delegation-v2x-docs.md
  next_agent: coder-tester
  status: ready
  task:
  - Co-draft a concept note (not yet a Gate 1 plan) covering vocabulary, threat model outline, and a safe-mode capability/sub-agent inventory approach
  - Critique/extend the starting outline below before either of us commits to a full plan draft
id: 20260721T165549Z-planner-architect-k4mp
immutable: true
kind: handoff
model: claude-sonnet-5
references-to:
- 01KVKBF4M1CVSH3JBR9F0C226P.md
- 01KVKFRWVM2CGWQ3R7G7NN592K.md
- 20260623T171114Z-coder-tester-bmkm.md
- 20260623T172743Z-governor-reviewer-gcap.md
requires-ack: true
timestamp: 2026-07-21T16:55:49Z
to: coder-tester
topic: subagent-delegation-vocabulary-concept-draft
---

# Re-opening: sub-agent/delegation concept, picking up where bmkm/gcap left off

The governor asked (in chat with Claude, not yet in this channel) what
happened to the "MCPs, agents, sub-agents" question from `01KVKBF4M1CVSH3JBR9F0C226P`
/ `01KVKFRWVM2CGWQ3R7G7NN592K`. Answer, traced through the channel: MCP got
fully built out (`agov-mcp`, Level 4 Validated, plus the coordination-dir
contract fix). The sub-agent/delegation third of the question stalled after
`bmkm`'s teardown and `gcap`'s agreement — both of you converged on a cheap
next step (vocabulary + threat model as a docs-only Gate 1 artifact, safe-mode
inventory only, write-capable delegation blocked until enforcement exists)
that was never drafted. Re-opening it, per the governor's explicit request
that we draft the concept together before either of us commits to a full
Gate 1 plan.

## Starting outline — please critique/extend before I write more

1. **Reframe the abstraction.** MCP servers, sub-agents, local models, and
   validators are all capability providers under `tool-authority-boundary-rule`'s
   existing principle (tools produce evidence; named rules grant authority).
   This concept note should not create a sub-agent-specific model — it should
   extend the same one MCP already uses.

2. **Vocabulary (carried over from `bmkm`, still holds):**
   `capability_available` / `authority_granted` / `evidence_observed` /
   `action_taken`. Propose keeping these four terms as-is rather than
   re-litigating them.

3. **Claim worth stress-testing: sub-agents mostly already work today with
   zero new AGF feature.** A sub-agent with file read/write tools already
   follows `agf/rules/` and `agf/agents/` exactly like the primary session,
   because AGF's whole design is "any AI assistant that reads instructions
   from files." If this holds, the interesting gap isn't "do we support
   sub-agents" — it's narrower.

4. **The actual gap, if (3) holds: attribution.** A coordination entry today
   doesn't distinguish "written by the primary session" from "written by a
   sub-agent the primary session spawned." That's an audit-trail granularity
   problem (an `action_taken` with an ambiguous actor), not a missing product
   feature. Worth scoping the concept note around fixing *this* specifically,
   rather than a general delegation model.

5. **Threat model, narrowed to what's actually relevant here** (drop the
   MCP-specific items from `bmkm`'s list that don't apply to same-process
   sub-agents: credential laundering across network boundaries, external-world
   action surfaces). Keep: confused deputy, forged evidence, capability
   escalation through chained sub-agent spawns, action replay, stale/ambiguous
   attribution.

6. **Non-delegables stay untouched.** `deputy-governor-rule`'s red list
   (Gate 1/Gate 2 approval, agent-definition changes) is not reopened by this
   concept. Nothing here proposes sub-agents gaining approval authority.

7. **Maturity ceiling: Level 1-2 (Planned/Schema-defined) only**, per
   `implementation-honesty-rule`, until real usage — foundry's own dogfooding
   or an adopter's — validates any of it. No adopter-facing "AGF supports
   sub-agent delegation" claim until then.

8. **Explicitly out of scope for this concept note:** new schema fields, new
   `agov validate agents` enforcement, write-capable delegation. Those come
   later, gated behind evidence, if this concept survives review.

## Open question for you

Does this stay a standalone concept, or is it now a natural extension of
`agov-mcp`'s existing types (MCP shipped since this thread stalled — is there
reusable shape there for sub-agent attribution, or is that a false economy)?

Reply with critique, and I'll fold it into a joint draft before this becomes
a Gate 1 plan.
