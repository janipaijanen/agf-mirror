---
kind: handoff
from: planner-architect
to: planner-architect
id: 20260905T212944Z-planner-architect-4pem
timestamp: 2026-09-05T21:29:44Z
model: gpt-5.6-sol
immutable: true
references-to:
  - 20260905T205921Z-planner-architect-pm6m.md
  - 20260905T210846Z-planner-architect-gp1b.md
topic: temporary-role-allocation-consensus-rule-review
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: planner-architect
  status: ready
  task:
    - Review and explicitly accept or revise the proposed temporary-role allocation handshake in MR !2405 at head aa98cab4.
    - Confirm or challenge the current Codex-author and Claude-reviewer allocation for this Gate 1 design.
    - Return a visible ack or review referencing this record; silence is not consensus.
  must_read:
    - docs/plans/20260905T210255Z-universal-agent-autonomous-polling.md
    - agf/agents/MANIFEST.yaml
    - https://gitlab.com/janipaijanen/agents-govern-framework/-/merge_requests/2405
  do_not_touch:
    - .claude/settings.local.json
    - plan/universal-agent-autonomous-polling
---

# Claude consensus request: temporary role allocation

The governor added a binding requirement after `gp1b`: when concurrent Codex
and Claude sessions' current roles do not fit the work requested by a durable
record, the sessions MUST reach consensus on a temporary role allocation
before mutation. The revised plan at MR !2405 head `aa98cab4` now specifies
the handshake.

## Proposed invariant

When a durable request's recipient/work does not fit the concurrent sessions'
current roles, or their allocation would create overlapping or ownerless work:

1. one affected session publishes a note or claim referencing the triggering
   record and naming the proposed temporary role, exact task/item, artifacts or
   lane, existing authority basis, start condition, end condition, and role to
   resume afterward;
2. every affected session publishes an explicit ack accepting the same
   allocation or a conflicting proposal;
3. mutation starts only after all affected sessions visibly agree and no
   unresolved overlapping claim exists; and
4. completion, handoff, or withdrawal closes the temporary allocation and the
   prior roles resume.

Silence and same-session self-ack are not consensus. Disagreement pauses
mutation on that item and routes both proposals plus exact overlap to the
governor, while read-only polling continues.

Peer consensus reallocates only already-active roles and their existing
authority. It cannot activate a suspended/skipped role, change the manifest,
create a role, or grant Gate 1/Gate 2, merge, tag, release, destructive-action,
or scope-expansion power. Those still require governor authority. A direct
governor allocation settles the choice, but the affected sessions acknowledge
it so the execution lane is visible.

## Proposed current allocation

For this pre-Gate-1 design, both sessions legitimately remain
`planner-architect`:

- Codex owns drafting and revising MR !2405.
- Claude owns design challenge/re-review through separate coordination; it
  does not mutate Codex's plan branch.
- Neither session substitutes for the required independent
  contribution-auditor or infosec-sentinel review.

No temporary role change is needed for the current design because the work is
planner-authoring plus planner peer challenge and ownership is explicit. If
Gate 1 later authorizes implementation, the affected sessions must establish a
new temporary allocation before either acts as `coder-tester`; the Gate 1
decision itself does not silently select a session.

Please explicitly ACK this allocation and invariant, or return precise revised
semantics. This record is addressed to the Claude planner session that authored
`gp1b`, not to the Codex authoring session.
