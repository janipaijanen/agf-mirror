# Agent Cold-Start Habits

This internal page records foundry habits for AI agents returning to AGF work after a
fresh session, compaction, model handoff, or long pause. It complements
`agov session resume`; it does not replace gate discipline or coordination
records. The adopter-facing canonical procedures are the public
`agf/docs/site/docs/operate/runbook.md`, `session-continuity.md`, and
`poll-and-coordinate.md`; do not copy this foundry checklist downstream.

## Start From Governed Evidence

Use repository state and pushed coordination records as the source of truth.
Session memory, chat summaries, and unpushed files are advisory until they are
backed by governed artifacts.

1. Run `agov session resume --base-branch main`.
2. Choose the role you are actually playing.
3. Run `agov session resume --role <role> --base-branch main`.
4. If repository-root `ai-cold-start.md` exists, read it as a local overlay.
5. Check `agov coord status --recent 20 --roles` or the relevant graph/index
   view when available.
6. Read pending handoffs, must-read files, and recent decisions before editing.
7. If Git transport is stale or broken, use the GitLab/API source of truth and
   say that you did.

The local overlay is for workstation/session reminders only. It must not
override `AGENTS.md`, AGF rules, gate discipline, coordination records, or role
authority, and it must not carry obligations that another agent is expected to
discover through the coordination channel.

When the coordination channel is large, run `agov coord graph-index` after
`coord status`. Treat `pending` as the first handoff queue to inspect, and use
`nodes[].resolved` only as a handoff state: `true` means resolved, `false`
means pending, and `null` means the message is not a handoff.

## Treat Timers As Intake

A timer is useful only if it drives action. AGF timer notation such as
`15mins/180min` means:

- scan every 15 minutes;
- stop only after 180 minutes of no new incoming activity;
- reset the 180-minute idle window whenever a new relevant coordination message
  or merge-request event arrives.

For framework development, a poll scans at least:

- fetched Git refs (`git fetch --prune origin` comes before comparison), recent
  coordination, and `agov coord visibility --base-branch main`; a checked-out
  `docs/coordination/` directory alone is not the shared inbox;
- every open merge request, regardless of author: new discussions and notes,
  approval/draft state, source and target SHAs, merge/close state, and
  exact-head pipeline and job changes;
- GitLab Issues: new notes, assignment/label changes, and open/close state;
- every exact SHA, branch, pipeline, job, tag, or release state the active role
  is already waiting on; and
- branch-watch context and last-seen IDs, timestamps, and SHAs, so “nothing
  new” names the fetched refs and GitLab scope actually checked.

MR-note bridges are intake, not noise. They may point to coordination published
on another branch. Follow the bridge, validate the referenced record, and use
the coordination record as the durable instruction. A poll that checks only
local files can miss both the record and a cost-sensitive pre-push instruction.

Use the assistant surface's internal reminder or wake-up mechanism when it
exists. Do not replace an internal timer with an external shell timer, cron job,
background `sleep`, or detached process: the assistant will not reliably notice
that process. If no internal timer exists, say so and scan when active or after
resume.

After each poll, read new messages addressed to your role or `all`, classify
them, and choose the highest-priority obligation. `Poll` is an action loop: do
not leave the human as the dispatcher for work your role can already see.

Act without another human prompt when all three conditions hold: the action is
inside the active role, it is already authorized by an approved plan or
handoff, and it stays inside the current gate. Typical actions are:

- acknowledge or claim the obligation and publish the causal coordination
  record;
- inspect fetched state, run diagnostics, validators, or local tests;
- continue an approved implementation or answer review findings within its
  accepted scope;
- correct an owned MR description, note, or handoff without creating an
  unnecessary code push;
- batch and push an owned correction after checking for newer instructions and
  predicting which CI sources the push will create;
- advance the next already-authorized handoff when its stated dependency
  clears; and
- cancel a demonstrably redundant paid pipeline for the same SHA while
  preserving the canonical acceptance pipeline, then record pipeline/job IDs
  and runner time already spent.

End the cycle only when the obligation is complete, actively running and being
monitored, or blocked with the concrete blocker recorded. Reporting an
actionable item while doing nothing is not completion.

Ask the governor only when a new binding decision is actually required: Gate 1
approval, Gate 2 approval or merge, release/tag/deployment, scope expansion,
destructive or irreversible action, new authority, a secrets/security
ambiguity, or materially conflicting instructions. Polling never grants these
powers. Do not retry failed paid CI merely to obtain green status; diagnose the
failure and fold the correction into the next required acceptance run unless a
governing test policy explicitly requires retry evidence.

## Keep The Audit Trail Honest

Coordination messages count only when they are visible to the intended
recipient through the shared remote, an approved shared path, or a visible
bridge note that names where to read them. A local file, untracked file, or
local-only commit is draft state.

- Validate coordination messages before publishing them.
- Commit and push, or publish through an approved API path.
- Verify the record is visible on the branch or MR where the recipient will
  look.
- If the recipient needs to act, tell them the coordination ID, branch, or MR
  where the record is visible.
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
