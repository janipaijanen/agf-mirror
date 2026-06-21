---
kind: note
from: planner-architect
to: coder-tester
id: 20260621T065512Z-planner-architect-q5ph
timestamp: 2026-06-21T06:55:12Z
immutable: true
model: claude-sonnet-4-6
references-to: []
topic: docs-site-adoption-rust-tooling
requires-ack: false
artifacts_owned: []
---

# Docs collaboration request: adoption flow + Rust tooling page

## What I've done on branch `docs/site-adoption-rust-tooling`

I've made an initial pass on three documentation changes:

1. **`agf/docs/site/index.md`** — restructured homepage. Added a concrete
   "How to adopt" section (three numbered steps) immediately after the status
   block. Added a "Token savings with the Rust tooling" block with a command
   maturity table (Level 5 / Level 1). Removed the old "The situation" and
   "What AGF is and is not" sections from the top half; their substance is
   preserved in shorter form.

2. **`agf/docs/site/docs/adopt/tooling-and-tokens.md`** — new page covering
   the Rust tooling: what it does, current Level 5 commands, Level 1 planned
   commands, three setup options (wrapper / release archive / CI), the token
   savings estimate model, and the local telemetry POC. Added to mkdocs nav
   and `docs/index.md`.

3. **`agf/docs/site/reference/tooling-conformance.md`** — updated command
   table to reflect Phase 1 parity (coord validate, coord brief, plan validate,
   validate, handoff next now show "✓ Phase 1" in the Rust column rather than
   "not wrapped yet").

## Where I'd like your input

The goal the governor gave us is to make the docs **attractive and lower risk
for readers to adopt**. I have covered the structure and accuracy side. I'd
like your eye on:

1. **Tone and flow on the homepage.** Does the "How to adopt" section feel
   achievable or does it still imply too much ceremony? Are there places where
   the framing is too internal (written for someone already familiar with AGF
   rather than a first-time evaluator)?

2. **The tooling-and-tokens page.** Is there a concrete example — a short
   before/after showing what an agent would have to read manually vs. what
   `agov handoff next planner-architect` returns — that would make the token
   savings tangible? I deliberately left out specific numbers because we don't
   have public measurement data, but a qualitative "here is what the agent
   previously had to read" example would not violate the honesty model.

3. **use-cases.md.** That page exists and is linked from the nav but I did not
   touch it. If there is a use case that illustrates both the governance value
   and the tooling value together (e.g. "a team runs daily AI sessions; without
   tooling, each session starts with the agent reading 30 coordination files;
   with tooling, session resume summarises state in one command"), that would
   be a good addition there.

4. **The maturity table wording.** I used "Level 5 — Deployable" and "Level 1
   — Planned" following the implementation-honesty-rule. Does this read clearly
   to a first-time evaluator, or does the level numbering need a one-sentence
   gloss before the table?

## What I am not asking you to do

- Implement Phase 2A Rust commands (that is a separate Gate 1 plan, MR !41).
- Change any of the honesty or maturity claims — those are accurate as stated.
- Touch `MR_DESCRIPTION.md` yet — I will update it before MR creation.

## Branch state

Branch: `docs/site-adoption-rust-tooling` (just pushed).
All changes are docs-only; no schema or tooling changes.
I have not validated the mkdocs build yet — if you run
`mkdocs build -f agf/docs/build/mkdocs.yml --strict` and it reports broken
links, please fix them and push to the branch. That is a coder-tester task.

Ping back with a note when you have reviewed or added content.
