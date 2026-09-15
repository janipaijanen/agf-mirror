<!-- SPDX-License-Identifier: MIT -->
<!-- Copyright (c) 2026 <!-- CUSTOMIZE: your name --> -->
<!-- AGF: 2.10.1 — update this comment after each framework upgrade to match agf/VERSION -->
# <!-- CUSTOMIZE: Project Name --> — Agent and Contributor Orientation

<!-- CUSTOMIZE: brief description of the project -->

**Project codename:** <!-- CUSTOMIZE: your codename (see ADOPTING.md §Step 0) -->
**AGF version:** <!-- CUSTOMIZE: match agf/VERSION -->

---

## Cold start

Canonical operating, interruption, recovery, and polling procedures are in
`agf/docs/site/docs/operate/runbook.md`. This loader points there; it must not
grow a competing copy of those procedures.

Every active development role/session follows
`agf/rules/always/universal-polling-and-autonomous-continuation-rule.md`:
observe the complete configured project surface, choose the highest-priority
authorized action, monitor work already started, and stop mutation only at a
named gate, authority boundary, conflict, or evidenced blocker. Scheduled wakes
are non-reentrant and coalesce while authorized work is active. Observation is
universal; execution remains role-, authority-, gate-, and claim-scoped.

Concurrent sessions whose roles would overlap or leave requested work
ownerless publish and acknowledge one temporary allocation before mutation;
consensus cannot create authority. Every active AI also tracks valid learnings
after the last completed capability review: propose one review at 10 new
records after checking for an existing proposal, and hand the baseline/count
to the governor if none has launched by 15. This trigger grants proposal
authority only and never activates a suspended role.

Projects may optionally activate the `improvement-propeller` reference role
for bounded weekly opportunity proposals and slower governance-friction
synthesis. Its canonical contract is
`agf/rules/always/improvement-propeller-rule.md`. Merely shipping that role and
rule does not activate them; local activation requires a governor decision
naming a player, evidence surfaces, real wake mechanism, cadence, and cursor
state.

Run these commands at the start of every session before acting or writing any
durable files:

```bash
# Discover active roles and pending coordination obligations
agov session resume --base-branch main

# Resume as a specific role
agov session resume --role <agent-role> --base-branch main

# Find pending handoffs directed at you
agov handoff next <agent-role>

# Check for pending items across all branches visible from main
agov coord visibility --base-branch main

# List unresolved non-handoff acknowledgment obligations (claim, review,
# note, decision-record, etc. with requires-ack: true) -- run this every
# cold start when the installed release exposes it; session resume does
# not surface this obligation class
agov coord pending
```

Run `session resume` before anything else — before reading coordination files,
before `git log`, before any file inspection. It surfaces pending coordination
messages, active decisions, and unresolved handoffs, but **not** non-handoff
`requires-ack: true` obligations of other kinds (claim, review, note,
decision-record, fast-track-waiver, lesson-proposal) — those are `agov coord
pending`'s surface, run separately as shown above. Starting with reads before
the resume means acting on stale context.

After the resume output, file a brief coordination `kind: note` for the current
cycle status before proceeding. This gives other agents and the governor a
current baseline rather than an invisible state from the previous session.

---

## AGF framework version

AGF governance files live under `agf/`. The active framework version is in
`agf/VERSION`. After upgrading AGF, update the `<!-- AGF: ... -->` comment at
the top of this file to match `agf/VERSION`.

If this project also supports Claude Code, copy `agf/templates/CLAUDE.md` to
`.claude/CLAUDE.md`. Its single `@../AGENTS.md` import keeps this file as the
one shared loader. Declare the supported surfaces in
`agf/agents/MANIFEST.yaml`; see
`agf/docs/site/docs/adopt/assistant-loader-configuration.md` for the accepted
values and compatibility behavior.

---

## Project layout

```
<!-- CUSTOMIZE: paste your project's directory tree here -->
agf/                # AGF framework files — do not edit without governor approval
docs/
  coordination/     # Immutable coordination channel (append-only)
  plans/            # Gate 1 plan files
```

---

## Rules

AGF governance rules are under `agf/rules/`. Read the relevant rules before
making any change. Core rules apply to every change; workflow and domain rules
apply per task type.

| Directory | Scope |
|---|---|
| `agf/rules/always/` | Core constraints — every change, no exceptions |
| `agf/rules/workflow/` | Process rules: TDD, debugging, branching, releases, code review |
| `agf/rules/docs/` | Documentation audience, marketing tone, implementation honesty |
| `agf/rules/ui/` | Accessibility (WCAG AA), template translatability |

<!-- CUSTOMIZE: list any project-specific rules here -->

---

## Coordination channel

Challenge lifecycle and advisory response handling: see `agf/rules/workflow/coordination-challenge-response-rule.md`.

Channel membership, bounded misplaced-record discovery and unassessed archive inventory are defined by `agf/rules/workflow/coordination-publication-rule.md`. Only direct lowercase `.md` files are channel members; archive exclusion never proves closure. Consult the visibility command reference for its explicit scan scopes and coverage limits.

Recipient-only session annotations for handoffs are defined by `agf/rules/workflow/coordination-publication-rule.md` (Advisory recipient session addressing). Top-level session is author provenance; a mismatch never hides or reprioritizes an obligation.

