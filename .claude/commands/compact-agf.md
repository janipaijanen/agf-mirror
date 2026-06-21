---
name: compact-agf
origin: foundry
description: >
  Write current session state to memory, then compact. Use this instead of
  /compact so you can resume after a cold-start without losing in-flight
  context. Triggers /compact automatically after writing state.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Compact with session state checkpoint

Write the current session state to the memory system, then trigger /compact.
The memory file is loaded automatically on cold-start so you can resume
without re-reading the full coordination channel.

## Step 1 — Gather current state

Run in parallel:

```bash
git fetch --prune origin
git branch --show-current
./tooling/agov session resume --role <current-role>
```

Also run:

```bash
ls docs/coordination/ | grep -E "^202" | sort | tail -20
git branch -r | grep -vE "(HEAD|main$)" | sort
```

## Step 2 — Identify what is in-flight

From the session resume output, identify:

- **Pending handoffs for planner-architect** — tasks Codex has completed and
  returned to us for Gate 1 review or decision.
- **Active branches** — branches on origin that are not yet merged to main.
- **Gate 2 handoffs awaiting governor** — coordination files where
  `to: governor-reviewer` and the topic needs human approval.
- **Decisions filed this session** — `decision-record` entries we wrote.
- **Timers** — any active polling window (interval, expiry time).
- **Next expected event** — what Codex is working on, what the governor needs
  to act on, what the planner is waiting for.

## Step 3 — Write session state to memory

Write a file to the memory system:
**Path:** `/home/devi/.claude/projects/-home-devi-src-AGF-NG/memory/session-state.md`

Use this exact frontmatter and structure:

```markdown
---
name: session-state
description: In-flight session state as of <ISO timestamp> — active work, pending handoffs, governor actions needed
metadata:
  type: project
---

# Session state — <ISO timestamp>

## Role
<current role, e.g. planner-architect>

## Active branches
<list of non-merged branches on origin with one-line description of what each contains>

## Gate 2 handoffs awaiting governor review
<for each: coordination file ID, topic, branch, what the governor needs to decide>

## Pending Codex handoffs (planner action needed)
<for each: coordination file ID, topic, what we need to do>

## Decisions filed this session
<list of decision-record IDs and topics>

## Polling timer
<interval / window expiry / what we are watching for>

## Next expected event
<who does what next — Codex implementing X, governor reviewing Y, planner watching for Z>

## Do not re-derive
<facts established in this session that would be expensive to re-derive:
specific constraints, format decisions, approved plans, etc.>
```

**Why:** On cold-start, `agov session resume` shows pending handoffs but not
the narrative context — what is actively in flight, what the governor needs
to act on, and decisions made this session. This file bridges that gap.

## Step 4 — Update MEMORY.md

Ensure `MEMORY.md` has an entry for `session-state.md`. If the entry already
exists, update the description line to reflect the new timestamp. The entry
belongs in the index as:

```
- [Session state](session-state.md) — in-flight work as of <date>; active branches, pending handoffs, governor actions
```

## Step 5 — Compact

After writing the memory file, tell the user:

> "Session state written to memory. Ready to compact — run /compact now."

Do NOT call /compact yourself. The user triggers it. This step is a
reminder to the user to run /compact immediately after this command
completes.
