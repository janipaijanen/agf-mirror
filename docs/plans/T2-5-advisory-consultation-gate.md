---
id: T2-5-advisory-gate
status: pending-gate1
topic: advisory-consultation-gate
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: tooling addition only — no security boundary, auth, or secrets changes
  - role: contribution-auditor
    status: consulted
    reason: reviewed as seq 10 (contribution-auditor, codex-gpt-5)
---

# Plan: T2-5 — Advisory Consultation Gate Enforcement

**Date:** 2026-06-11T00:18:35Z
**Revised:** 2026-06-11 — addresses contribution-auditor review (seq 10)
**Status:** PENDING GATE 1 APPROVAL
**Depends on:** T1-1 Phase B (tooling/elixir/ Mix project, merged ✓)

---

## Intent

Make advisory consultation a hard gate at Gate 1: every plan file must declare
the status of each required advisory role (consulted, waived with reason, or
not-applicable). A plan missing any required role, or with a `waived` entry
lacking a reason, fails validation.

The current enforcement gap: advisory roles are defined but nothing in the flow
forces a decision about them. Plans are submitted without the planner asking
"should infosec-sentinel or contribution-auditor see this?"

---

## Scope

Three deliverables, all self-contained:

1. **`spec/schemas/gate1-plan.schema.json`** — JSON schema for plan front matter shape
2. **`mix agov.plan.validate <file>`** — Mix task combining schema validation +
   required-role enforcement
3. **Retrofit existing plan files** with conformant front matter

**Trigger mapping is explicitly deferred.** This plan enforces role accounting
(every required role has a declared status). Automatic detection of which roles
a plan *should* consult based on paths/topics is a follow-up task. Without the
accounting gate, trigger detection has nothing to enforce against.

---

## Affected files

| File | Action |
|---|---|
| `spec/schemas/gate1-plan.schema.json` | Create — JSON schema for plan front matter |
| `tooling/elixir/lib/agov/plan_validator.ex` | Create — schema + required-role validation |
| `tooling/elixir/lib/mix/tasks/agov.plan.validate.ex` | Create — Mix task |
| `tooling/elixir/test/agov/plan_validator_test.exs` | Create — unit tests |
| `tooling/elixir/test/fixtures/valid_plan.md` | Create — valid fixture |
| `tooling/elixir/test/fixtures/invalid_plan_no_advisory.md` | Create — missing section |
| `tooling/elixir/test/fixtures/invalid_plan_waived_no_reason.md` | Create — waived, no reason |
| `tooling/elixir/test/fixtures/invalid_plan_missing_infosec.md` | Create — missing required role |
| `tooling/elixir/test/fixtures/invalid_plan_missing_auditor.md` | Create — missing required role |
| `tooling/elixir/test/fixtures/invalid_plan_duplicate_role.md` | Create — duplicate entry |
| `docs/plans/T1-1-coordination-kind-schema.md` | Update — add front matter |
| `docs/plans/T1-1-phase-b-elixir-validator.md` | Update — add front matter |
| `docs/plans/T1-3-handoff-state-tooling.md` | Update — add front matter |
| `docs/plans/T2-5-advisory-consultation-gate.md` | Update — self-applied (done in this revision) |

---

## Schema design

```yaml
---
id: T2-5-advisory-gate
status: pending-gate1
topic: advisory-consultation-gate
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: no security boundary changes
  - role: contribution-auditor
    status: consulted
---
```

**Advisory status rules:**

| Status | `reason` field | Enforced by |
|---|---|---|
| `consulted` | optional | schema |
| `waived` | REQUIRED | schema `if/then` |
| `not-applicable` | optional (encouraged) | schema |

**Required advisory roles (enforced in Elixir, not JSON Schema):**
- `infosec-sentinel`
- `contribution-auditor`

**Why Elixir, not JSON Schema:** JSON Schema's `contains`/`allOf` can express
required array entries but is verbose and opaque. The split is clean:
JSON Schema owns shape (field types, status enum, waived→reason);
Elixir owns semantics (required roles present, no duplicates, no unknown roles).

