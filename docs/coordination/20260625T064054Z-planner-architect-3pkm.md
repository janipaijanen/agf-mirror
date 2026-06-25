---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260625T064054Z-planner-architect-3pkm
timestamp: 2026-06-25T06:40:54Z
immutable: true
references-to:
  - 20260625T063048Z-coder-tester-r5ek
topic: ai-cold-start-local-overlay-implementation
requires-ack: false
artifacts_owned: []
---

# Handoff: implement ai-cold-start.md local overlay convention

Governor approved the proposal in r5ek. Please implement as follows:

## Specification

An optional untracked file `ai-cold-start.md` at the repository root.
Agents read it on cold start after `AGENTS.md` and before (or alongside)
`agov session resume`. It is subordinate to governed sources — it may add
local reminders, machine-specific paths, current operator preferences, or
assistant-surface quirks. It must not override gate discipline,
coordination rules, role authority, or product boundaries.

## Normative wording location

1. **Root `AGENTS.md`** — add a short section describing the overlay:
   - Filename: `ai-cold-start.md` at repo root
   - When to read: during cold start, after AGENTS.md, before session resume
   - What it may contain: local paths, workstation quirks, operator
     reminders, assistant-surface preferences
   - What it must not override: gate discipline, coordination rules,
     role authority, product scope
   - Tracking: `.gitignore`d by default; adopters may track a template
     or pointer if needed

2. **`docs/adopt/assistant-loader-configuration.md`** — add a section
   describing the local overlay pattern, when to use it, what to put in
   it, and a minimal example.

3. **`.gitignore`** — add `ai-cold-start.md` to the root `.gitignore`
   so it is untracked by default on fresh clones.

## What NOT to do

- Do not create a governed or validated schema for the file — it is
  intentionally freeform local state.
- Do not add it to the coordination channel or agov validators.
- Do not create a required template — the file should not exist on a
  fresh clone; it is created locally when needed.

## Branch

Please use a new branch following the naming convention:
`feat/ai-cold-start-overlay` or similar.

Gate 1 is approved. Proceed to implementation.
