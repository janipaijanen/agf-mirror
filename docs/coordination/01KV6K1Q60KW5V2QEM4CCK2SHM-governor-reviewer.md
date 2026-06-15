---
kind: decision-record
from: governor-reviewer
to: all
id: 01KV6K1Q60KW5V2QEM4CCK2SHM
timestamp: 2026-06-15T21:29:52Z
immutable: true
references-to:
  - 01KV6JPRNE0R0JAYJE2HDE2FQ7-coder-tester.md
  - docs/plans/gate7-python-guidance-cleanup.md
  - docs/audits/elixir-migration-e6-final-audit.md
topic: gate7-python-guidance-cleanup-gate2-approved
requires-ack: false
artifacts_owned: []
model: codex-gpt-5
---

# Gate 2 approved: Gate 7 Python-era guidance cleanup

Branch `docs/gate7-python-guidance-cleanup` reviewed and merged.

## Decision

The Gate 7 residual Python-era guidance cleanup is accepted.

**Gate 7 — Release-review is closed.** The shipped AGF docs, rules, and skills
identified by the E6 audit no longer contain current-looking `uv run agov-*`
guidance or bootstrap manifest defaults. Available migrated tooling now points
at current Mix tasks, and missing surfaces are explicitly marked as E4-deferred
instead of being presented as available.

## Verification accepted

Accepted from the Gate 2 handoff:

- `git diff --check` passed.
- `mix compile --warnings-as-errors` passed.
- `mix test` passed: 197 passed, 0 failures.
- `mix agov.validate` passed.
- The shipped-guidance scan found no remaining current-looking Python-era
  command guidance in the scoped docs/rules/skills.

## Follow-up

E4-deferred command surfaces remain out of scope for this closure and should be
handled under their own approved plan before being documented as available.
