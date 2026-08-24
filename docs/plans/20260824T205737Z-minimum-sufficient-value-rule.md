---
id: minimum-sufficient-value-rule
status: pending-gate1
topic: minimum-sufficient-value-rule
advisory_consultation:
  - role: contribution-auditor
    status: consulted
    reason: >-
      This plan implements the exact scope handed off in
      `20260824T204903Z-contribution-auditor-8prd.md` and its addendum
      `20260824T205102Z-contribution-auditor-mbs6.md` — a single minimal
      Gate 1 plan consolidating three governor-supplied simplicity
      directives (communication, implementation, delivery) plus a fourth
      (planning-before-work) into `agf/rules/workflow/simplicity-rule.md`,
      as one rule edit with one acceptance matrix. Per that handoff's own
      instruction, a fresh review of this exact plan is requested next as
      the single consultation round — either CLEAR or one consolidated
      revision with a material reason, not phrase-by-phrase rounds.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      This plan edits governance rule prose only. It adds no credential,
      network-facing surface, external dependency, or change to
      authentication, authorization, secrets handling, or tenant
      isolation.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: minimum-sufficient-value rule (communication, implementation, delivery, planning-before-work)

## Intent

Consolidate four governor-supplied simplicity directives into
`agf/rules/workflow/simplicity-rule.md` as one new section, per handoff
`8prd`/addendum `mbs6`. The directives elevate existing, mostly-implicit
simplicity preferences (say things simply, build simply, deliver simply,
plan before starting) to explicit MUST-level guidance, without inventing
a new rule file, schema, command, or validator.

Motivated directly by this session's own evidence: the eighteen-hour MR
!1847 convergence failure (`20260824T203312Z-contribution-auditor-61gp.md`)
included a case-by-case patching loop that violated clause 2
(implementation), and a subsequent over-scoped CI-policy MR opened from a
delivery-routing handoff that violated clause 3 (delivery) — both
independently found and corrected within this same session
(`20260824T202150Z-contribution-auditor-5g74.md`,
`20260824T204000Z-planner-architect-gseh.md`).

## Affected files

- `agf/rules/workflow/simplicity-rule.md` — one new section inserted
  after the existing intro paragraph and before "## The Simplicity
  Test". No other file changes; explicitly not touching
  `docs/plans/20260824T195950Z-review-convergence-abstraction-reset.md`
  or `agf/learnings/foundry.yaml`, per `8prd`'s `do_not_touch` list.

## Migration needed

No. This is prose guidance, not a schema, code, or data change — no
migration path applies. The rule takes effect for review/work starting
after Gate 2, per the existing calibration table this plan's own
clause 4 references.

## Proposed rule text

