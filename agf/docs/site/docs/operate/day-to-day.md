# Day-to-day operation

*Status: Released (v2.1 documentation). Audience: developers and
governors running a governed project. After this page you know the
rhythm: claims, fast track vs full gates, coordination hygiene, and the
failure modes to watch for.*

!!! note "AGF v2 layout"
    The examples use the v2 default evidence paths: `docs/plans/` and
    `docs/coordination/`. Keep one coordination channel per project.

## The daily shape

The operating model is easiest to read as three responsibility modes:
human-only authority, AI-executed work, and mixed work in the audit
trail. **AI-executed** means an agent may do the task after delegation;
it does not mean the agent can approve its own gate, accept project
risk, or decide exit.

![Day-to-day responsibility modes](../../reference/figures/operating-responsibility-map.svg)

Table 1 separates the work by authority mode so teams can see what remains
human-only, what an AI agent may execute, and what both sides handle together.

| Mode | Applies to | Typical artifacts |
|---|---|---|
| Human-only authority | Gate approvals, merge decisions, risk acceptance, role activation/deactivation, adopting or exiting the framework | Decision records, merge approvals, explicit governor instructions |
| AI-executed work | Drafting plans or docs, implementing scoped changes, running tests and validators, generating candidate figures, preparing handoffs | Branch commits, validation logs, generated artifacts, Gate 2 handoffs |
| Mixed human + AI work | Reading the coordination channel, sizing tasks, reviewing evidence, sharpening docs and rules, recording learnings | Claims, reviews, handoffs, learning records, rule or documentation changes |

A governed working session, in order:

Figure 2 shows the daily loop as a cycle: read the channel, claim or size the
work, pass the appropriate gate, implement, review, and close the loop with a
completion or learning.

![Governance loop](../../reference/figures/governance-loop.svg)

1. **Read the channel.** New coordination entries since you last looked —
   pending handoffs addressed to your role, decision records that
   unblock work, acks owed.
2. **Claim before working.** If the task is non-trivial or another agent
   could plausibly pick it up, file a claim. Claims prevent two agents
   solving the same problem — or worse, colliding on the same branch.
3. **Size the task.** Fast track if it qualifies; plan + Gate 1 if not.
   When in doubt, the simplicity rule says choose the lighter process and
   upgrade if the task grows — stopping to write a plan mid-task is
   normal, not failure.
4. **Branch, implement, self-review, Gate 2, merge, completion.** The
   cycle from [your first governed change](../start/first-governed-change.md),
   now routine.

## Coordination hygiene

- **Acknowledge before acting.** A message that triggers action gets an
  entry referencing it — before acting (claiming) or immediately after
  (recording the outcome). Silent action breaks the causal chain.
- **Never edit a filed entry.** Files are immutable; corrections are new
  entries referencing the old.
- **Use tooling-generated readable IDs for new entries.** Historical
  sequence-numbered and ULID files remain immutable history, but new
  coordination records should use `agov coord brief --id-format readable`.
  If tooling is unavailable, create a valid manual ID and say why in the
  record.

## Context hygiene

Long AI sessions become expensive and less reliable as context grows. AGF
treats durable project artifacts as shared memory, not chat history.

Before compacting, clearing, switching tasks, or handing work to another
agent, write or request a short durable state note when the task is
non-trivial. In AI-assisted sessions, the shared instruction `/compact-agf`
means: save the current AGF state to `docs/coordination/` first, then compact.
After a restart, model switch, or cold start, run `agov session resume` before
continuing. The saved note should identify:

- current branch and latest relevant commit;
- files or artifacts changed;
- validations run and their result;
- open decisions or blockers;
- the next concrete action;
- coordination records or plans the next agent must read.

Prefer references to repository files over restating large content in chat.
Coordination records, plans, reviews, and generated summaries are the source of
continuity. Use compaction during long tasks only after state has been
externalized, and clear the session when switching to an unrelated task.

Do not rely on an AI session's private conversation memory for governance
decisions, approvals, or obligations.

## Multi-agent sessions

When more than one assistant works the repository concurrently:

- **Verify the branch before every commit** — `git branch --show-current`
  immediately before commit or push. The documented failure mode is an
  agent (or the human's IDE) switching branches under another agent's
  feet; the framework's own history includes foreign commits landing on a
  claimed feature branch.
- **One role, one session.** Two models filing under the same role
  concurrently need distinguishing claims, or their authority and their
  artifacts blur.
- **Force-push only with `--force-with-lease`**, and never on a shared
  branch.

## The governor's day

The governor reads pending handoffs, decides gates, and audits at the
agreed cadence. Two disciplines matter most: **answer gates promptly**
(a stalled gate trains everyone to route around it) and **pull anything
that smells wrong into full review without justification** — the right
to pull is what makes lightweight delegation safe.

## Failure modes to watch

Table 2 lists common operating smells and the usual governance weakness behind
each one.

| Smell | What it usually means |
|---|---|
| Coordination entries getting longer and rarer | Work is happening off-channel; the trail is decaying |
| Fast-track waivers on everything | Sizing discipline has slipped; audit the recent waivers |
| Gates approved in seconds, always | The reviewer is rubber-stamping; evidence isn't being read |
| Branches with mixed concerns | The sizing test is being skipped; reviews are getting harder |

## Next

- Gate 2 in depth, releases, and upgrades:
  [review and release](review-and-release.md)
