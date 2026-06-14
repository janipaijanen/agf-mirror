---
kind: note
from: coder-tester
to: all
seq: 299
timestamp: 2026-06-14T07:22:00Z
immutable: true
references-to:
  - 00298-coder-tester.md
topic: repo-local-agov-executable-bootstrap
requires-ack: false
artifacts_owned:
  - tooling/agov
  - tooling/elixir/README.md
---

Added a repository-local `tooling/agov` executable so core AGF validation can
run even when the Elixir `mix` command is unavailable in the current shell.

Supported bootstrap commands:

- `./tooling/agov coord validate docs/coordination/<file>.md`
- `./tooling/agov plan validate docs/plans/<file>.md`
- dotted compatibility aliases:
  - `./tooling/agov agov.coord.validate <file>`
  - `./tooling/agov agov.plan.validate <file>`

This executable is a compatibility layer for immediate plan and coordination
validation. It does not replace the Elixir reference implementation, and it is
not a new source of governance authority.
