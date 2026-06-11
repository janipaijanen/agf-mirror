# Plan: T2-5 — Advisory Consultation Gate Enforcement

**Date:** 2026-06-11T00:18:35Z
**Status:** PENDING GATE 1 APPROVAL
**Depends on:** T1-1 Phase B (tooling/elixir/ Mix project, merged ✓)

---

## Intent

Make advisory consultation a hard gate at Gate 1: every plan file must declare
the status of each advisory role (consulted, waived with reason, or
not-applicable). A plan without this section fails validation.

The current enforcement gap: advisory roles are defined but nothing in the flow
forces a decision about them. Plans are submitted without the planner ever asking
"should infosec-sentinel or contribution-auditor see this?"

---

## Scope

Three deliverables, all self-contained:

1. **`spec/schemas/gate1-plan.schema.json`** — JSON schema for plan front matter
2. **`mix agov.plan.validate <file>`** — Mix task that validates a plan file
3. **Retrofit existing plan files** with conformant front matter

No changes to `spec/schemas/coordination-message.schema.json`. No changes to
agent YAML files. The development-workflow-rule already mentions advisory
consultation — we formalize what "done" means.

---

## Affected files

| File | Action |
|---|---|
| `spec/schemas/gate1-plan.schema.json` | Create — JSON schema for plan front matter |
| `tooling/elixir/lib/agov/plan_validator.ex` | Create — parse + validate plan front matter |
| `tooling/elixir/lib/mix/tasks/agov.plan.validate.ex` | Create — Mix task |
| `tooling/elixir/test/agov/plan_validator_test.exs` | Create — unit tests |
| `tooling/elixir/test/fixtures/valid_plan.md` | Create — valid plan fixture |
| `tooling/elixir/test/fixtures/invalid_plan_no_advisory.md` | Create — missing advisory section |
| `tooling/elixir/test/fixtures/invalid_plan_waived_no_reason.md` | Create — waived without reason |
| `docs/plans/T1-1-coordination-kind-schema.md` | Update — add front matter |
| `docs/plans/T1-1-phase-b-elixir-validator.md` | Update — add front matter |
| `docs/plans/T1-3-handoff-state-tooling.md` | Update — add front matter |
| `docs/plans/T2-5-advisory-consultation-gate.md` | Update — add front matter (self-applies) |

---

## Schema design

```yaml
---
id: T2-5-advisory-gate               # kebab-case task identifier
status: pending-gate1                 # pending-gate1 | approved | active | complete
topic: advisory-consultation-gate
gate1_approved_by: ""                 # filled on approval
gate1_approved_at: ""
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: no security boundary changes in this task
  - role: contribution-auditor
    status: not-applicable
    reason: internal tooling, not an upstream proposal
---
```

**Advisory status rules (enforced by schema):**
- `consulted` — role was invoked and responded; no reason field required
- `waived` — explicitly skipped; `reason` field REQUIRED
- `not-applicable` — role's domain is not touched; `reason` field RECOMMENDED
  (allowed without, but encouraged)

**Required advisory roles for this project** (from MANIFEST active + suspended set):
- `infosec-sentinel` — any change to security posture, secrets, auth, tenant isolation
- `contribution-auditor` — any change intended for upstream submission

Other roles are suspended/skipped and are always `not-applicable` for internal tasks.

---

## Migration needed

No database migrations. Existing plan files get YAML front matter prepended
(additive; no content removed).

---

## Implementation plan

### Step 1 — Define and write the JSON schema

`spec/schemas/gate1-plan.schema.json`:
- Required top-level: `id`, `status`, `advisory_consultation`
- `status`: enum `["pending-gate1", "approved", "active", "complete"]`
- `advisory_consultation`: array, minItems 1
- Each entry: required `role` (string), `status` (enum consulted|waived|not-applicable)
- If `status == "waived"`: `reason` is required (enforced via `if/then`)

### Step 2 — Write tests FIRST (TDD RED)

Fixtures:
- `valid_plan.md` — full front matter, advisory section with all three statuses
- `invalid_plan_no_advisory.md` — missing `advisory_consultation` field
- `invalid_plan_waived_no_reason.md` — waived entry with no reason

Tests for `PlanValidator`:
- Valid plan returns `{:ok, data}`
- Missing advisory returns `{:error, errors}`
- Waived without reason returns `{:error, errors}`
- No front matter returns `{:error, :no_frontmatter}`

### Step 3 — Implement PlanValidator (GREEN)

Reuse `CoordValidator.extract_frontmatter/1` and the shared schema-load pattern.
`PlanValidator.validate_file/1` follows the same contract as `CoordValidator.validate_file/1`.

Schema path: `AGOV_SCHEMA_DIR` env var (same convention, default `../../spec/schemas`).

### Step 4 — Implement Mix task

`mix agov.plan.validate <file> [<file>...]` — same OK/ERR pattern as `agov.coord.validate`.

### Step 5 — Retrofit existing plan files

Add conformant front matter to the three existing plan files and this plan.
Each gets `advisory_consultation` with project-standard entries.

---

## Test strategy

| Tier | What | Where |
|---|---|---|
| Unit | `PlanValidator.validate_file/1` for valid + 3 invalid cases | `test/agov/plan_validator_test.exs` |
| Manual | `mix agov.plan.validate docs/plans/T2-5-advisory-consultation-gate.md` | Run locally |

TDD order: schema → fixtures → tests → RED → implementation → GREEN.

---

## Risks

| Risk | Mitigation |
|---|---|
| Retrofitting old plans breaks something | Old plans are documentation only; front matter is purely additive |
| Advisory role list grows | Schema allows any string for `role`; project list is enforced by convention, not enum |
| `not-applicable` without reason accepted | Allowed by design — the discipline is for `waived` (deliberate skip) |
