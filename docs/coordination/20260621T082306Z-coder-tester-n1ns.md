---
id: 20260621T082306Z-coder-tester-n1ns
kind: handoff
from: coder-tester
to: governor-reviewer
timestamp: 2026-06-21T08:23:06Z
immutable: true
references-to:
  - 20260621T065512Z-planner-architect-q5ph.md
topic: ai-first-governed-change-docs
requires-ack: false
artifacts_owned: []
model: gpt-5-codex
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review and merge the first-governed-change docs fix after or together with MR !42.
    - Confirm the first-run path defaults to AI-assisted setup rather than manual template copying.
  must_read:
    - agf/docs/site/docs/start/first-governed-change.md
  do_not_touch:
    - mise.toml
    - mise.lock
---

# Gate 2 handoff: make first governed change AI-first

The live first-governed-change page asked adopters to manually copy six starter
files before they could experience the governed workflow. That is adoption
friction in the worst possible place: a potential adopter has already decided
to try the framework, and the page immediately makes them do boilerplate.

I changed the page so the default path is to ask the coding assistant to
prepare the starter files and directories. The human still reviews the diff and
validates the generated plan and coordination entries.

## What changed

- Replaced the manual `cp` block with an AI-assistant prompt.
- Asked the assistant to create missing `docs/plans/`, `docs/coordination/`,
  and `agf/learnings/` directories.
- Kept validation commands in the human path.
- Replaced fixed `COORD-20260614-*` filenames with readable-ID placeholders.
- Made clear that the assistant may prepare decision skeletons but must not
  invent approvals, commits, tests, or validation results.

## Verification

- `uvx --from 'mkdocs>=1.6,<2' --with 'mkdocs-material>=9.5,<10' --with 'mkdocs-awesome-pages-plugin>=2.9,<3' --with 'mkdocs-gen-files>=0.5,<1' mkdocs build -f agf/docs/build/mkdocs.yml --strict` passed.
- `git diff --check` passed.

The local shell emits a `mise` trust warning for this linked worktree before
commands run; it did not affect the MkDocs build result.
