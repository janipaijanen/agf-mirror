---
kind: decision-record
from: governor-reviewer
to: coder-tester
id: 20260803T094702Z-governor-reviewer-3q28
timestamp: 2026-08-03T09:47:02Z
model: codex-gpt-5
immutable: true
references-to:
  - 20260803T082149Z-coder-tester-atm1.md
  - 20260803T084017Z-contribution-auditor-mmde.md
  - 20260803T084747Z-governor-reviewer-pkqk.md
  - 20260803T085310Z-planner-architect-jpj6.md
  - 20260803T085945Z-contribution-auditor-smhs.md
  - 20260803T090605Z-planner-architect-7hbs.md
topic: mr995-learning-supersession-gate1-approved-path-b
requires-ack: true
artifacts_owned: []
---

# Gate 1 APPROVED: MR !995, Path B

The human governor explicitly approved MR !995 Path B in the active session
on 2026-08-03. This record operationalizes that binding decision; Codex does
not independently supply governor authority.

## Binding decision

**Gate 1 APPROVED** for MR !995 at reviewed exact head
`55bdd4d5caaca5623e8c3ca57d59a30913c64f62`, selecting **Path B —
deprecate, do not remove `superseded_by`**.

The approved implementation contract is:

1. Add optional `status` with exactly `proposed | implemented`; do not add a
   third `superseded` status and do not rewrite the existing status-less
   record.
2. Add optional, single-cardinality `supersedes` on the new/superseding entry,
   referencing a stable learning ID in the same file.
3. Enforce that `supersedes` references an existing earlier-positioned entry;
   include the plan's positive and negative tests for unknown, self, and
   later-position references.
4. Derive reverse supersession relationships at read time; do not store or
   maintain a bidirectional backlink.
5. Keep `superseded_by` schema-legal, mark it deprecated, stop documenting or
   enforcing it as the current mechanism, and make no Path A removal in this
   implementation.
6. Treat the change as Additive/PREVENTIVE under the plan's Path B SemVer
   classification. Retain the explicitly disclosed deferred risk that the
   learning schema has no per-record schema-version dispatch.

Any later removal of `superseded_by` requires a separate, evidence-backed
compatibility/deprecation decision. Neither zero in-repository usage nor
`paying_customers: false` proves zero external dependency on this released
public schema.

## Evidence and correction chain

- Contribution-auditor review `mmde` is CLEAR at the approved exact head and
  independently recommends Path B.
- Planner-architect's original recommendation `jpj6` also selected Path B but
  cited the generic rule-template path incorrectly.
- `smhs` correctly blocked use of that citation until correction.
- `7hbs` is the citable corrected consultation: the project-specific Phase 0
  language lives in `.claude/rules/workflow/deprecation-policy-rule.md`, while
  uncertainty about external field users remains. The decision cites `7hbs`,
  not the superseded path rationale in `jpj6`.

## Next authorized action

Before implementation begins, update MR !995's plan metadata to `approved`,
set `gate1_approved_by: governor-reviewer` and
`gate1_approved_at: 2026-08-03T09:47:02Z`, record this decision ID, and update
the prose/checklist from pending review to the selected Path B decision.
Return that bounded metadata-only head with an exact-head green pipeline for
governor verification and merge of the approved plan. Implementation may then
proceed strictly within the approved Path B scope; Gate 2 remains separate.
