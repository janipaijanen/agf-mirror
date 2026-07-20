---
kind: handoff
from: planner-architect
to: coder-tester
id: 20260720T152054Z-planner-architect-q4dj
timestamp: 2026-07-20T15:20:54Z
model: claude-sonnet-5
immutable: true
references-to: []
topic: nutshell-flyer-redraft-3rd-year-engineering-audience
requires-ack: true
artifacts_owned: []
handoff:
  next_agent: coder-tester
  status: ready
  task:
    - >-
      Redirect the "AGF in a nutshell" A3 orientation flyer (a Claude
      Artifact, not a repo file) to a 3rd-year engineering bachelor's-student
      audience, replacing the current practitioner/adopter framing.
    - >-
      Correct the command framing: the cheat-sheet's raw `agov` command list
      contradicts the AI-executes/human-decides model this repo just shipped
      in agf/docs/site/docs/index.md (MR !229) — show what the AI does for
      the student, not a CLI reference.
    - >-
      Propose content and visual direction for the next draft (see body for
      the two open questions) and reply with a `review` or `note`
      referencing this handoff before Claude re-renders it.
  must_read:
    - docs/plans/docs-audience-boundary-proposal.md
    - agf/docs/site/docs/index.md
  do_not_touch: []
---

# Redraft the nutshell flyer for a student audience

## Context

Claude published a first-draft A3 flyer ("AGF in a Nutshell") as a Claude
Artifact (not a repo file): roles, the two gates, the coordination channel,
intended benefits, and a practitioner cheat sheet (`agov` command list,
message-kind glossary, fast-track criteria). It was built for a general
practitioner/adopter reader — someone already running a real project with an
AI coding assistant, weighing adoption cost and vendor lock-in.

The human governor reviewed it and flagged two misalignments with the actual
intended audience:

1. **Audience.** The intended reader is a **3rd-year engineering bachelor's
   student**, learning what AGF is and how they personally would benefit —
   not a professional evaluating adoption cost for an existing production
   codebase. The current draft assumes prior context (git/MR workflows, CI,
   YAML frontmatter, "vendor lock-in," "token cost") that a student audience
   should not be assumed to have going in.
2. **Command framing.** The current "Core commands" cheat-sheet block lists
   five `agov` commands as if the human reader runs them directly and
   routinely. That contradicts what `agf/docs/site/docs/index.md` (the
   audience-boundary work we just shipped in MR !229) says explicitly: most
   `agov`/coordination-file procedure is **AI-operator work a human delegates
   and reviews**, not a personal command reference. A student-facing flyer
   should show *what the AI does on the student's behalf* and *where the
   student's own judgment is still required* (Gate 1/Gate 2 decisions) —
   not a CLI cheat sheet.

## Ask

Before Claude produces the next draft, propose your read on:

- **Content**: what a 3rd-year engineering student most needs explained
  (what problem AGF solves, why it matters for their own AI-assisted coding
  habits, what they'd actually do vs. what the AI does) and what should be
  cut from the current draft (token-cost/vendor-lock-in framing, the raw
  `agov` command list) as too practitioner-oriented.
- **Visuals**: whether the current direction (IBM Plex type family, AGF's
  existing blue/amber/green role-color tokens from `agf.css`, A3 blueprint
  layout) still fits a student/teaching context, or whether a more
  pedagogical treatment (e.g., a single annotated diagram as the hero,
  fewer dense text blocks) would land better for a poster meant to be read
  in passing, not studied like a reference card.

Reply as a `review` or `note` coordination message referencing this handoff.
Claude will fold the response into the next draft and reply here before
re-rendering.
