---
id: necessary-scope-expansion-minimalism-rule
status: pending-gate1
topic: necessary-scope-expansion-minimalism-rule
gate1_approved_by: ""
gate1_approved_at: ""
advisory_consultation:
  - role: contribution-auditor
    status: waived
    reason: >-
      Round 2 correction (per coder-tester exact-head review `srhr`,
      finding B-NSE-4): round 1's claim that a contribution-auditor
      review had already been requested "alongside this plan" was false
      — no such handoff existed. A genuine contribution-auditor review
      request is now filed alongside this correction
      (`20260826T055026Z-planner-architect-nseq`).
      `status` stays `waived` only because the schema's enum has no
      `pending`/`requested` value and no response has returned yet; this
      is a provisional filing-time placeholder, not a final Gate 1
      disposition. Gate 1 approval must not proceed while this still
      reads `waived` for single-agent unavailability — the response must
      either flip this to `consulted` with the response cited, or the
      governor must issue a binding waiver decision-record cited here in
      its place. The main risk this review should verify: that
      "necessary correction" is bounded tightly enough that it cannot be
      read as a general self-approval path for scope growth, consistent
      with the governor handoff (`trms`) that commissioned this plan
      explicitly warning against exactly that reading.
  - role: infosec-sentinel
    status: not-applicable
    reason: >-
      This plan edits governance rule prose only. It grants no new tool,
      merge, or approval authority to any agent — every "necessary
      correction" still requires a governor (or authorized deputy)
      decision-record before the correction is applied, per
      `deputy-governor-rule.md`'s red list. No credential, network-facing
      surface, or authentication/authorization change is introduced.
---
<!-- SPDX-License-Identifier: CC-BY-SA-4.0 OR LicenseRef-AgentsGovern-Commercial -->
<!-- Copyright (c) 2026 Jani Päijänen -->

# Plan: necessary scope-expansion minimalism rule

## Intent

Add one new subsection to `agf/rules/workflow/simplicity-rule.md`'s
"Minimum sufficient value" section (inserted by the prior approved plan
`docs/plans/20260824T205737Z-minimum-sufficient-value-rule.md`) that
gives agents a named, bounded path for exactly one recurring situation:
an approved implementation scope proves impossible to complete correctly
because an existing mandatory invariant, test, or required integration
surface was missing from the original file-list estimate.

Commissioned directly by governor handoff
`20260825T045831Z-governor-reviewer-trms`, itself triggered by tonight's
concrete incident: coder-tester's implementation of the approved
review-convergence plan could not pass the pre-existing, mandatory
`roi_redaction_audit_matches_real_clap_surface` test without also
editing `tooling/rust/crates/agov-core/src/roi.rs` — one file outside
the plan's named list, needed only to register the new `convergence`
command token and `--series` flag in the exhaustive CLI redaction
inventory. Coder-tester correctly stopped and asked
(`20260825T044258Z-coder-tester-5djr`) rather than either silently
expanding scope or blocking on an unreachably narrow file list; governor
approved the exact minimum correction
(`20260825T045829Z-governor-reviewer-mtms`) before it was applied. This
plan turns that ad hoc but correct judgment call into a named,
repeatable rule so the next agent in the same situation does not have to
reason it out from first principles under time pressure.

This is a Gate 1 plan only. It does not itself amend the rule file,
approve any gate, or authorize implementation.

## Affected files

- `agf/rules/workflow/simplicity-rule.md` — one new subsection inserted
  immediately after clause 4 ("Planning before work") of the existing
  "Minimum sufficient value" section and before `## The Simplicity
  Test`. No other file changes. Per the commissioning handoff's
  `do_not_touch` list, this plan does not touch the current
  review-convergence implementation branch or
  `docs/plans/20260824T201645Z-review-convergence-abstraction-reset.md`.

**Implementation dependency (per coder-tester exact-head review `srhr`,
finding B-NSE-3):** the "Minimum sufficient value" section this plan
inserts into is not yet on `main` — it exists only on the still-open MR
!2069 (`rule/minimum-sufficient-value`, exact head `29f083f7`, pipeline
`2790700363` green). This plan's insertion point does not exist until
!2069 merges. Implementation of this plan MUST NOT begin until !2069
merges to `main`; once it has, this plan's diff must be refreshed
against the resulting `main` and the exact insertion point and
insertion-only diff rechecked before Gate 2. If !2069 changes
materially before merging, or is declined, this plan returns for impact
reassessment.

Per this project's own `documentation-rule-projection-rule.md`, the
normative owner is this rule file alone; no adopter-facing Pages page,
runbook, or AI-loader references this level of internal agent
self-review process today (the sibling minimum-sufficient-value plan
found the same for its four clauses), so no other surface requires an
update.

