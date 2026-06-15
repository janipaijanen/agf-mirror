---
kind: handoff
from: coder-tester
to: governor-reviewer
id: 01KV6JPRNE0R0JAYJE2HDE2FQ7
timestamp: 2026-06-15T21:23:52Z
immutable: true
references-to:
  - 01KV6JGXZR8QQY32Z9B0SZF5YE-governor-reviewer.md
  - docs/plans/gate7-python-guidance-cleanup.md
  - docs/audits/elixir-migration-e6-final-audit.md
topic: gate7-python-guidance-cleanup-gate2
requires-ack: true
artifacts_owned:
  - agf/docs/site/reference/environment-variables.md
  - agf/agents/MANIFEST.yaml
  - agf/rules/always/gate-self-test-rule.md
  - agf/rules/always/external-anonymization-rule.md
  - agf/rules/workflow/release-via-merge-request-rule.md
  - agf/skills/capability-review/SKILL.md
  - agf/skills/learning/SKILL.md
  - agf/skills/release/SKILL.md
  - agf/skills/contribution-audit/SKILL.md
  - agf/skills/upgrade/SKILL.md
  - agf/skills/health/SKILL.md
  - agf/skills/infosec-screen/SKILL.md
model: codex-gpt-5
handoff:
  next_agent: governor-reviewer
  status: ready
  task:
    - Review the Gate 7 guidance cleanup implementation.
    - Close Gate 7 if the shipped-guidance cleanup is acceptable.
  must_read:
    - docs/plans/gate7-python-guidance-cleanup.md
    - docs/audits/elixir-migration-e6-final-audit.md
    - docs/coordination/01KV6JGXZR8QQY32Z9B0SZF5YE-governor-reviewer.md
  do_not_touch:
    - .env
    - agents-govern/
---

# Gate 2 handoff: Gate 7 Python-era guidance cleanup

Implemented the approved Gate 7 residual Python-era guidance cleanup on branch
`docs/gate7-python-guidance-cleanup`.

## Summary

- Replaced current-looking `uv run agov-*` shipped guidance with current Mix
  task guidance where migrated equivalents exist.
- Corrected `AGOV_MANIFEST_PATH` documentation to the AGF role authority
  defaults.
- Replaced the stale manifest validation comment with
  `cd tooling/elixir && mix agov.validate.agents`.
- Marked E4-deferred release bump, health summary, graph, strict topology, and
  Markdown infosec inclusion coverage explicitly instead of presenting missing
  commands as available.
- Updated one additional shipped rule reference to avoid pointing at the old
  Python infosec tool.

## Verification

- `git diff --check` passed.
- `mix compile --warnings-as-errors` passed.
- `mix test` passed: 197 passed, 0 failures.
- `mix agov.validate` passed.
- Audit scan passed with no matches:
  `rg -n "uv run agov|cd agents-govern && uv|agents-govern/agents/MANIFEST|src/agents_govern|pyproject\\.toml" agf/docs/site/reference/environment-variables.md agf/agents/MANIFEST.yaml agf/rules agf/skills`
- Legacy command-name scan now leaves only a hook id:
  `agf/skills/health/SKILL.md:76:- id: agov-validate`

## Gate 7 recommendation

Gate 7 is ready for governor-reviewer closure after review of this cleanup.
The residual current-looking Python-era shipped guidance identified by the E6
audit has either been replaced with current Mix guidance or explicitly marked
as E4-deferred.
