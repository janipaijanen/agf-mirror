---
id: incident-workflow-cast
status: pending-gate1
topic: governance-meta-process
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: incident records feed the governance amendment process this
      auditor role ultimately reviews (rule changes proposed in response
      to an incident) — the auditor should see the incident-record shape
      before it's adopted, same as any other governance-process design
  - role: infosec-sentinel
    status: not-applicable
    reason: incident records live inside the existing learnings/*.yaml
      files, which already go through the infosec-sentinel PII/anonymization
      screen on every commit (agov validate infosec); this plan adds no
      new externally-destined artifact type and no new screening surface
---

# Plan: Incident Workflow + Governance Evolution Loop, CAST-style (Issue #46)

**Date:** 2026-07-20T07:51:21Z
**Status:** PENDING GATE 1 APPROVAL
**Depends on:** `governance-amendment-workflow.md` (Issue #43) — this
plan's incident record ends with a link field pointing at the ADR
artifact that #43's plan defines as the amendment mechanism. If #43's
plan is revised before implementation (e.g. the human reconciler
changes the artifact from an ADR to something else), this plan's
`rule_amendment_ref` field description must be updated to match.

---

## Intent

AGF has a learning-record system for gaps discovered incidentally (see
`agf/docs/governance/learning-system.md`), but nothing that guarantees
a bad outcome — an incident — actually produces a rule change. A
learning can sit as `category: gap` indefinitely without a structural
diagnosis of *why the gate chain let it through*, and without a
required link to the fix.

STAMP/STPA's companion method, CAST (Causal Analysis based on System
Theory), reframes incident analysis away from "who made the mistake"
and toward "what control structure allowed the mistake to happen and
reach production." Applied to AGF's own gate chain (Gate 1, Gate 2,
the advisory-consultation checks, the systematic-code-review checklist,
etc.), an incident becomes a diagnosis of which control component
failed to prevent the hazard, not a blame exercise aimed at whichever
agent or human touched the change last.

This also has a regulatory tie-in worth naming but not building for:
NIS2 and CRA both impose incident-reporting duties on in-scope
organizations. An AGF-native incident workflow, structured with
who/what/when/why-the-gate-missed-it/what-changed fields, could feed
those external reporting duties if AGF is ever operated in an
NIS2/CRA-scoped context — but building that export mapping is out of
scope here (see Out of scope below).

---

## What already exists (read before designing)

- **`agf/schemas/learning.schema.json`** — the per-entry object under
  `learnings[]` has `additionalProperties` **not restricted to false**
  at the entry level (confirmed by reading the schema: the `items`
  object schema lists `properties` but never sets
  `"additionalProperties": false`). This means extra fields on a
  learning entry are already structurally permitted. This plan does
  not rely on that silent permissiveness, though — it adds the new
  `incident` field explicitly to the schema so the shape is documented
  and reviewable, not accidental (per `workflow/concept-modelling-rule.md`'s
  spirit: make the field's shape and meaning an explicit, reviewable
  decision, not something that "just works" because the schema didn't
  forbid it).
- **`tooling/rust/crates/agov-validators/src/learnings.rs`** — the
  Rust validator does not enum-enforce `category` (it's a free-text
  string per schema; the validator only checks `id` format, PII
  patterns, and `confidence` structure). Adding an `incident` object
  requires no changes to this file for the *plan* to be valid, since
  the field would be additive and unenforced in this first pass —
  though the implementation session should decide whether to add a
  light structural check (e.g. "if `incident.rule_amendment_ref` is
  set, it should look like a file path") as a stretch goal.
- **`learnings/_template.yaml`** — the source template every adopter
  copies. This is where the CAST shape should be demonstrated with a
  commented-out example, so new authors see it without needing to read
  a separate doc first.
- **The existing `learning-system.md` categories** (`gap`, `overkill`,
  `validation`, `adaptation`, `tension`) already cover "the framework
  missed something" (`gap`) and "two principles conflicted" (`tension`).
  Incidents are not a new taxonomy branch; they are `gap` or `tension`
  entries that additionally underwent a structured CAST analysis. This
  plan does **not** add an `incident` value to the category taxonomy —
  it adds an `incident:` object that any `gap`- or `tension`-categorized
  entry may carry once analyzed. This directly satisfies the issue's
  instruction to extend the existing schema "not duplicate it wholesale."
- **`agf/docs/governance/governance.md`'s Gate 2 checklist** and
  `.claude/rules/workflow/systematic-code-review-rule.md` are examples
  of exactly the kind of "gate" an incident record's
  `gate_expected_to_catch` field should be able to name concretely
  (e.g. `"Gate 2 — systematic-code-review-rule.md section 4 (Tests and
  regression guard)"`), not a vague "code review should have caught
  this."
- **This project's own incident history is itself CAST-shaped
  evidence.** `always/gate-self-test-rule.md` documents three
  gate-shipped-broken incidents (v0.25.0, v0.38.0, v0.41.0) with almost
  exactly the control-structure/unsafe-control-action shape this plan
  formalizes (a gate existed, was never invoked at the moment that
  mattered, or was silently non-executable) — but they are recorded as
  prose in a rule's "why this exists" section, not as structured,
  queryable incident records. This plan's shape would let that same
  content be captured as data, not just prose.

---

## Scope (first pass)

### 1. Incident-record shape — extension of the learning-record schema

Add one new optional object property to the `learnings[]` item schema
in `agf/schemas/learning.schema.json`:

```json
"incident": {
  "type": "object",
  "description": "CAST-style structured analysis for gap/tension learnings that trace back to a governance-gate failure. Optional; present only on entries that underwent this analysis.",
  "properties": {
    "status": {
      "type": "string",
      "enum": ["open", "analyzed", "remediated"],
      "description": "open: incident recorded, analysis not yet complete. analyzed: control structure and UCAs documented. remediated: rule_amendment_ref points at an adopted fix."
    },
    "control_structure": {
      "type": "array",
      "description": "The gate(s)/component(s) that were supposed to prevent this hazard, and what each was responsible for guaranteeing.",
      "items": {
        "type": "object",
        "required": ["component", "responsibility"],
        "properties": {
          "component": {"type": "string"},
          "responsibility": {"type": "string"}
        }
      }
    },
    "unsafe_control_actions": {
      "type": "array",
      "description": "What the control structure did (or failed to do), under what condition, and what hazard resulted. Not a single root-cause field — CAST expects multiple contributing control actions.",
      "items": {
        "type": "object",
        "required": ["action", "context", "hazard"],
        "properties": {
          "action": {"type": "string"},
          "context": {"type": "string"},
          "hazard": {"type": "string"}
        }
      }
    },
    "contributing_factors": {
      "type": "array",
      "items": {"type": "string"},
      "description": "Systemic conditions that made the unsafe control actions possible (process gaps, missing tooling, ambiguous ownership) — not individual blame."
    },
    "gate_expected_to_catch": {
      "type": "string",
      "description": "Name the specific gate, rule, or checklist item by path/section that should have caught this (e.g. 'Gate 2 — systematic-code-review-rule.md section 4')."
    },
    "why_gate_missed_it": {"type": "string"},
    "rule_amendment_ref": {
      "type": "string",
      "description": "Path to the ADR (per the governance-amendment-workflow plan, Issue #43) that proposes or adopted the resulting rule/gate change. This is the explicit handoff point into the amendment workflow."
    }
  }
}
```

**Design choices explained:**

- **Array, not single field, for `unsafe_control_actions` and
  `contributing_factors`.** This is the load-bearing CAST-vs-root-cause
  distinction the issue calls for: "control structure ... unsafe
  control actions ... contributing factors, rather than a single
  root-cause field." A schema with one `root_cause: string` field would
  structurally invite the anti-pattern `debugging-rule.md` already
  warns against ("do not propose a fix until you can state the root
  cause in one sentence" is fine for a *code* bug; a *governance*
  incident routinely has more than one contributing control failure,
  and forcing it into one field would silently discard the others).
- **`status` enum mirrors the workflow, not a generic state machine.**
  `open → analyzed → remediated` is the minimum useful lifecycle; it
  does not need `closed`/`wontfix`/etc. states because a governance
  incident that is decided *not* to warrant a rule change is still
  `analyzed` with an empty or explanatory `rule_amendment_ref` — the
  decision not to act is itself a recorded outcome (matching
  `learning-system.md`'s existing "Do NOT change the framework when"
  section, which already anticipates analyzed-but-no-action outcomes).
- **`rule_amendment_ref` is a plain string path, not a structured
  object**, matching the same "don't over-engineer for a single
  consumer" principle as the rest of this schema (`simplicity-rule`).
  If the amendment-workflow plan (#43) ships a different artifact type
  than an ADR path, this field's description should be updated to
  match — flagged in the "Depends on" header above.
- **No new top-level artifact or directory.** Incidents are learning
  entries, in the same `learnings/<codename>.yaml` files, going through
  the same InfoSec Sentinel PII/anonymization screen on every commit.
  This keeps the removability constraint intact (issue #39) — an
  incident record is a plain YAML entry in a file already under version
  control, not a row in a database or an external tracker.

### 2. CAST analysis template + worked example — new companion doc

Add `agf/docs/governance/incident-analysis.md`:

- Explains the CAST framing in the project's own words (control
  structure, unsafe control actions, contributing factors — as opposed
  to a single root cause), with a citation-appropriate hedge per
  `verifiable-claims-rule` (CAST is Leveson's published method; cite it
  as such, don't claim more rigor than a lightweight adaptation
  provides).
- Includes one **worked example** using this project's own
  already-documented incident: the v0.41.0 gate-shipped-broken case
  from `always/gate-self-test-rule.md` (pre-push hook shipped
  non-executable), reframed into the `incident:` schema shape. This
  gives implementers a concrete, already-true example rather than a
  hypothetical, and cross-links back to `gate-self-test-rule.md` rather
  than restating its "why this exists" prose (per the marketing-tone
  rule's message-integrity principle: one canonical statement, others
  link).
- One-line pointer to `learnings/_template.yaml`'s commented example
  for the exact field syntax.

### 3. Template update

Add a commented-out `incident:` example block to
`learnings/_template.yaml`, immediately below the existing learning
entry template fields, so new authors see the shape without needing to
open a separate document first.

### 4. Linkage into the amendment workflow (the explicit #43 handoff)

`rule_amendment_ref` is the field this issue's scope explicitly calls
out as "exactly where it hands off into #43's amendment workflow."
Concretely: once `governance-amendment-workflow.md` ships, an incident
record's `status: remediated` transition requires `rule_amendment_ref`
to be populated with a path under `agf/docs/adr/`, and that ADR's
`Decision` section should, per that plan's Step 1, cite the incident
learning ID back (bidirectional link: incident → ADR via
`rule_amendment_ref`; ADR → incident via its evidence citation).
Nothing in this plan requires modifying `agf/schemas/rule.schema.json`
directly — the bidirectional link is achieved entirely through the two
plans' existing `evidence` and `rule_amendment_ref` string fields.

---

## Out of scope (explicitly, per Issue #46)

- Building the amendment workflow mechanism itself — that is #43.
  This plan defines the incident→analysis step and the field that
  points at #43's output; it does not define how the resulting rule
  change is proposed or adopted.
- Any NIS2/CRA external-reporting export tooling. The fields are
  structured so such an export *could* be built later if AGF is
  operated in an NIS2/CRA-scoped context, but no exporter, mapping
  table, or compliance claim is produced by this plan
  (`implementation-honesty-rule`: naming a future possible use is not
  a maturity claim about a capability that exists today).
- Retroactively converting existing `gap`/`tension` learning entries
  into the new `incident:` shape. The v0.41.0 example in the new
  companion doc is illustrative prose, not a retrofit of the actual
  `learnings/foundry.yaml` entry (if one exists) — deciding whether to
  retrofit historical entries is a follow-on task.
- Any change to `tooling/rust/crates/agov-validators/src/learnings.rs`
  beyond what's needed to keep existing tests green. Adding real
  structural validation of the `incident` object (e.g. requiring
  `rule_amendment_ref` when `status: remediated`) is a stretch goal for
  the implementation session, not committed to by this plan.

---

## Affected files (for the later implementation session — not touched in this planning session)

| File | Action |
|---|---|
| `agf/schemas/learning.schema.json` | Add optional `incident` object property to the `learnings[]` item schema (ADDITIVE) |
| `agf/docs/governance/incident-analysis.md` | **New** — CAST framing + worked v0.41.0 example + field-syntax pointer |
| `agf/docs/governance/learning-system.md` | Add a short cross-reference to `incident-analysis.md` (one line, per message-integrity: link, don't restate) |
| `learnings/_template.yaml` | Add commented-out `incident:` example block |
| `tooling/rust/crates/agov-validators/src/learnings.rs` | No change required for this pass (optional field, schema-level only); note as a stretch goal in code comments if the implementer chooses to add structural checks |

No migration needed — schema addition is optional; no existing
`learnings/*.yaml` file needs to change to remain valid.

---

## Test strategy

| Tier | What | Where |
|---|---|---|
| Schema conformance | `agf/schemas/learning.schema.json` with `incident` added still validates all existing `learnings/*.yaml` files unchanged | Manual: run `cargo run -q -p agov -- validate` (or the specific learnings validator path) against the current `learnings/` tree before and after the schema edit, confirm identical pass/fail set |
| Validator regression | `cargo test -p agov-validators` (specifically `learnings.rs`'s existing test module) stays green — no existing test asserts on the *absence* of an `incident` field, so this should be a no-op regression check | `tooling/rust/` |
| New fixture (implementation session, TDD) | Add one fixture learning file with a fully-populated `incident:` block (all sub-fields) and confirm it validates successfully; this is the "positive case using the real shape" the `boundary-shape-testing-rule` would ask for if this were parsing *external* data — here it's the project's own schema, but the same discipline (test against the real intended shape, not a simplified stand-in) applies | `tooling/rust/crates/agov-validators/src/learnings.rs` test module, per `workflow/tdd-rule.md` — write this test FIRST in the implementation session, before touching the schema file |
| Manual | Worked example in `incident-analysis.md` cross-checked against `always/gate-self-test-rule.md`'s actual v0.41.0 narrative for factual accuracy (per `verifiable-claims-rule`) | Manual read-through at Gate 2 |

No integration or e2e tier applies.

---

## Risks

| Risk | Mitigation |
|---|---|
| `incident:` schema ships but nobody uses it (structurally invisible until an actual incident happens, same failure mode `audit-log-completeness-rule.md` documents for CUD-without-audit) | The companion doc's worked example uses a *real*, already-documented incident (v0.41.0) so the shape is proven useful on day one, not theoretical |
| CAST is a heavyweight academic method; lightweight adaptation risks either being too shallow to be useful or too heavy to fill in during a real incident | Scope is deliberately minimal: three arrays + two strings + one enum, not the full STAMP/STPA control-structure diagram notation; the worked example demonstrates the minimal-but-real bar |
| Dependency on #43's artifact shape (ADR path) for `rule_amendment_ref` — if #43 is revised to use a different mechanism, this field's description is stale | Flagged explicitly in the "Depends on" header; the field is a plain string, so no schema breakage results from #43 changing its artifact type, only a documentation update here |
| Blurring the line between `gap`/`tension` category and `incident:` presence could tempt future authors to treat `incident:` as its own category, drifting from the "extension, not new taxonomy branch" design | `incident-analysis.md` states explicitly that `incident:` is a sub-analysis of an existing `gap`/`tension` entry, not a fourth/sixth category value |

---

## Rollback

Optional schema field addition plus new/updated documentation and a
template comment block — no code logic, no data migration, no
validator behavior change in this pass. Rollback is a straightforward
revert; no existing learning record depends on the new field.
