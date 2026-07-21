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
      Touches agf/schemas/*.json, agf/agents/*.yaml, agf/rules/**, and
      agf/framework.md, all downstream-impact-rule trigger paths.
      Self-performed in-session per the formal-advisory-role pattern
      (learning 20260703T095527Z-fadv). Revision 2 corrects five
      independently-verified defects Codex's Gate 1 review (`ad63`) found
      in revision 1 — a same-agent self-check caught none of them, which
      is itself evidence for `ad63`'s implicit point that this plan
      needed independent review, not just self-review, before Gate 1.
      Recommend Gate 2 get an independent pass if a second reviewing
      agent is available.
---

# Plan: adopter-facing role/rule applicability catalog

**Date:** 2026-07-20 (revision 2, 2026-07-21T00:05:00Z)
**Status:** Gate 1 proposed — responds to Codex's post-MR-!275 finding
(`docs/coordination/20260720T225540Z-coder-tester-rgrs.md`) and to Codex's
Gate 1 review of revision 1
(`docs/coordination/20260720T231159Z-coder-tester-ad63.md`, five BLOCKs).
Not yet governor-approved.

## Revision note (what changed and why)

Revision 1 was reviewed and found to have five real defects, each verified
directly against the repository before being accepted, not taken on
Codex's word:

1. **The single `classification: core/conditional/optional` field
   conflated two different questions** — whether a role is part of AGF's
   foundational Phase 1 design, and whether it is active in a given
   adopter's specific commissioning profile. Fixed by dropping that field
   and using the schemas' *existing* signals for library membership
   (agents: `tier` + `phase`, already populated; rules: `scope`, already
   populated) instead of inventing a redundant parallel axis. The new
   fields in this revision are activation- and consequence-only.
2. **`framework.md` is itself internally inconsistent**, not a settled
   source of truth to reconcile `ADOPTING.md` toward. Verified directly:
   `framework.md:1399-1406` calls Build Engineer "core (always active)";
   `framework.md:1430` says "projects without CI can defer it" — two
   sentences describing library membership and activation as if they were
   the same property. Revision 2 proposes correcting `framework.md`'s own
   language, not just `ADOPTING.md`'s, and flags the exact semantics as a
   governor decision, not a unilateral resolution.
3. **The plan cited `agov validate rules`, which does not exist.**
   Verified: only `agents`, `adoption`, `adopter-docs`, `learnings`,
   `infosec`, `contributions`, `framework-config`, `readme-stats` are
   real subcommands. Dropped the validator-check claim from this tranche
   entirely; named as an explicit, unimplemented follow-up.
4. **Only 5 of 46 rule files have the front matter `rule.schema.json`
   requires**, and both files this plan names as illustrations
   (`security-hardening-rule.md`, `accessibility-rule.md`) were among the
   41 that don't. Verified directly. Revision 2 adds the missing required
   fields to both files as part of this plan, and states plainly that no
   validator currently enforces this — the two files become
   schema-*consistent* by manual construction, not
   validator-*confirmed*, until the follow-up in point 3 exists.
5. **Two quantitative claims were wrong**, verified directly: it is 6
   remaining unclassified roles (12 total, 6 classified here), not 9; and
   the claim that the three contested roles have "zero mentions" in
   public commissioning docs is false — `quick-reference.md` and
   `install-and-layouts.md`'s commissioning matrix both name
   `contribution-auditor`, `infosec-sentinel`, and `build-engineer` by
   their lowercase slugs (the original grep used title-case names and
   missed them). The actual gap is inconsistent *phase and trigger*
   guidance across documents, not absence from any of them. Both
   corrected below.

Also applied, per Codex's WARN: the catalog page must be labeled as an
incomplete classification pilot with a named, tracked completion
condition for the full backfill, not left as an indefinitely partial page
linked from the install flow without qualification.

## Trigger and problem

MR !275 correctly distinguishes the complete shipped library from the
subset an adopter activates, and correctly labels a reduced starter
package Level 1 Planned. But it does not — and per its own scope
statement, does not attempt to — tell an adopter *which* of the 46 rule
files and 12 role definitions belong in which category, or what is lost
by removing one.

The actual gap, corrected from revision 1:

- `agf/schemas/rule.schema.json` has `scope` (directory) and `applies_to`
  (actor type) but no field for activation trigger, safe-removal
  condition, dependency, or forfeited-guarantee.
- `agf/schemas/agent.schema.json` has `tier` and `phase`, which already
  classify a role's *library membership* — but no field for the
  adopter-specific activation trigger or the consequence of deactivating
  versus deleting the definition.
- `agf/rules/README.md` catalogs only a historical rule subset, not all
  46.
- **`framework.md` conflates library membership with activation status in
  its own prose**, and `ADOPTING.md`'s growth-signals table inherits that
  conflation with an additional phase-number error. `framework.md`
  Phase 1 lists Build Engineer as a "core agent (always active)" and, two
  paragraphs later, says CI-less projects "can defer it" — both true, but
  only if "core" (library membership) and "always active" (activation
  status) are read as two different claims, which the current prose does
  not signal. `ADOPTING.md`'s "signals to grow" table lists Build
  Engineer, Contribution Auditor, and InfoSec Sentinel as Phase 3/4
  growth triggers — a phase-number error given `framework.md` places all
  three in Phase 1 — but the underlying idea that each has a real
  activation trigger (has CI; receives a contribution; submits a learning
  record) is correct and worth keeping, just relabeled to the right
  phase. Grepping the current public commissioning docs
  (`should-you-adopt.md`, `install-and-layouts.md`, `roles-and-gates.md`,
  `quick-reference.md`) for these three roles' lowercase slugs — not
  their title-case names — finds them named in `quick-reference.md` and
  `install-and-layouts.md`'s commissioning matrix. They are not absent;
  the guidance about *when* to activate them is inconsistent across
  documents.

## What this plan does

Three things, deliberately kept separable:

1. **Correct `framework.md`'s own conflation** of library membership and
   activation status for Build Engineer, Contribution Auditor, and
   InfoSec Sentinel — proposed semantics below, explicitly flagged for
   governor confirmation, not decided unilaterally.
2. **Reconcile `ADOPTING.md`'s "signals to grow" table** to the corrected
   `framework.md` language: same three roles, corrected phase number
   (Phase 1, not 3/4), corrected trigger description matching
   `framework.md`'s actual stated triggers.
3. **Add optional, schema-validated activation and consequence metadata**
   to rules and agent definitions, for a first representative slice — not
   a full backfill (see Explicitly out of scope).

## Proposed semantics (governor decision required)

Two separate properties, not one:

- **Library membership** — is this role/rule part of AGF's foundational
  design? Already captured by existing fields: agents via `tier` (all six
  roles in this plan's slice are `phase: 1`, and `tier` is `production`,
  `governance`, or `cross-cutting`); rules via `scope` (`always` signals
  foundational; `workflow`/`docs`/`ui` signal narrower applicability).
  This plan does not add a new field for this axis — it already exists
  and is already populated.
- **Activation status** — is this role/rule's function actually needed
  *right now*, in a specific adopter's project? New field:
  `activation_condition` (string), populated for every classified
  artifact, including ones that are library-core. Examples: Build
  Engineer's is "project has a CI pipeline"; Contribution Auditor's is
  "project receives contributions, upstream or downstream"; InfoSec
  Sentinel's is "project submits learning records";
  Planner-Architect/Coder-Tester/Governor-Reviewer's is "always" (no
  real gate — they're needed from the first governed change).

This resolves the framework.md self-contradiction: Build Engineer is
library-core (`tier: production`, `phase: 1`) *and* has a real activation
condition ("has CI"). Both statements are true simultaneously once they
answer different questions. The governor's decision is whether this
two-property split is the right semantics, and whether the three
proposed `activation_condition` values above are accurate — not whether
framework.md needs *a* fix (it does; verified independently).

## Design

### Schema additions

`agf/schemas/rule.schema.json` — new optional properties (existing
`required: ["id", "scope", "applies_to", "evidence"]` unchanged, so no
existing rule file becomes invalid):

```json
"activation_condition": {
  "type": "string",
  "description": "The concrete signal that this rule's subject is present, e.g. 'project has a UI', 'project handles PII'. For rules whose scope is 'always', this is typically 'always' — present regardless of stack."
},
"safe_to_remove_when": {
  "type": "string",
  "description": "The condition under which deleting this rule file from the checkout does not silently reintroduce the failure mode it guards against."
},
"removal_forfeits": {
  "type": "string",
  "description": "The specific AGF guarantee or conformance claim lost by removing this rule."
},
"depends_on": {
  "type": "array",
  "items": {"type": "string"},
  "description": "Rule IDs this rule assumes are also active."
}
```

`agf/schemas/agent.schema.json` — new optional properties under `agent`
(no new field for library membership — `tier` and `phase` already cover
it):

```json
"activation_condition": {
  "type": "string",
  "description": "The concrete adopter-specific trigger for this role's function being needed, independent of tier/phase library membership. 'always' for roles needed from the first governed change."
},
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

Both additions are **optional** at the schema level. Existing rule/agent
files with no new fields remain valid — this plan does not force a flag
day across 46 rules and 12 roles in one MR (see Explicitly out of scope).

### framework.md correction

Reword the Phase 1 tables (`framework.md:1399-1413`) so "always active"
is not applied to roles that have a real, stated activation condition.
Concretely: keep Build Engineer, Contribution Auditor, and InfoSec
Sentinel in the Phase 1 tables (they are library-core — that part of
revision 1 was correct), but replace "always active" with language that
states both properties: library-core status, plus the specific
activation trigger already present elsewhere in the same document (the
"Activate Build Engineer when..." sentence, and the "mandatory from
first contribution"/"mandatory from first learning record" phrases for
the other two). This does not remove or weaken any existing claim — it
makes explicit the distinction the prose already implies but doesn't
state.

### ADOPTING.md reconciliation

Update the "signals to grow" table (lines ~1086-1092) so Build Engineer,
Contribution Auditor, and InfoSec Sentinel are listed under Phase 1 (not
3/4), with trigger descriptions matching the corrected `framework.md`
language above — not removed from a "when to activate" framing entirely,
since that framing is legitimate for roles with real activation
conditions, just mislabeled by phase number in revision 1's target
document.

### First representative slice

Populate the new fields for exactly the artifacts this plan's own
trigger concerns:

- **Build Engineer** (`agf/agents/build-engineer.yaml`):
  `activation_condition: "project has a CI pipeline"`.
- **Contribution Auditor** (`agf/agents/contribution-auditor.yaml`):
  `activation_condition: "project receives contributions, upstream or downstream"`.
- **InfoSec Sentinel** (`agf/agents/infosec-sentinel.yaml`):
  `activation_condition: "project submits learning records"`.
- **Planner-Architect, Coder-Tester, Governor-Reviewer**:
  `activation_condition: "always"` (needed from the first governed
  change; uncontested, included for completeness of the slice, not
  because their status was in question).
- Two illustrative rules, normalized to full schema compliance as part
  of this plan (both currently have only `provenance:` front matter, no
  `id`/`scope`/`applies_to`/`evidence` — verified directly, a real gap
  this plan closes rather than papers over):
  - `always/security-hardening-rule.md`: add
    `id: security-hardening-rule`, `scope: always`,
    `applies_to: [llm-agent, human-agent, ci]`,
    `evidence: foundry-native`, `strength: MUST`, `status: active`, plus
    `activation_condition: "always"`.
  - `ui/accessibility-rule.md`: add `id: accessibility-rule`,
    `scope: ui`, `applies_to: [llm-agent, human-agent]`,
    `evidence: foundry-native`, `strength: MUST`, `status: active`, plus
    `activation_condition: "project has a UI"`,
    `safe_to_remove_when: "the project has and will have no user-facing interface"`,
    `removal_forfeits: "WCAG AA conformance checking for any UI added later without re-adding this rule"`.

**Note on schema compliance without a validator:** these two rule files
become internally consistent with `rule.schema.json`'s required-field
list by manual construction in this plan. No validator currently checks
this (see Explicitly out of scope) — "schema-consistent" here means
"manually verified against the schema's required-field list," not
"validator-confirmed." The follow-up validator, once built, is what
converts this from a manual claim into an enforced one.

### Catalog page

New page `agf/docs/site/docs/adopt/applicability-catalog.md`, linked from
`should-you-adopt.md`'s "What ships and what you activate" section (added
by MR !275) and from `install-and-layouts.md`'s rule-review step. The
page:

- Lists the classified slice above (8 artifacts).
- **States prominently, above the list, that this is an incomplete
  classification pilot** covering 8 of 58 artifacts (46 rules + 12
  roles), not a complete adopter-facing removal guide.
- Names the tracked completion condition for the full backfill: a
  follow-up plan, sized per this project's branch-sizing discipline (one
  reviewable batch per yes/no question), referenced by a coordination
  handoff filed at the same time this plan is Gate 1 approved — so the
  follow-up is trackable rather than an implied, unscheduled intention.

### Validator behavior — deferred, not claimed

Revision 1 claimed an `agov validate rules` advisory check as part of
this tranche. That command does not exist. This tranche adds schema
fields and documentation only. A follow-up — building a Rust `rules`
validator command, wiring it into umbrella validation, with fixtures and
docs — is named as explicit future work, not implemented or claimed here.

## Explicitly out of scope

- **Full backfill across all 46 rules and the remaining 6 agent
  definitions** (corrected from revision 1's "9" — 12 total roles, 6
  classified in this slice, 6 remain: fact-checker,
  capability-developer, documentation-writer, deployment-engineer,
  academic-peer-reviewer, academic-visual-reviewer). Real editorial
  judgment per rule/role, belongs in a follow-up plan sized so a reviewer
  can answer one yes/no question per batch.
- **A Rust `agov validate rules` command.** Named as follow-up; not
  implemented here (see Validator behavior above).
- **The profile generator** Codex's original finding mentioned as a
  possible consumer of this metadata. Stays Level 1 Planned.
- **Making any new field required** on the rule/agent schemas. Required
  now would force classifying all remaining artifacts in this same
  change to keep existing files schema-valid — exactly the flag-day this
  plan avoids.
- **Any change to which rules/roles are actually shipped, activated by
  default, or removed.** This plan documents and corrects existing
  claims; it does not change Phase 1 default composition.

## Downstream impact and migration

This plan touches `agf/schemas/*.json`, `agf/agents/*.yaml`,
`agf/rules/**/*.md`, and `agf/framework.md` — all `downstream-impact-rule`
trigger paths.

Classification: **ADDITIVE**, with one **correction** component
(`framework.md`'s own Phase 1 language, fixing an internal
inconsistency, not weakening any existing claim). New optional schema
fields (existing `required` arrays on both schemas are unchanged); no
existing rule or agent file becomes invalid.

```
Impact:
  Foundry:          No — this project's own agov-mcp/agov CLI and coordination
                     tooling are unaffected; only descriptive schema fields,
                     framework.md prose, and docs change.
  Direct adopters:  ADDITIVE — new optional fields adopters may ignore
                     entirely; the framework.md/ADOPTING.md correction
                     changes displayed guidance (phase number and trigger
                     wording for three roles) to resolve an internal
                     inconsistency, not to change what's actually shipped
                     or activated by default.
  Downstream:       Adopter-produced artifact shape unchanged (no new
                     required fields); agent behavior toward adopter's users
                     unchanged (no capability added or removed, only
                     documented and, for three roles, correctly phased).
  Migration:        None required.
  Evidence:         docs/coordination/20260720T225540Z-coder-tester-rgrs.md;
                     docs/coordination/20260720T231159Z-coder-tester-ad63.md
                     (revision 1's five defects, each independently
                     re-verified in this revision — see Revision note);
                     framework.md:1399-1413,1430 vs ADOPTING.md:1086-1092.
