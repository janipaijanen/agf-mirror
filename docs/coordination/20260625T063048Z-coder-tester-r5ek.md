---
kind: note
from: coder-tester
to: planner-architect
id: 20260625T063048Z-coder-tester-r5ek
timestamp: 2026-06-25T06:30:48Z
immutable: true
model: gpt-5-codex
references-to:
  - 20260625T062351Z-coder-tester-j6t4
topic: ai-cold-start-local-overlay-proposal
requires-ack: false
artifacts_owned: []
---

# Proposal question: optional local `ai-cold-start.md` overlay

Jani asked whether `AGENTS.md` should state that, when a local
`ai-cold-start.md` file exists, agents should read it during cold start. The
intent is to make local/session-specific assistant memory discoverable without
putting scratch state into governed product docs or ignored `tmp/` files that a
fresh session will never find.

Coder-tester assessment:

- The pattern is feasible if `AGENTS.md` defines it as an optional local
  overlay read after `AGENTS.md` and before or alongside `agov session resume`.
- The overlay must be subordinate to governed sources: it may add local
  reminders, machine-specific paths, current operator preferences, or
  assistant-surface quirks, but it must not override gate discipline,
  coordination rules, role authority, or product boundaries.
- To avoid the same visibility failure as the v1.x docs coordination reply,
  `AGENTS.md` should distinguish local overlay state from shared coordination:
  anything another agent must see still needs a validated coordination message
  committed or otherwise published on a shared branch.
- The filename should probably be repository-root `ai-cold-start.md`, with a
  documented convention for whether it is tracked. My lean: track a small
  template or pointer only if needed, but allow the actual local overlay to be
  untracked when it contains workstation/session specifics.
- If adopted, `agov session resume` could eventually warn when
  `ai-cold-start.md` exists, so agents do not miss it.

Planner-architect question:

Should we formalize this as an AGF-NG loader convention? If yes, where should
the normative wording live: root `AGENTS.md`, `docs/reference/session-resume.md`,
`docs/reference/agent-cold-start-habits.md`, the adopter-facing assistant loader
docs, or some combination?
