---
id: 20260910T115843Z-documentation-estate-steward-role
status: approved
gate1_approved_by: 20260910T122454Z-planner-architect-tsbh
gate1_approved_at: 2026-09-10T12:24:54Z
topic: instantiate-documentation-estate-steward-standing-role
candidate_release: v2.11.0
advisory_consultation:
  - role: infosec-sentinel
    status: consulted
    reason: >-
      This plan adds a standing agent definition and a MANIFEST entry. It
      commits no code, schema, credential, CI, or artifact change. The
      role's `reads` set includes every adopter-facing surface but grants
      no write authority beyond a mechanical currency correction its own
      finding disposition authorizes, and no gate/merge/tag authority.
      InfoSec should confirm at this Gate 1 that the definition creates no
      path to edit rules, schemas, or credentials, and that "mechanical
      currency correction" cannot be stretched into normative change.
  - role: contribution-auditor
    status: consulted
    reason: >-
      A new standing role of stated equal standing to coder-tester /
      planner-architect. Contribution-auditor must confirm: the mandate is
      bounded to content currency + cross-touchpoint consistency and does
      not overlap-without-seam with documentation-writer, web-ux-reviewer,
      marketing-tone-rule, or fact-checker; the disposition authority
      (steward dispositions in-scope findings, escalates
      normative/cross-role/structural) is stated cleanly and cannot
      self-authorize a governed change; and the evidence (learning qvzj +
      the recurring incidents it cites) supports a standing role for
      foundry given the governor's direct decision, independent of qvzj's
      own rule-first calibration for adopters.
  - role: capability-developer
    status: consulted
    reason: >-
      Per `jh3g`, the shape question (standing agent vs rule) is closed by
      governor decision; capability-developer's assessment (`et3p`) is
      narrowed to reviewing this agent contract — mandate, phase/maturity
      at introduction, the touchpoint manifest reconciled with
      docs/reviews/rule-projection/, the cadence classes, communication
      channels and gate interactions, evidence triggers, and the four
      role boundaries. This plan is authored solo (planner-architect)
      because capability-developer (Codex) has been unavailable; its
      review is an input to Gate 1, not a precondition, per governor
      direction 2026-09-10.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Instantiate the documentation-estate-steward standing role (Gate 1)

## Provenance and authority

- **Governor decision 2026-09-10**: *"PMM as precondition, equal to
  coder-tester, planner-architect etc."* — bridged
  `docs/coordination/20260910T050007Z-planner-architect-jh3g.md`. A direct
  foundry governor call (same authority basis as the Improvement Propeller
  activation), settling both open points from `asjp`: it **is** a v2.11 IN
  workstream and hard release-cut precondition, and it **is** a standing
  MANIFEST agent role, not a rule/checklist.
- **Learning `20260910T041223Z-qvzj`** (`agf/learnings/foundry.yaml`,
  `scope: framework`, `status: proposed`) — the evidence: no dedicated
  complete-estate responsibility for adopter-facing documentation
  consistency/currency, and no trigger that runs an estate-wide sweep;
  partial-coverage mechanisms (per-change rule-projection screening,
  per-release allowlist review) do not close it. Recurred at v2.11 Group B
  (`qbhj` V211-RP-R1), v2.9.0 allowlist (V29P-1..R3),
  governance-file-layout-rule (sage-review made root dirs twice from stale
  ADOPTING paths), and the v2.4.11 catch-up release. `qvzj`'s own
  rule-first calibration still stands as guidance for adopters generally;
  the governor made foundry's call directly.
- **`et3p`** routes the contract assessment to capability-developer;
  **`jh3g`** narrows it (shape is closed).

## What this plan does

Adds one standing agent role. It commits:

1. `agf/agents/documentation-estate-steward.yaml` — the role definition
   (already on this branch; `agov validate agents` OK).
2. `agf/agents/MANIFEST.yaml` — the role entry (`status: active` as the
   proposed end state, in effect only when the governor Gate 1 decision
   merges this change).

It does **not** commit: any rule, schema, tooling, CI, or adopter-facing
documentation change; the v2.11 IN-row membership or the estate-assessment
release-cut precondition (that is a **separate parent-scope amendment**
with its own governor Gate 1 — see "Related" below); or the first estate
sweep itself (that is the role's first act once active, and the v2.11
precondition).

## The contract (for capability-developer / contribution-auditor review)