```

## Affected files

- `agf/schemas/rule.schema.json` — new optional properties (no
  `classification` field; revision 1's conflated axis dropped).
- `agf/schemas/agent.schema.json` — new optional properties (no
  `classification` field; `tier`/`phase` already serve that purpose).
- `agf/agents/build-engineer.yaml`, `contribution-auditor.yaml`,
  `infosec-sentinel.yaml`, `planner-architect.yaml`, `coder-tester.yaml`,
  `governor-reviewer.yaml` — add `activation_condition`.
- `agf/rules/always/security-hardening-rule.md`,
  `agf/rules/ui/accessibility-rule.md` — add full required front matter
  (currently missing) plus the new optional fields.
- `agf/framework.md` — correct Phase 1 table language to separate
  library membership from activation status for three roles.
- `agf/ADOPTING.md` — correct the "signals to grow" table's phase number
  and trigger wording for the same three roles.
- `agf/docs/site/docs/adopt/applicability-catalog.md` — new page,
  explicitly labeled incomplete pilot.
- `agf/docs/site/docs/start/should-you-adopt.md`,
  `agf/docs/site/docs/adopt/install-and-layouts.md` — link to the new
  page.
- `agf/CHANGELOG.md` — record the framework.md/ADOPTING.md correction and
  new catalog page.

## Test strategy

1. `agov plan validate docs/plans/adopter-applicability-catalog.md`.
2. Schema changes: existing rule/agent fixture tests must still pass
   unmodified (proves the additions are non-breaking).
3. `mkdocs build --strict` for the new catalog page and its inbound
   links.
4. Manual diff review of the `framework.md` and `ADOPTING.md` edits
   against each other to confirm no remaining disagreement on these
   three roles' phase or trigger.
5. Manual check that both normalized rule files' front matter validates
   against `rule.schema.json`'s required-field list (no automated
   validator exists yet — see Validator behavior).

## Risks and controls

| Risk | Control |
|---|---|
| Partial classification (8 of 58) reads as claiming full coverage | Catalog page states this prominently, above the list, not in a footnote |
| No validator enforces the new fields or the normalized rule front matter | Stated explicitly in this plan and on the catalog page; follow-up validator named as required future work, not implied as already covering this |
| `framework.md`/`ADOPTING.md` correction drifts apart again later | Cross-reference the corrected `ADOPTING.md` wording to the specific `framework.md` line numbers in a comment |
| Follow-up full-backfill plan never happens, leaving catalog permanently partial | Named explicitly in this plan and in the coordination handoff filed alongside Gate 1 approval, so it is trackable rather than implied |
| Proposed activation_condition values for the three contested roles are wrong | Explicitly flagged above as the governor's decision, not asserted as already correct |

## Rollback

Revert the schema field additions, the six agent-file and two rule-file
edits, the `framework.md` and `ADOPTING.md` corrections, the new catalog
page, and its inbound links. No data, no released schema version, no
adopter migration to reverse — all additive or self-correcting.
