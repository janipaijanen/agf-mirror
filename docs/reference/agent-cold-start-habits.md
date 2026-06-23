# Agent Cold-Start Habits

This page records durable habits for AI agents returning to AGF work after a
fresh session, compaction, model handoff, or long pause. It complements
`agov session resume`; it does not replace gate discipline or coordination
records.

## Start From Governed Evidence

Use repository state and pushed coordination records as the source of truth.
Session memory, chat summaries, and unpushed files are advisory until they are
backed by governed artifacts.

1. Run `agov session resume --base-branch main`.
2. Choose the role you are actually playing.
3. Run `agov session resume --role <role> --base-branch main`.
4. Check `agov coord status --recent 20 --roles` or the relevant graph/index
   view when available.
5. Read pending handoffs, must-read files, and recent decisions before editing.
6. If Git transport is stale or broken, use the GitLab/API source of truth and
   say that you did.

When the coordination channel is large, run `agov coord graph-index` after
`coord status`. Treat `pending` as the first handoff queue to inspect, and use
`nodes[].resolved` only as a handoff state: `true` means resolved, `false`
means pending, and `null` means the message is not a handoff.

## Treat Timers As Intake

A timer is useful only if it drives action. After each poll, read new messages
addressed to your role and classify them:

- act under gate discipline;
- acknowledge;
- write a coordination response explaining a blocker;
- record that no action is available yet.

Do not leave the human as the dispatcher for messages your role can see.

## Keep The Audit Trail Honest

Coordination messages count only when they are visible to other agents through
the shared remote. A local file or local commit is draft state.

- Validate coordination messages before publishing them.
- Commit and push, or publish through an approved API path.
- Verify the record is visible on the branch or MR where the recipient will
  look.
- Do not put mutable generated indexes inside `docs/coordination/`.

## Preserve Design Boundaries

When a design discussion is still forming, keep these boundaries explicit:

- Roles carry authority; model names and session labels are provenance only.
- Prefer session references over unstable actor/model strings when explaining
  why a decision happened.
- Use deterministic summaries for indexes; do not turn LLM-generated summaries
  into governance evidence.
- Build reversible, read-only infrastructure before implementing validators or
  manifests that grant, revoke, or block authority.
- Ask for hard teardown before schema work, and make the teardown useful to the
  next colleague: name rejected assumptions, surviving claims, open questions,
  and the next smallest reversible step.

## Leave The Next Agent A Handle

Before compaction, shutdown, or handoff, write enough durable state for a
cold-starting agent to continue:

- branch and latest relevant commit;
- files changed or proposed;
- validation/test results;
- open blockers;
- next concrete action;
- coordination IDs that explain the decision path.

If there is no work to do, say so in coordination when another agent is waiting
on your role.
