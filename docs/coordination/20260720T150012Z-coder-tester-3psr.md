---
kind: handoff
from: coder-tester
model: claude-sonnet-5
to: governor-reviewer
id: 20260720T150012Z-coder-tester-3psr
timestamp: 2026-07-20T15:00:12Z
immutable: true
references-to:
  - 20260720T145752Z-coder-tester-dgst.md
  - docs/plans/docs-audience-boundary-proposal.md
topic: docs-audience-implementation-mr3-gate2
requires-ack: false
artifacts_owned:
  - agf/docs/site/docs/start/first-governed-change.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review MR-3 (branch docs/first-governed-change-execution-markers-mr3) for Gate 2.
    - Added a per-step "Who does this" marker to each of the 8 numbered steps in agf/docs/site/docs/start/first-governed-change.md, using the taxonomy vocabulary from MR-1 (Human decision / AI-operator / Shared).
    - Added a one-paragraph legend explaining the three marker kinds, right after the gate-pipeline figure.
    - Did not touch install-and-layouts.md (Codex's MR-4) or do any audience-line relabeling (MR-2's scope).
  must_read:
    - docs/plans/docs-audience-boundary-proposal.md
    - docs/coordination/20260720T145752Z-coder-tester-dgst.md
    - agf/docs/site/docs/start/first-governed-change.md
  do_not_touch:
    - agf/docs/site/docs/adopt/install-and-layouts.md
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Gate 2 handoff: MR-3 first-governed-change.md execution markers

Implemented MR-3 from the approved documentation audience-boundary plan on
branch `docs/first-governed-change-execution-markers-mr3`, worked in
parallel with MR-1 (`!229`) while it was on CI, per direct user
authorization this session.

Implementation summary:

- One-paragraph legend after the gate-pipeline figure, defining "Human
  decision" (no AI substitute), "AI-operator" (assistant executes, human
  reviews), and "Shared" (both have a part).
- A `**Who does this: ...**` line added directly under each of the 8
  numbered step headings, classifying each step:
  1. Know where the files will go — reader's choice (run it or delegate to
     step 2's assistant prompt)
  2. Ask your assistant to prepare starter files — AI-operator
  3. Validate the plan and first handoff — AI-operator
  4. Gate 1: decide before implementation — Human decision
  5. Branch and implement test-first — AI-operator
  6. Gate 2: review before merge — Human decision
  7. Merge and close the loop — Human decision (merge itself), assistant
     may prepare the completion entry
  8. Record what you learned — Shared

This directly answers the plan's problem-statement example: the page
previously mixed "paste this into your assistant" blocks with plain
second-person procedural steps (`git checkout -b`, `$EDITOR ...`) with no
marker distinguishing who does what. It now states that explicitly, step
by step.

Validation:

- `mkdocs build -f agf/docs/build/mkdocs.yml --strict` passed — only the
  pre-existing, unrelated warnings.
- `git diff --check` passed.
- Reviewed the full diff manually before committing; markers are
  consistently formatted across all 8 steps.
