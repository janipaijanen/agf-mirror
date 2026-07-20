# plan: governance amendment workflow + CAST incident record (issues #43, #46)

Branch: `plan/incident-amendment-governance`

## Summary

Two sequentially-dependent issues: #46 (incident → analysis) hands off
into #43 (analysis → rule amendment). Planned together so the handoff
point is a real, checkable interface rather than two independently
guessed shapes.

## What's in this branch

- **`docs/plans/governance-amendment-workflow.md`** (#43): reuses
  existing mechanisms instead of inventing new ones — the amendment
  artifact is the existing `agf/docs/adr/` ADR template; promotion
  (provisional→recommended→MUST) reuses the already-validator-enforced
  `learning.schema.json` `confidence.level` corroboration bar; sunset
  audits attach to the existing Sentinel review cadence. Flags an
  explicit conflict rather than silently resolving it: issue #43's
  prose describes a three-step ladder, but `rule.schema.json` actually
  has two orthogonal fields (`status`, `strength`) with no
  `recommended` value — proposes a mapping table and leaves the final
  call to the human reconciler.
- **`docs/plans/incident-workflow-cast.md`** (#46, depends on #43):
  extends `learning.schema.json` with an optional `incident` object
  (`control_structure`, `unsafe_control_actions`, `contributing_factors`
  — deliberately not a single root-cause field, per the CAST-vs-root-
  cause distinction the issue calls for) plus a `rule_amendment_ref`
  handoff field into #43's ADR mechanism.

## Reconciliation applied (see second commit)

Cross-cluster review
(`docs/coordination/20260720T080808Z-governor-reviewer-mrp3.md`,
Finding 1) settled on one canonical accountability field name,
`accountable_human`, across all clusters. This branch originally
assumed `proposed_by`/`approved_by`; the second commit replaces
`approved_by` with `accountable_human` and drops `proposed_by` in
favor of the coordination message's existing `from` field.

## Open decision for the governor

The `status`/`strength` mapping table (Finding 2 in the reconciliation
note) — confirm or override before implementation.

## Review checklist

- [x] Both plans validate against `agf/schemas/gate1-plan.schema.json`
- [ ] Status/strength mapping table reviewed and confirmed
- [ ] Incident-record schema addition reviewed against existing
      `learning.schema.json` consumers
- [ ] No `Co-authored-by` trailers in commit
