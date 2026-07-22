# Coordination Message: Always Acknowledge Before Acting
#
# Framework principle: Audit trail completeness
# See: framework.md — Principle 2 (Artifact Ownership), Pattern 1 (Pipeline)
# Evidence: AGF-NG operational use — polling loop discipline

**Scope:** Any agent that receives a coordination message directed to it
(`to: <agent-role>` or `to: all`) and takes action in response.

## The rule

When a coordination message requires action, the acting agent MUST:

1. **Read the message completely** before taking any action.
2. **Write a coordination entry** that references the triggering message
   in its `references-to:` field — either before acting (to claim the
   action) or immediately after (to record the outcome).
3. **Never act silently** — an action without a coordination entry leaves
   a gap in the audit trail that no reviewer can reconstruct.

## What counts as "action"

- Implementing a change in response to a handoff or decision-record
- Filing a subsequent handoff or completion
- Merging a branch
- Running a validator and recording the result
- Any modification to tracked artifacts

## What does NOT require a coordination entry

- Reading state when a complete poll finds no new actionable obligation
- An ack that requires no substantive response (`requires-ack: false`
  and no action triggered)
- Internal reasoning steps that produce no external artifact

“Passive polling” ends as soon as the cycle finds an actionable obligation.
Reporting that obligation and waiting for the governor to repeat or redispatch
it is not an acceptable substitute for action. After fetching remote refs, the
agent must scan cross-branch coordination visibility, all open MR notes and
exact-head pipelines/jobs, and GitLab Issues. It then acts on the
highest-priority obligation already inside its role, approved scope, and
current gate, or records the concrete blocker. See
`docs/reference/agent-cold-start-habits.md` for the complete poll contract and
the decisions that still require governor authority.

## Entry format

The coordination entry referencing a triggering message:

```yaml
references-to:
  - 000NN-sender.md   # the message being acted on
```

The entry kind should reflect what the agent is doing:
- `note` — acknowledgment, status update, or interim response
- `handoff` — passing work to another agent
- `completion` — closing a task
- `review` — filing findings on a gate submission

## Why this exists

Silent actions break the coordination channel as an audit trail. When an
agent merges a branch, runs a validator, or files a completion without a
coordination entry, the only record is git history — which captures *what*
changed but not *why* or *in response to what*. The coordination channel's
value is the causal chain: message → action → outcome. A gap in that chain
forces reviewers to reconstruct intent from commit messages, which is
slower and less reliable than an explicit record.

## Related rules

- `workflow/systematic-code-review-rule.md` — Gate 2 reviewers check that
  the coordination trail is complete before approving merge.
- `always/coordination-sequence-guard-rule.md` — confirms before breaking
  the sequence; this rule ensures the sequence is written at all.
