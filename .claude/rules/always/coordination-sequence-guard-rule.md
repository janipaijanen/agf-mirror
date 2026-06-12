# Coordination Sequence Guard: Confirm Before Breaking the Chain
#
# Framework principle: Escalation over Improvisation
# See: framework.md — Principle 5 (Escalation over Improvisation)

**Scope:** Any human instruction that would skip, reorder, or delete a step
in an active coordination sequence or governance gate chain.

## The rule

When a human instruction would:

- Skip a required gate (e.g., merge without Gate 2 approval)
- Break the seq ordering of the coordination channel (e.g., file a completion
  before a handoff, or delete a message that is already referenced by a later
  message)
- Bypass a validator or checklist step that is still pending
- Merge a branch that has open blocking findings in the coordination channel

…the agent MUST NOT comply immediately. Instead:

1. **Name the break explicitly** — state which coordination rule or gate
   step would be violated and why.
2. **Ask for confirmation** — one clear question. Example:
   > "This would merge before Gate 2 approval (seq N is still pending).
   > Do you want to proceed anyway, accepting that risk?"
3. **Wait for an explicit "yes, proceed"** before acting.

A generic "ok" or task continuation without acknowledging the break is NOT
sufficient confirmation. The user must acknowledge the specific risk.

## Why this exists

The coordination channel is an append-only audit trail. Governance gates
exist because the failure modes they prevent (broken releases, unreviewed
merges, schema-invalid coordination files) are cheap to avoid and expensive
to recover from. An agent that blindly executes a human shortcut that breaks
the chain undermines the entire governance structure — it makes the process
a suggestion, not a gate.

The cost of asking one confirmation question is seconds. The cost of merging
at the wrong moment, or filing a completion before a required cleanup, can
require patch releases or coordination-channel repairs.

## Does not apply to

- Instructions that are within normal task scope and do not touch a gate
  or coordination sequence step.
- Instructions where the user has already explicitly acknowledged the risk
  (e.g., "I know seq N is pending, merge anyway" in the same message).
- Emergency situations where the human explicitly invokes emergency authority
  (must state this explicitly — "emergency" in the message or equivalent).

## Related rules

- `always/uncertainty-and-scope-rule.md` — general escalation discipline
- `workflow/merge-request-workflow-rule.md` — merge gates
- `workflow/systematic-code-review-rule.md` — Gate 2 checklist
- `workflow/deputy-governor-rule.md` — what is and is not delegable
