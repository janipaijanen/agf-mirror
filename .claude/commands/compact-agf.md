---
name: compact-agf
origin: project
description: >
  Save the current AGF session state to a durable coordination note before
  context compaction, shutdown, model switch, or cold-start handoff.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Compact AGF

Preserve the current session state before context is compacted or abandoned.
This command writes a durable coordination note that a cold-starting agent can
read before continuing the work.

Do not rely on background processes, timers, terminal sessions, or model memory
to signal later. The saved file is the handoff surface.

## When to run

- Before `/compact`
- Before a model switch or tool-surface switch
- Before a long pause with active work
- Before leaving unmerged branch work for another agent
- Whenever the user asks to "save before compact"

If there is no active work and nothing material would be lost, say that clearly
instead of writing noise to the coordination channel.

## Step 1 - Identify role and branch

Determine the role you are currently playing. Prefer an explicit role from the
session or current task. If uncertain, run the project resume command first:

```bash
agov session resume --base-branch main
```

If `agov session resume` is unavailable in this checkout, use the reference
implementation:

```bash
cd tooling/elixir
mix agov.session.resume --base-branch main
```

Then collect the live repository facts:

```bash
date -u +%Y-%m-%dT%H:%M:%SZ
git branch --show-current
git status --short --branch
git log --oneline --decorate -5
```

If you know the active role, also run:

```bash
agov session resume --role <role> --base-branch main
```

Treat any hidden coordination, dirty-tree, or pending-handoff warnings as
important resume context.

## Step 2 - Generate a coordination ID

Use the coordination tooling whenever available:

```bash
agov coord brief --id-format readable --from <role>
```

Use the generated `id` as both the front-matter `id` and the filename:

```text
docs/coordination/<id>.md
```

If the tooling is unavailable, create a readable ID manually in this shape:

```text
YYYYMMDDThhmmssZ-<role-slug>-<4-to-6-random-lowercase-alnum>
```

Do not use a sequence number.

## Step 3 - Write the closeout note

Create a `kind: note` coordination message addressed to `all` unless the saved
state creates a concrete obligation for a specific role. If another agent or
the governor must act next, use `kind: handoff` instead and fill the required
`handoff:` object.

For an ordinary compaction save, use this note shape:

````markdown
---
kind: note
from: <role>
to: all
id: <id>
timestamp: <UTC timestamp>
immutable: true
references-to: []
topic: session-closeout-before-compact
requires-ack: false
artifacts_owned: []
---

# Session closeout before compact

## Resume first

Read this note before continuing the interrupted session. Then run:

```bash
agov session resume --role <role> --base-branch main
```

## Current state

- Role: `<role>`
- Branch: `<branch>`
- Worktree: `<absolute or repo-relative path>`
- Latest commit: `<short hash and subject>`
- Dirty tree: `<yes/no; summarize changed files>`
- Active request: `<one sentence>`

## What changed in this session

- `<completed work, files touched, commits made, MRs updated, or none>`

## Verification

- `<commands run and pass/fail/skip>`

## Open work

- `<next concrete action>`
- `<blockers or decisions needed>`

## Must read

- `<coordination files, plan files, docs, or MR links needed to continue>`

## Do not assume

- `<anything session memory might falsely imply>`
- `<branches/files that must not be touched>`
````

Keep the note factual and short. Prefer filenames, commits, MR URLs, and exact
commands over prose memory.

## Step 4 - Validate

Validate the file before treating the save as durable:

```bash
agov coord validate docs/coordination/<id>.md
```

If `agov` is unavailable, use the Mix reference task:

```bash
cd tooling/elixir
mix agov.coord.validate ../../docs/coordination/<id>.md
```

Fix validation failures before proceeding.

## Step 5 - Report compact readiness

Tell the user:

- the path of the saved coordination note;
- whether validation passed;
- whether there is still uncommitted work;
- that the session is ready for `/compact`.

Do not run more implementation work after this command unless the user
explicitly cancels compaction or gives a new request.
