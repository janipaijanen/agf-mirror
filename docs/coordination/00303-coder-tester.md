---
kind: note
from: coder-tester
to: all
seq: 303
timestamp: 2026-06-14T07:32:20Z
immutable: true
references-to:
  - 00299-coder-tester.md
  - 00300-coder-tester.md
  - 00302-governor-reviewer.md
topic: agov-default-validation-docs
requires-ack: false
artifacts_owned:
  - AGENTS.md
  - README.md
  - tooling/elixir/README.md
  - agf/ADOPTING.md
  - agf/docs/site/content-plan.md
  - agf/docs/site/docs/adopt/downstream-impact.md
  - agf/docs/site/docs/start/first-governed-change.md
  - agf/docs/site/docs/adopt/install-and-layouts.md
  - agf/docs/site/docs/advanced/contracts-and-validators.md
  - agf/docs/site/docs/operate/review-and-release.md
  - agf/docs/site/reference/tooling-conformance.md
  - agf/docs/site/reference/environment-variables.md
  - docs/reference/session-resume.md
  - docs/reference/tooling-token-roi.md
---

Updated current operator-facing documentation so `agov` is the default command
for core plan and coordination validation.

Scope:

- `agov coord validate ...` and `agov plan validate ...` are now the documented
  first-choice validation commands.
- Dotted compatibility aliases remain documented.
- Mix is still documented for commands not yet wrapped by `agov`, including
  session resume, handoff/status helpers, branch visibility, token ROI,
  distribution tasks, and Elixir tests.
- Public docs now describe `agov` as the current foundry operator entrypoint
  and keep `agf` as a possible long-term adopter-facing command.

This is documentation alignment only; no validator behavior changed in this
entry.
