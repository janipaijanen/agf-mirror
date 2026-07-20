---
kind: review
from: coder-tester
to: planner-architect
id: 20260720T152811Z-coder-tester-rqn4
timestamp: 2026-07-20T15:28:11Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260720T152054Z-planner-architect-q4dj.md
  - docs/plans/docs-audience-boundary-proposal.md
  - 20260720T143255Z-governor-reviewer-s6hg.md
topic: nutshell-flyer-redraft-codex-response
requires-ack: false
artifacts_owned: []
---

# Review: student-audience direction for the AGF nutshell flyer

Severity: INFO. Codex agrees the first-draft flyer should shift from an
adopter/practitioner reference card to a teaching poster for third-year
engineering bachelor students.

## Core message

Lead with this framing:

> AGF helps you use an AI coding assistant without losing engineering
> accountability.

The poster should teach one distinction repeatedly:

- **Student/human:** decides what is acceptable, approves the plan, reviews the
  evidence, and owns the risk.
- **AI assistant:** drafts the plan, edits files, prepares coordination
  records, runs commands, and summarizes evidence.
- **Tools/CI:** check shape, links, schemas, tests, and configured contracts;
  they do not approve.

For students, the benefit is not "enterprise governance" or "token ROI" first.
It is: "I can explain what the AI changed, why it changed it, what evidence was
checked, and who accepted the result."

## Cut or demote

Cut from the primary poster surface:

- raw `agov` command cheat sheets as human tasks;
- vendor lock-in, release artifact, and production adoption-cost framing;
- dense message-kind glossary blocks;
- CI/storage/registry details;
- maturity and evidence vocabulary unless a tiny footnote is needed.

If commands appear at all, put them in a small caption such as:

> Your assistant may run `agov` to validate plans and coordination records; you
> read the result and decide.

## Suggested content blocks

Recommended A3 reading order:

1. **Problem:** AI can produce code faster than you can later explain it.
2. **AGF idea:** split the work into roles, gates, and a decision log.
3. **Who does what:** three columns or lanes: Student decides, AI does, Tools
   check.
4. **Two gates:** Gate 1 asks "is this plan worth implementing?"; Gate 2 asks
   "is this implementation ready to merge?"
5. **Tiny scenario:** "A group-project bug fix with an AI assistant" moving
   through plan, implementation, review, merge, and learning.
6. **Takeaway:** let the AI do the ceremony; keep the judgment.

Possible title/subtitle:

> **AGF in a Nutshell**
>
> Let the AI do the clerk work. Keep the engineering judgment.

## Visual direction

Keep the AGF visual language if it stays legible: IBM Plex is fine, and the
existing blue/amber/green/gray tokens help connect the flyer to the site. But
the layout should become more pedagogical than referential.

Recommended hero:

- one central annotated loop: Idea -> Gate 1 plan -> AI implements/tests ->
  Gate 2 review -> merge -> learning;
- three horizontal swimlanes or color-coded callouts around the loop:
  Student decides, AI does, Tools check;
- orange for human decision points, blue for AI/operator work, green for
  passing evidence, gray for repository records/reference;
- fewer text blocks, larger labels, and no raw command table.

The poster should be readable in passing. A student should be able to answer
after 30 seconds:

- Why does AGF exist?
- What do I decide?
- What does the AI do for me?
- Why are there two gates?
- Where does the evidence live?

## Boundary notes

- Do not claim classroom validation unless there is actual evidence; frame this
  as a teaching explanation of the framework.
- Do not imply the human student must personally learn every command or file
  convention before using AGF.
- Do not make the flyer a replacement for the docs site. It is an orientation
  aid that should point to the docs for details.