## Migration needed

No. Prose guidance only; no schema, code, or data change.

## Proposed rule text

Insert after clause 4 ("Planning before work") and its closing
paragraph, before `## The Simplicity Test`:

```markdown
### Necessary scope-estimate correction

An approved plan's file list is an impact estimate and a control
boundary — not a promise that no other file can ever be touched. This
path applies only to work already governed by an approved, full Gate 1
plan whose own file list is the boundary in question. It does not apply
to either of this rule's existing fast paths: the fast-track waiver
above (which already requires stopping and writing a full plan the
moment work grows beyond its lighter process) or the sources/radar
research-maintenance fast path (whose own disqualifier list already
requires stopping before committing expanded scope and returning to
ordinary Gate 1). Neither fast path gains a new self-approval route
from this subsection — their existing stop-and-return-to-Gate-1
conditions are unchanged. When work already in progress under an
approved full Gate 1 plan discovers that the approved outcome cannot be
completed correctly without touching a file outside that list, classify
the discovery before acting:

1. **Necessary correction.** An existing invariant, mandatory test, or
   required integration surface proves — with objective, checkable
   evidence (a failing test, a violated invariant, an audit that
   demonstrably cannot pass) — that the approved outcome is incomplete
   without the additional file. State the intent and the evidence,
   request the smallest necessary expansion through the authority the
   governing gate still reserves, and apply only the minimum change
   once that authority responds. Do not wait for a full new Gate 1
   planning cycle or a retrospective to make the correction — the
   request itself is the lightweight artifact; a single scope-
   clarification handoff, not a new plan file, is normally sufficient.
   An agent may not label its own desired change "necessary" and skip
   the request: the classification requires evidence a reviewer can
   independently check, and applying the correction still requires a
   decision-record from the authority the plan already names, never
   the same agent's own say-so.
2. **Optional expansion.** Cleanup, redesign, convenience refactoring,
   or added capability that is not required to complete the approved
   outcome remains out of scope regardless of how adjacent or
   beneficial it looks. It follows normal planning — a fresh plan, or
   an explicit amendment to the current one — not this path.
3. **Material expansion.** Any change to the approved outcome,
   interfaces, architecture, dependencies, authority, security or
   privacy posture, cost, release scope, or other governed commitment
   is never a "necessary correction," no matter how small the diff.
   Material expansion stops work and returns to full Gate 1 review.
   This path grants no exception to `deputy-governor-rule.md`'s red
   list — a necessary correction that would touch a red-list area is
   automatically material and stops here.
4. **Learning after stabilization.** Once the correction is applied and
   validated, record — proportionate to the miss, not ceremonially —
   what the original impact estimate missed, where the gap was caught,
   and one concrete improvement to future impact assessment. This
   record follows the correction; it must never block it. A miss that
   recurs across more than one plan should be promoted to the project's
   governed learning channel rather than re-documented locally each
   time.

The default is deny: absent objective evidence and a decision-record
from the authority the current gate already reserves, an out-of-list
file stays out of scope. This subsection narrows how small an approved
scope may still grow without a new Gate 1 cycle — it does not widen who
may approve that growth.
```

`## The Simplicity Test` and every existing section after it are
unchanged verbatim.

## Acceptance matrix (observable checks, human/agent review — no new linter)

Consistent with the sibling minimum-sufficient-value plan, enforcement
stays in human/agent review; no validator or schema change is
introduced.

| Check | Reviewable question | Fails when |
|---|---|---|
| Classification | Is the out-of-list change classified as necessary correction, optional expansion, or material expansion, with the classification stated before the change is made? | No classification is stated, or the classification is asserted without the evidence category below |
| Evidence (necessary correction only) | Does the request cite objective, independently checkable evidence (a failing test, a violated invariant, a demonstrable audit failure) that the approved outcome cannot be complete without this file? | The justification is convenience, style, or unverified assertion rather than a checkable failure |
| Authority | Was a decision-record from the authority the governing gate already reserves obtained before the correction was applied? | The correction was applied before that decision-record exists, or the same agent that requested it also approved it |
| Red-list boundary | Does the correction avoid every area `deputy-governor-rule.md`'s red list names? | The correction touches a red-list area and was not escalated as material expansion instead |
| Learning | Was a proportionate post-correction learning note recorded after stabilization, without having blocked the correction itself? | No learning note follows, or the correction was delayed waiting for the note to be written first |

## Downstream impact classification (`downstream-impact-rule.md`)

This plan touches `agf/rules/**/*.md` — a trigger path.