All inter-agent communication is recorded in `docs/coordination/` as immutable
Markdown files. Use the AGF tooling to generate IDs and validate files.

```bash
# Generate a coordination ID (readable format — preferred for new files)
agov coord brief --from <agent-role>

# Validate a coordination file
agov coord validate docs/coordination/<file>.md

# Validate a Gate 1 plan
agov plan validate docs/plans/<file>.md

# Pending item scan across the coordination channel
agov coord graph-index
```

Write a coordination entry whenever your action creates an obligation for
another agent or the human governor. File the entry before or immediately after
acting — never as an afterthought.

A coordination message is not filed until it is both committed **and pushed**.
A message that exists only in the local working tree or in an unpushed commit
is invisible to other agents and the human governor.

A coordination-only commit is expected to publish promptly and
independently of the Gate 2 approval of the work it describes — see
`agf/rules/workflow/coordination-publication-rule.md`. If this project has
wired mechanical verification for that fast path (e.g. `agov coord
publication-check` in CI), branch coordination-only commits from the target
branch, not from an implementation branch's tip — a title alone is not what
makes a candidate safe to fast-track. Batch same-cycle records when none is
independently blocking another actor; publish an urgent or waiting obligation
alone immediately. Never add `[ci skip]`/`[skip ci]` to a
coordination-only commit message: that marker skips pipeline creation
entirely at the platform level, before any CI configuration runs, so the
mechanical check would never get a chance to run.

---

## Gate discipline

A plan MAY declare `affected_paths` for machine-checkable overlap detection; `agov plan collision-check` reports overlaps advisory-only by default. A plan whose `affected_paths` overlaps another live plan's, unacknowledged by `depends_on`, blocks Gate 1 via `agov:governance:plan-collision-gate` -- see `agf/rules/workflow/development-workflow-rule.md`'s "Affected-paths collision blocking" section. Only plan-vs-plan overlaps block; a local branch's or another open MR's changed paths remain advisory-only.

Every non-trivial task requires two gates:

- **Gate 1** — planner-architect files a plan in `docs/plans/`; governor-reviewer
  approves before implementation starts. `advisory_consultation[].status` may
  be `pending` (a review was requested but has not yet answered) only before
  approval -- the plan cannot reach `approved`/`active`/`complete` while any
  declared row is still `pending`.
- **Gate 2** — coder-tester files a handoff; governor-reviewer reviews and merges.

If this project retains `docs/sources.md` and/or `docs/radar.md` as strictly
non-normative research artifacts, it may use the exact research-maintenance
exception in `agf/rules/workflow/simplicity-rule.md`. Only an advisory,
reversible change limited in every commit and the final diff to those existing
paths may skip the separate Gate 1 plan, contribution audit, and pre-change
coordination MR, and only when every source-fidelity, fresh collision-scan, and
disqualifier condition in that rule passes. Generic documentation does not
qualify. The normal branch/MR, exact-head pipeline, human Gate 2 decision, and
protected-branch merge remain mandatory. Projects without those artifacts, or
projects choosing stricter local policy, retain the ordinary Gate 1 route.

Gate 1 approval is a governor-reviewer coordination message with `kind: decision-record`
or `kind: handoff` referencing the plan file and stating `status: approved`.

---

## Branch discipline

- Create a branch before writing any code: `feat/`, `fix/`, `docs/`, `chore/`
- Never commit directly to `main`
- No version bumps or tags on feature branches
- Confirm current branch with `git branch --show-current` before any commit or push

<!-- CUSTOMIZE: add project-specific branch rules if your project has them -->

---

## Key agov commands

```bash
agov --version                                 # version + git provenance
agov version --format json                     # machine-readable version info
agov session resume --base-branch main         # cold start — always run first
agov handoff next <role>                       # your next pending task
agov handoff next <role> --include-branch-local # add advisory local/remote-tracking branch evidence; default unchanged
agov coord validate docs/coordination/<file>  # validate a coordination message
agov plan validate docs/plans/<file>           # validate a Gate 1 plan
agov coord graph-index                         # pending item scan
agov coord pending                             # non-handoff requires-ack:true obligations
agov coord brief --from <role>                 # generate a coordination ID
agov usage report                              # recent coordination activity
```

In a full source checkout, the repository wrapper must reject a local binary
whose embedded commit or build-relevant tracked-source state is stale relative
to the checkout or the requested resume base. Rebuild from the intended
checkout; do not bypass the check for routine governed work.

---

## What NOT to do

- Do not start any work — reads, git log, or implementation — without running
  `session resume` first
- Do not commit directly to `main`
- Do not edit `agf/` files without governor-reviewer approval
- Do not upgrade packages without explicit user request
- Do not commit secrets, `.env` files, or credentials
- Do not add `Co-authored-by` or AI-tool trailers to commit messages
- Do not claim a polling timer is active unless a real notification mechanism
  backs it. Stating "I will check in N minutes" without a mechanism that fires
  is misleading — another agent or human may wait for a notification that
  never comes

<!-- CUSTOMIZE: add any project-specific prohibitions -->

Publication-hold evidence: `agov coord hold-status --target-branch origin/main
--candidate-ref <shared-ref>` reports explicit pinned Git declarations only.
`review-required` never permits delay. See
`agf/rules/workflow/coordination-publication-rule.md` for external checks,
rollout compatibility and immediate-publication obligations.
