<!-- SPDX-License-Identifier: MIT -->
<!-- Copyright (c) 2026 <!-- CUSTOMIZE: your name --> -->
<!-- AGF: 2.2.0 — update this line after each framework upgrade (match agf/VERSION) -->
# <!-- CUSTOMIZE: Project Name --> — Agent and Contributor Orientation

<!-- CUSTOMIZE: brief description of the project -->

**Project codename:** <!-- CUSTOMIZE: your codename (see ADOPTING.md §Step 0) -->
**AGF version:** <!-- CUSTOMIZE: match agf/VERSION -->

---

## Cold start

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

Never start implementing before running `session resume`. It surfaces pending
coordination messages, active decisions, and unresolved handoffs that must be
read before any new work begins.

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
agov coord graph-index docs/coordination/
```

Write a coordination entry whenever your action creates an obligation for
another agent or the human governor. File the entry before or immediately after
acting — never as an afterthought.

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
agov coord graph-index docs/coordination/      # pending item scan
agov coord brief --from <role>                 # generate a coordination ID
agov usage report                              # recent coordination activity
```

---

## What NOT to do

- Do not implement without running `session resume` first
- Do not commit directly to `main`
- Do not edit `agf/` files without governor-reviewer approval
- Do not upgrade packages without explicit user request
- Do not commit secrets, `.env` files, or credentials
- Do not add `Co-authored-by` or AI-tool trailers to commit messages

<!-- CUSTOMIZE: add any project-specific prohibitions -->
