---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260621T214311Z-coder-tester-sjqb
timestamp: 2026-06-21T21:43:11Z
model: GPT-5
immutable: true
references-to:
  - 20260621T212942Z-coder-tester-hn3e.md
topic: first-run-loader-and-guardrail-docs
requires-ack: true
artifacts_owned:
  - agf/docs/site/docs/adopt/assistant-loader-configuration.md
  - agf/docs/site/docs/adopt/install-and-layouts.md
  - agf/docs/site/docs/advanced/contracts-and-validators.md
  - agf/docs/build/mkdocs.yml
  - docs/coordination/20260621T214311Z-coder-tester-sjqb.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the first-run assistant-loader and minimum structural guardrail docs slice.
  must_read:
    - agf/docs/site/docs/adopt/assistant-loader-configuration.md
    - agf/docs/site/docs/adopt/install-and-layouts.md
    - agf/docs/site/docs/advanced/contracts-and-validators.md
  do_not_touch:
    - agf/rules/
    - agf/schemas/
    - agf/agents/
---

# Gate 2 handoff: first-run loader and guardrail docs

This slice responds to the Lumo.ai first-time adopter fit assessment recorded
as GitLab external review work item #4:

`https://gitlab.com/janipaijanen/agents-govern-framework/-/work_items/4`

## Changes

- Added `agf/docs/site/docs/adopt/assistant-loader-configuration.md`.
- Added the page to the Adopt navigation.
- Linked the loader page from install prerequisites and rule-loading review.
- Added a first-real-change pointer from install docs to the minimum structural
  guardrail.
- Added `Minimum structural guardrail` to Contracts and validators.

The loader page keeps the v2 carve-out intact: tool-specific loader files
point to canonical AGF files under `agf/`; they do not copy the AGF rule tree
into tool directories.

## Boundaries

- No `agf/rules/`, `agf/schemas/`, or `agf/agents/` changes.
- No `agov init` or other tooling implementation.
- No learning-record threshold or governor time-cost wording in this slice.

## Validation

- `git diff --check`: pass.
- `uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict`: pass.

The shell emitted the local untrusted `mise.toml` warning before commands in
this worktree, but each command completed successfully.