**Unknown roles:** Rejected with a clear error — prevents silent typos from
producing a false-positive pass.

---

## Migration needed

No database migrations. Existing plan files get YAML front matter prepended
(additive, no content removed).

---

## Implementation plan

### Step 1 — Define and write the JSON schema

`spec/schemas/gate1-plan.schema.json`:
- Required: `id`, `status`, `advisory_consultation`
- `status`: enum `["pending-gate1", "approved", "active", "complete"]`
- `advisory_consultation`: array, minItems 1
- Each entry: required `role` (string), `status` (enum consulted|waived|not-applicable)
- If `status == "waived"`: `reason` is required (enforced via `if/then`)

### Step 2 — Write tests FIRST (TDD RED)

Fixtures (7 total — 1 valid, 6 invalid):
- `valid_plan.md` — infosec-sentinel + contribution-auditor, all statuses valid
- `invalid_plan_no_advisory.md` — missing `advisory_consultation` entirely
- `invalid_plan_waived_no_reason.md` — waived entry, no reason field
- `invalid_plan_missing_infosec.md` — only contribution-auditor present
- `invalid_plan_missing_auditor.md` — only infosec-sentinel present
- `invalid_plan_duplicate_role.md` — infosec-sentinel listed twice
- `invalid_plan_unknown_role.md` — unknown role string (e.g. `made-up-advisor`)

Tests for `PlanValidator.validate_file/1`:
- Valid plan → `{:ok, data}`
- Missing advisory → `{:error, errors}` (schema error)
- Waived without reason → `{:error, errors}` (schema error)
- Missing infosec-sentinel → `{:error, errors}` (Elixir semantic check)
- Missing contribution-auditor → `{:error, errors}` (Elixir semantic check)
- Duplicate role entry → `{:error, errors}` (Elixir semantic check)
- Unknown role → `{:error, errors}` (Elixir semantic check)
- No front matter → `{:error, :no_frontmatter}`

### Step 3 — Implement PlanValidator (GREEN)

`lib/agov/plan_validator.ex`:

1. `validate_file(path)` — same pipeline as `CoordValidator`:
   read → extract frontmatter → parse YAML → validate schema → validate semantics
2. `validate_semantics(data)` — Elixir-only checks:
   - All required roles present (exactly once)
   - No unknown roles
   - No duplicate roles
3. Schema path: `AGOV_SCHEMA_DIR` env var, default `../../spec/schemas`,
   resolved at runtime against cwd (same pattern as `CoordValidator`).

### Step 4 — Implement Mix task

`mix agov.plan.validate <file> [<file>...]`

Run from `tooling/elixir/` (the Mix project root). Paths to plan files are
relative to cwd. Example from repo root:

```bash
cd tooling/elixir && mix agov.plan.validate ../../docs/plans/T2-5-advisory-consultation-gate.md
```

Same OK/ERR output pattern and exit codes as `agov.coord.validate`.

### Step 5 — Retrofit existing plan files

Add conformant front matter to T1-1-a, T1-1-b, T1-3 plan files.
All three are internal tooling tasks → both required roles are `not-applicable`
with reasons.

---

## Test strategy

| Tier | What | Where |
|---|---|---|
| Unit | `PlanValidator.validate_file/1` — 1 valid + 7 invalid cases | `test/agov/plan_validator_test.exs` |
| Manual | `mix agov.plan.validate` from `tooling/elixir/` against all 4 plan files | Run locally, inspect output |

TDD order: schema → fixtures → tests → RED → PlanValidator → GREEN → task.

---

## Risks

| Risk | Mitigation |
|---|---|
| Retrofitting old plans breaks something | Old plans are documentation only; front matter is purely additive |
| Required role list expands | Change the Elixir constant in `PlanValidator`; tests catch any drift |
| Trigger mapping pressure | Explicitly deferred — note in this plan prevents scope creep |
| CWD confusion for manual invocation | Documented: always run from `tooling/elixir/` |
