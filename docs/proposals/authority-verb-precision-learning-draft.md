# Draft Learning Entry: Authority Verb Precision

Status: draft for governor-reviewer / planner-architect review.

This is a proposed entry for `agents-govern/learnings/foundry.yaml`. It has not
been appended to the learning record yet.

```yaml
  - id: L012
    date: "2026-06-13"
    category: gap
    agents_involved: [coder-tester, contribution-auditor, governor-reviewer]
    summary: >
      Informal approval language can blur advisory and binding governance
      authority.
    detail: >
      During the AGF v2.0 Layout S1 flow, Codex summarized the next state as
      "if contribution-auditor approves" even though contribution-auditor has
      advisory review authority, not binding Gate 2 approval authority. The
      underlying coordination records routed the work correctly, but the
      summary used ordinary engineering shorthand where "audit approved it"
      means "audit review passed." In AGF, that shorthand is unsafe: approval
      is role-authority-specific.

      The correction was recorded at seq 218: contribution-auditor reviews and
      recommends; governor-reviewer makes the binding Gate 2 approval and
      merge decision. This is a language precision failure, not a technical
      validator failure, but repeated imprecision could confuse state,
      authority, and accountability.
    framework_impact: medium
    status: proposed
    recommendation: >
      AGF coordination summaries and handoff prose should use authority verbs
      precisely. Advisory roles "review", "find", "recommend", or "block with
      findings" when their role definition allows it. Binding decision roles
      "approve", "reject", "waive", "merge", or "reopen" according to their
      authority. Documentation and tooling should prefer labels such as
      "recommend-ready-for-gate2" over "approved" for advisory outcomes.
```

## Notes

- This learning complements the seq 218 authority clarification.
- Future lifecycle tooling should avoid rendering advisory recommendations as
  approvals in queue summaries.
- Reopening remains a binding/high-friction action and requires explicit
  reasoning per seq 214.
