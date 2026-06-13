---
kind: handoff
from: coder-tester
to: governor-reviewer
seq: 227
timestamp: 2026-06-13T19:01:13Z
immutable: true
references-to:
  - 00226-coder-tester.md
  - 00225-governor-reviewer.md
  - 00225-coder-tester.md
topic: agf-public-docs-polish-gate2
requires-ack: true
artifacts_owned:
  - .worktrees/agf-public-docs-polish
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review branch docs/agf-public-docs-polish.
    - Decide whether to merge the public documentation polish.
    - Note whether contribution-auditor review is required before merge or can be fast-followed.
  must_read:
    - docs/coordination/00226-coder-tester.md
    - docs/coordination/00225-governor-reviewer.md
    - docs/coordination/00225-coder-tester.md
  do_not_touch:
    - agents-govern/
    - .claude/settings.local.json
    - agents-govern/uv.lock
---

Public documentation polish is ready for Gate 2 review on branch
`docs/agf-public-docs-polish`.

## Why this exists

After AGF v2 layout S1 merged, the public documentation still had reader-facing
drift:

- install/adoption pages still explained two layouts instead of the single
  supported `agf/` boundary;
- public docs and `ADOPTING.md` still referenced Python/`uv run
  agov-validate-*` commands while current tooling is Elixir/Mix;
- contracts-and-validators presented future validator checks as shipped
  commands;
- exit/decommission text did not clearly state that full removal includes the
  `agf/` directory and copied assistant-rule install target.

## Branch commits

- `0277a87 docs: align public docs with agf layout`
- `a69451f docs: correct public docs tooling guidance`

## Verification

From `.worktrees/agf-public-docs-polish`:

```bash
MISE_TRUSTED_CONFIG_PATHS=/home/devi/src/AGF-NG/.worktrees/agf-public-docs-polish/mise.toml \
  mise exec -- uvx --from 'mkdocs>=1.6,<2' \
  mkdocs build -f agf/docs/build/mkdocs.yml --strict

MISE_TRUSTED_CONFIG_PATHS=/home/devi/src/AGF-NG/.worktrees/agf-public-docs-polish/mise.toml \
  mise exec -- mix test
```

Result:

- MkDocs strict build passed.
- Elixir test suite passed: 125 tests.

## Review notes

Seq 225 has a known coordination collision:

- `00225-governor-reviewer.md` records Gate 2 approval for `feat/agf-v2-layout`.
- `00225-coder-tester.md` is an earlier public-docs addendum handoff.

This handoff uses seq 227 and keeps the public-docs follow-up visible on
`master` so it is not hidden only on the feature branch.
