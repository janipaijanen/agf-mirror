---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 20260726T215227Z-coder-tester-mdn1
timestamp: 2026-07-26T21:52:27Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260726T214713Z-governor-reviewer-3nmp
topic: tool-interface-authority-documentation-gate2
requires-ack: true
artifacts_owned:
  - agf/docs/site/docs/adopt/assistant-loader-configuration.md
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the authority-versus-supported-interface clarification.
    - Decide Gate 2 for the documentation-only merge request.
  must_read:
    - agf/docs/site/docs/adopt/assistant-loader-configuration.md
    - agf/rules/always/tool-authority-boundary-rule.md
  do_not_touch:
    - .claude/settings.local.json
    - tooling/elixir/mix.lock
---

# Tool-interface authority documentation ready for Gate 2

The human governor requested that AGF document this distinction:
tool-specific surfaces are lower in governance authority but may remain
first-class supported operational interfaces whose semantics derive from
canonical AGF definitions.

The single product-documentation edit adds an **Authority and supported
interfaces** section to the assistant-loader guide. It distinguishes authority
sources, canonical `agf/skills/` workflows, and tool-specific adapters; states
that adapters may translate invocation but may not redefine authority or gates;
and treats adapter drift as a defect.

This used the documentation fast track: one product documentation file, no
code, schema, security-boundary, role-authority, or architecture change. No
separate Gate 1 plan was required.

Validation:

- `agov validate adopter-docs` — OK.
- `git diff --check` — OK.
- Local strict MkDocs build — not run because `mkdocs` is unavailable on this
  machine; the MR pipeline remains required evidence.
