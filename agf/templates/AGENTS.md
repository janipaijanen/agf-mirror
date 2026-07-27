<!-- SPDX-License-Identifier: MIT -->
<!-- Copyright (c) 2026 <!-- CUSTOMIZE: your name --> -->
<!-- AGF: 2.4.4 — update this comment after each framework upgrade to match agf/VERSION -->
# <!-- CUSTOMIZE: Project Name --> — Agent and Contributor Orientation

<!-- CUSTOMIZE: brief description of the project -->

**Project codename:** <!-- CUSTOMIZE: your codename (see ADOPTING.md §Step 0) -->
**AGF version:** <!-- CUSTOMIZE: match agf/VERSION -->

---

## Cold start

Canonical operating, interruption, recovery, and polling procedures are in
`agf/docs/site/docs/operate/runbook.md`. This loader points there; it must not
grow a competing copy of those procedures.

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
```

Run `session resume` before anything else — before reading coordination files,
before `git log`, before any file inspection. It surfaces pending coordination
messages, active decisions, and unresolved handoffs. Starting with reads before
the resume means acting on stale context.

After the resume output, file a brief coordination `kind: note` for the current
cycle status before proceeding. This gives other agents and the governor a
current baseline rather than an invisible state from the previous session.

---

## AGF framework version

AGF governance files live under `agf/`. The active framework version is in
`agf/VERSION`. After upgrading AGF, update the `<!-- AGF: ... -->` comment at
the top of this file to match `agf/VERSION`.

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

---

## Gate discipline

Every non-trivial task requires two gates:

- **Gate 1** — planner-architect files a plan in `docs/plans/`; governor-reviewer
  approves before implementation starts.
- **Gate 2** — coder-tester files a handoff; governor-reviewer reviews and merges.

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
agov coord validate docs/coordination/<file>  # validate a coordination message
agov plan validate docs/plans/<file>           # validate a Gate 1 plan
agov coord graph-index                         # pending item scan
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