```
Impact:
  Foundry:          BREAKING (per coder-tester exact-head review `srhr`,
                     finding B-NSE-2). Formalizes a judgment call this
                     session already made correctly once (coder-tester's
                     `5djr` request, governor's `mtms` approval), but
                     the formalization itself newly requires two
                     actions that were not previously mandatory for
                     every future occurrence of this situation: an
                     authority decision-record before the correction is
                     applied, and a proportionate post-stabilization
                     learning record. "No authority is weakened" does
                     not make these new required actions additive — the
                     approved sibling minimum-sufficient-value plan
                     classified its own new review obligations the same
                     way, for the same reason.
  Direct adopters:  BREAKING for any adopter using this rule template
                     as shipped: agents following the template gain a
                     newly-defined, narrower decision procedure the
                     moment an out-of-list file is discovered mid-work,
                     replacing whatever ad hoc judgment call they used
                     before. ADDITIVE for an adopter who has already
                     customized this rule file locally and does not
                     pull the upstream update.
  Downstream:       No adopter-produced artifact shape changes. No
                     schema, CLI, or validator surface changes. No
                     change to who holds approval authority — see
                     `deputy-governor-rule.md` cross-reference in the
                     rule text itself.
  Migration:        None required — no data, schema, or artifact needs
                     updating. Adopters who pull this rule update must,
                     going forward: (1) distinguish the three expansion
                     classes (necessary correction / optional expansion
                     / material expansion) the first time an out-of-list
                     file is discovered mid-work; (2) continue routing
                     the necessary-correction request through the same
                     authority the governing gate already reserves —
                     no new authority is granted; and (3) record a
                     proportionate post-stabilization learning note
                     after each necessary correction, escalating a
                     recurring miss to the project's governed learning
                     channel. No grace period is needed — this is
                     process guidance, not a code or schema contract.
  Evidence:         `20260825T044258Z-coder-tester-5djr.md` (the
                     request), `20260825T045829Z-governor-reviewer-mtms.md`
                     (the approval actually granted), and
                     `20260825T045831Z-governor-reviewer-trms.md` (the
                     commissioning handoff naming this exact plan).
```

**Adopter docs parity: not applicable**, for the same reason as the
sibling minimum-sufficient-value plan — this governs agent/reviewer
self-review during ordinary work, with no separate adopter-facing
surface describing it. No Pages, runbook, or AI-loader update is
required under `documentation-rule-projection-rule.md`.

## Test strategy

No automated test applies — prose guidance with no executable surface.

1. `agov plan validate` on this plan file.
2. Manual read-through confirming the inserted subsection renders
   correctly and every existing line below it is byte-identical
   (`git diff` shows only an insertion in the implementation commit).
3. One consultation round from contribution-auditor (genuine request
   filed alongside this correction — see frontmatter) before Gate 1
   approval is requested.

## Risks

- **Risk:** "necessary correction" could be stretched to cover
  changes that are really convenient rather than required.
  **Mitigation:** the rule text requires objective, independently
  checkable evidence and an explicit statement that an agent may not
  self-label its own desired change necessary; the acceptance matrix's
  evidence row is the reviewable check.
- **Risk:** the faster request path could be read as loosening
  `deputy-governor-rule.md`'s red list. **Mitigation:** the rule text
  states explicitly, twice (in the "Material expansion" clause and the
  closing paragraph), that it narrows scope growth without widening who
  may approve it, and that a red-list-touching correction is
  automatically material.
- **Risk:** "learning after stabilization, never blocking" could be
  read as license to skip the learning entirely once the correction
  ships. **Mitigation:** clause 4 requires the record and gives it a
  proportionality bound, not an optionality bound; a recurring miss
  must still reach the governed learning channel.

## Rollback

Revert the single inserted subsection; every other line in the file is
untouched, so a revert is a clean one-commit operation with no residual
state, schema, or data to unwind.

## Revision history

**Round 2** (this correction), addressing coder-tester exact-head review
`20260826T052933Z-coder-tester-srhr` at prior exact head `02226296`, four
blocking findings, all resolved in this one consolidated revision:

- `B-NSE-1`: added explicit exclusion of both the fast-track waiver and
  the sources/radar fast path from the new subsection's scope, and
  restricted it to work already governed by an approved full Gate 1
  plan.
- `B-NSE-2`: reclassified Foundry/direct-adopter downstream impact from
  ADDITIVE to BREAKING and added the operational migration note.
- `B-NSE-3`: named MR !2069 as an implementation dependency in
  "Affected files" — this plan's insertion point does not exist on
  `main` until !2069 merges.
- `B-NSE-4`: corrected the false round-1 claim that a contribution-
  auditor handoff had already been filed alongside the plan; a genuine
  request is filed alongside this correction, and the frontmatter now
  states plainly that `waived` is a provisional placeholder pending that
  response, not a final disposition.