Insert immediately after the existing intro paragraph ("Complexity is
the primary source of bugs... Simplicity does not.") and before
"## The Simplicity Test":

```markdown
## Minimum sufficient value

Plan first, then say, build, and deliver the smallest complete solution
that meets the approved requirements and quality standard. Remove
unnecessary wording, scope, states, abstractions, options, process, and
delay. Do not remove meaning, correctness, safety, evidence,
maintainability, uncertainty, or required user value.

1. **Communication:** Use the shortest clear wording that preserves
   necessary meaning. Remove jargon, repetition, and decorative detail.
   Do not omit material facts, constraints, uncertainty, or required
   actions.
2. **Implementation:** Implement the simplest solution that fully meets
   the approved requirements. Remove unnecessary states, abstractions,
   options, and special cases. Do not simplify away correctness,
   safety, evidence, or required behavior. (See "The over-engineering
   warning signs" below for concrete signals.)
3. **Delivery:** Deliver the smallest complete solution that meets the
   approved quality requirements. Remove unnecessary scope, process,
   abstraction, and delay. Do not remove correctness, safety, evidence,
   maintainability, or required user value.
4. **Planning before work:** Before starting new work, confirm that a
   plan proportional to its scope and risk already exists. If not,
   create and approve the required plan first. At minimum, the
   planning must state the intended outcome, scope and exclusions,
   acceptance checks, authority and gates, owner, material risks or
   dependencies, and the simplest sufficient approach. Small eligible
   work may use this rule's own fast-track waiver or a compact plan
   note (see "Calibrating workflow overhead to task size" below);
   planning must be adequate, not ceremonially large.
```

`## The Simplicity Test` and all subsequent existing sections
(`## In code`, `## In architecture`, `## In tests`, `## The
over-engineering warning signs`, `## Calibrating workflow overhead to
task size`, the fast-track waiver, and the sources/radar fast path) are
unchanged verbatim — the new section is additive prose placed before
them, and clauses 2 and 4 explicitly cross-reference the existing
material they reinforce rather than duplicating it.

## Acceptance matrix (observable checks, human/agent review — no new linter)

Per `8prd`'s explicit instruction, enforcement stays in human/agent
review; no validator, schema, or prose-quality linter is introduced.
Each clause gets one reviewable question, added to how
`systematic-code-review-rule.md`'s existing "Specification compliance"
self-check is already applied — no edit to that file, since the
questions are usable directly from this rule's own text:

| Clause | Reviewable question | Fails when |
|---|---|---|
| 1. Communication | Is this message/response/commit description the shortest wording that still states every material fact, constraint, uncertainty, and required action? | Padded with decorative detail or repetition, OR a material fact/constraint/uncertainty is missing to make it shorter |
| 2. Implementation | Does this change introduce any state, abstraction, option, or special case the approved requirement does not need? | Yes to that question, OR correctness/safety/evidence/required behavior was simplified away |
| 3. Delivery | Is the delivered scope the smallest complete unit meeting the approved quality bar, with no unnecessary process or delay layered on top? | Scope, process, or delay exceeds what the approved quality bar requires, OR correctness/safety/evidence/maintainability/required user value was cut to shrink it |
| 4. Planning before work | Does an adequate plan (full plan file, compact plan note, or fast-track waiver) already exist for this work's scope and risk, per the existing calibration table? | No such plan exists and one was not created and approved before work started |

These four questions are self-review prompts, the same enforcement
class as this rule's existing "over-engineering warning signs" list —
advisory-strength unless a reviewer or gate elevates a specific
instance to a BLOCK, consistent with how the rest of this file already
operates.

## Downstream impact classification (`downstream-impact-rule.md`)

This plan touches `agf/rules/**/*.md` — a trigger path.

```
Impact:
  Foundry:          BREAKING. Four previously-implicit-or-absent
                     preferences become explicit MUST-level review
                     questions. Every session operating under this
                     rule template (including this one) gains four new
                     self-review obligations effective immediately on
                     merge — no grace period, since this is process
                     guidance, not a code or schema contract requiring
                     migration time.
  Direct adopters:  BREAKING for any adopter using this rule template
                     as shipped, in the same sense: agents following
                     the template gain four new review questions.
                     ADDITIVE for an adopter who has already
                     customized this rule file locally and does not
                     pull the upstream update.
  Downstream:       No adopter-produced artifact shape changes. No
                     schema, CLI, or validator surface changes. The
                     only behavior change is in agent/reviewer
                     self-checks during ordinary work.
  Migration:        None required — no data, schema, or artifact needs
                     updating. Adopters who pull this rule update
                     should read the new section once; no other action.
  Evidence:         This session's own eighteen-hour MR !1847
                     convergence-loop learning
                     (`20260824T203312Z-contribution-auditor-61gp.md`)
                     and the immediately following delivery-authority
                     overreach on MR !2002
                     (`20260824T202150Z-contribution-auditor-5g74.md`),
                     both independently confirmed against actual
                     session history before this plan was drafted.
```

## Test strategy

No automated test applies — this is prose guidance with no executable
surface. Verification is:

1. `agov plan validate` on this plan file — schema/front-matter
   conformance.
2. Manual read-through confirming the inserted section renders
   correctly in Markdown and every existing section below it is
   byte-identical to before the edit (verified via `git diff` showing
   only an insertion, zero deletions, in the implementation commit).
3. One fresh consultation round on this exact plan (already in
   progress via `8prd`'s own instruction), returning CLEAR or one
   consolidated revision — not phrase-by-phrase rounds, per the
   handoff's explicit scope.

## Risks

- **Risk:** the four clauses could be read as license to omit material
  detail in the name of brevity. **Mitigation:** every clause pairs its
  "remove" instruction with an explicit "do not remove [meaning /
  correctness / safety / evidence / required value]" clause — this
  mirrors the exact wording the governor supplied and is preserved
  verbatim.
- **Risk:** clause 4 (planning-before-work) could be read as requiring
  a full plan file for every task, reintroducing the ceremony this same
  rule file's "Calibrating workflow overhead to task size" table
  already exists to prevent. **Mitigation:** clause 4 explicitly
  cross-references that table and this rule's own fast-track waiver,
  and says planning "must be adequate, not ceremonially large."

## Rollback

Revert the single inserted section; every other line in the file is
untouched, so a revert is a clean one-commit operation with no residual
state, schema, or data to unwind.
