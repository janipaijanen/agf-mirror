# docs: add per-step human-vs-assistant execution markers (MR-3)

Branch: `docs/first-governed-change-execution-markers-mr3`

## Status

- **Pending:** Gate 2 review by governor-reviewer
  (`docs/coordination/20260720T150012Z-coder-tester-3psr.md`).
- **Codex collaboration:** branch rebased on current `main` after !229 merged;
  advisory cross-check recorded in
  `docs/coordination/20260720T150722Z-coder-tester-pp5e.md`.
- **Blocking Claude:** nothing. Next action (if any) will arrive as a
  pushed coordination handoff.
- Implemented in parallel with MR-1 (`!229`) while it was on CI, per
  direct user authorization this session — not a chat-relay trigger, a
  literal "please proceed."

## Summary

MR-3 of 6 from the approved documentation audience-boundary plan
(`docs/plans/docs-audience-boundary-proposal.md`, Gate 1 approved in
`20260720T143255Z-governor-reviewer-s6hg.md`). No handoff existed yet
assigning this specific MR — claimed directly
(`docs/coordination/20260720T145752Z-coder-tester-dgst.md`) since MR-3 has
no dependency on MR-1/MR-2 per the plan's own dependency notes.

Adds a per-step `**Who does this: ...**` marker to each of the 8 numbered
steps in `agf/docs/site/docs/start/first-governed-change.md`, plus a short
legend defining the three marker kinds (Human decision / AI-operator /
Shared). This directly answers the plan's own problem-statement example:
the page previously mixed "paste this into your assistant" blocks with
plain second-person steps (`git checkout -b`, `$EDITOR ...`) with nothing
distinguishing who does what.

## Files changed

| File | Change |
|---|---|
| `agf/docs/site/docs/start/first-governed-change.md` | Legend paragraph after the gate-pipeline figure; a `**Who does this**` marker under each of the 8 step headings |
| `docs/coordination/20260720T145752Z-coder-tester-dgst.md` | Claim, filed before implementation to avoid collision |
| `docs/coordination/20260720T150012Z-coder-tester-3psr.md` | Gate 2 handoff to governor-reviewer |
| `docs/coordination/20260720T150722Z-coder-tester-pp5e.md` | Codex advisory review after rebasing on top of merged MR-1 |

## Per-step classification

1. Know where the files will go — reader's choice
2. Ask your assistant to prepare starter files — AI-operator
3. Validate the plan and first handoff — AI-operator
4. Gate 1: decide before implementation — Human decision
5. Branch and implement test-first — AI-operator
6. Gate 2: review before merge — Human decision
7. Merge and close the loop — Human decision (merge), assistant may prepare the completion entry
8. Record what you learned — Shared

## Scope boundary

Does not touch `install-and-layouts.md` (Codex's MR-4) or do any audience-
line relabeling (MR-2's scope).

## Review checklist

- [x] `mkdocs build -f agf/docs/build/mkdocs.yml --strict` passes (only
      pre-existing, unrelated warnings)
- [x] `git diff --check` passes
- [x] Full diff reviewed manually for consistent formatting across all 8
      markers
- [x] No `Co-authored-by` trailers in commit
- [x] Rebased after !229; diff no longer deletes MR-1 diagram files or
      reverts `docs/index.md`
