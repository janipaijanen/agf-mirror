---
id: governance-amendment-workflow
status: pending-gate1
topic: governance-meta-process
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: this plan defines who may change AGF's own rules and how that
      change is recorded — directly the auditor's mandate (adversarial
      steering resistance, false-precedent resistance) applied to the
      framework's own amendment surface
  - role: infosec-sentinel
    status: not-applicable
    reason: no secrets, auth, or tenant-isolation surface touched; the
      only new field (review_date) is a plain date string on a public
      governance artifact
---

# Plan: Governance Amendment Workflow + Sunset Clauses (Issue #43)

**Date:** 2026-07-20T07:51:21Z
**Status:** PENDING GATE 1 APPROVAL
**Depends on:** none. Hands off into `incident-workflow-cast.md` (Issue #46)
— that plan's incident→rule-change linkage points at the artifact this
plan defines.

---

## Intent

AGF governs changes to an adopter's codebase but currently has no
equivalent auditable workflow for changes to **itself** — rule additions,
rule retirement, gate redefinition. The rule-maturity ladder described
informally in several `.claude/rules/*.md` "Provisional status" headers
(single-adopter evidence → corroborated → promoted) is real and already
practiced, but it is practiced as prose convention, not as a mechanism
with an artifact, a reviewer, and a record.

Two gaps, one issue because they are the same governance-of-governance
surface:

1. **Amendment workflow.** Who can propose a rule change, what review it
   needs, how the change is recorded. Ostrom's collective-choice
   arrangements: those affected by a rule need a documented way to
   propose changing it — not just the maintainer's private judgment call.
2. **Sunset clauses.** Any enforced gate metric eventually gets gamed
   (Goodhart's law). The structural defence is minimalism plus mandatory
   review/expiry dates, not more rules layered on top. This is also the
   direct counter to checklist fatigue (Gawande / WHO surgical-checklist
   literature: checklists reduce error but decay into box-ticking over
   time without periodic pruning).

This plan formalizes both using artifacts that already exist in this
repo — it does not invent a new tracker, database, or bespoke schema
where an existing mechanism already fits (removability constraint,
issue #39).

---

## What already exists (read before designing)

- **`agf/schemas/rule.schema.json`** — front-matter schema for
  `agf/rules/**/*.md`. Already has two orthogonal maturity-relevant
  fields: `status` (`active | provisional | deprecated`) and `strength`
  (`MUST | SHOULD | MAY`), plus `evidence` (a learning ID, codename, or
  citation) and `see_also`. **This schema is not currently enforced** —
  no `agov validate rule` subcommand exists (confirmed:
  `cargo run -q -p agov -- --help` lists no `rule` command), and the
  existing `agf/rules/**/*.md` files only populate `provenance:`, none
  of the schema's `required` fields (`id`, `scope`, `applies_to`,
  `evidence`). Per the implementation-honesty-rule's five-level model,
  this schema is **Level 2 (Schema-defined)** today, not Level 4/5.
  This plan does not change that maturity level — retrofitting every
  rule file is explicitly out of scope (see Issue #43's text and the
  "Out of scope" section below). It adds one new optional field and
  documents the workflow; enforcement and retrofit are follow-on work.

- **`agf/docs/adr/`** — Architecture Decision Records, already the
  project's mechanism for "propose → review → adopt → immutable record,
  with a living index for supersession." Template at
  `agf/docs/adr/_template.md`; lifecycle documented in
  `agf/docs/adr/README.md` (Proposed → Accepted → Superseded, files
  never modified after acceptance, index tracks current state).

- **`agf/schemas/coordination-message.schema.json`** — the coordination
  channel already has a `decision-record` message `kind` in its enum.
  Nothing currently defines when to use it for rule changes specifically.

- **`agf/docs/governance/governance.md`** (this project's GOVERNANCE.md
  equivalent) — already defines Gate 1 (plan/proposal approval) and
  Gate 2 (merge approval), both requiring Governor-Reviewer sign-off,
  and already lists `agf/docs/adr/*.md` as a Planner-Architect-owned,
  all-consumed artifact. It does not yet say ADRs go through Gate 1.

- **`agf/docs/governance/learning-system.md`** — already defines a
  periodic "Framework's Sentinel" review cadence ("after every 3rd
  learning record, or monthly, whichever comes first") with a
  "Standing items" checklist run at every review. This is an existing
  calendar mechanism this plan can attach a new standing item to,
  rather than inventing a second cadence.

- **`.claude/rules/*.md` "Provisional status" blocks** (e.g.
  `always/downstream-impact-rule.md`, `workflow/destructive-op-preflight-rule.md`,
  `workflow/deputy-governor-rule.md`) — these already state, in prose,
  a promotion path: single-adopter evidence → corroborated by a second
  adopter → promoted to MUST. This plan formalizes what these headers
  already do ad hoc; it does not replace their content, and existing
  headers do not need to change as part of this plan.

- **`agf/schemas/learning.schema.json`** — `confidence.level` already
  has a *validator-enforced* corroboration bar: `multi-adopter` and
  `longitudinal` levels require `evidence_count >= 2` or two
  `evidence_refs` (enforced in
  `tooling/rust/crates/agov-validators/src/learnings.rs::confidence_entry_errors`).
  This is a ready-made, already-machine-checked corroboration gate this
  plan reuses for promotion decisions rather than defining a new one.

---

## An explicit conflict, flagged rather than silently resolved

Issue #43's scope text says the amendment workflow should formalize
"how a rule moves between" **provisional / recommended / MUST** states,
citing the existing markers in `.claude/rules/`. But the actual
machine-checked schema (`agf/schemas/rule.schema.json`) does not have a
three-value ladder — it has two independent two/three-value fields
(`status`: active/provisional/deprecated; `strength`: MUST/SHOULD/MAY).
There is no `status: recommended`.

Per `always/uncertainty-and-scope-rule.md` ("if a rule conflicts with
what the code already does, point out the conflict explicitly rather
than silently resolving it in either direction"), this plan does not
silently invent a third `status` enum value. Instead it proposes
**mapping the informal three-step ladder onto combinations of the two
existing fields**, since that reuses machine-checked structure instead
of adding a third overlapping axis (simplicity-rule: prefer an existing
utility over a new one):

| Informal ladder step | `status` | `strength` | Meaning |
|---|---|---|---|
| Provisional | `provisional` | `SHOULD` (or `MUST` if the adopter judges the narrow case warrants it — adopter's call, per the rule's own text) | Single-adopter evidence; adopters may skip with documented reason |
| Recommended | `active` | `SHOULD` | Corroborated (see below); adopters should follow, deviation needs justification |
| MUST | `active` | `MUST` | Corroborated at the framework's MUST bar; no silent deviation |

This mapping is a **Gate 1 decision for this plan**, not a foregone
conclusion — the reconciling human should confirm it (or replace it
with a genuine third `status` value) before implementation. Flagging
it here satisfies the escalation obligation; implementing it is out of
scope for this planning-only session.

---

## Scope (first pass)

### 1. Amendment workflow (documented process, no new schema for the process itself)

A rule state change (new rule, promotion, demotion, retirement) is
proposed and recorded using the **existing ADR mechanism**
(`agf/docs/adr/`), not a new artifact type:

1. **Propose.** Anyone playing a GOVERNANCE.md role (human or agent) who
   is affected by a rule may draft an ADR from `agf/docs/adr/_template.md`
   describing the proposed change. The ADR's `Decision` section MUST
   state, explicitly:
   - Which rule file(s) are affected (path).
   - Current `status` / `strength` (or "new rule" if none yet exists).
   - Proposed `status` / `strength`.
   - Evidence cited (a learning ID from `agf/learnings/foundry.yaml`,
     or an adopter's `learnings/<codename>.yaml` entry, or a direct
     citation per `verifiable-claims-rule`).
   - The new `review_date` to set (see part 2).
   No change to the ADR template's section structure is proposed —
   this is a content convention layered on the existing `Decision`
   section, not a schema change to ADRs (ADRs have no JSON schema
   today; they are pure Markdown convention, and this plan does not
   introduce one).

2. **Promotion bar.** An ADR proposing `provisional → recommended`
   (i.e. `status: provisional → active`, `strength` unchanged) or
   `recommended → MUST` (`strength: SHOULD → MUST`) MUST cite at least
   one learning record whose `confidence.level` is `multi-adopter` or
   `longitudinal` — reusing the schema-enforced corroboration bar
   described above. An ADR proposing a **demotion** (retirement,
   `MUST → SHOULD`, or `active → deprecated`) may cite a single learning
   record (e.g. a `tension` or `overkill` category entry showing the
   rule was gamed, produced box-ticking, or no longer fits) — corroboration
   is not required to *retire* a rule, only to *escalate* its strength,
   mirroring the asymmetry already present in `learning-system.md`'s
   severity table (critical acts on single-project evidence; significant
   waits for a second project).

3. **Review.** The ADR goes through Gate 1 exactly as any other plan or
   proposal in this project (per `agf/docs/governance/governance.md`) —
   Governor-Reviewer approval. This plan does not invent a new gate; it
   states explicitly (as an addition to `governance.md`'s Gate 1
   checklist) that ADRs touching `agf/rules/**` are Gate-1-reviewed
   artifacts, closing a gap where `governance.md` lists ADRs as an
   owned artifact but never says which gate reviews them.

4. **Record.** On approval, the approving reviewer (or the proposing
   agent, on the reviewer's behalf) posts a `kind: decision-record`
   coordination message (already a valid enum value in
   `coordination-message.schema.json` — no schema change needed) with
   `references-to` pointing at the ADR file path, recording who
   approved and when. The ADR file itself is the durable rationale;
   the coordination message is the append-only audit-trail pointer,
   consistent with how every other Gate 1/Gate 2 decision in this
   project is already recorded.

5. **Apply.** The implementing agent updates the rule file's front
   matter (`status`, `strength`, `evidence` — pointing at the ADR path
   — and `review_date`) and commits. Because `agf/rules/**/*.md` is
   already a trigger path in `always/downstream-impact-rule.md`, the
   commit already requires an Impact statement; this plan adds no
   second impact-assessment mechanism on top of that one.

6. **Retire, don't delete.** A retired rule gets `status: deprecated`.
   The file is **not deleted** — consistent with the removability
   constraint (issue #39) and the ADR lifecycle convention ("files are
   never modified/removed after acceptance, only superseded"). Any
   rule that supersedes it should list the deprecated rule's id in
   `see_also`.

### 2. Sunset / review_date field

Add one new optional property to `agf/schemas/rule.schema.json`:

```json
"review_date": {
  "type": "string",
  "format": "date",
  "description": "Date by which this rule must be re-reviewed (renewed or retired). Optional until the retrofit follow-on task (out of scope here) makes it required."
}
```

Semantics (documented in `agf/rules/README.md` and this plan, not
enforced by a new validator in this pass):

- `review_date` in the past means the rule is **due for review**, not
  automatically invalid or inactive. A rule past its `review_date`
  keeps its current `status`/`strength` until a Sentinel review acts
  on it (this avoids a rule silently going inert mid-adoption-cycle,
  which would be worse than a stale rule — see `always/high-risk-operations-rule.md`'s
  own "STOP before, don't silently degrade" pattern).
- Renewing a rule (confirming it's still warranted) requires a fresh
  ADR citing why it's still needed and setting a new `review_date` —
  same mechanism as any other amendment, not a lighter-weight rubber
  stamp, because "renew without reasoning" is exactly the checklist-fatigue
  failure mode this field exists to prevent.

### 3. Periodic audit — reuse the existing Sentinel cadence

Add one new standing item to `agf/docs/governance/learning-system.md`'s
existing "Standing items (check at every sentinel review)" list:

> - **Rule review_date audit** — list every `agf/rules/**/*.md` whose
>   `review_date` has passed. For each: either renew (new ADR, fresh
>   `review_date`) or retire (`status: deprecated`, ADR recording why).

No new cadence is created; this uses the "after every 3rd learning
record, or monthly, whichever comes first" cadence that already exists.

---

## Assumption about the accountability field (cross-cluster — verify against #35/#45/#34/#47's actual output)

The identity/accountability cluster is defining a "named human
accountability" field for gate/rule ownership. This plan's amendment
workflow needs to record *who proposed* and *who approved* a rule
change. **Assumption made here:** that cluster will land a
`proposed_by` / `approved_by` (or equivalently-named) field, likely
attached to gate/rule artifacts' front matter or to coordination
messages. Until that field is defined and lands:

- This plan's interim accountability record is the ADR's prose (which
  already names who drafted it, per the template's implicit authorship)
  plus the coordination channel's existing **required** `from` / `to`
  role-identifier fields on the `decision-record` message (Step 4
  above) — both already exist and already carry a role identity, just
  not a dedicated named-human field.
- **When #45's field lands**, the amendment-workflow ADR convention
  (Step 1 above) should add it as an explicit ADR field, and the
  `decision-record` coordination message should populate it. The human
  reconciler should check that the field name and shape proposed here
  (`proposed_by` / `approved_by`, plain strings, attached to the ADR
  and to the `decision-record` message) match what #45 actually
  produces — if #45 defines a structured object (e.g. `{name, role,
  date}`) instead of a plain string, this plan's Step 1/4 wording
  should be updated to match rather than diverging.

---

## Out of scope (explicitly, per Issue #43)

- Retroactively adding `review_date` to every existing rule in
  `agf/rules/**` / `.claude/rules/**`. Follow-on task once this
  mechanism ships and is exercised at least once.
- Building an `agov validate rule` command that enforces
  `rule.schema.json`'s `required` fields. `rule.schema.json` stays
  Level 2 (schema-defined) after this plan; making it Level 4/5 is a
  separate, larger task (would also require retrofitting every
  existing rule file to have `id`/`scope`/`applies_to`/`evidence`,
  which is itself the retrofit this issue explicitly defers).
- Resolving the `status`/`strength` vs. informal three-step-ladder
  conflict noted above beyond proposing the mapping table — final
  call belongs to the human reconciler at Gate 1.
- Modifying the `agf/docs/adr/_template.md` section structure itself
  (this plan layers a content convention on the existing template's
  `Decision` section rather than adding new template sections).

---

## Affected files (for the later implementation session — not touched in this planning session)

| File | Action |
|---|---|
| `agf/schemas/rule.schema.json` | Add optional `review_date` property (ADDITIVE) |
| `agf/rules/README.md` | Document `review_date` semantics + link to amendment workflow doc |
| `agf/docs/governance/amendment-workflow.md` | **New** — the amendment workflow doc: propose/promote/demote/retire steps, promotion-bar rule, ADR content convention, cross-reference to the `status`/`strength` mapping table above |
| `agf/docs/governance/governance.md` | Add: ADRs touching `agf/rules/**` are Gate-1-reviewed; reference the new amendment-workflow doc |
| `agf/docs/governance/learning-system.md` | Add the "Rule review_date audit" standing item to the existing Sentinel cadence checklist |
| `agf/docs/adr/README.md` | One-line pointer to the amendment-workflow doc for rule-change ADRs specifically (no structural change) |

No migration needed (schema addition is optional; no existing rule
file's validity is affected). No code/tooling changes are required for
this first pass — the workflow is process-and-artifact, matching the
issue's explicit deferral of enforcement tooling.

---

## Test strategy

This is a documentation/schema-addition plan; the "tests" are
schema-conformance and validator-regression checks, not new business
logic:

| Tier | What | Where |
|---|---|---|
| Schema conformance | `agf/schemas/rule.schema.json` with `review_date` added still validates existing rule front matter unchanged (field is optional; no existing file needs to change) | Manual: run existing rule files (the ones that already set `provenance:`) through the schema by hand, or via a throwaway script, confirming no new failures |
| Validator regression | `cargo test -p agov-validators` still passes (this plan touches no Rust source; the schema file is not yet wired into any validator, so no regression surface exists, but the full validator suite is the correct check to run once the schema file changes) | `tooling/rust/` |
| Documentation consistency | New `amendment-workflow.md` doc doesn't restate claims already made elsewhere with different wording (per `docs/marketing-tone-rule.md`'s message-integrity rule) — it links to `governance.md`, `learning-system.md`, and `agf/docs/adr/README.md` rather than re-describing their content | Manual review at Gate 2 |

No integration or e2e tier applies — nothing here touches a route,
service, or database.

---

## Risks

| Risk | Mitigation |
|---|---|
| The `status`/`strength` mapping table (provisional/recommended/MUST) is the wrong reconciliation of the issue text vs. the schema | Flagged explicitly above as a Gate 1 decision point, not silently resolved; human reconciler confirms or overrides before implementation |
| `review_date` becomes another checklist item nobody looks at (the exact Goodhart/checklist-fatigue failure this issue exists to prevent) | Tied to the *existing* Sentinel cadence (not a new one) and requires a reasoned ADR to renew, not a rubber-stamp date bump |
| Accountability field assumption (`proposed_by`/`approved_by`) diverges from what #35/#45/#34/#47 actually ship | Explicitly flagged above for the human reconciler to check; this plan's Step 1/4 wording is provisional pending that cluster's output |
| Promotion-bar reuse of `confidence.level` corroboration creates a hard dependency on learning-record discipline that may not exist for framework-native (foundry) rules that have no adopter learning records yet | Demotion path requires no corroboration (asymmetric by design); promotion of a foundry-native rule with no adopter evidence yet simply stays blocked at `provisional`/`SHOULD` until evidence exists — this is the intended conservative default, not a gap |

---

## Rollback

This plan adds one optional schema field and several new/updated
markdown documents — no code changes, no data migration. Rollback is
a straightforward revert of the implementing commit(s); no existing
rule file's validity or behavior depends on the new field or the new
documents.
