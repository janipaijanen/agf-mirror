---
id: adopter-applicability-catalog
status: pending-gate1
topic: adopter-applicability-catalog
advisory_consultation:
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      Schema and documentation additions only; no credentials, access
      control, runtime, or security-boundary change. The rule/agent schema
      edits add optional descriptive metadata fields, not new capabilities.
  - role: contribution-auditor
    status: consulted
    reason: >
      Touches agf/schemas/*.json, agf/agents/*.yaml, and agf/rules/**, all
      downstream-impact-rule trigger paths. Self-performed in-session per
      the formal-advisory-role pattern (learning 20260703T095527Z-fadv),
      checked against contribution-auditor.yaml's stated red flags.
      Governance relaxation without evidence — none found; framework.md's
      existing, stricter claims are the reference point, not loosened.
      Boundary erosion — none found; classification/validator changes stay
      advisory-only, explicitly scoped out of changing what ships or
      activates by default. Absence-of-incident reasoning — none found;
      triggered by a concretely verified framework.md/ADOPTING.md
      contradiction, not an absence argument. Precedent injection — none
      found; every quantitative claim (45 rules, 12 roles, the framework.md
      line numbers) was independently re-verified via grep/count before
      inclusion, not taken on trust. Caveat, stated per
      verifiable-claims-rule — this is a same-agent self-check, not
      independent cross-model review, and therefore weaker evidence than
      Codex's review of MR !275 itself. Recommend Gate 2 get an independent
      pass if a second reviewing agent is available.
---

# Plan: adopter-facing role/rule applicability catalog

**Date:** 2026-07-20
**Status:** Gate 1 proposed — responds to Codex's post-MR-!275 finding
(`docs/coordination/20260720T225540Z-coder-tester-rgrs.md`, `requires-ack:
true`). Not yet governor-approved.

## Trigger and problem

MR !275 correctly distinguishes the complete shipped library from the subset
an adopter activates, and correctly labels a reduced starter package Level 1
Planned. But it does not — and per its own scope statement, does not attempt
to — tell an adopter *which* of the 45 rule files and 12 role definitions
belong in which category, or what is lost by removing one.

Codex's `rgrs` finding, independently verified before writing this plan:

- `agf/schemas/rule.schema.json` has `scope` (directory) and `applies_to`
  (actor type) but no field for adopter-facing classification, safe-removal
  condition, dependency, or forfeited-guarantee.
- `agf/schemas/agent.schema.json` has `tier` (production / governance /
  cross-cutting / publication-support / meta) and `phase` (integer), which
  classify a role's *kind*, not whether a given adopter should activate it.
- `agf/rules/README.md` catalogs only a historical rule subset, not all 45.
- **A real, verified reconciliation gap exists between `agf/framework.md` and
  the adopter-facing docs.** `framework.md:1399-1413` lists **four** "core
  agents (always active)" for Phase 1 — Planner-Architect, Coder-Tester,
  Governor-Reviewer, **and Build Engineer** — plus Contribution Auditor and
  InfoSec Sentinel as separately "always active" cross-cutting agents,
  mandatory from first contribution / first learning record respectively.
  `agf/ADOPTING.md:1088-1090`, by contrast, lists "Activate Build Engineer
  (Phase 1)", "Add Auditor agent (Phase 4)", and "Add Sentinel agent (Phase
  4)" in its "signals to grow" table — phrasing that reads as optional,
  trigger-based activation, and puts the two cross-cutting agents at Phase 4
  instead of Phase 1. Grepping the current public commissioning docs
  (`should-you-adopt.md`, `install-and-layouts.md`, `roles-and-gates.md`,
  `quick-reference.md`) for these three role names returns zero mentions —
  they are not named in adopter-facing guidance at all today. An
  applicability catalog built on top of this contradiction would just
  encode the disagreement, not resolve it.

## What this plan does

Two things, deliberately kept separable:

1. **Reconcile the Build Engineer / Contribution Auditor / InfoSec Sentinel
   activation-phase contradiction** between `framework.md` (source of truth
   for role design) and `agf/ADOPTING.md` (adopter-facing growth guidance),
   before any catalog can honestly describe them.
2. **Add optional, schema-validated applicability metadata** to rules and
   agent definitions, and a generated human-readable catalog page — for a
   first representative slice, not a full backfill (see Explicitly out of
   scope).

## Design

### Schema additions

`agf/schemas/rule.schema.json` — new optional properties (existing
`required: ["id", "scope", "applies_to", "evidence"]` unchanged, so no
existing rule file becomes invalid):

```json
"classification": {
  "type": "string",
  "enum": ["core", "conditional", "optional"],
  "description": "Adopter-facing: core rules apply regardless of stack; conditional rules apply when their subject is present; optional rules apply only for specific team/regulatory choices."
},
"activation_condition": {
  "type": "string",
  "description": "For conditional/optional only: the concrete signal that this rule's subject is present (e.g. 'project has a UI', 'project handles PII')."
},
"safe_to_remove_when": {
  "type": "string",
  "description": "The condition under which deleting this rule file from the checkout does not silently reintroduce the failure mode it guards against."
},
"removal_forfeits": {
  "type": "string",
  "description": "The specific AGF guarantee or conformance claim lost by removing this rule. Required alongside safe_to_remove_when when classification is not core."
},
"depends_on": {
  "type": "array",
  "items": {"type": "string"},
  "description": "Rule IDs this rule assumes are also active."
}
```

`agf/schemas/agent.schema.json` — analogous additions under `agent`, plus
the distinction Codex's finding asked for explicitly:

```json
"classification": {
  "type": "string",
  "enum": ["core", "conditional", "optional"]
},
"activation_condition": { "type": "string" },
"deactivation": {
  "type": "object",
  "properties": {
    "local_deactivation": {
      "type": "string",
      "description": "How to stop using this role in a project's own manifest/config without touching agf/. Reversible, no forfeited guarantee beyond this role's own function."
    },
    "definition_removal": {
      "type": "string",
      "description": "What deleting agf/agents/<name>.yaml itself forfeits — normally worse than local deactivation, since it removes the reusable definition, not just this project's use of it."
    }
  }
}
```

Both additions are **optional** at the schema level in this plan. Existing
rule/agent files with no new fields remain valid — this plan does not force
a flag day across 45 rules and 12 roles in one MR (see Explicitly out of
scope).

### Reconciliation fix

Update `agf/ADOPTING.md`'s "signals to grow" table (lines ~1086-1092) so
Build Engineer, Contribution Auditor, and InfoSec Sentinel are not listed as
Phase-3/4 growth triggers when `framework.md` classifies them Phase 1
always-active. Concretely: move Build Engineer's row out of the growth-signal
table into the Phase 1 minimum description (matching `framework.md`'s own
"core agents (always active)" table), and correct Contribution
Auditor/InfoSec Sentinel from "Phase 4" to "Phase 1, always active" with
their actual framework.md-stated triggers ("mandatory from first
contribution", "mandatory from first learning record submission") — not a
generic growth signal.

### First representative slice (this plan's actual backfill)

Populate the new fields for exactly the artifacts this plan's own trigger
concerns, so the reconciliation fix and the schema are demonstrated on real
data, not left as an empty capability:

- **Build Engineer** (`agf/agents/build-engineer.yaml`): `classification:
  core`, matching `framework.md`.
- **Contribution Auditor** (`agf/agents/contribution-auditor.yaml`):
  `classification: core`.
- **InfoSec Sentinel** (`agf/agents/infosec-sentinel.yaml`):
  `classification: core`.
- **Planner-Architect, Coder-Tester, Governor-Reviewer**: `classification:
  core` (uncontested; already treated as mandatory everywhere).
- Two illustrative rules, one at each end of the classification axis, to
  prove the rule-schema fields against real content: `always/security-
  hardening-rule.md` (`classification: core`) and `ui/accessibility-rule.md`
  (`classification: conditional`, `activation_condition: "project has a UI"`,
  `safe_to_remove_when: "the project has and will have no user-facing
  interface"`, `removal_forfeits: "WCAG AA conformance checking for any UI
  added later without re-adding this rule"`).

### Catalog page

New page `agf/docs/site/docs/adopt/applicability-catalog.md`, linked from
`should-you-adopt.md`'s "What ships and what you activate" section (added by
MR !275) and from `install-and-layouts.md`'s rule-review step. For this
plan's scope, the page lists the classified slice above plus an explicit,
honestly-labeled note that the remaining rules/roles are unclassified
pending the follow-up in "Explicitly out of scope" — not silently omitted.

### Validator behavior

`agov validate agents` / `agov validate rules` (or the umbrella `agov
validate`) gains an **advisory, non-blocking** check: if `classification` is
present and is `conditional` or `optional`, `activation_condition` and
`safe_to_remove_when`/deactivation fields must also be present (schema-level
`if`/`then`, same pattern already used elsewhere in these schemas for
kind-conditional requirements). If `classification` is absent entirely, no
warning — absence means "not yet classified," not "core by default." This
keeps the 45-rule/12-role gap honestly visible rather than defaulting
everything to `core` for convenience.

## Explicitly out of scope

- **Full backfill across all 45 rules and the remaining 9 agent
  definitions.** This plan classifies 8 artifacts (6 roles, 2 rules) to
  prove the mechanism on real, contested content. Classifying the rest is
  real editorial judgment per rule/role, not a mechanical migration, and
  belongs in a follow-up plan sized so a reviewer can answer one yes/no
  question per batch (per `branch-and-worktree-rule`'s sizing test) — not
  one MR asserting 57 individual judgment calls at once.
- **The profile generator** Codex's finding mentions as a possible consumer
  of this metadata. Stays Level 1 Planned, as Codex's own finding said,
  until this metadata exists and has been exercised.
- **Making `classification` required** on the rule/agent schemas. Required
  now would force classifying all 57 artifacts in this same change to keep
  existing files schema-valid — exactly the flag-day this plan avoids.
- **Any change to which rules/roles are actually shipped, activated by
  default, or removed.** This plan documents and reconciles existing
  claims; it does not change Phase 1 default composition.

## Downstream impact and migration

This plan touches `agf/schemas/*.json`, `agf/agents/*.yaml`, and
`agf/rules/**/*.md` — all `downstream-impact-rule` trigger paths.

Classification: **ADDITIVE**. New optional schema fields (existing
`required` arrays on both schemas are unchanged); one reconciliation
correction to an internal-consistency defect in `ADOPTING.md` (bringing it
into agreement with `framework.md`'s own existing claims, not changing
`framework.md` itself); a new adopter-facing page; a non-blocking validator
check that only fires on partial (not absent) classification. No existing
rule or agent file becomes invalid.

```
Impact:
  Foundry:          No — this project's own agov-mcp/agov CLI and coordination
                     tooling are unaffected; only descriptive schema fields and
                     docs change.
  Direct adopters:  ADDITIVE — new optional fields adopters may ignore
                     entirely; the ADOPTING.md correction changes displayed
                     guidance (Build Engineer/Contribution Auditor/InfoSec
                     Sentinel activation phase) to match framework.md's
                     existing, unchanged claims — no new obligation, just
                     accurate phase labeling.
  Downstream:       Adopter-produced artifact shape unchanged (no new
                     required fields); agent behavior toward adopter's users
                     unchanged (no capability added or removed, only
                     documented).
  Migration:        None required. Adopters who already follow framework.md's
                     Phase 1 table (not ADOPTING.md's contradicting table) see
                     no behavior change, only corrected documentation.
  Evidence:         docs/coordination/20260720T225540Z-coder-tester-rgrs.md;
                     framework.md:1399-1413 vs ADOPTING.md:1086-1092
                     (independently re-verified via grep before writing this
                     plan, not taken on Codex's word alone).
```

## Affected files

- `agf/schemas/rule.schema.json` — new optional properties.
- `agf/schemas/agent.schema.json` — new optional properties.
- `agf/agents/build-engineer.yaml`, `contribution-auditor.yaml`,
  `infosec-sentinel.yaml`, `planner-architect.yaml`, `coder-tester.yaml`,
  `governor-reviewer.yaml` — add `classification: core`.
- `agf/rules/always/security-hardening-rule.md`,
  `agf/rules/ui/accessibility-rule.md` — add the illustrative classification
  fields.
- `agf/ADOPTING.md` — reconcile the "signals to grow" table with
  `framework.md`'s Phase 1 always-active list.
- `agf/docs/site/docs/adopt/applicability-catalog.md` — new page.
- `agf/docs/site/docs/start/should-you-adopt.md`,
  `agf/docs/site/docs/adopt/install-and-layouts.md` — link to the new page.
- Validator: wherever `agov validate agents`/`rules` is implemented in
  `tooling/rust/crates/agov-validators` — add the advisory conditional-field
  check.
- `agf/CHANGELOG.md` — record the reconciliation fix and new catalog page.

## Test strategy

1. `agov plan validate docs/plans/adopter-applicability-catalog.md`.
2. Schema changes: existing rule/agent fixture tests must still pass
   unmodified (proves the additions are non-breaking).
3. New validator check: a fixture rule/agent file with `classification:
   conditional` and no `activation_condition` must produce the advisory
   warning; one with both fields present must not.
4. `agov validate agents` and `agov validate rules` (or umbrella `agov
   validate`) on the full current tree — must show zero new warnings for
   the 49 unclassified artifacts (absence is not a warning) and the correct
   advisory warning only if a future edit adds `classification` without the
   companion fields.
5. `mkdocs build --strict` for the new catalog page and its inbound links.
6. Manual diff review of the `ADOPTING.md` reconciliation against
   `framework.md:1399-1413` to confirm no remaining disagreement on these
   three roles' activation phase.

## Risks and controls

| Risk | Control |
|---|---|
| Partial classification (8 of 57) reads as claiming full coverage | Catalog page explicitly states what is and is not yet classified |
| `ADOPTING.md` reconciliation edit drifts from `framework.md` again later | Point the reconciled table's wording at the framework.md line numbers in a code comment, so a future framework.md edit has a visible cross-reference to update |
| New schema fields get treated as required by convention even though optional | Validator explicitly does not warn on absence; only warns on partial presence |
| Follow-up full-backfill plan never happens, leaving catalog permanently partial | Name it explicitly as follow-up in this plan and in the coordination handoff, so it is trackable rather than implied |

## Rollback

Revert the schema field additions, the six agent-file and two rule-file
edits, the `ADOPTING.md` reconciliation, the new catalog page, and the
validator check. No data, no released schema version, no adopter migration
to reverse — all additive.
