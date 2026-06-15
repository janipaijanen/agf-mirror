---
id: elixir-migration-e1-e2
status: complete
topic: python-tooling-inventory-and-command-matrix
gate1_approved_by: governor-reviewer
gate1_approved_at: "2026-06-15"
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: inventory and documentation only; no secrets, auth, or isolation changes
  - role: contribution-auditor
    status: not-applicable
    reason: internal foundry planning artifact; not an upstream submission
---

# Plan: E1-E2 — Python Tooling Inventory and Command Matrix

**Date:** 2026-06-15T19:14:52Z
**Status:** COMPLETE
**Context:** Elixir-only tooling direction approved (seq 322). E1 and E2 are research
and documentation artifacts; no code changes in this slice.

---

## Intent

E1: catalogue every Python tooling surface in `agents-govern/` that constitutes
migration debt under the Elixir-only direction.

E2: produce the command matrix mapping each Python command to its Elixir target,
establishing the replacement scope for E3–E6.

---

## E1: Python Tooling Inventory

### CLI Entry Points (pyproject.toml)

| Command | Module | Purpose |
|---|---|---|
| `agov` | `agents_govern.cli:main` | Top-level dispatcher |
| `agov-validate` | `agents_govern.orchestrator:main` | Orchestrate all validators |
| `agov-validate-learnings` | `agents_govern.validate_learnings:main` | Validate learning YAML records |
| `agov-validate-infosec` | `agents_govern.validate_infosec:main` | Anonymization / external-content check |
| `agov-validate-agents` | `agents_govern.validate_agents:main` | Agent definition YAML validation |
| `agov-validate-readme-stats` | `agents_govern.validate_readme_stats:main` | README statistics consistency |
| `agov-validate-acronyms` | `agents_govern.validate_acronyms:main` | Acronym register check |
| `agov-validate-adoption` | `agents_govern.validate_adoption:main` | Adoption document check |
| `agov-validate-contributions` | `agents_govern.validate_contributions:main` | Contribution record check |
| `agov-validate-binding` | `agents_govern.validate_binding:main` | Binding contract check |
| `agov-validate-framework-config` | `agents_govern.validate_framework_config:main` | Framework config check |
| `agov-pre-tag-check` | `agents_govern.pre_tag_check:main` | Version / VERSION / CHANGELOG guard before tag |
| `agov-validate-tag-push` | `agents_govern.validate_tag_push:main` | Tag push validation |
| `agov-dist` | `agents_govern.dist:main` | Distribution archive builder |
| `agov-bump` | `agents_govern.bump_version:main` | Version bump utility |
| `agov-health` | `agents_govern.health:main` | Health / sanity check |

Total: **16 commands** across 26 Python source files.

### Support Modules (no direct CLI, called internally)

`capabilities`, `correlation_id`, `graph`, `manifest`, `_files` (repo root detection),
`_output`, `_types`, `_yaml`, `_exit_codes`

### Non-Code Surfaces

| Surface | Type | Volume |
|---|---|---|
| `agf/docs/contracts/integration-contract.md` | Documentation | 39 `agov-*` references |
| 9 other `agf/docs/**` files | Documentation | ~41 further references |
| `tooling/elixir/lib/agov/brief_generator.ex:70` | Elixir code string | 1 `agents-govern/uv.lock` reference |
| `agents-govern/pyproject.toml` | Package manifest | v1.x package, maintenance mode |

Total doc surface: **~80 references across 10 files** (E5 scope).

### Already-Migrated Surfaces

| Python command | Elixir equivalent | Status |
|---|---|---|
| `agov-pre-tag-check` | `mix agov.dist.check_tag` | Functionally covered; verify equivalence before removing docs |
| `agov-validate-tag-push` | `mix agov.dist.check_tag` | Functionally covered (partial) |
| `agov-dist` | `mix agov.dist.build` | Covered |
| Coordination validation | `mix agov.coord.validate` | Covered |
| Plan validation | `mix agov.plan.validate` | Covered |
| Dist manifest validation | `mix agov.dist.validate` | Covered |
| Session resume | `mix agov.session.resume` | Covered |
| Token ROI | `mix agov.token.roi` | Covered |
| Usage report | `mix agov.usage.report` | Covered |
| Coord visibility/status/brief/stale/handoff | Mix tasks | Covered |

### CI Status

The root `.gitlab-ci.yml` is already Elixir-only. Python appears only in the
`mkdocs` base image (docs build, not agov tooling). No `uv run agov-*` in CI.

### Dist Manifest Status

`agents-govern/**` is in the `forbidden` list of `agf/dist/release-manifest.yaml`.
Python tooling is **not shipped** in AGF v2 distribution. E6 pre-condition is met
at the distribution layer.

### Gate 6 Re-Assessment

Gate 1 (seq 350–351) fixed the critical bootstrap defaults in Elixir tooling:
- `@default_manifest` → `agf/agents/MANIFEST.yaml`
- `@default_base_branch` → `main`

The only remaining `agents-govern` string in Elixir code is
`brief_generator.ex:70` — a session brief template listing `agents-govern/uv.lock`
as an expected project file. This is a content string, not a bootstrap default.
Gate 6 is substantially addressed; a final audit after E3 closes it formally.

---

## E2: Command Matrix

### Coverage Gaps (E3 scope)

| Python command | Elixir target | Priority | Notes |
|---|---|---|---|
| `agov-validate-learnings` | `mix agov.validate.learnings` | 1 | Active governance use; validates `agf/learnings/*.yaml` |
| `agov-validate-agents` | `mix agov.validate.agents` | 2 | Agent YAML against schema |
| `agov-validate-binding` | `mix agov.validate.binding` | 3 | Binding contract check |
| `agov-validate-adoption` | `mix agov.validate.adoption` | 4 | Adoption doc check |
| `agov-validate-contributions` | `mix agov.validate.contributions` | 5 | Contribution records |
| `agov-validate-infosec` | `mix agov.validate.infosec` | 6 | Anonymization / forbidden-term scan |
| `agov-validate-framework-config` | `mix agov.validate.framework_config` | 7 | Framework config check |
| `agov-validate` | `mix agov.validate` | 8 | Orchestrator; wraps the above |

### Lower Priority (E4 / deferred)

| Python command | Elixir target | Notes |
|---|---|---|
| `agov-bump` | `mix agov.bump` | Version bump utility; single-purpose, low risk |
| `agov-health` | `mix agov.health` | Sanity check; low operational frequency |
| `agov-validate-readme-stats` | `mix agov.validate.readme_stats` | Stats consistency; not in primary adoption path |
| `agov-validate-acronyms` | `mix agov.validate.acronyms` | Acronym register; not in primary adoption path |

### Proposed E3–E6 Scope

| Slice | Scope | Blocker for Gate |
|---|---|---|
| E3 | Implement 8 priority validators as Mix tasks | Gate 6 formal close |
| E4 | Implement `mix agov.bump`, `mix agov.health`, lower-priority validators | Gate 6 |
| E5 | Update 10 doc files to reference Mix commands; fix `brief_generator.ex:70` | Gate 7 |
| E6 | Final audit: confirm no bootstrap defaults remain; governor closes Gate 6 + 7 | Release |

---

## Fast-Track Waiver

Documentation and research artifact only. No code changes. No affected modules.
Plan file captures findings for E3–E6 Gate 1 planning.
