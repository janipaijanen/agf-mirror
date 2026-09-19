---
id: kungfu-sources-radar
status: approved
topic: kungfu-sources-radar
gate1_approved_by: 20260816T174459Z-governor-reviewer-68ep
gate1_approved_at: 2026-08-16T17:44:59Z
advisory_consultation:
  - role: contribution-auditor
    status: not-applicable
    reason: >
      This is a narrow, reversible classification of public primary-source
      material and one non-binding radar question. It adopts no external
      workflow or authority; independent source fidelity remains Gate 2
      evidence.
  - role: infosec-sentinel
    status: not-applicable
    reason: >
      The change summarizes public documentation only and touches no secrets,
      personal records, executable integration, authentication boundary,
      schema, validator, or CI configuration.
---

# Plan: classify Kungfu and add its durable-work control-plane question to radar

## Decision and scope

The governor directly approved the proposed disposition: add Kungfu to
`docs/sources.md` now and add one focused, cross-referenced question to
`docs/radar.md`. The approval is transcribed in
`20260816T174459Z-governor-reviewer-68ep`.

This plan does not install or execute Kungfu, adopt its `.kungfu/` workspace,
create an integration issue, change AGF authority, or claim independent
validation of Kungfu's effectiveness.

## Planned changes

1. Add a dated source-registry batch pinned to default-branch commit
   `f7c01a2f084d2c5939353a24c4bb920c1045eda4`.
2. Classify Kungfu as tooling, framework, and practice; relation: prior art,
   competitor in coordination/work continuity, complement, and threat model;
   action: watch, compare in documentation, and evaluate.
3. Describe only source-supported mechanics: Project/Work/Attempt state,
   cross-agent continuation, evidence receipts, live-work ownership, and
   independently governed review and settlement.
4. Preserve evidence limits: the current product is alpha, its demonstrations
   and capability claims are project-authored, and AGF has not independently
   trialled it.
5. Add one radar item about an external durable Work/Attempt/settlement control
   plane alongside AGF's repository-native trail. Cross-reference the existing
   external-context-graph, renewable-work-lease, and cross-harness-digest items
   instead of duplicating their broad questions. Set `review-by: 2026-11`.

## Acceptance and validation

- Links point to primary material pinned to the reviewed commit where practical.
- The radar entry is explicitly non-binding and implies no adoption,
  partnership, or implementation commitment.
- No Kungfu Work owner, reviewer, or settlement state acquires AGF Gate 1,
  Gate 2, merge, release, or governor authority.
- The entry distinguishes documented mechanics from unverified effectiveness
  claims and labels AGF's evaluation maturity as Level 1 only.
- `git diff --check` and applicable documentation validation pass.
- Gate 2 verifies the wording against the pinned repository and its published
  alpha/qualification documentation.

Coder-tester owns the documentation edit. Governor-reviewer retains the
binding Gate 2 merge decision.
