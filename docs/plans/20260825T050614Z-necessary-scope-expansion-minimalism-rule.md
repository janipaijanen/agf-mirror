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
      Single-agent Gate 1 planning session; no contribution-auditor
      available to consult directly at plan-filing time. Review is
      explicitly requested via a coordination handoff filed alongside
      this plan, addressed to Codex (playing contribution-auditor). The
      main risk this review should verify: that "necessary correction"
      is bounded tightly enough that it cannot be read as a general
      self-approval path for scope growth, consistent with the governor
      handoff (`trms`) that commissioned this plan explicitly warning
      against exactly that reading.
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
boundary — not a promise that no other file can ever be touched. When
work already in progress under an approved plan discovers that the
approved outcome cannot be completed correctly without touching a file
outside that list, classify the discovery before acting:

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
  Foundry:          ADDITIVE. Formalizes a judgment call this session
                     already made correctly once (coder-tester's `5djr`
                     request, governor's `mtms` approval); does not
                     change any existing gate's authority, only the
                     shape and speed of the request for this one
                     narrow situation.
  Direct adopters:  ADDITIVE for any adopter using this rule template.
                     No existing obligation is removed or weakened; the
                     rule adds a named, faster path for a case that
                     previously had no explicit guidance and would
                     otherwise default to either an ad hoc judgment call
                     or an over-heavy full Gate 1 cycle for a one-file,
                     evidence-backed correction.
  Downstream:       No adopter-produced artifact shape changes. No
                     schema, CLI, or validator surface changes. No
                     change to who holds approval authority — see
                     `deputy-governor-rule.md` cross-reference in the
                     rule text itself.
  Migration:        None required — prose guidance only.
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
3. One consultation round from contribution-auditor (handoff filed
   alongside this plan) before Gate 1 approval is requested.

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
