---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260621T145950Z-coder-tester-sqkh
timestamp: 2026-06-21T14:59:50Z
immutable: true
model: codex-gpt-5
references-to:
  - 20260621T145253Z-planner-architect-m3px.md
  - 20260621T145800Z-coder-tester-6qbm.md
  - docs/plans/agf-docs-factual-claims-cleanup.md
topic: docs-factual-claims-cleanup-gate2
requires-ack: true
artifacts_owned:
  - agf/docs/site/docs/adopt/tooling-and-tokens.md
  - agf/docs/site/docs/advanced/contracts-and-validators.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review Slice 2 factual-claims documentation cleanup.
    - Decide Gate 2 for branch docs/factual-claims-cleanup.
  must_read:
    - docs/plans/agf-docs-factual-claims-cleanup.md
    - agf/docs/site/docs/adopt/tooling-and-tokens.md
    - agf/docs/site/docs/advanced/contracts-and-validators.md
  do_not_touch:
    - agf/docs/site/reference/release-artifacts.md
---

# Gate 2 handoff: docs factual-claims cleanup

Slice 2 from `20260621T145253Z-planner-architect-m3px` is implemented on
branch `docs/factual-claims-cleanup`.

This branch is stacked on `docs/governance-safety-fix` because both slices touch
`agf/docs/site/docs/advanced/contracts-and-validators.md`.

## Changes

- Corrected the framework-only archive wording: no packaged `agov` tooling
  binary is currently available for download; use the wrapper with Mix as the
  backend until packaged binaries exist.
- Updated the "not in scope" list:
  - Windows x64 packaged release is not yet downloadable, but a preview CI
    artifact exists as of 2026-06-21.
  - Removed stale "Token ROI reporting in Rust" item.
- Updated validator table rows for agent definitions, learning records,
  InfoSec checks, release tags, and the coordination visibility command name.

## Claim Evidence

- Rust validate commands: `tooling/rust/crates/agov/src/main.rs` contains
  `ValidateCommand::Agents`, `ValidateCommand::Learnings`, and
  `ValidateCommand::Infosec`.
- Elixir validator tasks:
  - `tooling/elixir/lib/mix/tasks/agov.validate.agents.ex`
  - `tooling/elixir/lib/mix/tasks/agov.validate.learnings.ex`
  - `tooling/elixir/lib/mix/tasks/agov.validate.infosec.ex`
- Release tag check task:
  - `tooling/elixir/lib/mix/tasks/agov.dist.check_tag.ex`
- Rust token ROI support is present in `tooling/rust/crates/agov/src/main.rs`.

## Verification

Passed:

```bash
uv run --project agents-govern --group docs mkdocs build -f agf/docs/build/mkdocs.yml --strict
git diff --check
agov coord validate docs/coordination/20260621T145950Z-coder-tester-sqkh.md
```

Note: shell startup in this new worktree prints a local mise trust warning for
`mise.toml`; the commands still completed successfully.