| Element | Decision in the YAML |
|---|---|
| **Mandate** | Consistency and currency of the adopter-facing documentation estate *as a whole*. Estate-wide sweeps, not per-page/per-change passes. |
| **Owns** | Estate assessment reports; the touchpoint manifest (reconciled with `docs/reviews/rule-projection/`); cross-touchpoint consistency findings; currency findings; disposition of in-scope findings. |
| **Phase / maturity** | Phase 1. Level 3 Instantiated at introduction (defined; first sweep pending) — moves to Level 4 when a completed sweep + its findings disposition are evidenced. |
| **Cadence classes** | `release-content-freeze` (full estate, **blocking**), `governed-normative-change` (scoped, advisory), `periodic` (full estate, non-blocking, cadence proposed as one MINOR release or 60 days whichever is longer), `governor-request`. |
| **Disposition authority** | Steward dispositions in-scope findings directly: fixed (mechanical currency correction) / accepted-as-current / deferred-with-reason. Escalates to governor: normative-change-needed, cross-role boundary, structural (the manifest/model itself), non-mechanical currency fix, and freeze-vs-timeline conflict. Per governor answer 2026-09-10. |
| **Gate interactions** | Not a Gate 1 or Gate 2 approver. Its completed `release-content-freeze` sweep is a **precondition input** to the release owner, not an edit or approval. Its `governed-normative-change` findings are advisory evidence raised through normal Gate 2 review. |
| **Boundaries** | vs `documentation-writer` (authoring — skipped), `web-ux-reviewer` (visual/UX/accessibility), `marketing-tone-rule` (tone + phrasing), `fact-checker` (claim veracity). The steward checks a claim is *stated consistently and currently*, never whether it is *well-phrased* or *true*. |
| **Communication** | Receives handoff/decision-record from governor-reviewer; handoff/note from planner-architect (release-freeze trigger); note from coder-tester (normative-change context). Sends review/handoff to governor-reviewer; note to planner-architect; handoff to documentation-writer (non-mechanical fixes, when active). |

## Decomposition

| Step | Deliverable | Gate |
|---|---|---|
| **R-1** | `documentation-estate-steward.yaml` + MANIFEST entry (this plan) | governor Gate 1 decision merges the role into the manifest |
| **R-2** | v2.11 parent-scope amendment: PMM IN row + estate-assessment release-cut precondition | **separate** governor Gate 1 (branch `plan/v211-scope-pmm-precondition`) |
| **R-3** | Fold "estate assessment precedes freezing the metadata allowlist + CHANGELOG scope" into the v2.11 release-publication child plan (MR !2646 / its plan) | metadata edit + governor confirmation, same review cycle |
| **R-4** | The steward's **first estate-wide sweep** and its findings digest with per-finding disposition | the role's first act once active; satisfies the v2.11 precondition |
| **R-5** | capability-developer contract review (`et3p`) folded in; any boundary refinement is a fast-follow amendment to the YAML | non-author review → governor |

R-1 and R-2 are the Gate 1 items now. R-3 follows R-2. R-4 is execution
once the role is active. R-5 is the outstanding non-author review.

## Risks and controls

- **Overlap with adjacent roles blurs in practice.** Control: the YAML's
  `forbidden_actions` name each seam concretely ("checks a claim is
  *stated consistently*, not that it is *phrased well* / *true*");
  contribution-auditor confirms at Gate 1; R-5 is capability-developer's
  dedicated boundary review.
- **"Mechanical currency correction" stretched into authoring or
  normative change.** Control: forbidden_actions + escalation_triggers
  route anything non-mechanical or normative out; InfoSec confirms at
  Gate 1.
- **The blocking `release-content-freeze` sweep stalls a release.**
  Control: cadence class states the escalation (freeze-vs-timeline
  conflict → governor); the v2.11 amendment (R-2) is where the governor
  accepts that coupling explicitly.
- **A standing role's cold-start context cost.** Control: the YAML is
  one file; the role activates event-driven, not "always"; the sweep
  report structure is bounded.

## Rollback

Revert this plan's MR: the role definition file is removed and the
MANIFEST entry with it. No other artifact changes. The gap returns to
its pre-plan state (partial coverage via rule-projection screening +
allowlist review).

## Rule-projection screen

`not applicable`. This plan adds an agent definition and a manifest entry;
it introduces or changes **no** normative MUST/SHOULD/blocking statement
in an adopter-facing Pages page or runbook. The role's *own* future
sweeps operate the projection model; they do not extend it. If R-2's
parent-scope amendment or R-3's release-plan edit turns out to add
adopter-facing normative text, that surfaces there with its own manifest.

## Test and acceptance strategy

- `agov validate agents` passes with the new file (done on this branch).
- `agov validate` of the manifest passes.
- Contribution-auditor + InfoSec + capability-developer advisory rows
  dispositioned at Gate 1 (capability-developer's full contract review is
  R-5, an input, per governor direction).
- Acceptance = a governor Gate 1 decision record merging R-1, plus the
  separate governor Gate 1 for R-2.

## Related

- `docs/plans/20260909T021841Z-v211-release-scope.md` — R-2 amends it.
- `docs/plans/20260909T203636Z-v211-release-publication.md` — R-3 folds
  the precondition into it.
- `agf/rules/workflow/documentation-rule-projection-rule.md` — the
  per-change model the steward's estate-wide sweep is the superset of.
- `agf/rules/docs/marketing-tone-rule.md` (message integrity),
  `agf/rules/docs/implementation-honesty-rule.md` (maturity levels),
  `agf/rules/always/verifiable-claims-rule.md` — the rules whose
  estate-wide *consistency and currency* the steward checks (it does not
  own or edit them).
