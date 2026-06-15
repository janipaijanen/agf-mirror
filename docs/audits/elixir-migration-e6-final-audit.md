---
id: 01KV6HK3PF8AHFK5WBB7BP0FQZ
topic: elixir-migration-e6-final-audit
timestamp: 2026-06-15T21:04:24Z
status: complete-with-gate7-followup
---

# E6 Final Elixir Migration Audit

## Summary

E6 audit evidence supports closing **Gate 6 — Tooling defaults**.

E6 audit evidence does **not** cleanly support closing **Gate 7 —
Release-review** without a governor decision or follow-up, because shipped AGF
skills/rules and one environment-variable reference still contain current-looking
Python-era command guidance.

## Commands Run

```bash
git diff --check
rg -n "agents-govern|uv run agov|agents-govern/uv\\.lock|src/agents_govern|pyproject\\.toml" tooling/elixir agf .gitlab-ci.yml AGENTS.md
rg -n "agov-validate|agov-pre-tag-check|agov-validate-tag-push|agov-bump|agov-dist|agov-health" agf/docs agf/framework.md
rg -n "agents-govern/\\*\\*" agf/dist/release-manifest.yaml
cd tooling/elixir
mix compile --warnings-as-errors
mix test
mix agov.validate
mix agov.session.resume --role coder-tester --base-branch main
```

## Passing Evidence

- `git diff --check` passed.
- `mix compile --warnings-as-errors` passed.
- `mix test` passed: 197 passed, 0 failures.
- `mix agov.validate` passed against `../../agf`.
- `mix agov.session.resume --role coder-tester --base-branch main` completed
  with no warnings, pending handoffs, open claims, stale claims, or must-read
  items for coder-tester.
- `.gitlab-ci.yml` is Elixir-only for AGF tooling. Python appears only in the
  MkDocs documentation build image.
- `agf/dist/release-manifest.yaml` forbids `agents-govern/**`.
- Elixir code defaults point at `../../agf/...` and `main`:
  - `Agov.PlanValidator` default manifest:
    `../../agf/agents/MANIFEST.yaml`
  - `mix agov.session.resume` default manifest:
    `../../agf/agents/MANIFEST.yaml`
  - `mix agov.usage.report` default manifest:
    `../../agf/agents/MANIFEST.yaml`
  - schema/root defaults:
    `../../agf/schemas`, `../../agf`
  - dist manifest default:
    `../../agf/dist/release-manifest.yaml`

## Gate 6 Assessment

Gate 6 asks whether `mix agov.*` commands still have hidden bootstrap defaults
into `agents-govern/`.

Result: **ready to close**.

The audit found no `agents-govern/` bootstrap default in the Elixir Mix command
code path. Remaining `agents-govern` hits in `tooling/elixir/test/` are test
fixtures or assertions, not runtime defaults.

## Residual References Classified as Acceptable

These residual references are acceptable for Gate 6:

- `AGENTS.md` describes `agents-govern/` as the bootstrap reference and
  explicitly states it is not the product.
- Agent IDs use the `agents-govern:<agent>:<version>` identity format.
- `agf/learnings/foundry.yaml` preserves legacy learning provenance.
- `agf/docs/contracts/*` and ADR files retain historical v1.x context.
- `agf/dist/release-manifest.yaml` forbids `agents-govern/**`.
- `tooling/elixir/test/fixtures/*` use realistic `agents-govern:*` agent IDs.

## Gate 7 Finding

Gate 7 asks for a release-review decision that bootstrap authority is no longer
active governance authority.

Result: **not ready to close without governor disposition**.

The audit found residual shipped AGF references that still look like current
Python-era command guidance:

- `agf/docs/site/reference/environment-variables.md` still describes
  `AGOV_MANIFEST_PATH` as defaulting to
  `agents-govern/agents/MANIFEST.yaml`, but the Elixir code now defaults to
  `../../agf/agents/MANIFEST.yaml`.
- `agf/agents/MANIFEST.yaml` contains the comment:
  `Run: cd agents-govern && uv run agov-validate-agents`.
- `agf/rules/always/gate-self-test-rule.md` contains a Python-era example
  command and references `src/agents_govern/validate_*` in follow-up guidance.
- `agf/rules/workflow/release-via-merge-request-rule.md` references
  `uv run agov-dist`.
- Shipped skills under `agf/skills/` still contain Python-era command examples:
  - `agf/skills/capability-review/SKILL.md`
  - `agf/skills/learning/SKILL.md`
  - `agf/skills/release/SKILL.md`
  - `agf/skills/contribution-audit/SKILL.md`
  - `agf/skills/upgrade/SKILL.md`
  - `agf/skills/health/SKILL.md`
  - `agf/skills/infosec-screen/SKILL.md`

These are not hidden Elixir tooling defaults, so they do not block Gate 6.
They are shipped adopter-facing guidance, so they should either be updated in a
follow-up docs/skills slice or explicitly accepted by governor-reviewer before
Gate 7 closes.

## Recommendation

- Close Gate 6.
- Do not close Gate 7 yet unless governor-reviewer explicitly accepts the
  residual shipped Python-era guidance as non-blocking.
- Preferred follow-up: approve a narrow docs/skills cleanup slice that updates
  the residual command examples or marks them as E4-deferred/historical.
