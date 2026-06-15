---
id: gate7-python-guidance-cleanup
status: pending-gate1
topic: gate7-residual-python-guidance-cleanup
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: documentation, rule text, and skill command examples only; no secrets, auth, network, or isolation changes
  - role: contribution-auditor
    status: not-applicable
    reason: internal foundry release-gate cleanup; not an upstream contribution submission
---

# Plan: Gate 7 Residual Python Guidance Cleanup

**Date:** 2026-06-15T21:14:55Z
**Status:** PENDING GATE 1
**Depends on:** E6 final audit accepted; Gate 6 closed; Gate 7 left open.
**Proposed branch:** `docs/gate7-python-guidance-cleanup`
**Expected implementer:** coder-tester

---

## Intent

Remove or explicitly classify the residual shipped Python-era command guidance
identified by the E6 final audit so governor-reviewer can close Gate 7.

The cleanup should make adopter-facing AGF v2 guidance point at current Elixir
Mix tasks where equivalents exist, and mark E4-deferred commands plainly where
they do not.

---

## Scope

Update only the residual files identified by the E6 audit and follow-up scan:

| File | Planned action |
|---|---|
| `agf/docs/site/reference/environment-variables.md` | Correct `AGOV_MANIFEST_PATH` defaults to `agf/agents/MANIFEST.yaml` / `../../agf/agents/MANIFEST.yaml`. |
| `agf/agents/MANIFEST.yaml` | Replace stale `cd agents-govern && uv run agov-validate-agents` comment with current Mix validation guidance. |
| `agf/rules/always/gate-self-test-rule.md` | Replace generic Python-era validator example with Mix-era wording or mark as illustrative pseudo-command; replace `src/agents_govern/validate_*` follow-up path with current tooling path guidance. |
| `agf/rules/workflow/release-via-merge-request-rule.md` | Replace `uv run agov-dist` with current distribution Mix commands. |
| `agf/skills/capability-review/SKILL.md` | Replace migrated validator commands with Mix equivalents; mark graph/strict/health coverage deferred where no Mix equivalent exists. |
| `agf/skills/learning/SKILL.md` | Replace learning and infosec validator examples with Mix equivalents. |
| `agf/skills/release/SKILL.md` | Mark bump/health as E4-deferred; use current dist commands where release verification is discussed. |
| `agf/skills/contribution-audit/SKILL.md` | Replace contribution validator example with `mix agov.validate.contributions`. |
| `agf/skills/upgrade/SKILL.md` | Replace learning and infosec validator examples with Mix equivalents. |
| `agf/skills/health/SKILL.md` | Mark health as E4-deferred or replace with currently available aggregate checks. |
| `agf/skills/infosec-screen/SKILL.md` | Replace infosec validator examples with `mix agov.validate.infosec`; remove or mark unsupported `--include-md`. |

If implementation discovers additional current-looking Python-era command
guidance in shipped AGF v2 files, coder-tester may include it if it matches the
same cleanup intent.

---

## Non-Goals

- Do not implement E4 commands.
- Do not change validator behavior.
- Do not remove historical v1.x contract or ADR context.
- Do not rename `agents-govern`.
- Do not perform version bumps or tags.

---

## Implementation Guidance

- Use current Mix task names for migrated commands:
  - `mix agov.validate`
  - `mix agov.validate.agents`
  - `mix agov.validate.learnings`
  - `mix agov.validate.infosec`
  - `mix agov.validate.contributions`
  - `mix agov.dist.validate`
  - `mix agov.dist.build`
  - `mix agov.dist.check_tag`
- Mark missing commands as E4-deferred instead of pretending they exist:
  - `mix agov.bump`
  - `mix agov.health`
  - `mix agov.validate.acronyms`
  - `mix agov.validate.readme_stats`
  - `mix agov.validate.binding`
- Keep command examples runnable from the documented working directory, or
  state the required `cd tooling/elixir` context.
- Keep edits textual and narrowly scoped.

---

## Verification

Required checks:

```bash
git diff --check
cd tooling/elixir
mix compile --warnings-as-errors
mix test
mix agov.validate
```

Audit scans:

```bash
rg -n "uv run agov|cd agents-govern && uv|agents-govern/agents/MANIFEST|src/agents_govern|pyproject\\.toml" agf/docs/site/reference/environment-variables.md agf/agents/MANIFEST.yaml agf/rules agf/skills
rg -n "agov-validate|agov-dist|agov-health|agov-bump" agf/docs/site/reference/environment-variables.md agf/agents/MANIFEST.yaml agf/rules agf/skills
```

Expected result:

- No current-looking `uv run agov-*` examples remain in the scoped shipped
  guidance.
- Remaining `agov-*` references are either current Mix task names, E4-deferred
  notes, identifiers, or historical context.

---

## Gate 2 Acceptance Criteria

- The E6 residual shipped-guidance list is resolved or explicitly classified.
- Gate 7 is ready for governor-reviewer closure.
- A Gate 2 handoff routes the closure decision to governor-reviewer.
