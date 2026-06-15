---
id: elixir-migration-e5-docs
status: pending-gate1
topic: elixir-tooling-documentation-update
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: documentation and one static brief string only; no secrets, auth, network, or isolation changes
  - role: contribution-auditor
    status: not-applicable
    reason: internal foundry migration documentation; not an upstream contribution submission
---

# Plan: E5 — Elixir Tooling Documentation Update

**Date:** 2026-06-15T20:39:10Z
**Status:** PENDING GATE 1
**Depends on:** E1+E2 complete; E3 merged and Gate 2 closed.
**Proposed branch:** `docs/elixir-migration-e5-docs`
**Expected implementer:** coder-tester

---

## Intent

Update adopter-facing and internal AGF v2 documentation so it no longer points
readers at Python-era `agov-*` / `uv run agov-*` commands for tooling that now
has Elixir Mix equivalents.

This slice also fixes the remaining Elixir session-brief content string that
still lists `agents-govern/uv.lock` as an expected project file.

E5 is intentionally documentation-first. It should not implement new validators
or change validator behavior.

---

## Scope

### Replace migrated validator references

For validators migrated by E3, update command examples and prose to use Mix task
names:

| Python-era command | Current Elixir command |
|---|---|
| `agov-validate` | `mix agov.validate` |
| `agov-validate-learnings` | `mix agov.validate.learnings` |
| `agov-validate-agents` | `mix agov.validate.agents` |
| `agov-validate-adoption` | `mix agov.validate.adoption` |
| `agov-validate-infosec` | `mix agov.validate.infosec` |
| `agov-validate-contributions` | `mix agov.validate.contributions` |
| `agov-validate-framework-config` | `mix agov.validate.framework_config` |

### Preserve deferred/E4 status where needed

Do not claim Elixir coverage for commands deferred from E3:

| Python-era command | E5 treatment |
|---|---|
| `agov-validate-binding` | Mark as deferred to E4 or describe as v1.x/Python-era where historically referenced. |
| `agov-validate-readme-stats` | Mark as deferred to E4 where current-state docs require status. |
| `agov-validate-acronyms` | Mark as deferred to E4 where current-state docs require status. |
| `agov-bump` | Mark as deferred to E4. |
| `agov-health` | Mark as deferred to E4. |

Distribution commands already have Elixir equivalents and should remain or be
normalized to existing Mix task names:

| Python-era command | Current Elixir command |
|---|---|
| `agov-dist` | `mix agov.dist.build` |
| `agov-pre-tag-check` | `mix agov.dist.check_tag` |
| `agov-validate-tag-push` | `mix agov.dist.check_tag` where tag/version validation is the relevant behavior. |

---

## Affected Files

Reference scan:

```bash
rg -l "agov-validate|agov-pre-tag-check|agov-validate-tag-push|agov-bump|agov-dist|agov-health|uv run agov|pyproject\\.toml|src/agents_govern|agents-govern/uv\\.lock|pre-tag-check" agf/docs agf/framework.md tooling/elixir/lib/agov/brief_generator.ex
```

Current expected edit surface:

| File | Planned action |
|---|---|
| `tooling/elixir/lib/agov/brief_generator.ex` | Replace `agents-govern/uv.lock` session-brief project file hint with current Elixir/tooling evidence. |
| `agf/framework.md` | Update InfoSec Sentinel command reference from `uv run agov-validate-infosec` to the Mix task. |
| `agf/docs/governance/learning-system.md` | Update infosec and agent validator examples; mark README stats validator as deferred if retained. |
| `agf/docs/governance/governance.md` | Update verification checklist and InfoSec Sentinel command naming. |
| `agf/docs/contracts/contract-stability.md` | Reframe command-name stability from Python entry points to current Mix task surface plus deferred E4 items. |
| `agf/docs/contracts/integration-contract.md` | Update validator command table, path trigger table, and orchestrator references to current Mix tasks; preserve historical v1.x notes explicitly. |
| `agf/docs/contracts/binding-contract.md` | Mark binding validator command as deferred to E4 / Python-era until migrated. |
| `agf/docs/adopt/decommission.md` | Update adopter decommission command examples and CI cleanup wording. |
| `agf/docs/adopt/compliance-profile.md` | Update framework config validator reference; mark binding validator as deferred where applicable. |
| `agf/docs/adr/20260505-lean-agile-test-fly.md` | Preserve historical ADR context while avoiding present-tense claims that Python validators remain current. |
| `agf/docs/agent-communication-0.26.0.md` | Historical generated artifact; update only if current-state wording is misleading, otherwise leave as historical. |

If implementation discovery finds additional current-state references under
`agf/docs/` or `agf/framework.md`, coder-tester may include them if they match
the same command-migration intent.

---

## Non-Goals

- Do not implement E4 commands.
- Do not change validator behavior.
- Do not rename or delete `agents-govern/`.
- Do not alter historical coordination records.
- Do not perform version bumps or tags.
- Do not edit release archives.

---

## Implementation Guidance

1. Create branch `docs/elixir-migration-e5-docs`.
2. Update command references in the affected docs.
3. Use present-tense current-state wording only for commands that exist today.
4. Use explicit deferred wording for E4 commands.
5. Preserve historical documents as historical where rewriting them would erase
   decision context.
6. Replace `agents-govern/uv.lock` in the session brief template with a current
   AGF-NG tooling path, likely `tooling/elixir/mix.lock`.
7. Keep edits textual and narrowly scoped.

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

Documentation audit checks:

```bash
rg -n "uv run agov|agents-govern/uv\\.lock" agf/docs agf/framework.md tooling/elixir/lib/agov/brief_generator.ex
rg -n "agov-validate|agov-pre-tag-check|agov-validate-tag-push|agov-bump|agov-dist|agov-health" agf/docs agf/framework.md
```

Expected audit result:

- No current-state `uv run agov-*` examples remain for E3-migrated commands.
- No `agents-govern/uv.lock` session-brief hint remains.
- Remaining `agov-validate-*` references are either historical v1.x context or
  explicitly marked deferred to E4.

---

## Gate 2 Acceptance Criteria

- Current docs guide users to existing Mix tasks for E3-migrated validators.
- Deferred commands are not described as already migrated.
- `brief_generator.ex` no longer points users at `agents-govern/uv.lock`.
- Validation commands above pass.
- A Gate 2 handoff records the exact residual command references and why any
  remain.
